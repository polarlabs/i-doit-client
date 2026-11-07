use serde::{Deserialize, Serialize};

use std::str::FromStr;
use crate::client::Error;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ObjectTypeGroups(Vec<ObjectTypeGroup>);

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ObjectTypeGroup {
    #[serde(deserialize_with = "crate::util::deserialize_u64_from_string_or_number")]
    pub(crate) id: u64,

    pub(crate) title: String,

    #[serde(rename = "const")]
    pub(crate) constant: String,

    #[serde(deserialize_with = "crate::util::deserialize_u8_from_string_or_number")]
    pub(crate) status: u8,
}

impl IntoIterator for ObjectTypeGroups {
    type Item = ObjectTypeGroup;
    type IntoIter = std::vec::IntoIter<ObjectTypeGroup>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_object_type_groups() {
        let json = r#"[
        {
            "id": 1,
            "title": "Software",
            "const": "C__OBJTYPE_GROUP__SOFTWARE",
            "status": 2
        },
        {
            "id": 2,
            "title": "Infrastructure",
            "const": "C__OBJTYPE_GROUP__INFRASTRUCTURE",
            "status": 2
        }
        ]"#;

        let object_type_groups: ObjectTypeGroups =
            serde_json::from_str(json).expect("failed to deserialize Object Type Groups");

        assert_eq!(object_type_groups.0.len(), 2);
    }

    #[test]
    fn test_deserialize_object_type_group() {
        let json = r#"
        {
            "id": 1,
            "title": "Software",
            "const": "C__OBJTYPE_GROUP__SOFTWARE",
            "status": 2
        }
        "#;

        let object_type_group: ObjectTypeGroup =
            serde_json::from_str(json).expect("failed to deserialize Object Type Group");

        assert_eq!(object_type_group.id, 1);
        assert_eq!(object_type_group.title, "Software");
        assert_eq!(object_type_group.constant, "C__OBJTYPE_GROUP__SOFTWARE");
        assert_eq!(object_type_group.status, 2);
    }
}
