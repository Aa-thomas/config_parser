use crate::shared::{errors::ParseError, types::ConfigFormat};

#[cfg(feature = "with-serde-json")]
impl From<(ConfigFormat, &str, serde_json::Error)> for ParseError {
    fn from((format, src, err): (ConfigFormat, &str, serde_json::Error)) -> Self {
        // serde_json::Error exposes line()/column()

        use crate::shared::types::SourceLocation;
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
