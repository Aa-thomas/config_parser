use crate::errors::parse::{PathSeg, ValuePath};
use crate::errors::value_type::TypeKind;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PathError {
    #[error("Empty path is not allowed")]
    EmptyPath,
    #[error("Not an object at {prefix}: cannot access key `{key}` on {found}")]
    NotAnObject {
        prefix: ValuePath,
        key: String,
        found: TypeKind,
    },
    #[error("Not an array at {prefix}: cannot access index [{index}] on {found}")]
    NotAnArray {
        prefix: ValuePath,
        index: usize,
        found: TypeKind,
    },
    #[error("Key not found at {prefix}: missing key `{key}`")]
    KeyNotFound { prefix: ValuePath, key: String },
    #[error("Index out of bounds at {prefix}: index {index} >= len {len}")]
    IndexOutOfBounds {
        prefix: ValuePath,
        index: usize,
        len: usize,
    },
    #[error("Invalid path segment at {prefix}: segment `{segment}` is not valid ({reason})")]
    InvalidSegment {
        prefix: ValuePath,
        segment: String,
        reason: String,
    },
    #[error("Unsupported path operation at {prefix}: {message}")]
    Unsupported { prefix: ValuePath, message: String },
}

impl PathError {
    pub fn not_object(prefix: ValuePath, key: impl Into<String>, found: TypeKind) -> Self {
        Self::NotAnObject {
            prefix,
            key: key.into(),
            found,
        }
    }
    pub fn not_array(prefix: ValuePath, index: usize, found: TypeKind) -> Self {
        Self::NotAnArray {
            prefix,
            index,
            found,
        }
    }
    pub fn key_not_found(prefix: ValuePath, key: impl Into<String>) -> Self {
        Self::KeyNotFound {
            prefix,
            key: key.into(),
        }
    }
    pub fn oob(prefix: ValuePath, index: usize, len: usize) -> Self {
        Self::IndexOutOfBounds { prefix, index, len }
    }
    pub fn invalid_seg(
        prefix: ValuePath,
        segment: impl Into<String>,
        reason: impl Into<String>,
    ) -> Self {
        Self::InvalidSegment {
            prefix,
            segment: segment.into(),
            reason: reason.into(),
        }
    }
    pub fn unsupported(prefix: ValuePath, message: impl Into<String>) -> Self {
        Self::Unsupported {
            prefix,
            message: message.into(),
        }
    }
}

pub type PathResult<T> = Result<T, PathError>;

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

#[derive(Debug)]
pub enum TomlAt<'a> {
    Item(&'a toml_edit::Item),
    Value(&'a toml_edit::Value),
    Table(&'a toml_edit::Table),
}

impl<'a> TomlAt<'a> {
    pub fn as_value(&self) -> Option<&'a toml_edit::Value> {
        match self {
            TomlAt::Value(v) => Some(v),
            TomlAt::Item(item) => item.as_value(),
            TomlAt::Table(_) => None,
        }
    }

    fn type_kind(&self) -> TypeKind {
        match self {
            TomlAt::Item(item) => TypeKind::from_toml_item(item),
            TomlAt::Value(val) => TypeKind::from_toml_value(val),
            TomlAt::Table(_) => {
                TypeKind::from_toml_item(&toml_edit::Item::Table(toml_edit::Table::new()))
            }
        }
    }
}

impl<'a> std::fmt::Display for TomlAt<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TomlAt::Item(item) => write!(f, "{}", item),
            TomlAt::Value(val) => write!(f, "{}", val),
            TomlAt::Table(tbl) => write!(f, "{}", tbl),
        }
    }
}

enum TomlCursor<'a> {
    Item(&'a toml_edit::Item),
    Value(&'a toml_edit::Value),
    Table(&'a toml_edit::Table),
}

impl<'a> TomlCursor<'a> {
    fn type_kind(&self) -> TypeKind {
        match self {
            TomlCursor::Item(item) => TypeKind::from_toml_item(item),
            TomlCursor::Value(val) => TypeKind::from_toml_value(val),
            TomlCursor::Table(_) => {
                TypeKind::from_toml_item(&toml_edit::Item::Table(toml_edit::Table::new()))
            }
        }
    }
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

pub fn suggest<'a>(needle: &str, hay: impl IntoIterator<Item = &'a str>) -> Option<String> {
    let mut best: Option<(usize, &str)> = None;

    for h in hay {
        let len_diff = needle.len().abs_diff(h.len());
        let common_prefix = needle
            .chars()
            .zip(h.chars())
            .take_while(|(a, b)| a == b)
            .count();

        let score = len_diff.saturating_sub(common_prefix);

        if best.map_or(true, |(best_score, _)| score < best_score) {
            best = Some((score, h));
        }
    }

    best.map(|(_, h)| h.to_string())
}
