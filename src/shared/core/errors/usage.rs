use core::fmt;

use thiserror::Error;

#[derive(Error)]
pub enum UsageError {
    #[error("UsageError: missing required flag {flag}. {hint}")]
    MissingFlag {
        flag: &'static str,
        hint: &'static str,
    },

    #[error(
        "UsageError: invalid value '{provided}' for {flag}. \
        Valid options: {valid:?}. Example: --format json"
    )]
    InvalidChoice {
        provided: &'static str,
        flag: &'static str,
        valid: Vec<&'static str>,
    },

    #[error("UsageError: flags {a} and {b} cannot be used together. {hint}")]
    ConflictingFlags {
        a: &'static str,
        b: &'static str,
        hint: &'static str,
    },

    #[error("UsageError: missing argument {name}. Example: {example}")]
    MissingArgument {
        name: &'static str,
        example: &'static str,
    },

    #[error(
        "UsageError: invalid key-path syntax: '{input}'. \
    Example: {example}"
    )]
    InvalidPathSyntax {
        input: &'static str,
        example: &'static str,
    },
}

impl fmt::Debug for UsageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}
