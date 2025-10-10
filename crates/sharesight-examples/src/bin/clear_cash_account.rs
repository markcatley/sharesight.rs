use clap::Parser;
use log::info;
use sharesight_examples::init_logger;
use sharesight_reqwest::Client;
use sharesight_types::{
    CashAccountTransactionDelete, CashAccountTransactionDeleteParameters,
    CashAccountTransactionsList, CashAccountTransactionsListParameters,
    CashAccountTransactionsListSuccess,
};

/// List the portfolios using the Sharesight API
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The name of the portfolio of the cash account.
    portfolio_name: String,
    /// The name of the cash account to clear.
    cash_account_name: String,
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

    for transaction in cash_account_transactions.into_iter() {
        info!("Deleting cash account transaction: {:?}", transaction);

        let parameters = CashAccountTransactionDeleteParameters { id: transaction.id };

        client
            .execute::<CashAccountTransactionDelete, ()>(&parameters)
            .await?;
    }

    Ok(())
}
