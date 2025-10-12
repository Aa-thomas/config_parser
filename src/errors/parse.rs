use std::{fmt, path::PathBuf};

use crate::cli::ConfigFormat;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("{format:?} parse error at {loc}: unexpected token: expected {expected}, found {found}\n{snippet}")]
    UnexpectedToken {
        format: ConfigFormat,
        loc: SourceLocation,
        expected: String,
        found: String,
        snippet: String,
    },

    #[error("{format:?} parse error at {loc}: unexpected end of input\n{snippet}")]
    UnexpectedEof {
        format: ConfigFormat,
        loc: SourceLocation,
        snippet: String,
    },

    #[error("{format:?} parse error at {loc}: unterminated string literal\n{snippet}")]
    UnterminatedString {
        format: ConfigFormat,
        loc: SourceLocation,
        snippet: String,
    },

    #[error("{format:?} parse error at {loc}: invalid escape sequence: {source}\n{snippet}")]
    InvalidEscape {
        format: ConfigFormat,
        loc: SourceLocation,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync + 'static>,
        snippet: String,
    },

    #[error("{format:?} parse error at {loc}: trailing content after document\n{snippet}")]
    TrailingContent {
        format: ConfigFormat,
        loc: SourceLocation,
        snippet: String,
    },

    #[error("{format:?} parse error at {loc}: syntax error: expected {expected}, found {found}\n{snippet}")]
    SyntaxError {
        format: ConfigFormat,
        loc: SourceLocation,
        expected: String, // can be a single token or a small joined set
        found: String,
        snippet: String,
    },

    #[error("{format:?} parse error at {loc}: {source}\n{snippet}")]
    ForeignParseError {
        format: ConfigFormat,
        loc: SourceLocation,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync + 'static>,
        snippet: String,
    },
}

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

#[cfg(feature = "with-serde-json")]
impl From<(ConfigFormat, &str, serde_json::Error)> for ParseError {
    fn from((format, src, err): (ConfigFormat, &str, serde_json::Error)) -> Self {
        // serde_json::Error exposes line()/column()
        let line = err.line();
        let column = err.column();
        let loc = SourceLocation::new(line as usize, column as usize);
        // extract a short snippet around the column
        let snippet = extract_snippet(src, line as usize, column as usize);
        ParseError::ForeignParseError {
            format,
            loc,
            source: Box::new(err),
            snippet,
        }
    }
}

#[cfg(feature = "with-toml-edit")]
impl From<(ConfigFormat, &str, toml_edit::TomlError)> for ParseError {
    fn from((format, src, err): (ConfigFormat, &str, toml_edit::TomlError)) -> Self {
        // Try to get a byte offset from toml_edit's span; fall back to (1,1)
        let (line, column) = err
            .span()
            .map(|span| {
                // `span.start()` is a byte offset into `src`
                let start = span.start();
                offset_to_line_col(src, start)
            })
            .unwrap_or((1, 1));

        let loc = SourceLocation::new(line, column);
        let snippet = extract_snippet(src, line, column);

        ParseError::ForeignParseError {
            format,
            loc,
            #[allow(clippy::box_default)]
            source: Box::new(err), // preserves real error chain
            snippet,
        }
    }
}

#[cfg(feature = "with-toml-edit")]
fn offset_to_line_col(src: &str, offset: usize) -> (usize, usize) {
    let mut line = 1usize;
    let mut col = 1usize;

    for (i, ch) in src.char_indices() {
        if i >= offset {
            break;
        }
        if ch == '\n' {
            line += 1;
            col = 1;
        } else {
            col += 1;
        }
    }
    (line, col)
}

/// Tiny utility to show a caret-line snippet; adjust as you like.
fn extract_snippet(src: &str, line: usize, column: usize) -> String {
    let mut out = String::new();
    if let Some(l) = src.lines().nth(line.saturating_sub(1)) {
        out.push_str(l);
        out.push('\n');
        if column > 1 {
            out.push_str(&" ".repeat(column.saturating_sub(1)));
        }
        out.push('^');
    }
    out
}
