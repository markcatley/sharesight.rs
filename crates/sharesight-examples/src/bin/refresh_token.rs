use clap::Parser;
use sharesight_types::{Auth, AuthWithDetails};

/// Auth with an OAuth2 Authorization Code using the Sharesight API
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// A file to read and write auth details from and to.
    #[clap(parse(from_os_str))]
    file: std::path::PathBuf,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let auth_arg = serde_json::from_reader::<_, AuthWithDetails>(std::fs::File::open(&args.file)?)?;
    let client = reqwest::Client::new();

    let params = [
        ("grant_type", "refresh_token"),
        ("refresh_token", &auth_arg.auth.refresh_token.unwrap()),
        ("client_id", &auth_arg.client_id),
        ("client_secret", &auth_arg.client_secret),
    ];
    let resp = client
        .post(format!("https://{}/oauth2/token", auth_arg.host))
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

        let file = std::fs::File::create(&args.file)?;
        let auth = AuthWithDetails { auth, ..auth_arg };

        serde_json::to_writer_pretty(file, &auth)?;
    } else {
        println!("{:?}", resp);
        println!("{}", resp.text().await?);

        std::process::exit(-1);
    }

    Ok(())
}
