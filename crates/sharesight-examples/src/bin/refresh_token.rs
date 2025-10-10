use clap::Parser;
use sharesight_reqwest::Client;

/// Auth with an OAuth2 Authorization Code using the Sharesight API
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long, short)]
    token_only: bool,
    /// JSON file including api host, client_id and client_secret.
    client_credentials_file: std::path::PathBuf,
    /// The access token to use the api.
    user_credentials_file: std::path::PathBuf,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let _client = Client::new(args.user_credentials_file, args.client_credentials_file).await?;

    Ok(())
}
