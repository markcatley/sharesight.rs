use std::sync::Arc;

use log::warn;
use serde::de::DeserializeOwned;
use sharesight_types::ApiEndpoint;

const DEFAULT_API_HOST: &str = "https://api.sharesight.com";

pub struct Client {
    client: reqwest::Client,
    api_host: Arc<String>,
    credentials: Credentials,
}

enum Credentials {
    None,
    AccessToken(String),
}

impl Credentials {
    fn access_token(&self) -> &str {
        match self {
            Credentials::None => "",
            Credentials::AccessToken(t) => t,
        }
    }
}

impl Client {
    pub fn new_with_token_and_host(access_token: String, api_host: String) -> Self {
        Client {
            api_host: Arc::new(api_host),
            credentials: Credentials::AccessToken(access_token),
            client: reqwest::Client::default(),
        }
    }

    pub async fn execute<'a, T: ApiEndpoint<'a>, U: DeserializeOwned>(
        &'a self,
        parameters: &'a T::Parameters,
    ) -> Result<U, SharesightReqwestError> {
        let method = match T::HTTP_METHOD {
            sharesight_types::ApiHttpMethod::Get => reqwest::Method::GET,
            sharesight_types::ApiHttpMethod::Post => reqwest::Method::POST,
            sharesight_types::ApiHttpMethod::Put => reqwest::Method::PUT,
            sharesight_types::ApiHttpMethod::Delete => reqwest::Method::DELETE,
        };
        let resp = self
            .client
            .request(method, T::url(&self.api_host, parameters).to_string())
            .bearer_auth(self.credentials.access_token())
            .json(parameters)
            .send()
            .await?;

        if resp.status().is_success() {
            let full = resp.bytes().await?;

            let slice = if full.is_empty() {
                b"null".as_slice()
            } else {
                &full
            };

            Ok(serde_json::from_slice(slice).map_err(|e| {
                if let Ok(s) = std::str::from_utf8(&full) {
                    warn!("Error deserializing json: {:?}\n{}", e, s);
                } else {
                    warn!("Error deserializing json - not valid utf-8: {:?}", e);
                }
                e
            })?)
        } else {
            Err(SharesightReqwestError::Http(resp))
        }
    }
}

impl AsRef<reqwest::Client> for Client {
    fn as_ref(&self) -> &reqwest::Client {
        &self.client
    }
}

impl Default for Client {
    fn default() -> Self {
        Client {
            client: reqwest::Client::new(),
            api_host: Arc::new(DEFAULT_API_HOST.to_string()),
            credentials: Credentials::None,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SharesightReqwestError {
    #[error("Http request returned non-success status code\n{0:?}")]
    Http(reqwest::Response),
    #[error("Http error occurred\n{0:?}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Deserialize error occurred\n{0:?}")]
    Deserialize(#[from] serde_json::Error),
}
