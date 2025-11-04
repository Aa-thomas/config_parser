use thiserror::Error;

use crate::shared::types::{ConfigFormat, SourceLocation};

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
