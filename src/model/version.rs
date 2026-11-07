use serde::{Deserialize, Serialize};

use crate::model;

#[derive(Deserialize, Serialize, Debug)]
struct Version {
    step: String,
    #[serde(rename = "type")]
    t: String,
    version: String,
    login: model::Login,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_version() {
        let json = r#"
        {
            "step": "",
            "type": "",
            "version": "36",
            "login": {
                "language": "en",
                "mail": "admin@example.com",
                "name": "Administrator",
                "tenant": "example",
                "userid": "1",
                "username": "admin"
            }
        }
        "#;

        let version: Version = serde_json::from_str(json).expect("failed to deserialize Version");

        assert_eq!(version.step, "");
        assert_eq!(version.t, "");
        assert_eq!(version.version, "36");
    }
}
