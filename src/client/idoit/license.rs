use serde::Serialize;

use crate::Client;
use crate::client::Error;
use crate::{api, model};

const METHOD: &str = "idoit.license";

#[derive(Debug)]
pub struct LicenseReader<'a> {
    client: &'a Client,
    params: Option<RequestParams>,
}

#[derive(Serialize, Debug)]
struct RequestParams();

impl LicenseReader<'_> {
    pub(crate) fn new(client: &Client) -> LicenseReader {
        LicenseReader {
            client,
            params: None,
        }
    }

    pub async fn execute(&self) -> Option<model::Licensing> {
        match self.client.post(METHOD, &self.params).await {
            None => None,
            Some(value) => match value {
                api::Result::Licensing(o) => {
                    match model::Licensing::from_api(o, self.client.timezone()) {
                        Ok(licensing) => Some(licensing),
                        Err(error) => {
                            println!("Error: {:?}", error);
                            None
                        }
                    }
                }
                _ => None,
            },
        }
    }
}
