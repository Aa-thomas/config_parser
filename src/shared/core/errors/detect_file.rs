use thiserror::Error;

#[derive(Debug, Error)]
pub enum DetectError {
    #[error("input is empty or whitespace only")]
    EmptyInput,

    #[error("input is neither valid JSON nor valid TOML\n  json error: {json_error}\n  toml error: {toml_error}")]
    UnknownFormat {
        json_error: String,
        toml_error: String,
    },
}

pub type DetectResult<T> = Result<T, DetectError>;
