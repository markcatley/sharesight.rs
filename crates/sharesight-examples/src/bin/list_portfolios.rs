use clap::Parser;
use serde::de::DeserializeOwned;
use sharesight_types::{ApiEndpoint, PortfolioList, PortfolioListSuccess};

/// List the portfolios using the Sharesight API
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The host to use to access the API.
    #[clap(long, default_value = "api.sharesight.com")]
    api_host: String,
    /// The access token to use the api.
    access_token: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let client = reqwest::Client::new();

    let response = execute::<PortfolioList, PortfolioListSuccess>(
        client,
        &args.api_host,
        &args.access_token,
        &(),
    )
    .await;

    match response {
        Ok(result) => println!("{:#?}", result),
        Err(SharesightReqwestError::HttpError(resp)) => {
            println!("{:?}", resp);
            println!("{}", resp.text().await?)
        }
        resp => {
            resp?;
        }
    }

    Ok(())
}

#[derive(Debug, thiserror::Error)]
enum SharesightReqwestError {
    #[error("Http request returned non-success status code\n{0:?}")]
    HttpError(reqwest::Response),
    #[error("Http error occurred\n{0:?}")]
    ReqwestError(#[from] reqwest::Error),
}

async fn execute<'a, T: ApiEndpoint<'a>, U: DeserializeOwned>(
    client: reqwest::Client,
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
        Ok(resp.json().await?)
    } else {
        Err(SharesightReqwestError::HttpError(resp))
    }
}
