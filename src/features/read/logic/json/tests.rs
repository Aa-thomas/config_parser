// ============================================================
// get_json_at_path tests  ->  cargo test get_json_at_path_tests
// ============================================================
#[cfg(test)]
pub mod get_json_at_path_tests {
    use serde_json::json;

    use crate::{
        features::read::logic::json::get_json_at_path,
        shared::core::{
            errors::PathError,
            path::{create_value_path, PathResult, ValidatedPath, ValuePath},
            types::ConfigValue,
        },
    };

    // ---------- Positive ----------

    #[test]
    fn json_resolves_simple_dot() -> PathResult<()> {
        let document = json!({ "top": { "int": 1 } });
        let test_path = "top.int";
        let expected: i64 = 1;

        println!("JSON: {document}\nTest Path: {test_path}\nExpected: {expected}\n");

        let validated_path = ValidatedPath::new(test_path)?;
        let value_path = create_value_path(&validated_path)?;

        let result = get_json_at_path(&document, &value_path)?;
        let result = match result {
            ConfigValue::Json(v) => v
                .as_i64()
                .ok_or_else(|| PathError::unsupported(value_path.clone(), "expected integer"))?,
            ConfigValue::Toml(item) => {
                return Err(PathError::unsupported(
                    value_path.clone(),
                    format!("expected JSON value, got TOML item: {item}"),
                ));
            }
        };

        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn json_resolves_mixed_path_into_array_element() -> PathResult<()> {
        let document = json!({
            "nested": {
                "arr_in_obj": [
                    { "x": 1 },
                    { "y": [10, 20, 30] }
                ]
            }
        });
        let test_path = "nested.arr_in_obj[1].y[2]";
        let expected: i64 = 30;

        println!("JSON: {document}\nTest Path: {test_path}\nExpected: {expected}\n");

        let validated_path = ValidatedPath::new(test_path)?;
        let value_path = create_value_path(&validated_path)?;

        let result = get_json_at_path(&document, &value_path)?;
        let result = match result {
            ConfigValue::Json(v) => v
                .as_i64()
                .ok_or_else(|| PathError::unsupported(value_path.clone(), "expected integer"))?,
            ConfigValue::Toml(item) => {
                return Err(PathError::unsupported(
                    value_path.clone(),
                    format!("expected JSON value, got TOML item: {item}"),
                ));
            }
        };

        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn json_resolves_quoted_key_with_dot_and_space() -> PathResult<()> {
        let document = json!({
            "top": {
                "spaced key": { "dot.key": "v" }
            }
        });
        let test_path = r#"top["spaced key"]["dot.key"]"#;
        let expected = "v";

        println!("JSON: {document}\nTest Path: {test_path}\nExpected: {expected}\n");

        let validated_path = ValidatedPath::new(test_path)?;
        let value_path = create_value_path(&validated_path)?;

        let result = get_json_at_path(&document, &value_path)?;
        let result = match result {
            ConfigValue::Json(v) => v
                .as_str()
                .ok_or_else(|| PathError::unsupported(value_path.clone(), "expected string"))?
                .to_owned(),
            ConfigValue::Toml(item) => {
                return Err(PathError::unsupported(
                    value_path.clone(),
                    format!("expected JSON value, got TOML item: {item}"),
                ));
            }
        };

        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn json_resolves_unicode_keys() -> PathResult<()> {
        let document = json!({
            "i18n": { "ключ": "значение" }
        });
        let test_path = r#"i18n["ключ"]"#;
        let expected = "значение";

        println!("JSON: {document}\nTest Path: {test_path}\nExpected: {expected}\n");

        let validated_path = ValidatedPath::new(test_path)?;
        let value_path = create_value_path(&validated_path)?;

        let result = get_json_at_path(&document, &value_path)?;
        let result = match result {
            ConfigValue::Json(v) => v
                .as_str()
                .ok_or_else(|| PathError::unsupported(value_path.clone(), "expected string"))?
                .to_owned(),
            ConfigValue::Toml(item) => {
                return Err(PathError::unsupported(
                    value_path.clone(),
                    format!("expected JSON value, got TOML item: {item}"),
                ));
            }
        };

        assert_eq!(result, expected);
        Ok(())
    }

    // ---------- Negative Tests ----------

    #[test]
    fn json_errors_on_empty_path() -> PathResult<()> {
        use crate::shared::core::errors::PathError;

        let document = json!(null);
        let value_path = ValuePath::default();
        let expected = "PathError::EmptyPath";

        println!("JSON: {document}\nTest Path: <empty ValuePath>\nExpected: {expected}\n");

        let err = get_json_at_path(&document, &value_path).unwrap_err();
        assert!(
            matches!(err, PathError::EmptyPath),
            "unexpected error variant: {err}"
        );

        Ok(())
    }

    #[test]
    fn json_errors_on_key_not_found() -> PathResult<()> {
        use crate::shared::core::errors::PathError;

        let document = json!({ "top": {} });
        let test_path = "top.missing";
        let expected = "PathError::KeyNotFound";

        println!("JSON: {document}\nTest Path: {test_path}\nExpected: {expected}\n");

        let validated_path = ValidatedPath::new(test_path)?;
        let value_path = create_value_path(&validated_path)?;
        let err = get_json_at_path(&document, &value_path).unwrap_err();

        assert!(
            matches!(err, PathError::KeyNotFound { .. }),
            "unexpected error variant: {err}"
        );

        Ok(())
    }

    #[test]
    fn json_errors_on_not_an_object() -> PathResult<()> {
        use crate::shared::core::errors::PathError;

        // nums[0] is a number, so ".x" should cause NotAnObject
        let document = json!({ "arrays": { "nums": [0] } });
        let test_path = "arrays.nums[0].x";
        let expected = "PathError::NotAnObject";

        println!("JSON: {document}\nTest Path: {test_path}\nExpected: {expected}\n");

        let validated_path = ValidatedPath::new(test_path)?;
        let value_path = create_value_path(&validated_path)?;
        let err = get_json_at_path(&document, &value_path).unwrap_err();

        assert!(
            matches!(err, PathError::NotAnObject { .. }),
            "unexpected error variant: {err}"
        );

        Ok(())
    }

    #[test]
    fn json_errors_on_not_an_array() -> PathResult<()> {
        use crate::shared::core::errors::PathError;

        // "int" is not an array, so "[0]" should cause NotAnArray
        let document = json!({ "top": { "int": 1 } });
        let test_path = "top.int[0]";
        let expected = "PathError::NotAnArray";

        println!("JSON: {document}\nTest Path: {test_path}\nExpected: {expected}\n");

        let validated_path = ValidatedPath::new(test_path)?;
        let value_path = create_value_path(&validated_path)?;
        let err = get_json_at_path(&document, &value_path).unwrap_err();

        assert!(
            matches!(err, PathError::NotAnArray { .. }),
            "unexpected error variant: {err}"
        );

        Ok(())
    }

    #[test]
    fn json_errors_on_index_out_of_bounds() -> PathResult<()> {
        use crate::shared::core::errors::PathError;

        let document = json!({ "arrays": { "nums": [0] } });
        let test_path = "arrays.nums[99]";
        let expected = "PathError::IndexOutOfBounds";

        println!("JSON: {document}\nTest Path: {test_path}\nExpected: {expected}\n");

        let validated_path = ValidatedPath::new(test_path)?;
        let value_path = create_value_path(&validated_path)?;
        let err = get_json_at_path(&document, &value_path).unwrap_err();

        assert!(
            matches!(err, PathError::IndexOutOfBounds { .. }),
            "unexpected error variant: {err}"
        );

        Ok(())
    }
}
