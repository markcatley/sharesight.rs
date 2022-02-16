use clap::Parser;
use sharesight_examples::{execute, init_logger, SharesightReqwestError};
use sharesight_types::{PortfolioList, PortfolioListSuccess};

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
    init_logger();

    let args = Args::parse();
    let client = reqwest::Client::new();

    let response = execute::<PortfolioList, PortfolioListSuccess>(
        &client,
        &args.api_host,
        &args.access_token,
        &(),
    )
    .await;

    match response {
        Ok(result) => println!("{:#?}", result),
        Err(SharesightReqwestError::Http(resp)) => {
            println!("{:?}", resp);
            println!("{}", resp.text().await?)
        }
        resp => {
            resp?;
        }
    }

    Ok(())
}
