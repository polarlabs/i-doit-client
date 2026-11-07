use serde::Serialize;
use std::sync::atomic::AtomicU64;

use crate::api::rpc_request::RPCRequest;
use crate::{api, model};
use crate::client::{ClientBuilder};
use crate::client::{cache, cmdb, idoit};

const INITIAL_REQUEST_ID: u64 = 1;

#[derive(Debug)]
pub struct Client {
    http_client: reqwest::Client,
    endpoint: url::Url,
    timezone: chrono_tz::Tz,

    common_params: api::CommonParams,

    next_request_id: AtomicU64,

    cmdb_status_cache: Option<cache::CmdbStatus>,
    object_type_cache: Option<cache::ObjectType>,
    object_type_group_cache: Option<cache::ObjectTypeGroup>,
}

impl Client {
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    // Not exposed to crate users
    pub(crate) fn new(
        http_client: reqwest::Client,
        endpoint: url::Url,
        api_key: String,
        language: model::Language,
        timezone: chrono_tz::Tz,
    ) -> Client {
        let common_params = api::CommonParams::new(api_key, language);

        Client {
            http_client,
            endpoint,
            timezone,
            common_params,

            next_request_id: AtomicU64::new(INITIAL_REQUEST_ID),
            cmdb_status_cache: None,
            object_type_cache: None,
            object_type_group_cache: None,
        }
    }

    pub(crate) fn next_request_id(&self) -> u64 {
        self.next_request_id
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    }

    pub async fn cache_update(&mut self) -> &mut Client {
        match self.status().execute().await {
            None => {}
            Some(statuses) => {
                let mut cache = cache::CmdbStatus::new();
                cache.insert_all(statuses);
                self.cmdb_status_cache = Some(cache);
            }
        }

        match self.object_type_groups().execute().await {
            None => {}
            Some(object_type_groups) => {
                let mut cache = cache::ObjectTypeGroup::new();
                cache.insert_all(object_type_groups);
                self.object_type_group_cache = Some(cache);
            }
        }

        match self.object_types().execute().await {
            None => {}
            Some(object_types) => {
                let mut cache = cache::ObjectType::new();
                cache.insert_all(object_types);
                self.object_type_cache = Some(cache);
            }
        }

        self
    }

    pub(crate) fn common_params(&self) -> &api::CommonParams {
        &self.common_params
    }

    pub(crate) fn cmdb_status_cache(&self) -> &cache::CmdbStatus {
        self.cmdb_status_cache.as_ref().unwrap()
    }

    pub(crate) fn object_type_cache(&self) -> &cache::ObjectType {
        self.object_type_cache.as_ref().unwrap()
    }

    pub(crate) fn object_type_group_cache(&self) -> &cache::ObjectTypeGroup {
        self.object_type_group_cache.as_ref().unwrap()
    }

    pub(crate) fn timezone(&self) -> chrono_tz::Tz {
        self.timezone
    }

    pub fn object_type_groups(&self) -> cmdb::ObjectTypeGroupsReader {
        cmdb::ObjectTypeGroupsReader::new(self)
    }

    /// idoit.addons
    pub fn addons(&self) -> idoit::AddonsReader {
        idoit::AddonsReader::new(self)
    }

    /// idoit.constants
    pub fn constants(&self) -> idoit::ConstantsReader {
        idoit::ConstantsReader::new(self)
    }

    /// idoit.license
    pub fn license(&self) -> idoit::LicenseReader {
        idoit::LicenseReader::new(self)
    }

    /// cmdb.object.create
    pub fn create_object(&self) -> cmdb::ObjectCreator {
        cmdb::ObjectCreator::new(self)
    }
    
    /// cmdb.object.read
    pub fn object(&self) -> cmdb::ObjectReader {
        cmdb::ObjectReader::new(self)
    }

    /// cmdb.objects.read
    pub fn objects(&self) -> cmdb::ObjectsReader {
        cmdb::ObjectsReader::new(self)
    }

    pub fn object_types(&self) -> cmdb::ObjectTypesReader {
        cmdb::ObjectTypesReader::new(self)
    }

    pub fn status(&self) -> cmdb::StatusReader {
        cmdb::StatusReader::new(self)
    }

    pub(crate) async fn post<T>(&self, method: &str, params: &T) -> Option<api::Result>
    where
        T: Serialize,
    {
        let params = api::Params {
            common: &self.common_params(),
            request: params,
        };

        let id = self.next_request_id();

        let rpc_request = RPCRequest::new(method, &params, id);

        println!(
            "Request: {}",
            serde_json::to_string_pretty(&rpc_request).unwrap()
        );

        let response = self
            .http_client
            .post(self.endpoint.clone())
            .json(&rpc_request)
            .send()
            .await;

        // Local error handling
        match response {
            Ok(response) => {
                if !response.status().is_success() {
                    unimplemented!();
                    None
                } else {
                    let body = response.text().await.unwrap();

                    let response: Result<api::RPCResponse, _> = serde_json::from_str(&body);
                    match response {
                        Ok(response) => {
                            // todo: do this only when debug is enabled
                            // println!("Body: {}", body);
                            response.result()
                        },
                        Err(error) => {
                            println!("Error: {}", error);
                            println!("Body: {}", body);
                            None
                        }
                    }
                }
            }
            Err(error) => {
                unimplemented!();
                None
            }
        }
    }
}

#[derive(Serialize, Debug)]
struct ObjectReadParams {
    #[serde(rename = "id")]
    object_id: u64,
}
