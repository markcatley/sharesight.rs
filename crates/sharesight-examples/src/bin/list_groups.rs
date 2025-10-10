use clap::Parser;
use sharesight_examples::init_logger;
use sharesight_reqwest::Client;
use sharesight_types::{GroupsList, GroupsListSuccess};

/// List the portfolios using the Sharesight API
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// JSON file including api host, client_id and client_secret.
    client_credentials_file: std::path::PathBuf,
    /// The access token to use the api.
    user_credentials_file: std::path::PathBuf,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_logger();

    let args = Args::parse();
    let client = Client::new(args.user_credentials_file, args.client_credentials_file).await?;

    let result = client.execute::<GroupsList, GroupsListSuccess>(&()).await?;

    println!("{:#?}", result);

    Ok(())
}
