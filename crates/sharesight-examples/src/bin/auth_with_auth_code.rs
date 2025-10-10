use clap::Parser;
use serde::Deserialize;
use serde_json::Value;
use sharesight_reqwest::ClientCredentials;
use sharesight_types::DEFAULT_API_HOST;

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
    /// JSON file including api host, client_id and client_secret.
    client_credentials_file: std::path::PathBuf,
    /// The access token to use the api.
    user_credentials_file: std::path::PathBuf,
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

        let client_credentials = ClientCredentials {
            host: args.api_host,
            client_id: args.client_id.into(),
            client_secret: args.client_secret.into(),
        };
        serde_json::to_writer_pretty(
            std::fs::File::create(args.client_credentials_file)?,
            &client_credentials,
        )?;
        let user_credentials = [
            ("access_token", Value::String(auth.access_token)),
            (
                "id_token",
                auth.refresh_token.map(Value::String).unwrap_or(Value::Null),
            ),
            ("lifetime", Value::Number(auth.expires_in.into())),
            ("issued", Value::Number(auth.created_at.into())),
            (
                "stale",
                Value::Number((auth.created_at + auth.expires_in as i64).into()),
            ),
            (
                "expiry",
                Value::Number((auth.created_at + 365 * 24 * 60 * 60).into()),
            ),
        ];
        serde_json::to_writer_pretty(
            std::fs::File::create(args.user_credentials_file)?,
            &user_credentials,
        )?;
    } else {
        println!("{:?}", resp);

        std::process::exit(-1);
    }

    Ok(())
}

#[derive(Debug, Deserialize)]
pub struct Auth {
    pub access_token: String,
    pub expires_in: u32,
    pub refresh_token: Option<String>,
    pub created_at: i64,
}
