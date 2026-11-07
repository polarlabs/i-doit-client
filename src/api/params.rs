use serde::Serialize;

// todo: check best way to make request params optional in Params: either Option<&T> or &Option<T>

#[derive(Serialize, Debug)]
pub(crate) struct Params<'a, T>
where
    T: Serialize,
{
    #[serde(flatten)]
    pub(crate) common: &'a CommonParams,

    #[serde(flatten)]
    pub(crate) request: &'a T,
}

#[derive(Serialize, Debug)]
pub struct CommonParams {
    #[serde(rename = "apikey")]
    api_key: String,
    language: crate::Language,
}

impl CommonParams {
    pub fn new(api_key: String, language: crate::Language) -> Self {
        CommonParams { api_key, language }
    }
}
