use crate::shared::core::{
    adapters::{
        json::get_json_at_path,
        toml::{get_toml_at_path, TomlAt},
    },
    errors::PathError,
    types::{ConfigDocument, ConfigValue},
    validate::validate::validate_path_syntax,
};
use std::{fmt, str::FromStr};

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
    pub fn new() -> Self {
        ValuePath(Vec::new())
    }

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

pub struct ValidatedPath(String);

impl ValidatedPath {
    pub fn new(path: &str) -> Result<Self, PathError> {
        validate_path_syntax(path)?;
        Ok(ValidatedPath(path.to_string()))
    }
    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }
}

impl FromStr for ValidatedPath {
    type Err = PathError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ValidatedPath::new(s)
    }
}

impl TryFrom<&str> for ValidatedPath {
    type Error = PathError;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        ValidatedPath::new(s)
    }
}

pub type PathResult<T> = Result<T, PathError>;
