use chrono::NaiveDate;
use clap::Parser;
use sharesight_examples::init_logger;
use sharesight_reqwest::Client;
use sharesight_types::{
    Currency, ListPortfolioPayouts, ListPortfolioPayoutsParameters,
    ListPortfolioPayoutsPayoutsSuccess, ListPortfolioPayoutsSuccess, Market, PortfolioList,
    PortfolioListSuccess, DEFAULT_API_HOST,
};

/// List the portfolios using the Sharesight API
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The host to use to access the API.
    #[clap(long, default_value = DEFAULT_API_HOST)]
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
    let client = Client::new_with_token_and_host(args.access_token, args.api_host);
    let portfolio_name = args.portfolio_name;

    let PortfolioListSuccess { portfolios, .. } = client
        .execute::<PortfolioList, PortfolioListSuccess>(&())
        .await?;

    let portfolio = portfolios.iter().find(|p| p.name == portfolio_name);

    if let Some(portfolio) = portfolio {
        let payouts_params = ListPortfolioPayoutsParameters {
            portfolio_id: portfolio.id,
            start_date: None,
            end_date: None,
            use_date: None,
        };
        let ListPortfolioPayoutsSuccess { payouts, .. } = client
            .execute::<ListPortfolioPayouts, _>(&payouts_params)
            .await?;

        #[derive(serde::Serialize)]
        struct PayoutRecord {
            id: Option<i64>,
            portfolio_id: i64,
            holding_id: i64,
            instrument_id: i64,
            symbol: String,
            market: Market,
            paid_on: NaiveDate,
            ex_date: Option<NaiveDate>,
            amount: f64,
            gross_amount: f64,
            resident_withholding_tax: Option<f64>,
            non_resident_withholding_tax: Option<f64>,
            tax_credit: Option<f64>,
            currency: Currency,
            exchange_rate: f64,
            non_taxable: bool,
            comments: String,
            other_net_fsi: Option<f64>,
            company_event_id: i64,
            state: String,
            franked_amount: Option<f64>,
            unfranked_amount: Option<f64>,
            trust: Option<bool>,
            extra_interest_payment_amount: Option<f64>,
            capital_gains: Option<f64>,
            discounted_capital_gains: Option<f64>,
            interest_payment: Option<f64>,
            foreign_source_income: Option<f64>,
            deferred_income: Option<f64>,
            non_assessable: Option<bool>,
            amit_decrease_amount: Option<f64>,
            amit_increase_amount: Option<f64>,
        }

        let mut wtr = csv::Writer::from_writer(std::io::stdout());

        for payout in payouts.into_iter() {
            let ListPortfolioPayoutsPayoutsSuccess {
                id,
                portfolio_id,
                holding_id,
                instrument_id,
                symbol,
                market,
                paid_on,
                ex_date,
                amount,
                gross_amount,
                resident_withholding_tax,
                non_resident_withholding_tax,
                tax_credit,
                currency,
                exchange_rate,
                non_taxable,
                comments,
                other_net_fsi,
                company_event_id,
                state,
                franked_amount,
                unfranked_amount,
                trust,
                extra_interest_payment_amount,
                capital_gains,
                discounted_capital_gains,
                interest_payment,
                foreign_source_income,
                deferred_income,
                non_assessable,
                amit_decrease_amount,
                amit_increase_amount,
                drp_trade_attributes: _,
                links: _,
            } = payout;
            wtr.serialize(PayoutRecord {
                id,
                portfolio_id,
                holding_id,
                instrument_id,
                symbol,
                market,
                paid_on,
                ex_date,
                amount,
                gross_amount,
                resident_withholding_tax,
                non_resident_withholding_tax,
                tax_credit,
                currency,
                exchange_rate,
                non_taxable,
                comments,
                other_net_fsi,
                company_event_id,
                state,
                franked_amount,
                unfranked_amount,
                trust,
                extra_interest_payment_amount,
                capital_gains,
                discounted_capital_gains,
                interest_payment,
                foreign_source_income,
                deferred_income,
                non_assessable,
                amit_decrease_amount,
                amit_increase_amount,
            })?;
        }
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
