use serde::Serialize;

use crate::Client;
use crate::{api, model};

const METHOD: &str = "cmdb.object.read";

#[derive(Debug)]
pub struct ObjectReader<'a> {
    client: &'a Client,
    params: Option<RequestParams>,
}

#[derive(Serialize, Debug)]
pub struct RequestParams {
    id: u64,
}

impl ObjectReader<'_> {
    pub(crate) fn new(client: &Client) -> ObjectReader {
        ObjectReader {
            client,
            params: None,
        }
    }

    pub fn id(&mut self, id: u64) -> &mut Self {
        self.params.get_or_insert_with(|| RequestParams::new(id));

        self
    }

    pub async fn execute(&self) -> Option<model::Object> {
        match self.client.post(METHOD, &self.params).await {
            None => None,
            Some(value) => match value {
                api::Result::Object(o) => {
                    match model::Object::from_api(o, self.client.timezone(), self.client.cmdb_status_cache(), self.client.object_type_cache()) {
                        Ok(object) => Some(object),
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

impl RequestParams {
    fn new(id: u64) -> RequestParams {
        RequestParams { id }
    }
}
