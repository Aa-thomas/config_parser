mod test_detect;

use crate::shared::core::{errors::detect_file::DetectError, types::ConfigFormat};
use serde_json::Value as JsonValue;
use toml_edit::Document;

pub type DetectResult<T> = Result<T, DetectError>;

/// Pure content-based format detection.
/// No I/O, no filesystem, just the bytes.
pub fn detect_format(input: &str) -> DetectResult<ConfigFormat> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(DetectError::EmptyInput);
    }

    // 1. Try JSON
    let json_err = match serde_json::from_str::<JsonValue>(trimmed) {
        Ok(_) => return Ok(ConfigFormat::Json),
        Err(e) => e,
    };

    // 2. If JSON fails, try TOML
    let toml_err = match input.parse::<Document>() {
        Ok(_) => return Ok(ConfigFormat::Toml),
        Err(e) => e,
    };

    // 3. Neither worked → report both in a friendly way
    Err(DetectError::UnknownFormat {
        json_error: json_err.to_string(),
        toml_error: toml_err.to_string(),
    })
}
