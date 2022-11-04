use clap::Parser;
use sharesight_types::{Auth, AuthWithDetails, DEFAULT_API_HOST};

/// Auth with an OAuth2 Authorization Code using the Sharesight API
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The host to use to access the API.
    #[clap(long, default_value = DEFAULT_API_HOST)]
    api_host: String,
    /// The redirect URI of the API application.
    #[clap(long, default_value = "urn:ietf:wg:oauth:2.0:oob")]
    redirect_uri: String,
    /// The client id of the API application.
    client_id: String,
    /// The client secret of the API application.
    client_secret: String,
    /// The authorization code of the user to use to access the API.
    authorization_code: String,
    /// A file to write the output to.
    #[clap(long, short, parse(from_os_str))]
    file: Option<std::path::PathBuf>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let client = reqwest::Client::new();

    let params = [
        ("grant_type", "authorization_code"),
        ("code", &args.authorization_code),
        ("redirect_uri", &args.redirect_uri),
        ("client_id", &args.client_id),
        ("client_secret", &args.client_secret),
    ];
    let resp = client
        .post(format!("https://{}/oauth2/token", args.api_host))
        .form(&params)
        .send()
        .await?;

    if resp.status().is_success() {
        let auth = resp.json::<Auth>().await?;

        println!("Access token: {}", auth.access_token);
        if let Some(refresh_token) = &auth.refresh_token {
            println!("Refresh token: {}", refresh_token);
        }
        println!("Expires in: {}s", auth.expires_in);
        println!("Created at: {}", auth.created_at);

        if let Some(path) = args.file {
            let file = std::fs::File::create(path)?;
            let auth = AuthWithDetails {
                auth,
                host: args.api_host,
                client_id: args.client_id,
                client_secret: args.client_secret,
            };

            serde_json::to_writer_pretty(file, &auth)?;
        }
    } else {
        println!("{:?}", resp);

        std::process::exit(-1);
    }

    Ok(())
}
