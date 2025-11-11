use std::path::PathBuf;

use anyhow::Error;

use crate::shared::{
    errors::PathError,
    types::{PathResult, PathSeg, TomlAt, TomlCursor, TypeKind, ValidatedPath, ValuePath},
};

pub fn create_value_path(validated_path: &ValidatedPath) -> ValuePath {
    let mut output_path = ValuePath::new();
    let chars: Vec<char> = validated_path.as_str().chars().collect();
    let mut temp = String::new();

    enum State {
        Default,
        InBracket,
        InQuotes(char),
    }

    let mut state = State::Default;

    fn push_key(out: &mut ValuePath, buf: &mut String) {
        if !buf.is_empty() {
            out.push_key(std::mem::take(buf));
        }
    }

    for char in chars {
        match state {
            State::Default => match char {
                '.' => {
                    push_key(&mut output_path, &mut temp);
                }
                '[' => {
                    push_key(&mut output_path, &mut temp);
                    state = State::InBracket;
                }
                _ => temp.push(char),
            },
            State::InBracket => match char {
                '0'..='9' => temp.push(char),
                ']' => {
                    output_path
                        .push_index(temp.parse().expect("validator bug: non-digit in index"));
                    temp.clear();
                    state = State::Default;
                }
                '"' => {
                    state = State::InQuotes(char);
                }
                _ => unreachable!("validated input should not hit this branch"),
            },
            State::InQuotes(q_mark) => match char {
                char if char == q_mark => {
                    push_key(&mut output_path, &mut temp);
                    state = State::InBracket;
                }
                _ => temp.push(char),
            },
        }
    }

    output_path
}

pub fn get_json_at_path<'a>(
    document: &'a serde_json::Value,
    path: &ValuePath,
) -> PathResult<&'a serde_json::Value> {
    use serde_json::Value;

    if path.is_empty() {
        return Err(PathError::EmptyPath);
    }

    let mut cur = document;
    let mut prefix = ValuePath::default();

    for seg in &path.0 {
        match seg {
            PathSeg::Key(k) => {
                if let Value::Object(map) = cur {
                    cur = map
                        .get(k)
                        .ok_or_else(|| PathError::key_not_found(prefix.clone(), k))?;
                    prefix.push_key(k.clone());
                } else {
                    return Err(PathError::not_object(prefix, k, TypeKind::from_json(cur)));
                }
            }
            PathSeg::Index(i) => {
                if let Value::Array(arr) = cur {
                    let len = arr.len();
                    cur = arr
                        .get(*i)
                        .ok_or_else(|| PathError::oob(prefix.clone(), *i, len))?;
                    prefix.push_index(*i);
                } else {
                    return Err(PathError::not_array(prefix, *i, TypeKind::from_json(cur)));
                }
            }
        }
    }

    Ok(cur)
}

pub fn get_toml_at_path<'a>(
    document: &'a toml_edit::Item,
    path: &ValuePath,
) -> PathResult<TomlAt<'a>> {
    use toml_edit::{Item, Value};

    if path.is_empty() {
        return Err(PathError::EmptyPath);
    }

    let mut cursor = TomlCursor::Item(document);
    let mut prefix = ValuePath::default();

    for seg in &path.0 {
        match seg {
            PathSeg::Key(k) => {
                cursor = match cursor {
                    TomlCursor::Item(Item::Table(tbl)) => {
                        let next = tbl
                            .get(k)
                            .ok_or_else(|| PathError::key_not_found(prefix.clone(), k))?;
                        prefix.push_key(k.clone());
                        TomlCursor::Item(next)
                    }
                    TomlCursor::Item(Item::Value(val)) => {
                        if let Value::InlineTable(itbl) = val {
                            let next = itbl
                                .get(k)
                                .ok_or_else(|| PathError::key_not_found(prefix.clone(), k))?;
                            prefix.push_key(k.clone());
                            TomlCursor::Value(next)
                        } else {
                            return Err(PathError::not_object(
                                prefix,
                                k,
                                TypeKind::from_toml_value(val),
                            ));
                        }
                    }
                    TomlCursor::Value(val) => {
                        if let Value::InlineTable(itbl) = val {
                            let next = itbl
                                .get(k)
                                .ok_or_else(|| PathError::key_not_found(prefix.clone(), k))?;
                            prefix.push_key(k.clone());
                            TomlCursor::Value(next)
                        } else {
                            return Err(PathError::not_object(
                                prefix,
                                k,
                                TypeKind::from_toml_value(val),
                            ));
                        }
                    }
                    TomlCursor::Table(tbl) => {
                        let next = tbl
                            .get(k)
                            .ok_or_else(|| PathError::key_not_found(prefix.clone(), k))?;
                        prefix.push_key(k.clone());
                        TomlCursor::Item(next)
                    }
                    TomlCursor::Item(item) => {
                        return Err(PathError::not_object(
                            prefix,
                            k,
                            TypeKind::from_toml_item(item),
                        ));
                    }
                }
            }
            PathSeg::Index(i) => {
                cursor = match cursor {
                    TomlCursor::Item(Item::Value(val)) => {
                        if let Value::Array(arr) = val {
                            let len = arr.len();
                            let next = arr
                                .get(*i)
                                .ok_or_else(|| PathError::oob(prefix.clone(), *i, len))?;
                            prefix.push_index(*i);
                            TomlCursor::Value(next)
                        } else {
                            return Err(PathError::not_array(
                                prefix,
                                *i,
                                TypeKind::from_toml_value(val),
                            ));
                        }
                    }
                    TomlCursor::Value(val) => {
                        if let Value::Array(arr) = val {
                            let len = arr.len();
                            let next = arr
                                .get(*i)
                                .ok_or_else(|| PathError::oob(prefix.clone(), *i, len))?;
                            prefix.push_index(*i);
                            TomlCursor::Value(next)
                        } else {
                            return Err(PathError::not_array(
                                prefix,
                                *i,
                                TypeKind::from_toml_value(val),
                            ));
                        }
                    }
                    TomlCursor::Item(Item::ArrayOfTables(aot)) => {
                        let len = aot.len();
                        let tbl = aot
                            .get(*i)
                            .ok_or_else(|| PathError::oob(prefix.clone(), *i, len))?;
                        prefix.push_index(*i);
                        TomlCursor::Table(tbl)
                    }
                    TomlCursor::Item(item) => {
                        return Err(PathError::not_array(
                            prefix,
                            *i,
                            TypeKind::from_toml_item(item),
                        ));
                    }
                    TomlCursor::Table(_) => {
                        return Err(PathError::not_array(prefix, *i, cursor.type_kind()));
                    }
                }
            }
        }
    }

    Ok(match cursor {
        TomlCursor::Item(item) => TomlAt::Item(item),
        TomlCursor::Value(val) => TomlAt::Value(val),
        TomlCursor::Table(tbl) => TomlAt::Table(tbl),
    })
}
