use clap::Parser;
use sharesight_examples::init_logger;
use sharesight_reqwest::Client;
use sharesight_types::{
    Currency, Market, PortfolioList, PortfolioListSuccess, TradeDescription, Trades,
    TradesParameters, TradesSuccess, TradesTradesSuccess,
};

/// List the portfolios using the Sharesight API
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The host to use to access the API.
    #[clap(long, default_value = "api.sharesight.com")]
    api_host: String,
    /// The name of the portfolio to list.
    portfolio_name: String,
    /// The access token to use the api.
    access_token: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_logger();

    let args = Args::parse();
    let client = Client::new();

    let PortfolioListSuccess { portfolios, .. } = client
        .execute::<PortfolioList, PortfolioListSuccess>(&args.api_host, &args.access_token, &())
        .await?;

    let portfolio = portfolios.iter().find(|p| p.name == args.portfolio_name);

    if let Some(portfolio) = portfolio {
        let trades_params = TradesParameters {
            portfolio_id: portfolio.id.to_string(),
            start_date: None,
            end_date: None,
            unique_identifier: None,
        };
        let TradesSuccess { trades, .. } = client
            .execute::<Trades, TradesSuccess>(&args.api_host, &args.access_token, &trades_params)
            .await?;

        #[derive(serde::Serialize)]
        pub struct TradesRecord {
            pub id: Option<i64>,
            pub unique_identifier: Option<String>,
            pub transaction_date: chrono::NaiveDate,
            pub quantity: f64,
            pub price: f64,
            pub cost_base: Option<f64>,
            pub exchange_rate: f64,
            pub brokerage: f64,
            pub brokerage_currency_code: Option<Currency>,
            pub value: f64,
            pub paid_on: Option<chrono::NaiveDate>,
            pub company_event_id: Option<i64>,
            pub comments: String,
            pub portfolio_id: i64,
            pub holding_id: i64,
            pub state: String,
            pub transaction_type: TradeDescription,
            pub instrument_id: i64,
            pub symbol: String,
            pub market: Market,
            pub attachment_filename: Option<String>,
            pub attachment_id: Option<i64>,
            pub confirmed: bool,
        }

        let mut wtr = csv::Writer::from_writer(std::io::stdout());

        for trade in trades.into_iter() {
            let TradesTradesSuccess {
                id,
                unique_identifier,
                transaction_date,
                quantity,
                price,
                cost_base,
                exchange_rate,
                brokerage,
                brokerage_currency_code,
                value,
                paid_on,
                company_event_id,
                comments,
                portfolio_id,
                holding_id,
                state,
                transaction_type,
                instrument_id,
                symbol,
                market,
                attachment_filename,
                attachment_id,
                confirmed,
            } = trade;
            wtr.serialize(TradesRecord {
                id,
                unique_identifier,
                transaction_date,
                quantity,
                price,
                cost_base,
                exchange_rate,
                brokerage,
                brokerage_currency_code,
                value,
                paid_on,
                company_event_id,
                comments,
                portfolio_id,
                holding_id,
                state,
                transaction_type,
                instrument_id,
                symbol,
                market,
                attachment_filename,
                attachment_id,
                confirmed,
            })?;
        }
    } else {
        eprint!("Unknown portfolio: {}, ", args.portfolio_name);

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
