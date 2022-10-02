use std::borrow::Cow;

use log::warn;
use serde::de::DeserializeOwned;
use sharesight_types::ApiEndpoint;

const DEFAULT_API_HOST: &str = "https://api.sharesight.com";

pub struct Client<'a> {
    client: Cow<'a, reqwest::Client>,
    api_host: String,
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

impl Client<'_> {
    pub fn new_with_token(access_token: String, api_host: String) -> Client<'static> {
        Client {
            api_host,
            credentials: Credentials::AccessToken(access_token),
            client: Cow::Owned(reqwest::Client::default()),
        }
    }

    pub async fn execute<'a, T: ApiEndpoint<'a>, U: DeserializeOwned>(
        &'a self,
        parameters: &'a T::Parameters,
    ) -> Result<U, SharesightReqwestError> {
        let client = self.client.as_ref();
        let method = match T::HTTP_METHOD {
            sharesight_types::ApiHttpMethod::Get => reqwest::Method::GET,
            sharesight_types::ApiHttpMethod::Post => reqwest::Method::POST,
            sharesight_types::ApiHttpMethod::Put => reqwest::Method::PUT,
            sharesight_types::ApiHttpMethod::Delete => reqwest::Method::DELETE,
        };
        let resp = client
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

impl AsRef<reqwest::Client> for Client<'_> {
    fn as_ref(&self) -> &reqwest::Client {
        self.client.as_ref()
    }
}

impl Default for Client<'_> {
    fn default() -> Self {
        Client {
            client: Cow::Owned(reqwest::Client::new()),
            api_host: DEFAULT_API_HOST.into(),
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
