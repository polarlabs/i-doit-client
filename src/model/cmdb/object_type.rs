use crate::client::{cache, Error};
use crate::{api, model};

#[derive(Debug)]
pub struct ObjectTypes(Vec<ObjectType>);

#[derive(Clone, Debug)]
pub struct ObjectType {
    pub(crate) id: ObjectTypeId,
    pub(crate) title: String,
    pub(crate) container: String,
    pub(crate) constant: String,
    pub(crate) color: String,
    pub(crate) image: String,
    pub(crate) icon: String,
    pub(crate) tree_group: String,
    pub(crate) status: model::RecordStatus,
    pub(crate) object_type_group: model::ObjectTypeGroup,
    pub(crate) objectcount: Option<u64>,
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub(crate) struct ObjectTypeId(pub(crate) u64);

impl ObjectTypes {
    pub(crate) fn from_api(
        object_types: api::ObjectTypes,
        object_type_group_cache: &cache::ObjectTypeGroup
    ) -> Result<ObjectTypes, Error> {
        let object_types: Result<Vec<ObjectType>, _> = object_types
            .into_iter()
            .map(|o| ObjectType::from_api(o, object_type_group_cache))
            .collect();

        object_types.map(|o| ObjectTypes(o))
    }
    
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl ObjectType {
    pub(crate) fn from_api(object_type: api::ObjectType, object_type_group_cache: &cache::ObjectTypeGroup) -> Result<ObjectType, Error> {
        let record_status = model::RecordStatus::from_id(object_type.status)?;
        let object_type_group = object_type_group_cache.get(object_type.type_group).cloned().unwrap();

        Ok(ObjectType {
            id: ObjectTypeId(object_type.id),
            title: object_type.title,
            container: object_type.container,
            constant: object_type.constant,
            color: object_type.color,
            image: object_type.image,
            icon: object_type.icon,
            tree_group: object_type.tree_group,
            status: record_status,
            object_type_group,
            objectcount: object_type.objectcount
        })
    }

    pub(crate) fn id(&self) -> ObjectTypeId {
        self.id.clone()
    }
}

impl IntoIterator for ObjectTypes {
    type Item = ObjectType;
    type IntoIter = std::vec::IntoIter<ObjectType>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
