use serde::Deserialize;
use serde_json::Value;

use crate::api;

#[derive(Deserialize, Debug)]
pub(crate) struct RPCResponse {
    id: u64,
    jsonrpc: String,
    result: Option<api::Result>,
    error: Option<Value>,
}

impl RPCResponse {
    pub(crate) fn result(&self) -> Option<api::Result> {
        self.result.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_rpc_response() {
        let json = r#"
        {
            "id": 1,
            "jsonrpc": "2.0",
            "result": {
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
        }
        "#;

        let response: RPCResponse =
            serde_json::from_str(json).expect("failed to deserialize Response");

        assert_eq!(response.id, 1);
        assert_eq!(response.jsonrpc, "2.0");
    }
}
