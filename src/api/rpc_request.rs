use crate::api::Params;
use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct RPCRequest<'a, T>
where
    T: Serialize,
{
    version: &'static str,
    method: String,
    params: &'a Params<'a, T>,
    id: u64,
}

impl<T> RPCRequest<'_, T>
where
    T: Serialize,
{
    pub(crate) fn new<'a>(
        method: &str,
        params: &'a Params<T>,
        request_id: u64,
    ) -> RPCRequest<'a, T> {
        RPCRequest {
            version: "2.0",
            method: method.to_string(),
            params,
            id: request_id,
        }
    }
}
