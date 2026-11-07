use serde::{Deserialize, Serialize};

use crate::api;
use crate::api::Object;

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum Result {
    Addons(api::AddonsResult),
    Constants(api::Constants),
    Licensing(api::Licensing),
    Object(api::Object),
    Objects(api::Objects),
    ObjectType(api::ObjectType),
    ObjectTypes(api::ObjectTypes),
    ObjectTypeGroup(api::ObjectTypeGroup),
    ObjectTypeGroups(api::ObjectTypeGroups),
    Status(api::Status),
    Statuses(api::Statuses),

    Creator(CreatorResult),
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CreatorResult {
    id: u64,
    message: String,
    success: bool,
}

impl CreatorResult {
    pub fn id(&self) -> u64 {
        self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_creator_result() {
        let json = r#"
        {
            "id": 154,
            "message": "Object was successfully created",
            "success": true
        }
        "#;

        let result: CreatorResult = serde_json::from_str(json).expect("failed to deserialize Creator Result");
    }
}