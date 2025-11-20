use core::fmt;

use thiserror::Error;
use toml_edit::TomlError;

use crate::shared::{
    core::{parse::parse_types::SourceLocation, types::ConfigFormat},
    shell::present::extract_snippet,
};

#[derive(Error)]
pub enum ParseError {
    #[error("{format:?} parse error at {loc}: unexpected token: expected {expected}, found `{found}`\n{snippet}")]
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

    #[error("{format:?} parse error at {loc}: syntax error: expected {expected}, found `{found}`\n{snippet}")]
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

impl fmt::Debug for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self}")
    }
}

impl ParseError {
    pub fn toml(src: &str, err: TomlError) -> ParseError {
        Self::map_toml_error(src, err)
    }

    pub fn json(src: &str, err: serde_json::Error) -> ParseError {
        use serde_json::error::Category;

        // serde_json may give (line=0, col=0) for some IO/EOF cases,
        // so we clamp to at least 1 for nice UX in our SourceLocation/snippet.
        let (raw_line, raw_col) = match err.classify() {
            Category::Io | Category::Eof => (1_usize, 1_usize),
            _ => (err.line(), err.column()),
        };

        let line = raw_line.max(1);
        let column = raw_col.max(1);

        let loc = SourceLocation::new(line, column);
        let snippet = extract_snippet(src, line, column);

        ParseError::ForeignParseError {
            format: ConfigFormat::Json,
            loc,
            source: Box::new(err),
            snippet,
        }
    }

    fn offset_to_line_col(src: &str, offset: usize) -> (usize, usize) {
        let mut line = 1;
        let mut col = 1;

        for (i, ch) in src.chars().enumerate() {
            if i == offset {
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

    pub fn map_toml_error(src: &str, err: TomlError) -> ParseError {
        let span = err.span().unwrap_or(0..0);
        let (line, column) = Self::offset_to_line_col(src, span.start);

        let loc = SourceLocation::new(line, column);
        let snippet = extract_snippet(src, line, column);

        ParseError::ForeignParseError {
            format: ConfigFormat::Toml,
            loc,
            source: Box::new(err),
            snippet,
        }
    }
}
