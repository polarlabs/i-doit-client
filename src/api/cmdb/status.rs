use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub(crate) struct Statuses(Vec<Status>);

#[derive(Deserialize, Serialize, Debug, Clone)]
pub(crate) struct Status {
    pub(crate) id: u64,
    pub(crate) title: String,
    pub(crate) constant: String,
    pub(crate) color: String,
    pub(crate) editable: bool,
}

impl IntoIterator for Statuses {
    type Item = Status;
    type IntoIter = std::vec::IntoIter<Status>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_statuses() {
        let json = r#"
        [{
            "id": 5,
            "title": "tested",
            "constant": "C__CMDB_STATUS__TESTED",
            "color": "95C47C",
            "editable": true
        },
        {
            "id": 6,
            "title": "in operation",
            "constant": "C__CMDB_STATUS__IN_OPERATION",
            "color": "33C20A",
            "editable": false
        }]
        "#;

        let statuses: Statuses = serde_json::from_str(json).expect("failed to deserialize Statuses");

        assert_eq!(statuses.0.len(), 2);
    }

    #[test]
    fn test_deserialize_status() {
        let json = r#"
        {
            "id": 14,
            "title": "Template",
            "constant": "C__CMDB_STATUS__IDOIT_STATUS_TEMPLATE",
            "color": "CCCCCC",
            "editable": false
        }
        "#;

        let status: Status = serde_json::from_str(json).expect("failed to deserialize Status");

        assert_eq!(status.id, 14);
        assert_eq!(status.title, "Template");
        assert_eq!(status.constant, "C__CMDB_STATUS__IDOIT_STATUS_TEMPLATE");
        assert_eq!(status.color, "CCCCCC");
        assert_eq!(status.editable, false);
    }
}
