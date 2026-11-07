use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub(crate) struct ObjectTypes(Vec<ObjectType>);

#[derive(Deserialize, Serialize, Debug, Clone)]
pub(crate) struct ObjectType {
    #[serde(deserialize_with = "crate::util::deserialize_u64_from_string_or_number")]
    pub(crate) id: u64,

    pub(crate) title: String,
    pub(crate) container: String,

    #[serde(rename = "const")]
    pub(crate) constant: String,

    pub(crate) color: String,
    pub(crate) image: String,
    pub(crate) icon: String,
    pub(crate) tree_group: String,

    #[serde(deserialize_with = "crate::util::deserialize_u8_from_string_or_number")]
    pub(crate) status: u8,

    #[serde(deserialize_with = "crate::util::deserialize_u64_from_string_or_number")]
    pub(crate) type_group: u64,
    pub(crate) type_group_title: String,

    #[serde(default, deserialize_with = "crate::util::deserialize_optional_u64_from_string_or_number")]
    pub(crate) objectcount: Option<u64>,
}

impl IntoIterator for ObjectTypes {
    type Item = ObjectType;
    type IntoIter = std::vec::IntoIter<ObjectType>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

/// Builtin object type constants
#[derive(Serialize, Debug)]
#[allow(non_camel_case_types)]
pub enum ObjectTypeConst {
    C__OBJTYPE__PERSON,
    C__OBJTYPE__PERSON_GROUP,
    Other(String),
}

impl From<&str> for ObjectTypeConst {
    fn from(s: &str) -> Self {
        match s {
            "C__OBJTYPE__PERSON" => ObjectTypeConst::C__OBJTYPE__PERSON,
            "C__OBJTYPE__PERSON_GROUP" => ObjectTypeConst::C__OBJTYPE__PERSON_GROUP,
            _ => ObjectTypeConst::Other(s.to_owned()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_object_types() {
        let json = r#"
        [{
            "id": "5",
            "title": "Server",
            "container": "0",
            "const": "C__OBJTYPE__SERVER",
            "color": "A2BCFA",
            "image": "http://10.10.60.64/testinstance/images/objecttypes/server.png",
            "icon": "images/axialis/hardware-network/server-single.svg",
            "tree_group": "2",
            "status": "2",
            "type_group": "2",
            "type_group_title": "Infrastructure",
            "objectcount": "1"
        },
        {
            "id": "5",
            "title": "Server",
            "container": "0",
            "const": "C__OBJTYPE__SERVER",
            "color": "A2BCFA",
            "image": "http://10.10.60.64/testinstance/images/objecttypes/server.png",
            "icon": "images/axialis/hardware-network/server-single.svg",
            "tree_group": "2",
            "status": "2",
            "type_group": "2",
            "type_group_title": "Infrastructure",
            "objectcount": "1"
        }]
        "#;

        let object_types: ObjectTypes = serde_json::from_str(json).expect("failed to deserialize Object Types");

        assert_eq!(object_types.0.len(), 2);
    }

    #[test]
    fn test_deserialize_object_with_objectcount() {
        let json = r#"
        {
            "id": "5",
            "title": "Server",
            "container": "0",
            "const": "C__OBJTYPE__SERVER",
            "color": "A2BCFA",
            "image": "http://10.10.60.64/testinstance/images/objecttypes/server.png",
            "icon": "images/axialis/hardware-network/server-single.svg",
            "tree_group": "2",
            "status": "2",
            "type_group": "2",
            "type_group_title": "Infrastructure",
            "objectcount": "1"
        }
        "#;

        let object_type: ObjectType = serde_json::from_str(json).expect("failed to deserialize Object Type");

        assert_eq!(object_type.id, 5);
        assert_eq!(object_type.title, "Server");
        assert_eq!(object_type.container, "0");
        assert_eq!(object_type.constant, "C__OBJTYPE__SERVER");
        assert_eq!(object_type.color, "A2BCFA");
        assert_eq!(object_type.image, "http://10.10.60.64/testinstance/images/objecttypes/server.png");
        assert_eq!(object_type.icon, "images/axialis/hardware-network/server-single.svg");
        assert_eq!(object_type.tree_group, "2");
        assert_eq!(object_type.status, 2);
        assert_eq!(object_type.type_group, 2);
        assert_eq!(object_type.type_group_title, "Infrastructure");
        assert_eq!(object_type.objectcount, Some(1));
    }

    #[test]
    fn test_deserialize_object_without_objectcount() {
        let json = r#"
        {
            "id": "5",
            "title": "Server",
            "container": "0",
            "const": "C__OBJTYPE__SERVER",
            "color": "A2BCFA",
            "image": "http://10.10.60.64/testinstance/images/objecttypes/server.png",
            "icon": "images/axialis/hardware-network/server-single.svg",
            "tree_group": "2",
            "status": "2",
            "type_group": "2",
            "type_group_title": "Infrastructure"
        }
        "#;

        let object_type: ObjectType = serde_json::from_str(json).expect("failed to deserialize Object Type");

        assert_eq!(object_type.id, 5);
        assert_eq!(object_type.title, "Server");
        assert_eq!(object_type.container, "0");
        assert_eq!(object_type.constant, "C__OBJTYPE__SERVER");
        assert_eq!(object_type.color, "A2BCFA");
        assert_eq!(object_type.image, "http://10.10.60.64/testinstance/images/objecttypes/server.png");
        assert_eq!(object_type.icon, "images/axialis/hardware-network/server-single.svg");
        assert_eq!(object_type.tree_group, "2");
        assert_eq!(object_type.status, 2);
        assert_eq!(object_type.type_group, 2);
        assert_eq!(object_type.type_group_title, "Infrastructure");
        assert_eq!(object_type.objectcount, None);
    }
}
