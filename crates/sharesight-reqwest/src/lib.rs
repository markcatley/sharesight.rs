use std::{future::Future, path::PathBuf, pin::Pin, sync::Arc};

use log::warn;
use serde::de::DeserializeOwned;
use sharesight_types::{ApiEndpoint, AuthWithDetails, DEFAULT_API_HOST};

pub struct Client<T> {
    client: reqwest::Client,
    api_host: Arc<String>,
    auth: T,
}

impl<T> Client<T> {
    pub fn new_with_auth(auth: T) -> Self {
        Self::new_with_auth_and_host(auth, DEFAULT_API_HOST.to_string())
    }

    pub fn new_with_auth_and_host(auth: T, api_host: String) -> Self {
        Client {
            api_host: Arc::new(api_host),
            auth,
            client: reqwest::Client::default(),
        }
    }
}

impl Client<String> {
    pub fn new_with_token_and_host(access_token: String, api_host: String) -> Self {
        Self::new_with_auth_and_host(access_token, api_host)
    }
}

impl<T: GetToken> Client<T> {
    pub async fn execute<'a, U: ApiEndpoint<'a>, V: DeserializeOwned>(
        &'a self,
        parameters: &'a U::Parameters,
    ) -> Result<V, SharesightReqwestError> {
        let method = match U::HTTP_METHOD {
            sharesight_types::ApiHttpMethod::Get => reqwest::Method::GET,
            sharesight_types::ApiHttpMethod::Post => reqwest::Method::POST,
            sharesight_types::ApiHttpMethod::Put => reqwest::Method::PUT,
            sharesight_types::ApiHttpMethod::Delete => reqwest::Method::DELETE,
        };
        let url = U::url(&self.api_host, parameters).to_string();
        let auth_token = self.auth.get_token().await?;
        let resp = self
            .client
            .request(method.clone(), &url)
            .bearer_auth(auth_token)
            .json(parameters)
            .send()
            .await?;

        let resp = if resp.status() != 401 {
            resp
        } else {
            let auth_token = self.auth.refresh_token().await?;
            self.client
                .request(method, url)
                .bearer_auth(auth_token)
                .json(parameters)
                .send()
                .await?
        };

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

impl<T> AsRef<reqwest::Client> for Client<T> {
    fn as_ref(&self) -> &reqwest::Client {
        &self.client
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
    #[error("Failed to initialise from file")]
    InitialiseFromFile(PathBuf),
    #[error("Cannot refresh token due to type or expired refresh token")]
    CannotRefreshToken,
}

type TokenResult = Result<String, SharesightReqwestError>;

pub trait GetToken: GetTokenClone {
    fn get_token<'a>(&'a self) -> Pin<Box<dyn Future<Output = TokenResult> + 'a>>;

    fn refresh_token<'a>(&'a self) -> Pin<Box<dyn Future<Output = TokenResult> + 'a>> {
        Box::pin(async move { Err(SharesightReqwestError::CannotRefreshToken) })
    }
}

impl Clone for Box<dyn GetToken> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

pub trait GetTokenClone {
    fn clone_box(&self) -> Box<dyn GetToken>;
}

impl<T> GetTokenClone for T
where
    T: 'static + GetToken + Clone,
{
    fn clone_box(&self) -> Box<dyn GetToken> {
        Box::new(self.clone())
    }
}

impl GetToken for String {
    fn get_token<'a>(&'a self) -> Pin<Box<dyn Future<Output = TokenResult> + 'a>> {
        Box::pin(async move { Ok(self.clone()) })
    }
}

impl GetToken for AuthWithDetails {
    fn get_token<'a>(&'a self) -> Pin<Box<dyn Future<Output = TokenResult> + 'a>> {
        Box::pin(async move { Ok(self.auth.access_token.clone()) })
    }
}
