use serde::Serialize;

use crate::Client;
use crate::{api, model};

const METHOD: &str = "cmdb.object.create";

#[derive(Debug)]
pub struct ObjectCreator<'a> {
    client: &'a Client,
    params: Option<RequestParams>,
}

#[derive(Serialize, Debug)]
struct RequestParams {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    object_type: Option<api::ObjectTypeConst>,

    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
}

impl ObjectCreator<'_> {
    pub(crate) fn new(client: &Client) -> ObjectCreator {
        ObjectCreator {
            client,
            params: None,
        }
    }

    pub fn object_type(&mut self, object_type: impl Into<api::ObjectTypeConst>) -> &mut Self {
        match &mut self.params {
            None => {
                let mut params = RequestParams::new();
                params.object_type(object_type.into());
                self.params = Some(params);
            },
            Some(params) => params.object_type = Some(object_type.into()),
        }

        self
    }

    pub fn title(&mut self, title: impl Into<String>) -> &mut Self {
        match &mut self.params {
            None => {
                let mut params = RequestParams::new();
                params.title(title.into());
                self.params = Some(params);
            },
            Some(params) => params.title = Some(title.into()),
        }

        self
    }

    pub async fn execute(&self) -> Option<api::CreatorResult> {
        match self.client.post(METHOD, &self.params).await {
            None => None,
            Some(value) => match value {
                api::Result::Creator(result) => {
                    Some(result)
                }
                _ => None,
            },
        }
    }
}

impl RequestParams {
    fn new() -> RequestParams {
        RequestParams {
            object_type: None,
            title: None,
        }
    }

    fn object_type(&mut self, object_type: impl Into<api::ObjectTypeConst>) {
        self.object_type = Some(object_type.into());
    }

    fn title(&mut self, title: impl Into<String>) {
        self.title = Some(title.into());
    }
}
