use clap::Parser;
use sharesight_examples::init_logger;
use sharesight_reqwest::{Client, SharesightReqwestError};
use sharesight_types::{GroupsList, GroupsListSuccess};

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
    let client = Client::new_with_token(args.access_token, args.api_host);

    let response = client.execute::<GroupsList, GroupsListSuccess>(&()).await;
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
