use serde::{Deserialize, Serialize};

#[derive(PartialEq, Deserialize, Serialize, Debug)]
pub enum Language {
    #[serde(rename = "de")]
    Deutsch,

    #[serde(rename = "en")]
    English,
}

impl Default for Language {
    fn default() -> Self {
        Language::English
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_language() {
        let language = Language::default();

        assert_eq!(language, Language::English);
    }

    #[test]
    fn test_deserialize_language() {
        let json = r#"
        "en"
        "#;

        let language: Language =
            serde_json::from_str(json).expect("failed to deserialize Language");

        assert_eq!(language, Language::English);
    }

    #[test]
    fn test_serialize_language() {
        let language = Language::English;

        let json = serde_json::to_string(&language).expect("failed to serialize Language");

        assert_eq!(json, r#""en""#);
    }
}
