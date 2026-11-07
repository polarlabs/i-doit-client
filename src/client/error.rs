//#[derive(Debug, thiserror::Error)]
#[derive(Debug)]
pub enum Error {
    Config(ConfigError),
    Deserialization(DeserializationError),
    Unexpected(String),
    /*
       #[error("HTTP request failed")]
       Http(#[from] reqwest::Error),

       #[error("failed to serialize request")]
       Serialization(#[from] serde_json::Error),

       #[error("i-doit API error: {0}")]
       Api(#[from] ApiError),

       #[error("unexpected API response: {0}")]
       InvalidResponse(String),     thiserror::Error
    */
}

#[derive(Debug)]
pub(crate) enum ConfigError {
    InvalidTimezone(String),
    LocalTimezoneError(iana_time_zone::GetTimezoneError),
    MissingApiKey,
    MissingEndpoint,
    UrlParseError(url::ParseError),
}

#[derive(Debug)]
pub(crate) enum DeserializationError {
    InvalidOrAmbiguousDateTime,
}
