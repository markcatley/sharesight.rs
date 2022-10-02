use clap::Parser;
use log::info;
use sharesight_examples::init_logger;
use sharesight_reqwest::Client;
use sharesight_types::{
    CashAccountTransactionDelete, CashAccountTransactionDeleteParameters,
    CashAccountTransactionsList, CashAccountTransactionsListParameters,
    CashAccountTransactionsListSuccess, CashAccountsList, CashAccountsListParameters,
    CashAccountsListSuccess, PortfolioList, PortfolioListSuccess,
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

    let PortfolioListSuccess { portfolios, .. } = client
        .execute::<PortfolioList, PortfolioListSuccess>(&())
        .await?;

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

            for transaction in cash_account_transactions.into_iter() {
                info!("Deleting cash account transaction: {:?}", transaction);

                let parameters = CashAccountTransactionDeleteParameters { id: transaction.id };

                client
                    .execute::<CashAccountTransactionDelete, ()>(&parameters)
                    .await?;
            }
        } else {
            eprint!("Unknown cash account: {}, ", cash_account_name);

            let mut names = cash_accounts.iter().map(|p| p.name.as_str());

            match (names.next(), names.next_back()) {
                (Some(name_start), Some(name_end)) => {
                    eprint!("the cash accounts are: {}", name_start);
                    for name in names {
                        eprint!(", {}", name);
                    }
                    eprintln!(" or {}", name_end);
                }
                (Some(name), None) => {
                    eprintln!("the only cash account is: {}", name);
                }
                (None, None) => {
                    eprintln!("there are no cash accounts");
                }
                _ => unreachable!(),
            }
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
