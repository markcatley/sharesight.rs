use clap::Parser;
use sharesight_examples::init_logger;
use sharesight_reqwest::Client;
use sharesight_types::{
    PortfolioList, PortfolioListSuccess, Valuation, ValuationParameters, ValuationSuccess,
    DEFAULT_API_HOST,
};

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

    let PortfolioListSuccess { portfolios, .. } = client
        .execute::<PortfolioList, PortfolioListSuccess>(&())
        .await?;

    let portfolio = portfolios.iter().find(|p| p.name == portfolio_name);

    if let Some(portfolio) = portfolio {
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
    } else {
        eprint!("Unknown portfolio: {}, ", portfolio_name);

        let mut names = portfolios.iter().map(|p| p.name.as_str());

        match (names.next(), names.next_back()) {
            (Some(name_start), Some(name_end)) => {
                eprint!("the portfolios are: {}", name_start);
                for name in names {
                    eprint!(", {}", name);
                }
                eprintln!(" or {}", name_end);
            }
            (Some(name), None) => {
                eprintln!("the only portfolio is: {}", name);
            }
            (None, None) => {
                eprintln!("there are no portfolios");
            }
            _ => unreachable!(),
        }
    }

    Ok(())
}
