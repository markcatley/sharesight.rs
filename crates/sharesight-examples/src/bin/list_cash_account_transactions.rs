use chrono::{DateTime, FixedOffset};
use clap::Parser;
use sharesight_examples::init_logger;
use sharesight_reqwest::Client;
use sharesight_types::{
    CashAccountTransactionType, CashAccountTransactionTypeName, CashAccountTransactionsList,
    CashAccountTransactionsListCashAccountTransactionsSuccess,
    CashAccountTransactionsListParameters, CashAccountTransactionsListSuccess, Currency, Number,
    DEFAULT_API_HOST,
};

/// List the portfolios using the Sharesight API
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The host to use to access the API.
    #[clap(long, default_value = DEFAULT_API_HOST)]
    api_host: String,
    /// The name of the portfolio of the cash account.
    portfolio_name: String,
    /// The name of the cash account to clear.
    cash_account_name: String,
    /// The access token to use the api.
    access_token: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_logger();

    let args = Args::parse();

    log::info!("Running with args: {args:?}");

    let client = Client::new_with_token_and_host(args.access_token, args.api_host);
    let portfolio_name = args.portfolio_name;
    let cash_account_name = args.cash_account_name;

    let portfolios = client.build_portfolio_index().await?;
    let portfolio = portfolios.find(&portfolio_name).unwrap_or_else(|| {
        portfolios.log_error_for(&portfolio_name);
        std::process::exit(0)
    });

    let cash_accounts = client.build_cash_account_index(portfolio).await?;
    let cash_account = cash_accounts.find(&cash_account_name).unwrap_or_else(|| {
        cash_accounts.log_error_for(&cash_account_name);
        std::process::exit(0)
    });

    let transactions_params = CashAccountTransactionsListParameters {
        cash_account_id: cash_account.id,
        from: None,
        to: None,
        description: None,
        foreign_identifier: None,
    };
    let CashAccountTransactionsListSuccess {
        cash_account_transactions,
        ..
    } = client
        .execute::<CashAccountTransactionsList, _>(&transactions_params)
        .await?;

    let mut wtr = csv::Writer::from_writer(std::io::stdout());

    for CashAccountTransactionsListCashAccountTransactionsSuccess {
        id,
        date_time,
        amount,
        balance,
        cash_account_id,
        foreign_identifier,
        holding_id,
        trade_id,
        payout_id,
        cash_account_transaction_type:
            CashAccountTransactionType {
                name: cash_account_transaction_type,
            },
        links: _,
    } in cash_account_transactions.into_iter()
    {
        #[derive(serde::Serialize)]
        pub struct TransactionRecord<'a> {
            pub id: i64,
            pub account_name: &'a str,
            pub portfolio_id: i64,
            pub date_time: DateTime<FixedOffset>,
            pub currency: Currency,
            pub amount: Number,
            pub balance: Number,
            pub cash_account_id: i64,
            pub foreign_identifier: Option<String>,
            pub holding_id: Option<i64>,
            pub trade_id: Option<i64>,
            pub payout_id: Option<i64>,
            pub cash_account_transaction_type: CashAccountTransactionTypeName,
        }

        wtr.serialize(TransactionRecord {
            id,
            account_name: &cash_account.name,
            portfolio_id: portfolio.id,
            date_time,
            currency: cash_account.currency,
            amount,
            balance,
            cash_account_id,
            foreign_identifier,
            holding_id,
            trade_id,
            payout_id,
            cash_account_transaction_type,
        })?;
    }

    Ok(())
}
