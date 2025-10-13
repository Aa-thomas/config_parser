use thiserror::Error;

use crate::errors::parse::{PathSeg, ValuePath};
use crate::errors::value_type::TypeKind;
#[derive(Debug, Error)]
pub enum PathError {
    #[error("Empty path is not allowed")]
    EmptyPath,

    #[error("Not an object at {prefix}: cannot access key `{key}` on {found}")]
    NotAnObject {
        prefix: ValuePath, // path successfully resolved up to this prefix
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
        segment: String, // render of the problematic segment (e.g., "[−1]" or ".\0")
        reason: String,  // short reason to help the user
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

// #[cfg(feature = "with-serde-json")]
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
                    return Err(PathError::not_object(
                        prefix.clone(),
                        k,
                        TypeKind::from_json(cur),
                    ));
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
                    return Err(PathError::not_array(
                        prefix.clone(),
                        *i,
                        TypeKind::from_json(cur),
                    ));
                }
            }
        }
    }

    Ok(cur)
}

pub fn get_toml_at_path<'a>(
    root: &'a toml_edit::Value,
    path: &ValuePath,
) -> PathResult<&'a toml_edit::Value> {
    use toml_edit::Value;

    if path.is_empty() {
        return Err(PathError::EmptyPath);
    }

    let mut cur = root;
    let mut prefix = ValuePath::default();

    for seg in path.0 {
        match seg {
            PathSeg::Key(k) => {}
            PathSeg::Index(i) => {}
        }
    }
}

// #[cfg(feature = "suggest")]
pub fn suggest<'a>(needle: &str, hay: impl IntoIterator<Item = &'a str>) -> Option<String> {
    // very light levenshtein-like heuristic (length diff + common prefix)
    let mut best: Option<(usize, &str)> = None;
    for h in hay {
        let d = needle.len().abs_diff(h.len())
            + needle
                .chars()
                .zip(h.chars())
                .take_while(|(a, b)| a == b)
                .count();
        if best.map_or(true, |(bd, _)| d < bd) {
            best = Some((d, h));
        }
    }
    best.map(|(_, h)| h.to_string())
}
