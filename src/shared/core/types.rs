use crate::shared::core::adapters::toml::TomlAt;
use clap::ValueEnum;

//----- COMMON TYPES -----

pub enum ConfigDocument {
    Json(serde_json::Value),
    Toml(toml_edit::Document),
}

pub enum ConfigValue<'a> {
    Json(&'a serde_json::Value),
    Toml(TomlAt<'a>),
}

#[derive(Copy, Clone, Debug, ValueEnum, PartialEq)]
pub enum ConfigFormat {
    Json,
    Toml,
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

impl TypeKind {
    // ───────── JSON ─────────
    // #[cfg(feature = "with-serde-json")]
    pub fn from_json(v: &serde_json::Value) -> Self {
        use serde_json::Value::*;
        match v {
            Null => TypeKind::Null,
            Bool(_) => TypeKind::Bool,
            Number(n) => {
                if n.is_i64() {
                    TypeKind::Int
                } else if n.is_u64() {
                    TypeKind::Uint
                } else {
                    TypeKind::Float
                }
            }
            String(_) => TypeKind::String,
            Array(_) => TypeKind::Array,
            Object(_) => TypeKind::Object,
        }
    }

    // ───────── TOML (Item) ─────────
    // #[cfg(feature = "with-toml-edit")]
    pub fn from_toml_item(item: &toml_edit::Item) -> Self {
        use toml_edit::Item::*;
        match item {
            Value(v) => Self::from_toml_value(v),
            Table(_) => Self::Object,
            ArrayOfTables(_) => Self::Array,
            None => Self::Null,
        }
    }
    // ───────── TOML (Value) ─────────
    // #[cfg(feature = "with-toml-edit")]
    pub fn from_toml_value(v: &toml_edit::Value) -> Self {
        use toml_edit::Value;
        match v {
            Value::Boolean(_) => TypeKind::Bool,
            Value::Integer(_) => TypeKind::Int,
            Value::Float(_) => TypeKind::Float,
            Value::String(_) => TypeKind::String,
            Value::Array(_) => TypeKind::Array,
            Value::InlineTable(_) => TypeKind::Object, // inline table behaves like an object
            Value::Datetime(dt) => {
                let inner = dt.value();
                let has_date = inner.date.is_some();
                let has_time = inner.time.is_some();
                if has_date && has_time {
                    TypeKind::DateTime
                } else if has_date {
                    TypeKind::Date
                } else {
                    TypeKind::Time
                }
            }
        }
    }
}
