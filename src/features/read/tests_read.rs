use crate::features::read::logic::{create_value_path, get_json_at_path, get_toml_at_path};

#[cfg(test)]
mod tests {

    use crate::shared::types::{PathSeg, ValuePath};

    use super::*;
    use serde_json::json;

    // ==========================
    // Shared helpers / fixtures
    // ==========================

    /// Flatten a ValuePath into ["key", "1", "key", ...] for easy asserts.
    fn create_segments(vp: &ValuePath) -> Vec<String> {
        vp.0.iter()
            .map(|seg| match seg {
                PathSeg::Key(k) => k.clone(),
                PathSeg::Index(i) => i.to_string(),
            })
            .collect()
    }

    /// JSON fixture used across tests
    fn jfix() -> serde_json::Value {
        json!({
            "top": {
                "int": 1,
                "bool": true,
                "null": null,
                "str": "hi",
                "spaced key": { "dot.key": "v", "emoji": "🦀" }
            },
            "nested": {
                "a": { "b": { "c": 123 } },
                "arr_in_obj": [ { "x": 1 }, { "y": [10, 20, 30] } ]
            },
            "arrays": {
                "nums": [0, 1, 2],
                "mixed": [1, "two", false, null, { "k": "v" }, [9, 8]],
                "len1": ["only"],
                "empty": []
            },
            "i18n": {
                "ключ": "значение",
                "日本語": { "キー": "値" }
            },
            "containers": {
                "obj": { "inner": { "leaf": "end" } },
                "empty_obj": {}
            },
            "root_array_like": [
                { "k": "v0" },
                { "k": "v1", "arr": [1, 2, 3] },
                "leaf"
            ]
        })
    }

    /// Small TOML parser for inline fixtures
    fn parse_toml(doc: &str) -> toml_edit::Document {
        doc.parse::<toml_edit::Document>().unwrap()
    }

    // ============================================================
    // create_value_path tests  ->  cargo test create_value_path_tests
    // ============================================================
    mod create_value_path_tests {
        use crate::shared::errors::PathError;

        use super::*;

        // ---------- Positive ----------

        #[test]
        fn parses_dot_only() {
            let vp = create_value_path("top.int").unwrap();
            let result = create_segments(&vp);
            let expected = ["top", "int"];
            assert_eq!(result, expected);
        }

        #[test]
        fn parses_index_in_brackets() {
            let vp = create_value_path("arrays.nums[2]").unwrap();
            let result = create_segments(&vp);
            let expected = ["arrays", "nums", "2"];
            assert_eq!(result, expected);
        }

        #[test]
        fn parses_multiple_consecutive_indices() {
            let vp = create_value_path("root_array_like[1][2]").unwrap();
            let result = create_segments(&vp);
            let expected = ["root_array_like", "1", "2"];
            assert_eq!(result, expected);
        }

        #[test]
        fn parses_mixed_dot_and_brackets() {
            let vp = create_value_path("nested.arr_in_obj[1].y[2]").unwrap();
            let result = create_segments(&vp);
            let expected = ["nested", "arr_in_obj", "1", "y", "2"];
            assert_eq!(result, expected);
        }

        #[test]
        fn parses_quoted_key_double_quotes() {
            let vp = create_value_path(r#"top["spaced key"]["dot.key"]"#).unwrap();
            let result = create_segments(&vp);
            let expected = ["top", "spaced key", "dot.key"];
            assert_eq!(result, expected);
        }

        #[test]
        fn parses_quoted_key_single_quotes() {
            let vp = create_value_path(r#"top['spaced key']['emoji']"#).unwrap();
            let result = create_segments(&vp);
            let expected = ["top", "spaced key", "emoji"];
            assert_eq!(result, expected);
        }

        #[test]
        fn parses_unicode_keys() {
            let vp = create_value_path(r#"i18n["ключ"]"#).unwrap();
            let result = create_segments(&vp);
            let expected = ["i18n", "ключ"];
            assert_eq!(result, expected);

            let vp = create_value_path(r#"i18n["日本語"]["キー"]"#).unwrap();
            let result = create_segments(&vp);
            let expected = ["i18n", "日本語", "キー"];
            assert_eq!(result, expected);
        }

        // ---------- Negative (syntax → InvalidSegment) ----------
        // If your validator lives elsewhere, keep these here only if create_value_path returns these errors.

        #[test]
        fn errors_on_unclosed_bracket() {
            let err = create_value_path("a[1").unwrap_err();
            let result = matches!(err, PathError::InvalidSegment { .. });
            let expected = true;
            assert_eq!(result, expected);
        }

        #[test]
        fn errors_on_empty_accessor() {
            let err = create_value_path("a[]").unwrap_err();
            let result = matches!(err, PathError::InvalidSegment { .. });
            let expected = true;
            assert_eq!(result, expected);
        }

        #[test]
        fn errors_on_unclosed_quote() {
            let err = create_value_path(r#"a["x"#).unwrap_err();
            let result = matches!(err, PathError::InvalidSegment { .. });
            let expected = true;
            assert_eq!(result, expected);
        }

        #[test]
        fn errors_on_trailing_dot() {
            let err = create_value_path("a.").unwrap_err();
            let result = matches!(err, PathError::InvalidSegment { .. });
            let expected = true;
            assert_eq!(result, expected);
        }

        #[test]
        fn errors_on_double_dot() {
            let err = create_value_path("a..b").unwrap_err();
            let result = matches!(err, PathError::InvalidSegment { .. });
            let expected = true;
            assert_eq!(result, expected);
        }

        #[test]
        fn errors_on_unquoted_identifier_in_brackets() {
            let err = create_value_path("a[foo]").unwrap_err();
            let result = matches!(err, PathError::InvalidSegment { .. });
            let expected = true;
            assert_eq!(result, expected);
        }
    }

    // ============================================================
    // get_json_at_path tests  ->  cargo test get_json_at_path_tests
    // ============================================================
    mod get_json_at_path_tests {
        use crate::shared::errors::PathError;

        use super::*;

        // ---------- Positive ----------

        #[test]
        fn json_resolves_simple_dot() {
            let data = jfix();
            let vp = create_value_path("top.int").unwrap();
            let result = get_json_at_path(&data, &vp).unwrap();
            let expected = &json!(1);
            assert_eq!(result, expected);
        }

        #[test]
        fn json_resolves_mixed_path_into_array_element() {
            let data = jfix();
            let vp = create_value_path("nested.arr_in_obj[1].y[2]").unwrap();
            let result = get_json_at_path(&data, &vp).unwrap();
            let expected = &json!(30);
            assert_eq!(result, expected);
        }

        #[test]
        fn json_resolves_quoted_key_with_dot_and_space() {
            let data = jfix();
            let vp = create_value_path(r#"top["spaced key"]["dot.key"]"#).unwrap();
            let result = get_json_at_path(&data, &vp).unwrap();
            let expected = &json!("v");
            assert_eq!(result, expected);
        }

        #[test]
        fn json_resolves_unicode_keys() {
            let data = jfix();
            let vp = create_value_path(r#"i18n["ключ"]"#).unwrap();
            let result = get_json_at_path(&data, &vp).unwrap();
            let expected = &json!("значение");
            assert_eq!(result, expected);
        }

        // ---------- Negative (map to your PathError variants) ----------

        #[test]
        fn json_errors_on_empty_path() {
            let data = jfix();
            let vp = ValuePath::default();
            let err = get_json_at_path(&data, &vp).unwrap_err();
            let result = matches!(err, PathError::EmptyPath);
            let expected = true;
            assert_eq!(result, expected);
        }

        #[test]
        fn json_errors_on_key_not_found() {
            let data = jfix();
            let vp = create_value_path("top.missing").unwrap();
            let err = get_json_at_path(&data, &vp).unwrap_err();
            let result = matches!(err, PathError::KeyNotFound { .. });
            let expected = true;
            assert_eq!(result, expected);
        }

        #[test]
        fn json_errors_on_not_an_object() {
            let data = jfix();
            let vp = create_value_path("arrays.nums[0].x").unwrap(); // nums[0] is a number
            let err = get_json_at_path(&data, &vp).unwrap_err();
            let result = matches!(err, PathError::NotAnObject { .. });
            let expected = true;
            assert_eq!(result, expected);
        }

        #[test]
        fn json_errors_on_not_an_array() {
            let data = jfix();
            let vp = create_value_path("top.int[0]").unwrap(); // int is not an array
            let err = get_json_at_path(&data, &vp).unwrap_err();
            let result = matches!(err, PathError::NotAnArray { .. });
            let expected = true;
            assert_eq!(result, expected);
        }

        #[test]
        fn json_errors_on_index_out_of_bounds() {
            let data = jfix();
            let vp = create_value_path("arrays.nums[99]").unwrap();
            let err = get_json_at_path(&data, &vp).unwrap_err();
            let result = matches!(err, PathError::IndexOutOfBounds { .. });
            let expected = true;
            assert_eq!(result, expected);
        }
    }

    // ============================================================
    // get_toml_at_path tests  ->  cargo test get_toml_at_path_tests
    // ============================================================
    mod get_toml_at_path_tests {
        use crate::shared::{errors::PathError, types::TomlAt};

        use super::*;
        use toml_edit::value;

        // ---------- Positive ----------

        #[test]
        fn toml_resolves_table_dot() {
            let doc = parse_toml(
                r#"
                [top]
                int = 1
            "#,
            );
            let root = doc.as_item();
            let vp = create_value_path("top.int").unwrap();
            let result = get_toml_at_path(root, &vp).unwrap();

            let expected = value(1);
            let got = match result {
                TomlAt::Value(v) => v.clone(),
                other => panic!("expected Value, got {:?}", other),
            };
            let result = got;
            let expected = expected;
            assert_eq!(result, expected);
        }

        #[test]
        fn toml_resolves_inline_table_and_quoted_key() {
            let doc = parse_toml(
                r#"
                [top]
                "spaced key" = { "dot.key" = "v", emoji = "🦀" }
            "#,
            );
            let root = doc.as_item();
            let vp = create_value_path(r#"top["spaced key"]["dot.key"]"#).unwrap();
            let result = get_toml_at_path(root, &vp).unwrap();

            let expected = value("v");
            let got = match result {
                TomlAt::Value(v) => v.clone(),
                other => panic!("expected Value, got {:?}", other),
            };
            let result = got;
            let expected = expected;
            assert_eq!(result, expected);
        }

        #[test]
        fn toml_resolves_array_values_by_index() {
            let doc = parse_toml(
                r#"
                [arrays]
                nums = [0, 1, 2]
            "#,
            );
            let root = doc.as_item();
            let vp = create_value_path("arrays.nums[2]").unwrap();
            let result = get_toml_at_path(root, &vp).unwrap();

            let expected = value(2);
            let got = match result {
                TomlAt::Value(v) => v.clone(),
                other => panic!("expected Value, got {:?}", other),
            };
            let result = got;
            let expected = expected;
            assert_eq!(result, expected);
        }

        #[test]
        fn toml_resolves_array_of_tables_and_nested_array() {
            let doc = parse_toml(
                r#"
                [[root_array_like]]
                k = "v0"

                [[root_array_like]]
                k = "v1"
                arr = [1, 2, 3]
            "#,
            );
            let root = doc.as_item();

            // root_array_like[1].k == "v1"
            let vp = create_value_path("root_array_like[1].k").unwrap();
            let result = get_toml_at_path(root, &vp).unwrap();
            let expected = value("v1");
            let got = match result {
                TomlAt::Value(v) => v.clone(),
                other => panic!("expected Value, got {:?}", other),
            };
            let result = got;
            let expected = expected;
            assert_eq!(result, expected);

            // root_array_like[1].arr[2] == 3
            let vp = create_value_path("root_array_like[1].arr[2]").unwrap();
            let result = get_toml_at_path(root, &vp).unwrap();
            let expected = value(3);
            let got = match result {
                TomlAt::Value(v) => v.clone(),
                other => panic!("expected Value, got {:?}", other),
            };
            let result = got;
            let expected = expected;
            assert_eq!(result, expected);
        }

        // ---------- Negative ----------

        #[test]
        fn toml_errors_on_empty_path() {
            let doc = parse_toml(
                r#"
                [top]
                int = 1
            "#,
            );
            let root = doc.as_item();
            let vp = ValuePath::default();
            let err = get_toml_at_path(root, &vp).unwrap_err();
            let result = matches!(err, PathError::EmptyPath);
            let expected = true;
            assert_eq!(result, expected);
        }

        #[test]
        fn toml_errors_on_key_not_found() {
            let doc = parse_toml(
                r#"
                [top]
                int = 1
            "#,
            );
            let root = doc.as_item();
            let vp = create_value_path("top.missing").unwrap();
            let err = get_toml_at_path(root, &vp).unwrap_err();
            let result = matches!(err, PathError::KeyNotFound { .. });
            let expected = true;
            assert_eq!(result, expected);
        }

        #[test]
        fn toml_errors_on_not_an_object() {
            let doc = parse_toml(
                r#"
                [top]
                int = 1
            "#,
            );
            let root = doc.as_item();
            let vp = create_value_path("top.int.k").unwrap(); // int is scalar
            let err = get_toml_at_path(root, &vp).unwrap_err();
            let result = matches!(err, PathError::NotAnObject { .. });
            let expected = true;
            assert_eq!(result, expected);
        }

        #[test]
        fn toml_errors_on_not_an_array() {
            let doc = parse_toml(
                r#"
                [top]
                int = 1
            "#,
            );
            let root = doc.as_item();
            let vp = create_value_path("top.int[0]").unwrap(); // int is not array
            let err = get_toml_at_path(root, &vp).unwrap_err();
            let result = matches!(err, PathError::NotAnArray { .. });
            let expected = true;
            assert_eq!(result, expected);
        }

        #[test]
        fn toml_errors_on_index_out_of_bounds_array_values() {
            let doc = parse_toml(
                r#"
                [arrays]
                nums = [0, 1, 2]
            "#,
            );
            let root = doc.as_item();
            let vp = create_value_path("arrays.nums[9]").unwrap();
            let err = get_toml_at_path(root, &vp).unwrap_err();
            let result = matches!(err, PathError::IndexOutOfBounds { .. });
            let expected = true;
            assert_eq!(result, expected);
        }

        #[test]
        fn toml_errors_on_index_out_of_bounds_array_of_tables() {
            let doc = parse_toml(
                r#"
                [[root_array_like]]
                k = "v0"
            "#,
            );
            let root = doc.as_item();
            let vp = create_value_path("root_array_like[3].k").unwrap();
            let err = get_toml_at_path(root, &vp).unwrap_err();
            let result = matches!(err, PathError::IndexOutOfBounds { .. });
            let expected = true;
            assert_eq!(result, expected);
        }

        #[test]
        fn toml_errors_on_indexing_into_table() {
            let doc = parse_toml(
                r#"
                [containers]
                obj = { inner = { leaf = "end" } }
            "#,
            );
            let root = doc.as_item();
            let vp = create_value_path("containers.obj[0]").unwrap(); // obj is a table, not array
            let err = get_toml_at_path(root, &vp).unwrap_err();
            let result = matches!(err, PathError::NotAnArray { .. });
            let expected = true;
            assert_eq!(result, expected);
        }

        #[test]
        fn toml_errors_on_key_into_array_without_index() {
            let doc = parse_toml(
                r#"
                [[root_array_like]]
                k = "v0"
            "#,
            );
            let root = doc.as_item();
            let vp = create_value_path("root_array_like.k").unwrap(); // missing [i]
            let err = get_toml_at_path(root, &vp).unwrap_err();
            // Depending on traversal details this may appear as NotAnObject or KeyNotFound.
            let result = matches!(
                err,
                PathError::NotAnObject { .. } | PathError::KeyNotFound { .. }
            );
            let expected = true;
            assert_eq!(result, expected);
        }
    }
}
