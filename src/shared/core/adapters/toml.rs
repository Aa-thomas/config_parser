use crate::shared::core::{
    errors::PathError,
    path::{PathResult, PathSeg, ValuePath},
    types::TypeKind,
};

pub fn get_toml_at_path<'a>(
    document: &'a toml_edit::Item,
    path: &ValuePath,
) -> PathResult<TomlAt<'a>> {
    use toml_edit::{Item, Value};

    if path.is_empty() {
        return Err(PathError::EmptyPath);
    }

    let mut cursor = TomlCursor::Item(document);
    let mut prefix = ValuePath::default();

    for seg in &path.0 {
        match seg {
            PathSeg::Key(k) => {
                cursor = match cursor {
                    TomlCursor::Item(Item::Table(tbl)) => {
                        let next = tbl
                            .get(k)
                            .ok_or_else(|| PathError::key_not_found(prefix.clone(), k))?;
                        prefix.push_key(k.clone());
                        TomlCursor::Item(next)
                    }
                    TomlCursor::Item(Item::Value(val)) => {
                        if let Value::InlineTable(itbl) = val {
                            let next = itbl
                                .get(k)
                                .ok_or_else(|| PathError::key_not_found(prefix.clone(), k))?;
                            prefix.push_key(k.clone());
                            TomlCursor::Value(next)
                        } else {
                            return Err(PathError::not_object(
                                prefix,
                                k,
                                TypeKind::from_toml_value(val),
                            ));
                        }
                    }
                    TomlCursor::Value(val) => {
                        if let Value::InlineTable(itbl) = val {
                            let next = itbl
                                .get(k)
                                .ok_or_else(|| PathError::key_not_found(prefix.clone(), k))?;
                            prefix.push_key(k.clone());
                            TomlCursor::Value(next)
                        } else {
                            return Err(PathError::not_object(
                                prefix,
                                k,
                                TypeKind::from_toml_value(val),
                            ));
                        }
                    }
                    TomlCursor::Table(tbl) => {
                        let next = tbl
                            .get(k)
                            .ok_or_else(|| PathError::key_not_found(prefix.clone(), k))?;
                        prefix.push_key(k.clone());
                        TomlCursor::Item(next)
                    }
                    TomlCursor::Item(item) => {
                        return Err(PathError::not_object(
                            prefix,
                            k,
                            TypeKind::from_toml_item(item),
                        ));
                    }
                }
            }
            PathSeg::Index(i) => {
                cursor = match cursor {
                    TomlCursor::Item(Item::Value(val)) => {
                        if let Value::Array(arr) = val {
                            let len = arr.len();
                            let next = arr
                                .get(*i)
                                .ok_or_else(|| PathError::oob(prefix.clone(), *i, len))?;
                            prefix.push_index(*i);
                            TomlCursor::Value(next)
                        } else {
                            return Err(PathError::not_array(
                                prefix,
                                *i,
                                TypeKind::from_toml_value(val),
                            ));
                        }
                    }
                    TomlCursor::Value(val) => {
                        if let Value::Array(arr) = val {
                            let len = arr.len();
                            let next = arr
                                .get(*i)
                                .ok_or_else(|| PathError::oob(prefix.clone(), *i, len))?;
                            prefix.push_index(*i);
                            TomlCursor::Value(next)
                        } else {
                            return Err(PathError::not_array(
                                prefix,
                                *i,
                                TypeKind::from_toml_value(val),
                            ));
                        }
                    }
                    TomlCursor::Item(Item::ArrayOfTables(aot)) => {
                        let len = aot.len();
                        let tbl = aot
                            .get(*i)
                            .ok_or_else(|| PathError::oob(prefix.clone(), *i, len))?;
                        prefix.push_index(*i);
                        TomlCursor::Table(tbl)
                    }
                    TomlCursor::Item(item) => {
                        return Err(PathError::not_array(
                            prefix,
                            *i,
                            TypeKind::from_toml_item(item),
                        ));
                    }
                    TomlCursor::Table(_) => {
                        return Err(PathError::not_array(prefix, *i, cursor.type_kind()));
                    }
                }
            }
        }
    }

    Ok(match cursor {
        TomlCursor::Item(item) => TomlAt::Item(item),
        TomlCursor::Value(val) => TomlAt::Value(val),
        TomlCursor::Table(tbl) => TomlAt::Table(tbl),
    })
}

//----- TOML TYPES -----
#[derive(Debug)]
pub enum TomlAt<'a> {
    Item(&'a toml_edit::Item),
    Value(&'a toml_edit::Value),
    Table(&'a toml_edit::Table),
}

impl<'a> TomlAt<'a> {
    pub fn as_value(&self) -> Option<&'a toml_edit::Value> {
        match self {
            TomlAt::Value(v) => Some(v),
            TomlAt::Item(item) => item.as_value(),
            TomlAt::Table(_) => None,
        }
    }

    fn type_kind(&self) -> TypeKind {
        match self {
            TomlAt::Item(item) => TypeKind::from_toml_item(item),
            TomlAt::Value(val) => TypeKind::from_toml_value(val),
            TomlAt::Table(_) => {
                TypeKind::from_toml_item(&toml_edit::Item::Table(toml_edit::Table::new()))
            }
        }
    }
}

impl<'a> std::fmt::Display for TomlAt<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TomlAt::Item(item) => write!(f, "{}", item),
            TomlAt::Value(val) => write!(f, "{}", val),
            TomlAt::Table(tbl) => write!(f, "{}", tbl),
        }
    }
}

pub enum TomlCursor<'a> {
    Item(&'a toml_edit::Item),
    Value(&'a toml_edit::Value),
    Table(&'a toml_edit::Table),
}

impl<'a> TomlCursor<'a> {
    pub fn type_kind(&self) -> TypeKind {
        match self {
            TomlCursor::Item(item) => TypeKind::from_toml_item(item),
            TomlCursor::Value(val) => TypeKind::from_toml_value(val),
            TomlCursor::Table(_) => {
                TypeKind::from_toml_item(&toml_edit::Item::Table(toml_edit::Table::new()))
            }
        }
    }
}

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
