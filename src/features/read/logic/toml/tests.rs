// ============================================================
// get_toml_at_path tests  ->  cargo test get_toml_at_path_tests
// ============================================================
#[cfg(test)]
mod get_toml_at_path_tests {
    use crate::{
        features::read::logic::toml::get_toml_at_path,
        shared::core::{
            errors::PathError,
            parse::parse_toml::parse_toml,
            path::{create_value_path, PathResult, ValidatedPath, ValuePath},
            types::ConfigValue,
        },
    };

    // ---------- Positive ----------

    #[test]
    fn toml_resolves_table_dot() -> PathResult<()> {
        let doc = parse_toml(
            r#"
        [top]
        int = 1
    "#,
        )
        .unwrap();

        let test_path = "top.int";
        let expected: i64 = 1;

        println!("TOML:\n{doc}\nTest Path: {test_path}\nExpected: {expected}\n");

        let validated_path = ValidatedPath::new(test_path)?;
        let value_path = create_value_path(&validated_path)?;

        let result = get_toml_at_path(&doc, &value_path)?;
        let result = match result {
            ConfigValue::Toml(item) => {
                item.as_value()
                    .and_then(|v| v.as_integer())
                    .ok_or_else(|| {
                        PathError::unsupported(value_path.clone(), "expected integer TOML value")
                    })?
            }
            ConfigValue::Json(v) => {
                return Err(PathError::unsupported(
                    value_path.clone(),
                    format!("expected TOML value, got JSON value: {v}"),
                ));
            }
        };

        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn toml_resolves_inline_table_and_quoted_key() -> PathResult<()> {
        let doc = parse_toml(
            r#"
        [top]
        "spaced key" = { "dot.key" = "v", emoji = "🦀" }
    "#,
        )
        .unwrap();

        let test_path = r#"top["spaced key"]["dot.key"]"#;
        let expected = "v";

        println!("TOML:\n{doc}\nTest Path: {test_path}\nExpected: {expected}\n");

        let validated_path = ValidatedPath::new(test_path)?;
        let value_path = create_value_path(&validated_path)?;

        let result = get_toml_at_path(&doc, &value_path)?;
        let result = match result {
            ConfigValue::Toml(item) => item
                .as_value()
                .and_then(|v| v.as_str())
                .ok_or_else(|| {
                    PathError::unsupported(value_path.clone(), "expected string TOML value")
                })?
                .to_owned(),
            ConfigValue::Json(v) => {
                return Err(PathError::unsupported(
                    value_path.clone(),
                    format!("expected TOML value, got JSON value: {v}"),
                ));
            }
        };

        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn toml_resolves_array_values_by_index() -> PathResult<()> {
        let doc = parse_toml(
            r#"
        [arrays]
        nums = [0, 1, 2]
    "#,
        )
        .unwrap();

        let test_path = "arrays.nums[2]";
        let expected: i64 = 2;

        println!("TOML:\n{doc}\nTest Path: {test_path}\nExpected: {expected}\n");

        let validated_path = ValidatedPath::new(test_path)?;
        let value_path = create_value_path(&validated_path)?;

        let result = get_toml_at_path(&doc, &value_path)?;
        let result = match result {
            ConfigValue::Toml(item) => {
                item.as_value()
                    .and_then(|v| v.as_integer())
                    .ok_or_else(|| {
                        PathError::unsupported(value_path.clone(), "expected integer TOML value")
                    })?
            }
            ConfigValue::Json(v) => {
                return Err(PathError::unsupported(
                    value_path.clone(),
                    format!("expected TOML value, got JSON value: {v}"),
                ));
            }
        };

        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn toml_resolves_array_of_tables() -> PathResult<()> {
        let doc = parse_toml(
            r#"
        [[root_array_like]]
        k = "v0"
        [[root_array_like]]
        k = "v1"
    "#,
        )
        .unwrap();

        // root_array_like[1].k == "v1"
        let test_path = "root_array_like[1].k";
        let expected = "v1";

        println!("TOML:\n{doc}\nTest Path: {test_path}\nExpected: {expected}\n");

        let validated_path = ValidatedPath::new(test_path)?;
        let value_path = create_value_path(&validated_path)?;

        let result = get_toml_at_path(&doc, &value_path)?;
        let result = match result {
            ConfigValue::Toml(item) => item
                .as_value()
                .and_then(|v| v.as_str())
                .ok_or_else(|| {
                    PathError::unsupported(value_path.clone(), "expected string TOML value")
                })?
                .to_owned(),
            ConfigValue::Json(v) => {
                return Err(PathError::unsupported(
                    value_path.clone(),
                    format!("expected TOML value, got JSON value: {v}"),
                ));
            }
        };

        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn toml_resolves_nested_array() -> PathResult<()> {
        let doc = parse_toml(
            r#"
        [[root_array_like]]
        k = "v0"

        [[root_array_like]]
        k = "v1"
        arr = [1, 2, 3]
    "#,
        )
        .unwrap();

        let test_path = "root_array_like[1].arr[2]";
        let expected: i64 = 3;

        println!("TOML:\n{doc}\nTest Path: {test_path}\nExpected: {expected}\n");

        let validated_path = ValidatedPath::new(test_path)?;
        let value_path = create_value_path(&validated_path)?;

        let result = get_toml_at_path(&doc, &value_path)?;
        let result = match result {
            ConfigValue::Toml(item) => {
                item.as_value()
                    .and_then(|v| v.as_integer())
                    .ok_or_else(|| {
                        PathError::unsupported(value_path.clone(), "expected integer TOML value")
                    })?
            }
            ConfigValue::Json(v) => {
                return Err(PathError::unsupported(
                    value_path.clone(),
                    format!("expected TOML value, got JSON value: {v}"),
                ));
            }
        };

        assert_eq!(result, expected);
        Ok(())
    }

    // ---------- Negative ----------

    #[test]
    fn toml_errors_on_empty_path() -> PathResult<()> {
        let doc = parse_toml(
            r#"
        [top]
        int = 1
    "#,
        )
        .unwrap();

        let expected = "PathError::EmptyPath";

        println!("TOML:\n{doc}\nTest Path: <empty ValuePath>\nExpected: {expected}\n");

        let value_path = ValuePath::default();
        let err = get_toml_at_path(&doc, &value_path).unwrap_err();

        assert!(
            matches!(err, PathError::EmptyPath),
            "unexpected error variant: {err}"
        );

        Ok(())
    }

    #[test]
    fn toml_errors_on_key_not_found() -> PathResult<()> {
        let doc = parse_toml(
            r#"
        [top]
        int = 1
    "#,
        )
        .unwrap();

        let test_path = "top.missing";
        let expected = "PathError::KeyNotFound";

        println!("TOML:\n{doc}\nTest Path: {test_path}\nExpected: {expected}\n");

        let validated_path = ValidatedPath::new(test_path)?;
        let value_path = create_value_path(&validated_path)?;
        let err = get_toml_at_path(&doc, &value_path).unwrap_err();

        assert!(
            matches!(err, PathError::KeyNotFound { .. }),
            "unexpected error variant: {err}"
        );

        Ok(())
    }

    #[test]
    fn toml_errors_on_not_an_object() -> PathResult<()> {
        let doc = parse_toml(
            r#"
        [top]
        int = 1
    "#,
        )
        .unwrap();

        // "int" is scalar; ".k" should cause NotAnObject
        let test_path = "top.int.k";
        let expected = "PathError::NotAnObject";

        println!("TOML:\n{doc}\nTest Path: {test_path}\nExpected: {expected}\n");

        let validated_path = ValidatedPath::new(test_path)?;
        let value_path = create_value_path(&validated_path)?;
        let err = get_toml_at_path(&doc, &value_path).unwrap_err();

        assert!(
            matches!(err, PathError::NotAnObject { .. }),
            "unexpected error variant: {err}"
        );

        Ok(())
    }

    #[test]
    fn toml_errors_on_not_an_array() -> PathResult<()> {
        let doc = parse_toml(
            r#"
        [top]
        int = 1
    "#,
        )
        .unwrap();

        // "int" is not an array; "[0]" should cause NotAnArray
        let test_path = "top.int[0]";
        let expected = "PathError::NotAnArray";

        println!("TOML:\n{doc}\nTest Path: {test_path}\nExpected: {expected}\n");

        let validated_path = ValidatedPath::new(test_path)?;
        let value_path = create_value_path(&validated_path)?;
        let err = get_toml_at_path(&doc, &value_path).unwrap_err();

        assert!(
            matches!(err, PathError::NotAnArray { .. }),
            "unexpected error variant: {err}"
        );

        Ok(())
    }

    #[test]
    fn toml_errors_on_index_out_of_bounds_array_values() -> PathResult<()> {
        let doc = parse_toml(
            r#"
        [arrays]
        nums = [0, 1, 2]
    "#,
        )
        .unwrap();

        let test_path = "arrays.nums[9]";
        let expected = "PathError::IndexOutOfBounds";

        println!("TOML:\n{doc}\nTest Path: {test_path}\nExpected: {expected}\n");

        let validated_path = ValidatedPath::new(test_path)?;
        let value_path = create_value_path(&validated_path)?;
        let err = get_toml_at_path(&doc, &value_path).unwrap_err();

        assert!(
            matches!(err, PathError::IndexOutOfBounds { .. }),
            "unexpected error variant: {err}"
        );

        Ok(())
    }

    #[test]
    fn toml_errors_on_index_out_of_bounds_array_of_tables() -> PathResult<()> {
        let doc = parse_toml(
            r#"
        [[root_array_like]]
        k = "v0"
    "#,
        )
        .unwrap();

        let test_path = "root_array_like[3].k";
        let expected = "PathError::IndexOutOfBounds";

        println!("TOML:\n{doc}\nTest Path: {test_path}\nExpected: {expected}\n");

        let validated_path = ValidatedPath::new(test_path)?;
        let value_path = create_value_path(&validated_path)?;
        let err = get_toml_at_path(&doc, &value_path).unwrap_err();

        assert!(
            matches!(err, PathError::IndexOutOfBounds { .. }),
            "unexpected error variant: {err}"
        );

        Ok(())
    }

    #[test]
    fn toml_errors_on_indexing_into_table() -> PathResult<()> {
        let doc = parse_toml(
            r#"
        [containers]
        obj = { inner = { leaf = "end" } }
    "#,
        )
        .unwrap();

        // "obj" is a table, not an array
        let test_path = "containers.obj[0]";
        let expected = "PathError::NotAnArray";

        println!("TOML:\n{doc}\nTest Path: {test_path}\nExpected: {expected}\n");

        let validated_path = ValidatedPath::new(test_path)?;
        let value_path = create_value_path(&validated_path)?;
        let err = get_toml_at_path(&doc, &value_path).unwrap_err();

        assert!(
            matches!(err, PathError::NotAnArray { .. }),
            "unexpected error variant: {err}"
        );

        Ok(())
    }

    #[test]
    fn toml_errors_on_key_into_array_without_index() -> PathResult<()> {
        let doc = parse_toml(
            r#"
        [[root_array_like]]
        k = "v0"
    "#,
        )
        .unwrap();

        // Missing [i] before accessing "k"
        let test_path = "root_array_like.k";
        let expected = "PathError::NotAnObject | PathError::KeyNotFound";

        println!("TOML:\n{doc}\nTest Path: {test_path}\nExpected: {expected}\n");

        let validated_path = ValidatedPath::new(test_path)?;
        let value_path = create_value_path(&validated_path)?;
        let err = get_toml_at_path(&doc, &value_path).unwrap_err();

        // Depending on traversal details this may appear as NotAnObject or KeyNotFound.
        assert!(
            matches!(
                err,
                PathError::NotAnObject { .. } | PathError::KeyNotFound { .. }
            ),
            "unexpected error variant: {err}"
        );

        Ok(())
    }

    #[test]
    fn toml_resolves_simple_top_level_key() -> PathResult<()> {
        let doc = parse_toml(r#"title = "My Title""#).unwrap();
        let test_path = "title";
        let expected = "My Title";

        println!("TOML:\n{doc}\nTest Path: {test_path}\nExpected: {expected}\n");

        let validated_path = ValidatedPath::new(test_path)?;
        let value_path = create_value_path(&validated_path)?;

        let result = get_toml_at_path(&doc, &value_path)?;
        let result = match result {
            ConfigValue::Toml(item) => item
                .as_value()
                .and_then(|v| v.as_str())
                .ok_or_else(|| {
                    PathError::unsupported(value_path.clone(), "expected string TOML value")
                })?
                .to_owned(),
            ConfigValue::Json(v) => {
                return Err(PathError::unsupported(
                    value_path.clone(),
                    format!("expected TOML value, got JSON value: {v}"),
                ));
            }
        };

        assert_eq!(result, expected);
        Ok(())
    }
}
