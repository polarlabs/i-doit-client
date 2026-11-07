use serde::Serialize;

use crate::Client;
use crate::{api, model};

const METHOD: &str = "idoit.constants";

#[derive(Debug)]
pub struct ConstantsReader<'a> {
    client: &'a Client,
    params: Option<RequestParams>,
}

#[derive(Serialize, Debug)]
struct RequestParams();

impl ConstantsReader<'_> {
    pub(crate) fn new(client: &Client) -> ConstantsReader {
        ConstantsReader {
            client,
            params: None,
        }
    }

    pub async fn execute(&self) -> Option<model::Constants> {
        match self.client.post(METHOD, &self.params).await {
            None => None,
            Some(value) => match value {
                api::Result::Constants(o) => Some(model::Constants::from_api(o)),
                _ => None,
            },
        }
    }
}
