#[cfg(test)]
pub mod validate_path_syntax_tests {
    use crate::shared::core::{
        errors::PathError, path::PathResult, validate::validation_logic::validate_path_syntax,
    };

    // ============================================
    // validate_path_syntax tests
    //    -> cargo test validate_path_syntax_tests
    // ============================================

    // ---------- Valid: simple keys ----------

    #[test]
    fn accepts_single_simple_key() -> PathResult<()> {
        let test_path = "key";

        let result = validate_path_syntax(test_path).is_ok();
        let expected = true;

        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn accepts_simple_key_with_digits() -> PathResult<()> {
        let test_path = "key123";

        let result = validate_path_syntax(test_path).is_ok();
        let expected = true;

        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn accepts_simple_key_with_leading_underscore() -> PathResult<()> {
        let test_path = "_key";

        let result = validate_path_syntax(test_path).is_ok();
        let expected = true;

        assert_eq!(result, expected);
        Ok(())
    }

    // ---------- Valid: dot notation ----------

    #[test]
    fn accepts_two_segment_dot_notation() -> PathResult<()> {
        let test_path = "key.subkey";

        let result = validate_path_syntax(test_path).is_ok();
        let expected = true;

        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn accepts_three_segment_dot_notation() -> PathResult<()> {
        let test_path = "a.b.c";

        let result = validate_path_syntax(test_path).is_ok();
        let expected = true;

        assert_eq!(result, expected);
        Ok(())
    }

    // ---------- Valid: array indices ----------

    #[test]
    fn accepts_single_numeric_index() -> PathResult<()> {
        let test_path = "key[0]";

        let result = validate_path_syntax(test_path).is_ok();
        let expected = true;

        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn accepts_multi_digit_numeric_index() -> PathResult<()> {
        let test_path = "key[123]";

        let result = validate_path_syntax(test_path).is_ok();
        let expected = true;

        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn accepts_chained_numeric_indices() -> PathResult<()> {
        let test_path = "key[0][1]";

        let result = validate_path_syntax(test_path).is_ok();
        let expected = true;

        assert_eq!(result, expected);
        Ok(())
    }

    // ---------- Valid: quoted keys ----------

    #[test]
    fn accepts_quoted_subkey_with_double_quotes() -> PathResult<()> {
        let test_path = r#"key["subkey"]"#;

        let result = validate_path_syntax(test_path).is_ok();
        let expected = true;

        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn accepts_quoted_subkey_with_single_quotes() -> PathResult<()> {
        let test_path = r#"key['subkey']"#;

        let result = validate_path_syntax(test_path).is_ok();
        let expected = true;

        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn accepts_root_segment_as_quoted_key() -> PathResult<()> {
        let test_path = r#"["key"]"#;

        let result = validate_path_syntax(test_path).is_ok();
        let expected = true;

        assert_eq!(result, expected);
        Ok(())
    }

    // ---------- Valid: special characters in quoted keys ----------

    #[test]
    fn accepts_quoted_key_containing_dot_character() -> PathResult<()> {
        let test_path = r#"key["sub.key"]"#;

        let result = validate_path_syntax(test_path).is_ok();
        let expected = true;

        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn accepts_quoted_key_containing_brackets() -> PathResult<()> {
        let test_path = r#"key["sub[key]"]"#;

        let result = validate_path_syntax(test_path).is_ok();
        let expected = true;

        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn accepts_quoted_key_with_closing_bracket_inside() -> PathResult<()> {
        let test_path = r#"key["sub]key"]"#;

        let result = validate_path_syntax(test_path).is_ok();
        let expected = true;

        assert_eq!(result, expected);
        Ok(())
    }

    // ---------- Valid: mixed notation ----------

    #[test]
    fn accepts_mixed_quoted_key_and_dot_notation() -> PathResult<()> {
        let test_path = r#"key["sub"].other"#;

        let result = validate_path_syntax(test_path).is_ok();
        let expected = true;

        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn accepts_mixed_index_and_dot_notation() -> PathResult<()> {
        let test_path = r#"key[0].other"#;

        let result = validate_path_syntax(test_path).is_ok();
        let expected = true;

        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn accepts_complex_mixed_notation_path() -> PathResult<()> {
        let test_path = r#"a.b.c[0].d["e"]"#;

        let result = validate_path_syntax(test_path).is_ok();
        let expected = true;

        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn accepts_root_index_with_quoted_child_and_property() -> PathResult<()> {
        let test_path = r#"root[0]["child"].prop"#;

        let result = validate_path_syntax(test_path).is_ok();
        let expected = true;

        assert_eq!(result, expected);
        Ok(())
    }

    // ---------- Invalid: empty / dots ----------

    #[test]
    fn rejects_empty_path() -> PathResult<()> {
        let test_path = "";

        let result = validate_path_syntax(test_path).unwrap_err();
        let expected = true;
        let is_expected_variant = matches!(result, PathError::EmptyPath);

        assert_eq!(is_expected_variant, expected);
        Ok(())
    }

    #[test]
    fn rejects_path_starting_with_dot() -> PathResult<()> {
        let test_path = ".key";

        let result = validate_path_syntax(test_path).is_err();
        let expected = true;

        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn rejects_path_ending_with_dot() -> PathResult<()> {
        let test_path = "key.";

        let result = validate_path_syntax(test_path).is_err();
        let expected = true;

        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn rejects_path_with_consecutive_dots() -> PathResult<()> {
        let p1 = "key..subkey";
        let p2 = "a..b..c";

        let result = validate_path_syntax(p1).is_err() && validate_path_syntax(p2).is_err();
        let expected = true;

        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn rejects_empty_segment_after_dot() -> PathResult<()> {
        let test_path = "a.";

        let result = validate_path_syntax(test_path).is_err();
        let expected = true;

        assert_eq!(result, expected);
        Ok(())
    }

    // ---------- Invalid: brackets / indices ----------

    #[test]
    fn rejects_empty_brackets() -> PathResult<()> {
        let test_path = "key[]";

        let result = validate_path_syntax(test_path).is_err();
        let expected = true;

        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn rejects_unclosed_numeric_bracket() -> PathResult<()> {
        let test_path = "key[0";

        let result = validate_path_syntax(test_path).is_err();
        let expected = true;

        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn rejects_unclosed_bracket_with_quoted_key() -> PathResult<()> {
        let test_path = r#"key["sub"#;

        let result = validate_path_syntax(test_path).is_err();
        let expected = true;

        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn rejects_unclosed_bracket_without_content() -> PathResult<()> {
        let test_path = "key[";

        let result = validate_path_syntax(test_path).is_err();
        let expected = true;

        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn rejects_bare_non_digit_in_brackets() -> PathResult<()> {
        let p1 = "key[abc]";
        let p2 = "key[sub]";

        let result = validate_path_syntax(p1).is_err() && validate_path_syntax(p2).is_err();
        let expected = true;

        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn rejects_non_digit_in_numeric_index() -> PathResult<()> {
        let p1 = "key[12a]";
        let p2 = "key[1 2]";
        let p3 = "key[a12]";

        let result = validate_path_syntax(p1).is_err()
            && validate_path_syntax(p2).is_err()
            && validate_path_syntax(p3).is_err();
        let expected = true;

        assert_eq!(result, expected);
        Ok(())
    }

    // ---------- Invalid: quotes ----------

    #[test]
    fn rejects_unclosed_double_quoted_key_in_brackets() -> PathResult<()> {
        let test_path = r#"key["sub]"#;

        let result = validate_path_syntax(test_path).is_err();
        let expected = true;

        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn rejects_unclosed_single_quoted_key_in_brackets() -> PathResult<()> {
        let test_path = r#"key['sub]"#;

        let result = validate_path_syntax(test_path).is_err();
        let expected = true;

        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn rejects_characters_after_closing_quote_in_brackets() -> PathResult<()> {
        let p1 = r#"key["sub"x]"#;
        let p2 = r#"key["sub" ]"#;

        let result = validate_path_syntax(p1).is_err() && validate_path_syntax(p2).is_err();
        let expected = true;

        assert_eq!(result, expected);
        Ok(())
    }

    // ---------- Invalid: unmatched brackets ----------

    #[test]
    fn rejects_unmatched_closing_bracket_after_key() -> PathResult<()> {
        let test_path = "key]";

        let result = validate_path_syntax(test_path).is_err();
        let expected = true;

        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn rejects_unmatched_closing_bracket_before_key() -> PathResult<()> {
        let test_path = "]key";

        let result = validate_path_syntax(test_path).is_err();
        let expected = true;

        assert_eq!(result, expected);
        Ok(())
    }

    // ---------- Edge cases (valid) ----------

    #[test]
    fn accepts_single_character_key() -> PathResult<()> {
        let test_path = "a";

        let result = validate_path_syntax(test_path).is_ok();
        let expected = true;

        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn accepts_deep_dot_notation_chain() -> PathResult<()> {
        let test_path = "a.b.c.d.e.f";

        let result = validate_path_syntax(test_path).is_ok();
        let expected = true;

        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn accepts_multiple_chained_indices() -> PathResult<()> {
        let test_path = "a[0][1][2]";

        let result = validate_path_syntax(test_path).is_ok();
        let expected = true;

        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn accepts_large_array_index() -> PathResult<()> {
        let test_path = "key[999999]";

        let result = validate_path_syntax(test_path).is_ok();
        let expected = true;

        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn accepts_zero_array_index() -> PathResult<()> {
        let test_path = "key[0]";

        let result = validate_path_syntax(test_path).is_ok();
        let expected = true;

        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn accepts_empty_quoted_key_segment() -> PathResult<()> {
        let test_path = r#"key[""]"#;

        let result = validate_path_syntax(test_path).is_ok();
        let expected = true;

        assert_eq!(result, expected);
        Ok(())
    }
}
