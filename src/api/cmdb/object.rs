use crate::api::{ObjectTypeGroup, ObjectTypeGroups};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Objects(Vec<Object>);

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Object {
    pub(crate) id: u64,
    pub(crate) title: String,
    // todo: consider proper typing
    pub(crate) image: String,
    pub(crate) status: u8,
    pub(crate) sysid: String,

    #[serde(
        deserialize_with = "crate::util::deserialize_datetime",
        serialize_with = "crate::util::serialize_datetime"
    )]
    pub(crate) created: chrono::NaiveDateTime,
    #[serde(
        deserialize_with = "crate::util::deserialize_optional_datetime",
        serialize_with = "crate::util::serialize_optional_datetime"
    )]
    pub(crate) updated: Option<chrono::NaiveDateTime>,

    // Object's CMDB Status
    pub(crate) cmdb_status: u64,
    pub(crate) cmdb_status_title: String,

    // Object's Object Type
    #[serde(rename = "objecttype", alias = "type")]
    pub(crate) object_type: u64,
    pub(crate) type_title: String,

    // todo: included in cmdb.object.read but missing in cmdb.objects.read
    pub(crate) type_icon: Option<String>,

    // todo: included in cmdb.objects.read but missing in cmdb.object.read
    pub(crate) type_group_title: Option<String>,
}

impl IntoIterator for Objects {
    type Item = Object;
    type IntoIter = std::vec::IntoIter<Object>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_objects() {
        let json = r#"
        [{
            "id":67,
            "title":"Blue Shield Mobile Agent",
            "sysid":"SYSID_1773306525",
            "type":2,
            "created":"2026-03-12 10:07:38",
            "updated":"2026-03-12 10:08:12",
            "type_title":"Application",
            "type_group_title":"Software",
            "status":2,
            "cmdb_status":6,
            "cmdb_status_title":"in operation",
            "image":"\/cmdb\/object\/image\/67"
        },
        {
            "id":34,
            "title":"keepbit Office",
            "sysid":"SYSID_1764345659",
            "type":3,
            "created":"2025-11-28 17:00:25",
            "updated":"2025-12-17 16:14:22",
            "type_title":"Building",
            "type_group_title":"Infrastructure",
            "status":2,
            "cmdb_status":6,
            "cmdb_status_title":"in operation",
            "image":"\/cmdb\/object\/image\/34"
        },
        {
            "id":106,
            "title":"CMDB-Zabbix-Manager",
            "sysid":"SYSID_1785323161",
            "type":5,
            "created":"2026-07-29 13:04:15",
            "updated":"2026-07-29 13:05:13",
            "type_title":"Server",
            "type_group_title":"Infrastructure",
            "status":2,
            "cmdb_status":6,
            "cmdb_status_title":"in operation",
            "image":"\/cmdb\/object\/image\/106"
        }]
        "#;

        let objects: Objects = serde_json::from_str(json).expect("failed to deserialize Objects");

        assert_eq!(objects.0.len(), 3);
    }

    #[test]
    fn test_deserialize_object() {
        let json = r#"
        {
            "id": 22,
            "title": "Api System",
            "sysid": "SYSID_1323964020",
            "objecttype": 53,
            "type_title": "Persons",
            "type_icon": "/cmdb/object-type/image/53",
            "status": 2,
            "cmdb_status": 6,
            "cmdb_status_title": "in operation",
            "created": "2025-11-24 12:59:44",
            "updated": "2025-11-24 12:59:44",
            "image": "/cmdb/object/image/22"
        }
        "#;

        let object: Object = serde_json::from_str(json).expect("failed to deserialize Object");
    }
}
