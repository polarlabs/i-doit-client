use crate::api;

#[derive(Debug)]
pub struct Statuses(Vec<Status>);

#[derive(Clone, Debug)]
pub struct Status {
    id: StatusId,
    title: String,
    constant: String,
    color: String,
    editable: bool,
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub(crate) struct StatusId(pub(crate) u64);

impl Statuses {
    pub(crate) fn from_api(
        statuses: api::Statuses
    ) -> Statuses {
        let statuses: Vec<Status> = statuses
            .into_iter()
            .map(|s| Status::from_api(s))
            .collect();

        Statuses(statuses)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl Status {
    pub(crate) fn from_api(status: api::Status) -> Status {
        Status {
            id: StatusId(status.id),
            title: status.title,
            constant: status.constant,
            color: status.color,
            editable: status.editable,
        }
    }

    pub(crate) fn id(&self) -> StatusId {
        self.id.clone()
    }
}

impl IntoIterator for Statuses {
    type Item = Status;
    type IntoIter = std::vec::IntoIter<Status>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
