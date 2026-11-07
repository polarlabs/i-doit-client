use std::collections::HashMap;
use crate::model;

#[derive(Debug)]
pub(crate) struct CmdbStatus {
    statuses: HashMap<model::StatusId, model::Status>,
}

impl CmdbStatus {
    pub(crate) fn new() -> Self {
        Self {
            statuses: HashMap::new(),
        }
    }

    pub(crate) fn get(&self, id: u64) -> Option<&model::Status> {
        self.statuses.get(&model::StatusId(id))
    }

    pub fn insert_all(&mut self, statuses: model::Statuses) {
        self.statuses = statuses
            .into_iter()
            .map(|status| (status.id(), status))
            .collect();
    }

    pub fn clear(&mut self) {
        self.statuses.clear();
    }
}
