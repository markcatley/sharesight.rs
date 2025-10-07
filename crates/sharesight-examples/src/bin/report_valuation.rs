use clap::Parser;
use sharesight_examples::init_logger;
use sharesight_reqwest::Client;
use sharesight_types::{Valuation, ValuationParameters, ValuationSuccess, DEFAULT_API_HOST};

/// Generate a 'valuation' report using the sharesight API
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The host to use to access the API.
    #[clap(long, default_value = DEFAULT_API_HOST)]
    api_host: String,
    /// The name of the portfolio to report the valuation for.
    portfolio_name: String,
    /// The access token to use the api.
    access_token: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_logger();

    let args = Args::parse();
    let client = Client::new_with_token_and_host(args.access_token, args.api_host);
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
        portfolio.name, performance_report.portfolio_valuation.balance_date
    );
    println!("{:#?}", performance_report);

    Ok(())
}
