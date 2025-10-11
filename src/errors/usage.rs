use thiserror::Error;

#[derive(Debug, Error)]
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
        provided: String,
        flag: String,
        valid: Vec<String>,
    },

    #[error("UsageError: flags {a} and {b} cannot be used together. {hint}")]
    ConflictingFlags { a: String, b: String, hint: String },

    #[error("UsageError: missing argument {name}. Example: {example}")]
    MissingArgument { name: String, example: String },

    #[error(
        "UsageError: invalid key-path syntax: '{input}'. \
    Example: {example}"
    )]
    InvalidPathSyntax { input: String, example: String },
}
