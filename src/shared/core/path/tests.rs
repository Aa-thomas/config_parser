#[cfg(test)]
pub mod create_value_path_tests {
    use crate::shared::core::path::{PathSeg, ValuePath};

    // ==========================
    // Shared helpers / fixtures
    // ==========================

    /// Flatten a ValuePath into ["key", "1", "key", ...] for easy asserts.
    fn create_segments_from(vp: &ValuePath) -> Vec<String> {
        vp.0.iter()
            .map(|seg| match seg {
                PathSeg::Key(k) => k.clone(),
                PathSeg::Index(i) => i.to_string(),
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
            let expected: Vec<String> = vec!["top", "int"].into_iter().map(String::from).collect();

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
            let expected: Vec<String> = vec!["arrays", "nums", "2"]
                .into_iter()
                .map(String::from)
                .collect();

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
            let expected: Vec<String> = vec!["root_array_like", "1", "2"]
                .into_iter()
                .map(String::from)
                .collect();

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
            let expected: Vec<String> = vec!["nested", "arr_in_obj", "1", "y", "2"]
                .into_iter()
                .map(String::from)
                .collect();

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
            let expected: Vec<String> = vec!["top", "spaced key", "dot.key"]
                .into_iter()
                .map(String::from)
                .collect();

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
            let expected: Vec<String> = vec!["top", "spaced key", "emoji"]
                .into_iter()
                .map(String::from)
                .collect();

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
            let expected1: Vec<String> =
                vec!["i18n", "ключ"].into_iter().map(String::from).collect();

            println!("Test Path: {test_path1}\nExpected: {expected1:?}\n");

            let v1: ValidatedPath = test_path1.parse()?;
            let vp1 = create_value_path(&v1)?;
            let result1 = create_segments_from(&vp1);
            assert_eq!(result1, expected1);

            // Case 2
            let test_path2 = r#"i18n["日本語"]["キー"]"#;
            let expected2: Vec<String> = vec!["i18n", "日本語", "キー"]
                .into_iter()
                .map(String::from)
                .collect();

            println!("Test Path: {test_path2}\nExpected: {expected2:?}\n");

            let v2: ValidatedPath = test_path2.parse()?;
            let vp2 = create_value_path(&v2)?;
            let result2 = create_segments_from(&vp2);
            assert_eq!(result2, expected2);

            Ok(())
        }
    }
}
