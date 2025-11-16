use crate::shared::core::{
    errors::path::PathError,
    path::{PathResult, PathSeg, ValuePath},
    types::{ConfigValue, TypeKind},
};

pub fn get_json_at_path<'a>(
    document: &'a serde_json::Value,
    path: &ValuePath,
) -> PathResult<&'a serde_json::Value> {
    use serde_json::Value;

    if path.is_empty() {
        return Err(PathError::EmptyPath);
    }

    let mut cur = document;
    let mut prefix = ValuePath::default();

    for seg in &path.0 {
        match seg {
            PathSeg::Key(k) => {
                if let Value::Object(map) = cur {
                    cur = map
                        .get(k)
                        .ok_or_else(|| PathError::key_not_found(prefix.clone(), k))?;
                    prefix.push_key(k.clone());
                } else {
                    return Err(PathError::not_object(prefix, k, TypeKind::from_json(cur)));
                }
            }
            PathSeg::Index(i) => {
                if let Value::Array(arr) = cur {
                    let len = arr.len();
                    cur = arr
                        .get(*i)
                        .ok_or_else(|| PathError::oob(prefix.clone(), *i, len))?;
                    prefix.push_index(*i);
                } else {
                    return Err(PathError::not_array(prefix, *i, TypeKind::from_json(cur)));
                }
            }
        }
    }

    Ok(cur)
}

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
