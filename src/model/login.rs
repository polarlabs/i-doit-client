use serde::{Deserialize, Serialize};

use crate::model;

#[derive(Deserialize, Serialize, Debug)]
pub struct Login {
    language: model::Language,
    mail: String,
    name: String,
    tenant: String,
    userid: String,
    username: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_login() {
        let json = r#"
        {
            "language": "en",
            "mail": "admin@example.com",
            "name": "Administrator",
            "tenant": "example",
            "userid": "1",
            "username": "admin"
        }
        "#;

        let login: Login = serde_json::from_str(json).expect("failed to deserialize Login");

        assert_eq!(login.language, model::Language::English);
        assert_eq!(login.mail, "admin@example.com");
        assert_eq!(login.name, "Administrator");
        assert_eq!(login.tenant, "example");
        assert_eq!(login.userid, "1");
        assert_eq!(login.username, "admin");
    }

    #[test]
    fn test_serialize_login() {
        let login = Login {
            language: model::Language::English,
            mail: "admin@example.com".to_string(),
            name: "Administrator".to_string(),
            tenant: "example".to_string(),
            userid: "1".to_string(),
            username: "admin".to_string(),
        };

        let json = serde_json::to_value(&login).expect("failed to serialize Login");

        let expected = serde_json::json!({
            "language": "en",
            "mail": "admin@example.com",
            "name": "Administrator",
            "tenant": "example",
            "userid": "1",
            "username": "admin",
        });

        assert_eq!(json, expected);
    }
}
