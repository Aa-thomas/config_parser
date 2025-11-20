use toml_edit::{self, Document};

use crate::shared::core::{errors::ParseError, parse::parse_types::ParseResult};

pub fn parse_toml(data: &str) -> ParseResult<Document> {
    let document = data
        .parse::<Document>()
        .map_err(|error| ParseError::toml(&data, error));
    document
}
