pub mod tests;

use crate::shared::core::{errors::PathError, validate::validate::validate_path_syntax};
use std::{fmt, str::FromStr};

pub fn create_value_path(validated_path: &ValidatedPath) -> PathResult<ValuePath> {
    let mut output_path = ValuePath::new();
    let characters: Vec<char> = validated_path.as_str().chars().collect();
    let mut temporary_buffer = String::new();

    enum State {
        Default,
        InBracket,
        InQuotes(char),
    }

    let mut state = State::Default;

    fn push_key(out: &mut ValuePath, buffer: &mut String) {
        if !buffer.is_empty() {
            out.push_key(std::mem::take(buffer));
        }
    }

    for character in characters {
        match state {
            State::Default => match character {
                '.' => {
                    push_key(&mut output_path, &mut temporary_buffer);
                }
                '[' => {
                    push_key(&mut output_path, &mut temporary_buffer);
                    state = State::InBracket;
                }
                _ => temporary_buffer.push(character),
            },
            State::InBracket => {
                match character {
                    '0'..='9' => temporary_buffer.push(character),
                    ']' => {
                        if !temporary_buffer.is_empty() {
                            println!("2 output_path: {output_path}, temporary_buffer: {temporary_buffer}");
                            output_path.push_index_from_str(&temporary_buffer)?;
                            temporary_buffer.clear();
                        }
                        state = State::Default;
                    }
                    '"' | '\'' => {
                        state = State::InQuotes(character);
                    }
                    _ => unreachable!("Validator bug: validated input should not hit this branch"),
                }
            }

            State::InQuotes(q_mark) => match character {
                character if character == q_mark => {
                    push_key(&mut output_path, &mut temporary_buffer);
                    state = State::InBracket;
                }
                _ => temporary_buffer.push(character),
            },
        }
    }

    match state {
        State::Default => {
            if !temporary_buffer.is_empty() {
                push_key(&mut output_path, &mut temporary_buffer);
            }
        }
        State::InBracket | State::InQuotes(_) => {
            // If validate_path_syntax works, we should never end here.
            unreachable!("Validator bug: unterminated bracket or quotes")
        }
    }

    Ok(output_path)
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
            PathSeg::Key(key) => write!(f, ".{key}"),
            PathSeg::Index(idx) => write!(f, "[{idx}]"),
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

    pub fn push_index(&mut self, i: usize) -> PathResult<()> {
        self.0.push(PathSeg::Index(i));
        Ok(())
    }

    pub fn push_index_from_str(&mut self, s: &str) -> PathResult<()> {
        let prefix = self.clone();

        let idx = s
            .parse::<usize>()
            .map_err(|_| PathError::invalid_index(prefix, s.to_string()))?;
        self.push_index(idx)
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
                        write!(f, "{k}")?
                    } else {
                        write!(f, ".{k}")?
                    }
                }
                PathSeg::Index(idx) => write!(f, "[{idx}]")?,
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
