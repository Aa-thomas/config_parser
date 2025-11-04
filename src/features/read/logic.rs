use crate::shared::{
    errors::PathError,
    types::{PathResult, PathSeg, TomlAt, TomlCursor, TypeKind, ValuePath},
};

pub fn get_json_at_path<'a>(
    root: &'a serde_json::Value,
    path: &ValuePath,
) -> PathResult<&'a serde_json::Value> {
    use serde_json::Value;

    if path.is_empty() {
        return Err(PathError::EmptyPath);
    }

    let mut cur = root;
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

pub fn get_toml_at_path<'a>(root: &'a toml_edit::Item, path: &ValuePath) -> PathResult<TomlAt<'a>> {
    use toml_edit::{Item, Value};

    if path.is_empty() {
        return Err(PathError::EmptyPath);
    }

    let mut cursor = TomlCursor::Item(root);
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
