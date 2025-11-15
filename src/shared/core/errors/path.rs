use core::fmt;

use thiserror::Error;

use crate::shared::core::{path::ValuePath, types::TypeKind};

#[derive(Error)]
pub enum PathError {
    #[error(
        "Empty path is not allowed. \
         Expected at least one segment (for example: `top` or `top.int`)."
    )]
    EmptyPath,

    #[error(
        "Type error at {prefix}: \
         tried to access key `{key}`, but the value at this path is {found}, not an object."
    )]
    NotAnObject {
        /// Path to the non-object value (e.g. `top.int`)
        prefix: ValuePath,
        /// The key that was attempted (e.g. `x`)
        key: String,
        /// The actual type found at `prefix` (e.g. `integer`, `array`)
        found: TypeKind,
    },

    #[error(
        "Type error at {prefix}: \
         tried to access index [{index}], but the value at this path is {found}, not an array."
    )]
    NotAnArray {
        /// Path to the non-array value
        prefix: ValuePath,
        /// Index that was attempted
        index: usize,
        /// The actual type found at `prefix`
        found: TypeKind,
    },

    #[error(
        "Type error at {prefix}: \
         attempted to list children, but the value at this path is a scalar {found}, \
         not an object or array."
    )]
    NotAContainer {
        /// Path to the scalar value
        prefix: ValuePath,
        /// The scalar type that was found (e.g. `string`, `integer`)
        found: TypeKind,
    },

    #[error(
        "Key not found at {prefix}: \
         there is no key named `{key}` at this location."
    )]
    KeyNotFound {
        /// Path to the parent container
        prefix: ValuePath,
        /// The missing key
        key: String,
    },

    #[error(
        "Index out of bounds at {prefix}: \
         tried to access index {index}, but valid indices are 0..{len}."
    )]
    IndexOutOfBounds {
        /// Path to the array value
        prefix: ValuePath,
        /// The index that was attempted
        index: usize,
        /// The length of the array at `prefix`
        len: usize,
    },

    #[error(
        "Invalid path segment at {prefix}: \
         segment `{segment}` is not valid here ({reason})."
    )]
    InvalidSegment {
        /// Path leading up to the invalid segment
        prefix: ValuePath,
        /// The raw segment that failed (e.g. between dots or brackets)
        segment: String,
        /// Human-readable reason why the segment is invalid
        reason: String,
    },

    #[error(
        "Invalid numeric index at `{prefix}`: \
         segment between '[' and ']' was `{raw}` (expected a non-negative integer)."
    )]
    InvalidIndex {
        /// Path leading up to the array index
        prefix: ValuePath,
        /// The raw substring inside the brackets, e.g. `abc`, `-1`, or empty
        raw: String,
    },

    #[error(
        "Unsupported path operation at {prefix}: \
         {message}"
    )]
    Unsupported {
        /// Path where the unsupported operation was attempted
        prefix: ValuePath,
        /// Additional detail about what was unsupported
        message: String,
    },
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

    pub fn invalid_index(prefix: ValuePath, raw: impl Into<String>) -> Self {
        Self::InvalidIndex {
            prefix,
            raw: raw.into(),
        }
    }

    pub fn unsupported(prefix: ValuePath, message: impl Into<String>) -> Self {
        Self::Unsupported {
            prefix,
            message: message.into(),
        }
    }
}

impl fmt::Debug for PathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self}")
    }
}
