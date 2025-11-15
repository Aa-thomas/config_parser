#[cfg(test)]
pub mod tests_read {

    use std::borrow::Cow;

    use crate::shared::core::path::{PathSeg, ValuePath};

    use serde_json::json;

    // ==========================
    // Shared helpers / fixtures
    // ==========================

    /// Flatten a ValuePath into ["key", "1", "key", ...] for easy asserts.
    fn create_segments_from<'a>(vp: &'a ValuePath) -> Vec<Cow<'a, str>> {
        vp.0.iter()
            .map(|seg| match seg {
                PathSeg::Key(k) => Cow::Borrowed(k.as_str()),
                PathSeg::Index(i) => Cow::Owned(i.to_string()),
            })
            .collect()
    }

    /// Small TOML parser for inline fixtures
    fn parse_toml(doc: &str) -> toml_edit::Document {
        doc.parse::<toml_edit::Document>().unwrap()
    }

    // ============================================================
    // create_value_path tests  ->  cargo test create_value_path_tests
    // ============================================================
    mod create_value_path_tests {
        use crate::shared::core::path::{create_value_path, PathResult, ValidatedPath};

        use super::*;

        #[test]
        fn parses_dot_only() -> PathResult<()> {
            let test_path = "top.int";
            let expected = ["top", "int"];

            println!("Test Path: {test_path}\nExpected: {expected:?}\n");

            let validated_path = ValidatedPath::new(test_path)?;
            let value_path = create_value_path(&validated_path)?;
            let result = create_segments_from(&value_path);

            assert_eq!(result, expected);
            Ok(())
        }

        #[test]
        fn parses_index_in_brackets() -> PathResult<()> {
            let test_path = "arrays.nums[2]";
            let expected = ["arrays", "nums", "2"];

            println!("Test Path: {test_path}\nExpected: {expected:?}\n");

            let validated_path = ValidatedPath::new(test_path)?;
            let value_path = create_value_path(&validated_path)?;
            let result = create_segments_from(&value_path);

            assert_eq!(result, expected);
            Ok(())
        }

        #[test]
        fn parses_multiple_consecutive_indices() -> PathResult<()> {
            let test_path = "root_array_like[1][2]";
            let expected = ["root_array_like", "1", "2"];

            println!("Test Path: {test_path}\nExpected: {expected:?}\n");

            let validated_path = ValidatedPath::new(test_path)?;
            let value_path = create_value_path(&validated_path)?;
            let result = create_segments_from(&value_path);

            assert_eq!(result, expected);
            Ok(())
        }

        #[test]
        fn parses_mixed_dot_and_brackets() -> PathResult<()> {
            let test_path = "nested.arr_in_obj[1].y[2]";
            let expected = ["nested", "arr_in_obj", "1", "y", "2"];

            println!("Test Path: {test_path}\nExpected: {expected:?}\n");

            let validated_path = ValidatedPath::new(test_path)?;
            let value_path = create_value_path(&validated_path)?;
            let result = create_segments_from(&value_path);

            assert_eq!(result, expected);
            Ok(())
        }

        #[test]
        fn parses_quoted_key_double_quotes() -> PathResult<()> {
            let test_path = r#"top["spaced key"]["dot.key"]"#;
            let expected = ["top", "spaced key", "dot.key"];

            println!("Test Path: {test_path}\nExpected: {expected:?}\n");

            let validated_path = ValidatedPath::new(test_path)?;
            let value_path = create_value_path(&validated_path)?;
            let result = create_segments_from(&value_path);

            assert_eq!(result, expected);
            Ok(())
        }

        #[test]
        fn parses_quoted_key_single_quotes() -> PathResult<()> {
            let test_path = r#"top['spaced key']['emoji']"#;
            let expected = ["top", "spaced key", "emoji"];

            println!("Test Path: {test_path}\nExpected: {expected:?}\n");

            let validated_path = ValidatedPath::new(test_path)?;
            let value_path = create_value_path(&validated_path)?;
            let result = create_segments_from(&value_path);

            assert_eq!(result, expected);
            Ok(())
        }

        #[test]
        fn parses_unicode_keys() -> PathResult<()> {
            // Case 1
            let test_path1 = r#"i18n["ключ"]"#;
            let expected1 = ["i18n", "ключ"];

            println!("Test Path: {test_path1}\nExpected: {expected1:?}\n");

            let v1: ValidatedPath = test_path1.parse()?;
            let vp1 = create_value_path(&v1)?;
            let result1 = create_segments_from(&vp1);
            assert_eq!(result1, expected1);

            // Case 2
            let test_path2 = r#"i18n["日本語"]["キー"]"#;
            let expected2 = ["i18n", "日本語", "キー"];

            println!("Test Path: {test_path2}\nExpected: {expected2:?}\n");

            let v2: ValidatedPath = test_path2.parse()?;
            let vp2 = create_value_path(&v2)?;
            let result2 = create_segments_from(&vp2);
            assert_eq!(result2, expected2);

            Ok(())
        }
    }

    // ============================================================
    // get_json_at_path tests  ->  cargo test get_json_at_path_tests
    // ============================================================
    mod get_json_at_path_tests {
        use crate::shared::core::{
            adapters::json::get_json_at_path,
            errors::PathError,
            path::{create_value_path, PathResult, ValidatedPath},
        };

        use super::*;

        // ---------- Positive ----------

        #[test]
        fn json_resolves_simple_dot() -> PathResult<()> {
            let document = json!({ "top": { "int": 1 } });
            let test_path = "top.int";
            let expected = 1;

            println!("JSON: {document}\nTest Path: {test_path}\nExpected: {expected}\n");

            let validated_path = ValidatedPath::new(test_path)?;
            let value_path = create_value_path(&validated_path)?;
            let result = get_json_at_path(&document, &value_path)?;

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
            let expected = 30;

            println!("JSON: {document}\nTest Path: {test_path}\nExpected: {expected}\n");

            let validated_path = ValidatedPath::new(test_path)?;
            let value_path = create_value_path(&validated_path)?;
            let result = get_json_at_path(&document, &value_path)?;

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

            assert_eq!(result, expected);
            Ok(())
        }

        // ---------- Negative Tests ----------

        #[test]
        fn json_errors_on_empty_path() -> PathResult<()> {
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

    // ============================================================
    // get_toml_at_path tests  ->  cargo test get_toml_at_path_tests
    // ============================================================
    mod get_toml_at_path_tests {
        use crate::shared::core::{
            adapters::toml::{get_toml_at_path, TomlAt},
            errors::PathError,
            path::{create_value_path, PathResult, ValidatedPath},
        };

        use super::*;

        // ---------- Positive ----------

        #[test]
        fn toml_resolves_table_dot() -> PathResult<()> {
            let doc = parse_toml(
                r#"
        [top]
        int = 1
    "#,
            );
            let doc = doc.as_item();

            let test_path = "top.int";
            let expected = 1;

            println!("TOML:\n{doc}\nTest Path: {test_path}\nExpected: {expected}\n");

            let validated_path = ValidatedPath::new(test_path)?;
            let value_path = create_value_path(&validated_path)?;

            let result = get_toml_at_path(doc, &value_path).unwrap();
            let result = match result {
                TomlAt::Value(v) => v.as_integer().unwrap(),
                other => panic!("expected Value, got {:?}", &other),
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
            );
            let doc = doc.as_item();

            let test_path = r#"top["spaced key"]["dot.key"]"#;
            let expected = "v";

            println!("TOML:\n{doc}\nTest Path: {test_path}\nExpected: {expected}\n");

            let validated_path = ValidatedPath::new(test_path)?;
            let value_path = create_value_path(&validated_path)?;

            let result = get_toml_at_path(doc, &value_path).unwrap();
            let result = match result {
                TomlAt::Value(v) => v.as_str().unwrap(),
                other => panic!("expected Value, got {:?}", &other),
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
            );
            let doc = doc.as_item();

            let test_path = "arrays.nums[2]";
            let expected = 2;

            println!("TOML:\n{doc}\nTest Path: {test_path}\nExpected: {expected}\n");

            let validated_path = ValidatedPath::new(test_path)?;
            let value_path = create_value_path(&validated_path)?;

            let result = get_toml_at_path(doc, &value_path).unwrap();
            let result = match result {
                TomlAt::Value(v) => v.as_integer().unwrap(),
                other => panic!("expected Value, got {:?}", &other),
            };

            assert_eq!(result, expected);
            Ok(())
        }

        #[test]
        fn toml_resolves_array_of_tables_and_nested_array() -> PathResult<()> {
            let doc = parse_toml(
                r#"
        [[root_array_like]]
        k = "v0"

        [[root_array_like]]
        k = "v1"
        arr = [1, 2, 3]
    "#,
            );
            let doc = doc.as_item();

            // root_array_like[1].k == "v1"
            {
                let test_path = "root_array_like[1].k";
                let expected = "v1";

                println!("TOML:\n{doc}\nTest Path: {test_path}\nExpected: {expected}\n");

                let validated_path = ValidatedPath::new(test_path)?;
                let value_path = create_value_path(&validated_path)?;

                let result = get_toml_at_path(doc, &value_path).unwrap();
                let result = match result {
                    TomlAt::Value(v) => v.as_str().unwrap(),
                    other => panic!("expected Value, got {:?}", &other),
                };

                assert_eq!(result, expected);
            }

            // root_array_like[1].arr[2] == 3
            {
                let test_path = "root_array_like[1].arr[2]";
                let expected = 3;

                println!("TOML:\n{doc}\nTest Path: {test_path}\nExpected: {expected}\n");

                let validated_path = ValidatedPath::new(test_path)?;
                let value_path = create_value_path(&validated_path)?;

                let result = get_toml_at_path(doc, &value_path).unwrap();
                let result = match result {
                    TomlAt::Value(v) => v.as_integer().unwrap(),
                    other => panic!("expected Value, got {:?}", &other),
                };

                assert_eq!(result, expected);
            }

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
            );
            let doc = doc.as_item();

            let expected = "PathError::EmptyPath";

            println!("TOML:\n{doc}\nTest Path: <empty ValuePath>\nExpected: {expected}\n");

            let value_path = ValuePath::default();
            let err = get_toml_at_path(doc, &value_path).unwrap_err();

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
            );
            let doc = doc.as_item();

            let test_path = "top.missing";
            let expected = "PathError::KeyNotFound";

            println!("TOML:\n{doc}\nTest Path: {test_path}\nExpected: {expected}\n");

            let validated_path = ValidatedPath::new(test_path)?;
            let value_path = create_value_path(&validated_path)?;
            let err = get_toml_at_path(doc, &value_path).unwrap_err();

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
            );
            let doc = doc.as_item();

            // "int" is scalar; ".k" should cause NotAnObject
            let test_path = "top.int.k";
            let expected = "PathError::NotAnObject";

            println!("TOML:\n{doc}\nTest Path: {test_path}\nExpected: {expected}\n");

            let validated_path = ValidatedPath::new(test_path)?;
            let value_path = create_value_path(&validated_path)?;
            let err = get_toml_at_path(doc, &value_path).unwrap_err();

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
            );
            let doc = doc.as_item();

            // "int" is not an array; "[0]" should cause NotAnArray
            let test_path = "top.int[0]";
            let expected = "PathError::NotAnArray";

            println!("TOML:\n{doc}\nTest Path: {test_path}\nExpected: {expected}\n");

            let validated_path = ValidatedPath::new(test_path)?;
            let value_path = create_value_path(&validated_path)?;
            let err = get_toml_at_path(doc, &value_path).unwrap_err();

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
            );
            let doc = doc.as_item();

            let test_path = "arrays.nums[9]";
            let expected = "PathError::IndexOutOfBounds";

            println!("TOML:\n{doc}\nTest Path: {test_path}\nExpected: {expected}\n");

            let validated_path = ValidatedPath::new(test_path)?;
            let value_path = create_value_path(&validated_path)?;
            let err = get_toml_at_path(doc, &value_path).unwrap_err();

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
            );
            let doc = doc.as_item();

            let test_path = "root_array_like[3].k";
            let expected = "PathError::IndexOutOfBounds";

            println!("TOML:\n{doc}\nTest Path: {test_path}\nExpected: {expected}\n");

            let validated_path = ValidatedPath::new(test_path)?;
            let value_path = create_value_path(&validated_path)?;
            let err = get_toml_at_path(doc, &value_path).unwrap_err();

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
            );
            let doc = doc.as_item();

            // "obj" is a table, not an array
            let test_path = "containers.obj[0]";
            let expected = "PathError::NotAnArray";

            println!("TOML:\n{doc}\nTest Path: {test_path}\nExpected: {expected}\n");

            let validated_path = ValidatedPath::new(test_path)?;
            let value_path = create_value_path(&validated_path)?;
            let err = get_toml_at_path(doc, &value_path).unwrap_err();

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
            );
            let doc = doc.as_item();

            // Missing [i] before accessing "k"
            let test_path = "root_array_like.k";
            let expected = "PathError::NotAnObject | PathError::KeyNotFound";

            println!("TOML:\n{doc}\nTest Path: {test_path}\nExpected: {expected}\n");

            let validated_path = ValidatedPath::new(test_path)?;
            let value_path = create_value_path(&validated_path)?;
            let err = get_toml_at_path(doc, &value_path).unwrap_err();

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
    }
}
