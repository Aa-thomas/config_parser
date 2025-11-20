use crate::shared::core::{errors::ParseError, parse::parse_types::ParseResult};

pub fn parse_json(data: &str) -> ParseResult<serde_json::Value> {
    serde_json::from_str::<serde_json::Value>(data).map_err(|e| ParseError::json(data, e))
}
