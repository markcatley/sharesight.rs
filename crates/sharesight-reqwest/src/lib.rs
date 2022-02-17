use std::borrow::Cow;

use log::warn;
use serde::de::DeserializeOwned;
use sharesight_types::ApiEndpoint;

pub struct Client<'a> {
    client: Cow<'a, reqwest::Client>,
}

impl Client<'_> {
    pub fn new() -> Client<'static> {
        Default::default()
    }

    pub async fn execute<'a, T: ApiEndpoint<'a>, U: DeserializeOwned>(
        &self,
        api_host: &'a str,
        access_token: &str,
        parameters: &'a T::Parameters,
    ) -> Result<U, SharesightReqwestError> {
        let client = self.client.as_ref();
        let resp = client
            .get(T::url(api_host, parameters).to_string())
            .bearer_auth(access_token)
            .json(parameters)
            .send()
            .await?;

        if resp.status().is_success() {
            let full = resp.bytes().await?;

            Ok(serde_json::from_slice(&full).map_err(|e| {
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
