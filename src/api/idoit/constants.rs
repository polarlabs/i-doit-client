use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub(crate) struct Constants {
    #[serde(rename = "objectTypes")]
    pub(crate) object_types: ObjectTypes,
    pub(crate) categories: Categories,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub(crate) struct ObjectTypes(pub(crate) HashMap<String, String>);

#[derive(Deserialize, Serialize, Clone, Debug)]
pub(crate) struct Categories {
    pub(crate) g: HashMap<String, String>,
    pub(crate) s: HashMap<String, String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_constants() {
        let json = r#"
        {
            "objectTypes": {
                "C__OBJTYPE__SERVER": "Server"
            },
            "categories": {
                "g": {
                    "C__CATG__GLOBAL": "General",
                    "C__CATG__MODEL": "Model"
                },
                "s": {
                    "C__CATS__MONITOR": "Monitor"
                }
            }
        }
        "#;

        let constants: Constants =
            serde_json::from_str(json).expect("failed to deserialize Constants");

        assert_eq!(constants.object_types.0.len(), 1);
        assert_eq!(constants.categories.g.len(), 2);
        assert_eq!(constants.categories.s.len(), 1);
    }
}
