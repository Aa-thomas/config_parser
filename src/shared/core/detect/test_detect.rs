#[cfg(test)]
mod detect_format_tests {
    use crate::shared::core::{detect::detect_format, types::ConfigFormat};

    // ==========================
    // JSON detection
    // ==========================

    #[test]
    fn detects_json_object() {
        let input = r#"{ "key": 1, "ok": true }"#;

        let result = detect_format(input).unwrap();
        let expected = ConfigFormat::Json;

        assert_eq!(result, expected);
    }

    #[test]
    fn detects_json_array() {
        let input = r#"[1, 2, 3]"#;

        let result = detect_format(input).unwrap();
        let expected = ConfigFormat::Json;

        assert_eq!(result, expected);
    }

    #[test]
    fn detects_json_with_whitespace() {
        let input = r#"
        
            {
                "nested": { "x": 1 },
                "flag": false
            }
        "#;

        let result = detect_format(input).unwrap();
        let expected = ConfigFormat::Json;

        assert_eq!(result, expected);
    }

    // ==========================
    // TOML detection
    // ==========================

    #[test]
    fn detects_toml_table() {
        let input = r#"
            [top]
            int = 1
            str = "hi"
        "#;

        let result = detect_format(input).unwrap();
        let expected = ConfigFormat::Toml;

        assert_eq!(result, expected);
    }

    #[test]
    fn detects_toml_inline_table() {
        let input = r#"
            [top]
            inline = { a = 1, b = "two" }
        "#;

        let result = detect_format(input).unwrap();
        let expected = ConfigFormat::Toml;

        assert_eq!(result, expected);
    }

    #[test]
    fn detects_toml_array_of_tables() {
        let input = r#"
            [[servers]]
            ip = "192.168.1.1"
            port = 8080

            [[servers]]
            ip = "192.168.1.2"
            port = 8081
        "#;

        let result = detect_format(input).unwrap();
        let expected = ConfigFormat::Toml;

        assert_eq!(result, expected);
    }

    // ==========================
    // Error cases
    // ==========================

    #[test]
    fn rejects_empty_input() {
        let input = "   ";

        let result = detect_format(input).unwrap_err();
        let expected = "input is empty or whitespace only";

        assert_eq!(result.to_string(), expected);
    }

    #[test]
    fn rejects_garbage_input() {
        let input = "this is not json or toml";

        let result = detect_format(input).unwrap_err();
        let expected = "input is neither valid JSON nor valid TOML";

        // Only assert on the main part of the message, ignore the embedded parser errors.
        assert!(
            result.to_string().starts_with(expected),
            "expected error starting with {expected:?}, got {result:?}"
        );
    }
}
