use crate::shared::types::{TypeKind, ValuePath};
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
    #[error("Not a container at {prefix}: attempted to list children on a scalar {found}")]
    NotAContainer {
        prefix: ValuePath,
        found: TypeKind, // e.g., "String", "Integer", "Boolean"
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
