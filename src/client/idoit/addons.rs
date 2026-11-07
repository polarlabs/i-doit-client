use crate::Client;
use crate::{api, model};
use serde::Serialize;

const METHOD: &str = "idoit.addons";

#[derive(Debug)]
pub struct AddonsReader<'a> {
    client: &'a Client,
    params: Option<RequestParams>,
}

#[derive(Serialize, Debug)]
struct RequestParams();

impl AddonsReader<'_> {
    pub(crate) fn new(client: &Client) -> AddonsReader {
        AddonsReader {
            client,
            params: None,
        }
    }

    pub async fn execute(&self) -> Option<model::Addons> {
        match self.client.post(METHOD, &self.params).await {
            None => None,
            Some(value) => match value {
                api::Result::Addons(o) => Some(model::Addons::from_api(o.addons)),
                _ => None,
            },
        }
    }
}
