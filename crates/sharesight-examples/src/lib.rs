use std::env;

use serde::de::DeserializeOwned;
use sharesight_types::ApiEndpoint;

#[derive(Debug, thiserror::Error)]
pub enum SharesightReqwestError {
    #[error("Http request returned non-success status code\n{0:?}")]
    Http(reqwest::Response),
    #[error("Http error occurred\n{0:?}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Deserialize error occurred\n{0:?}")]
    Deserialize(#[from] serde_json::Error),
}

pub async fn execute<'a, T: ApiEndpoint<'a>, U: DeserializeOwned>(
    client: &reqwest::Client,
    api_host: &'a str,
    access_token: &str,
    parameters: &'a T::Parameters,
) -> Result<U, SharesightReqwestError> {
    let resp = client
        .get(T::url(api_host, parameters).to_string())
        .bearer_auth(access_token)
        .json(parameters)
        .send()
        .await?;

    if resp.status().is_success() {
        let full = resp.bytes().await?;

        Ok(serde_json::from_slice(&full).map_err(|e| {
            eprintln!("{:?}", e);
            if let Ok(s) = std::str::from_utf8(&full) {
                eprintln!("{}", s);
            }
            e
        })?)
    } else {
        Err(SharesightReqwestError::Http(resp))
    }
}

pub fn init_logger() {
    if Err(env::VarError::NotPresent) == env::var("RUST_LOG") {
        env::set_var("RUST_LOG", "warn");
    }

    env_logger::init();
}
