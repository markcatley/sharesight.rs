use clap::Parser;
use sharesight_examples::init_logger;
use sharesight_reqwest::Client;
use sharesight_types::{Valuation, ValuationParameters, ValuationSuccess};

/// Generate a 'valuation' report using the sharesight API
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The name of the portfolio to report the valuation for.
    portfolio_name: String,
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
    let portfolio_name = args.portfolio_name;

    let portfolios = client.build_portfolio_index().await?;
    let portfolio = portfolios.find(&portfolio_name).unwrap_or_else(|| {
        portfolios.log_error_for(&portfolio_name);
        std::process::exit(0)
    });

    let performance_parameters = ValuationParameters {
        portfolio_id: portfolio.id,
        consolidated: None,
        include_sales: Some(true),
        grouping: None,
        custom_group_id: None,
        balance_date: None,
    };
    let performance_report = client
        .execute::<Valuation, ValuationSuccess>(&performance_parameters)
        .await?;

    println!(
        "Valuation report for portfolio '{}' as of {}",
        portfolio.name, performance_report.balance_date
    );
    println!("{:#?}", performance_report);

    Ok(())
}
