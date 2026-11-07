use std::collections::HashMap;
use crate::model;

#[derive(Debug)]
pub(crate) struct ObjectType {
    object_types: HashMap<model::ObjectTypeId, model::ObjectType>,
}

impl ObjectType {
    pub(crate) fn new() -> Self {
        Self {
            object_types: HashMap::new(),
        }
    }

    pub(crate) fn get(&self, id: u64) -> Option<&model::ObjectType> {
        self.object_types.get(&model::ObjectTypeId(id))
    }

    pub fn insert_all(&mut self, object_types: model::ObjectTypes) {
        self.object_types = object_types
            .into_iter()
            .map(|object_type| (object_type.id(), object_type))
            .collect();
    }

    pub fn clear(&mut self) {
        self.object_types.clear();
    }
}
