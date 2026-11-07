use crate::api;
use crate::client::Error;
use crate::model::RecordStatus;

#[derive(Debug)]
pub struct ObjectTypeGroups(Vec<ObjectTypeGroup>);

#[derive(Clone, Debug)]
pub struct ObjectTypeGroup {
    id: ObjectTypeGroupId,
    title: String,
    constant: String,
    status: RecordStatus,
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub(crate) struct ObjectTypeGroupId(pub(crate) u64);

impl ObjectTypeGroups {
    pub(crate) fn from_api(object_type_groups: api::ObjectTypeGroups) -> Result<ObjectTypeGroups, Error> {
        let object_type_groups: Result<Vec<ObjectTypeGroup>, _> = object_type_groups
            .into_iter()
            .map(|o| ObjectTypeGroup::from_api(o))
            .collect();

        object_type_groups.map(|o| ObjectTypeGroups(o))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl ObjectTypeGroup {
    pub(crate) fn from_api(object_type_group: api::ObjectTypeGroup) -> Result<ObjectTypeGroup, Error> {
        let record_status = RecordStatus::from_id(object_type_group.status)?;

        Ok(ObjectTypeGroup {
            id: ObjectTypeGroupId(object_type_group.id),
            title: object_type_group.title,
            constant: object_type_group.constant,
            status: record_status,
        })
    }

    pub(crate) fn id(&self) -> ObjectTypeGroupId {
        self.id.clone()
    }
}

impl IntoIterator for ObjectTypeGroups {
    type Item = ObjectTypeGroup;
    type IntoIter = std::vec::IntoIter<ObjectTypeGroup>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
