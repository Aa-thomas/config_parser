use crate::errors::parse::ValuePath;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TypeError {
    #[error("Type mismatch at {path}: expected {expected}, found {found}")]
    TypeMismatch {
        path: ValuePath,
        expected: TypeKind,
        found: TypeKind,
    },

    #[error("Number out of range at {path}: value {value} is not within {target} bounds{hint}")]
    NumberOutOfRange {
        path: ValuePath,
        target: &'static str, // e.g., "u16", "i32", "f32"
        value: String,        // keep as string to preserve original text
        hint: String,         // e.g., " (min=0, max=65535)"; can be "" if unknown
    },

    #[error("Invalid enum variant at {path}: found {found}; allowed: {allowed:?}")]
    InvalidEnumVariant {
        path: ValuePath,
        found: String,
        allowed: Vec<String>,
    },

    #[error("Invalid pattern at {path}: expected {expect_desc}, found {found}")]
    InvalidPattern {
        path: ValuePath,
        expect_desc: String, // e.g., "hostname", "regex=^[a-z0-9_-]+$"
        found: String,
    },

    #[error("Array element type mismatch at {path}[{index}]: expected {expected}, found {found}")]
    ArrayElementTypeMismatch {
        path: ValuePath,
        index: usize,
        expected: TypeKind,
        found: TypeKind,
    },

    #[error("Field type mismatch at {path}: field `{field}` expected {expected}, found {found}")]
    ObjectFieldTypeMismatch {
        path: ValuePath,
        field: String,
        expected: TypeKind,
        found: TypeKind,
    },

    #[error("Null not allowed at {path}")]
    NullNotAllowed { path: ValuePath },
}

impl TypeError {
    pub fn mismatch(path: impl Into<ValuePath>, expected: TypeKind, found: TypeKind) -> Self {
        Self::TypeMismatch {
            path: path.into(),
            expected,
            found,
        }
    }

    pub fn out_of_range(
        path: impl Into<ValuePath>,
        target: &'static str,
        value: impl ToString,
        hint: impl Into<String>,
    ) -> Self {
        Self::NumberOutOfRange {
            path: path.into(),
            target,
            value: value.to_string(),
            hint: hint.into(),
        }
    }

    pub fn invalid_enum(
        path: impl Into<ValuePath>,
        found: impl Into<String>,
        allowed: Vec<String>,
    ) -> Self {
        Self::InvalidEnumVariant {
            path: path.into(),
            found: found.into(),
            allowed,
        }
    }

    pub fn pattern(
        path: impl Into<ValuePath>,
        expect_desc: impl Into<String>,
        found: impl Into<String>,
    ) -> Self {
        Self::InvalidPattern {
            path: path.into(),
            expect_desc: expect_desc.into(),
            found: found.into(),
        }
    }

    pub fn array_elem(
        path: impl Into<ValuePath>,
        index: usize,
        expected: TypeKind,
        found: TypeKind,
    ) -> Self {
        Self::ArrayElementTypeMismatch {
            path: path.into(),
            index,
            expected,
            found,
        }
    }

    pub fn null_not_allowed(path: impl Into<ValuePath>) -> Self {
        Self::NullNotAllowed { path: path.into() }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypeKind {
    Null,
    Bool,
    Int,
    Uint,
    Float,
    String,
    Array,
    Object,
    Date,     // TOML date
    Time,     // TOML time
    DateTime, // TOML datetime
}

impl std::fmt::Display for TypeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use TypeKind::*;
        let s = match self {
            Null => "null",
            Bool => "bool",
            Int => "int",
            Uint => "uint",
            Float => "float",
            String => "string",
            Array => "array",
            Object => "object",
            Date => "date",
            Time => "time",
            DateTime => "datetime",
        };
        f.write_str(s)
    }
}
