use serde::Serialize;

use crate::Client;
use crate::{api, model};

const METHOD: &str = "cmdb.status.read";

#[derive(Debug)]
pub struct StatusReader<'a> {
    client: &'a Client,
    params: Option<RequestParams>,
}

#[derive(Serialize, Debug)]
pub struct RequestParams();

impl StatusReader<'_> {
    pub(crate) fn new(client: &Client) -> StatusReader {
        StatusReader {
            client,
            params: None,
        }
    }

    pub async fn execute(&self) -> Option<model::Statuses> {
        match self.client.post(METHOD, &self.params).await {
            None => None,
            Some(value) => match value {
                api::Result::Statuses(s) => Some(model::Statuses::from_api(s)),
                _ => None,
            },
        }
    }
}
