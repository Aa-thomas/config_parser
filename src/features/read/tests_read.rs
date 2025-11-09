use crate::features::read::logic::{get_json_at_path, get_toml_at_path};
use serde_json::{json, Value};

pub fn test_data() -> Value {
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

#[cfg(test)]
mod tests {
    use crate::{
        features::read::logic::create_value_path,
        shared::types::{PathResult, PathSeg::Key, ValuePath},
    };

    use super::*;

    #[test]
    fn parses_dot_notation() {
        let result = create_value_path("a.b.c.d").unwrap();

        let expected = ValuePath(vec![
            Key("a".to_string()),
            Key("b".to_string()),
            Key("c".to_string()),
        ]);

        assert_eq!(result, expected)
    }
}
