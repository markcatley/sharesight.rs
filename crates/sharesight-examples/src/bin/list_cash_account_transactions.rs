use chrono::{DateTime, FixedOffset};
use clap::Parser;
use sharesight_examples::init_logger;
use sharesight_reqwest::Client;
use sharesight_types::{
    CashAccountTransactionType, CashAccountTransactionTypeName, CashAccountTransactionsList,
    CashAccountTransactionsListCashAccountTransactionsSuccess,
    CashAccountTransactionsListParameters, CashAccountTransactionsListSuccess, CashAccountsList,
    CashAccountsListParameters, CashAccountsListSuccess, Currency, PortfolioList,
    PortfolioListSuccess,
};

/// List the portfolios using the Sharesight API
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The host to use to access the API.
    #[clap(long, default_value = "api.sharesight.com")]
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
    let client = Client::new_with_token(args.access_token, args.api_host);
    let portfolio_name = args.portfolio_name;
    let cash_account_name = args.cash_account_name;

    let PortfolioListSuccess { portfolios, .. } = client.execute::<PortfolioList, _>(&()).await?;

    let portfolio = portfolios.iter().find(|p| p.name == portfolio_name);

    if let Some(portfolio) = portfolio {
        let account_params = CashAccountsListParameters { date: None };

        let CashAccountsListSuccess { cash_accounts, .. } = client
            .execute::<CashAccountsList, _>(&account_params)
            .await?;
        let cash_accounts = cash_accounts
            .into_iter()
            .filter(|a| a.portfolio_id == portfolio.id)
            .collect::<Vec<_>>();
        let cash_account = cash_accounts.iter().find(|a| a.name == cash_account_name);

        if let Some(cash_account) = cash_account {
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
                    pub amount: f64,
                    pub balance: f64,
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
        }
    }

    Ok(())
}
