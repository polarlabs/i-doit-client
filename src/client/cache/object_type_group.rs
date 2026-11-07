use std::collections::HashMap;
use crate::model;

#[derive(Debug)]
pub(crate) struct ObjectTypeGroup {
    object_type_groups: HashMap<model::ObjectTypeGroupId, model::ObjectTypeGroup>,
}

impl ObjectTypeGroup {
    pub(crate) fn new() -> Self {
        Self {
            object_type_groups: HashMap::new(),
        }
    }

    pub(crate) fn get(&self, id: u64) -> Option<&model::ObjectTypeGroup> {
        self.object_type_groups.get(&model::ObjectTypeGroupId(id))
    }

    pub fn insert_all(&mut self, object_type_groups: model::ObjectTypeGroups) {
        self.object_type_groups = object_type_groups
            .into_iter()
            .map(|object_type_group| (object_type_group.id(), object_type_group))
            .collect();
    }

    pub fn clear(&mut self) {
        self.object_type_groups.clear();
    }
}
