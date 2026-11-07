use super::client::Client;
use crate::client::{ConfigError, Error};
use crate::{model, Language};

pub struct ClientBuilder {
    http_client: Option<reqwest::Client>,
    url: Option<String>,
    endpoint: Option<url::Url>,
    api_key: Option<String>,
    language: Option<model::Language>,
    timezone: Option<chrono_tz::Tz>,
}

/*
Normal usage:
let client = Client::builder()
    .url("https://idoit.example/api")
    .api_key("secret")
    .build()?;

Custom HTTP client:
let http = reqwest::Client::builder()
    .timeout(std::time::Duration::from_secs(30))
    .user_agent("my-idoit-client/1.0")
    .build()?;

let client = Client::builder()
    .http_client(http)
    .url("https://idoit.example/api")
    .api_key("secret")
    .build()?;

*/

impl ClientBuilder {
    pub fn new() -> ClientBuilder {
        ClientBuilder {
            http_client: None,
            url: None,
            endpoint: None,
            api_key: None,
            language: None,
            timezone: None,
        }
    }

    pub fn http_client(mut self, client: reqwest::Client) -> Self {
        self.http_client = Some(client);
        self
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    pub fn api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }
    
    pub fn language(mut self, language: Language) -> Self {
        self.language = Some(language);
        self
    }

    pub fn timezone(mut self, timezone: chrono_tz::Tz) -> Self {
        self.timezone = Some(timezone);
        self
    }

    pub fn build(self) -> Result<Client, Error> {
        let http_client = self.http_client.unwrap_or_else(reqwest::Client::default);

        let url = self
            .url
            .ok_or(Error::Config(ConfigError::MissingEndpoint))?;
        let endpoint =
            url::Url::parse(&url).map_err(|err| Error::Config(ConfigError::UrlParseError(err)))?;

        let api_key = self
            .api_key
            .ok_or(Error::Config(ConfigError::MissingApiKey))?;
        let language = self.language.unwrap_or_else(model::Language::default);

        let timezone = match self.timezone {
            Some(tz) => tz,

            None => {
                let name = iana_time_zone::get_timezone()
                    .map_err(|err| Error::Config(ConfigError::LocalTimezoneError(err)))?;

                name.parse()
                    .map_err(|_| Error::Config(ConfigError::InvalidTimezone(name)))?
            }
        };

        Ok(Client::new(
            http_client,
            endpoint,
            api_key,
            language,
            timezone,
        ))
    }
}
