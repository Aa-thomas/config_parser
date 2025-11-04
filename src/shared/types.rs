use std::{fmt, path::PathBuf};
use crate::shared::errors::{ParseError, PathError};
use clap::ValueEnum;

//----- COMMON TYPES -----
#[derive(Copy, Clone, Debug, ValueEnum)]
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

// ----- PARSE TYPES -----
#[derive(Debug, Clone, PartialEq)]
pub struct SourceLocation {
    pub line: usize,
    pub column: usize,
    pub file: Option<PathBuf>,
}

impl SourceLocation {
    pub fn new(line: usize, column: usize) -> Self {
        SourceLocation {
            line,
            column,
            file: None,
        }
    }

    pub fn with_file(mut self, file: PathBuf) -> Self {
        self.file = Some(file);
        self
    }
}

impl fmt::Display for SourceLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.file {
            Some(p) => write!(f, "{}:{}:{}", p.display(), self.line, self.column),
            None => write!(f, "{}:{}", self.line, self.column),
        }
    }
}

pub type ParseResult<T> = Result<T, ParseError>;

//----- PATH TYPES -----
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PathSeg {
    Key(String),
    Index(usize),
}

impl fmt::Display for PathSeg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PathSeg::Key(key) => write!(f, ".{}", key),
            PathSeg::Index(idx) => write!(f, "[{}]", idx),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ValuePath(pub Vec<PathSeg>);

impl ValuePath {
    pub fn push_key(&mut self, k: impl Into<String>) {
        self.0.push(PathSeg::Key(k.into()));
    }
    pub fn push_index(&mut self, i: usize) {
        self.0.push(PathSeg::Index(i));
    }
    pub fn pop(&mut self) {
        let _ = self.0.pop();
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
impl fmt::Display for ValuePath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, seg) in self.0.iter().enumerate() {
            match seg {
                PathSeg::Key(k) => {
                    if i == 0 {
                        write!(f, "{}", k)?
                    } else {
                        write!(f, ".{}", k)?
                    }
                }
                PathSeg::Index(idx) => write!(f, "[{}]", idx)?,
            }
        }
        Ok(())
    }
}

pub type PathResult<T> = Result<T, PathError>;

//----- TOML TYPES -----
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

pub enum TomlCursor<'a> {
    Item(&'a toml_edit::Item),
    Value(&'a toml_edit::Value),
    Table(&'a toml_edit::Table),
}

impl<'a> TomlCursor<'a> {
    pub fn type_kind(&self) -> TypeKind {
        match self {
            TomlCursor::Item(item) => TypeKind::from_toml_item(item),
            TomlCursor::Value(val) => TypeKind::from_toml_value(val),
            TomlCursor::Table(_) => {
                TypeKind::from_toml_item(&toml_edit::Item::Table(toml_edit::Table::new()))
            }
        }
    }
}
