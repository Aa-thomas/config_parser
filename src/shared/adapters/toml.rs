#[cfg(feature = "with-toml-edit")]
use crate::shared::{errors::ParseError, types::ConfigFormat};

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
