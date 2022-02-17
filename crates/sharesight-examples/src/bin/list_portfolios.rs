use clap::Parser;
use sharesight_examples::init_logger;
use sharesight_reqwest::{Client, SharesightReqwestError};
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
    let client = Client::new();

    let response = client
        .execute::<PortfolioList, PortfolioListSuccess>(&args.api_host, &args.access_token, &())
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
