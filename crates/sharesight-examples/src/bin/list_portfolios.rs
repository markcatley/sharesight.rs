use clap::Parser;
use sharesight_examples::init_logger;
use sharesight_reqwest::Client;
use sharesight_types::{
    PortfolioList, PortfolioListParameters, PortfolioListSuccess, DEFAULT_API_HOST,
};

/// List the portfolios using the Sharesight API
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The host to use to access the API.
    #[clap(long, default_value = DEFAULT_API_HOST)]
    api_host: String,
    /// The access token to use the api.
    access_token: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_logger();

    let args = Args::parse();
    let client = Client::new_with_token_and_host(args.access_token, args.api_host);

    let parameters = PortfolioListParameters {
        consolidated: Some(true),
        instrument_id: None,
    };
    let result = client
        .execute::<PortfolioList, PortfolioListSuccess>(&parameters)
        .await?;

    println!("{:#?}", result);

    Ok(())
}
