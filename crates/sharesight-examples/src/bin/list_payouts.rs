use chrono::NaiveDate;
use clap::Parser;
use sharesight_examples::init_logger;
use sharesight_reqwest::Client;
use sharesight_types::{
    Currency, ListPortfolioPayouts, ListPortfolioPayoutsParameters,
    ListPortfolioPayoutsPayoutsSuccess, ListPortfolioPayoutsSuccess, Market, Number,
    DEFAULT_API_HOST,
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

    let portfolios = client.build_portfolio_index().await?;
    let portfolio = portfolios.find(&portfolio_name).unwrap_or_else(|| {
        portfolios.log_error_for(&portfolio_name);
        std::process::exit(0)
    });

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
        amount: Number,
        gross_amount: Number,
        resident_withholding_tax: Option<Number>,
        non_resident_withholding_tax: Option<Number>,
        tax_credit: Option<Number>,
        currency: Currency,
        exchange_rate: Number,
        non_taxable: bool,
        comments: String,
        other_net_fsi: Option<Number>,
        company_event_id: Option<i64>,
        state: String,
        franked_amount: Option<Number>,
        unfranked_amount: Option<Number>,
        trust: Option<bool>,
        extra_interest_payment_amount: Option<Number>,
        capital_gains: Option<Number>,
        discounted_capital_gains: Option<Number>,
        interest_payment: Option<Number>,
        foreign_source_income: Option<Number>,
        deferred_income: Option<Number>,
        non_assessable: Option<Number>,
        amit_decrease_amount: Option<Number>,
        amit_increase_amount: Option<Number>,
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

    Ok(())
}
