use std::sync::Arc;

use log::warn;
use serde::de::DeserializeOwned;
use sharesight_types::{
    ApiEndpoint, CashAccountsList, CashAccountsListCashAccountsSuccess, CashAccountsListParameters,
    CashAccountsListSuccess, PortfolioList, PortfolioListParameters,
    PortfolioListPortfoliosSuccess, PortfolioListSuccess,
};

pub struct Client {
    client: reqwest::Client,
    api_host: Arc<String>,
    credentials: Credentials,
}

enum Credentials {
    AccessToken(String),
}

impl Credentials {
    fn access_token(&self) -> &str {
        match self {
            Credentials::AccessToken(t) => t,
        }
    }
}

impl Client {
    pub fn new_with_token_and_host(access_token: String, api_host: String) -> Self {
        Client {
            api_host: Arc::new(api_host),
            credentials: Credentials::AccessToken(access_token),
            client: reqwest::Client::default(),
        }
    }

    pub async fn execute<'a, T: ApiEndpoint<'a>, U: DeserializeOwned>(
        &'a self,
        parameters: &'a T::Parameters,
    ) -> Result<U, SharesightReqwestError> {
        let method = match T::HTTP_METHOD {
            sharesight_types::ApiHttpMethod::Get => reqwest::Method::GET,
            sharesight_types::ApiHttpMethod::Post => reqwest::Method::POST,
            sharesight_types::ApiHttpMethod::Patch => reqwest::Method::PATCH,
            sharesight_types::ApiHttpMethod::Put => reqwest::Method::PUT,
            sharesight_types::ApiHttpMethod::Delete => reqwest::Method::DELETE,
        };
        let resp = self
            .client
            .request(method, T::url(&self.api_host, parameters).to_string())
            .bearer_auth(self.credentials.access_token())
            .json(parameters)
            .send()
            .await?;

        if resp.status().is_success() {
            let full = resp.bytes().await?;

            let slice = if full.is_empty() {
                b"null".as_slice()
            } else {
                &full
            };

            Ok(serde_json::from_slice(slice).map_err(|e| {
                if let Ok(s) = std::str::from_utf8(&full) {
                    warn!("Error deserializing json: {:?}\n{}", e, s);
                } else {
                    warn!("Error deserializing json - not valid utf-8: {:?}", e);
                }
                e
            })?)
        } else {
            Err(SharesightReqwestError::Http(
                resp.url().clone(),
                resp.status(),
                resp.text().await?,
            ))
        }
    }

    pub async fn build_portfolio_index(
        &self,
    ) -> Result<NameIndex<PortfolioListPortfoliosSuccess>, SharesightReqwestError> {
        let mut index = NameIndex::default();
        let parameters = PortfolioListParameters {
            consolidated: Some(true),
            instrument_id: None,
        };
        let PortfolioListSuccess { portfolios, .. } =
            self.execute::<PortfolioList, _>(&parameters).await?;
        index.extend(portfolios);

        let parameters = PortfolioListParameters {
            consolidated: Some(false),
            instrument_id: None,
        };
        let PortfolioListSuccess { portfolios, .. } =
            self.execute::<PortfolioList, _>(&parameters).await?;
        index.extend(portfolios);

        Ok(index)
    }

    pub async fn build_cash_account_index(
        &self,
        portfolio: &PortfolioListPortfoliosSuccess,
    ) -> Result<NameIndex<CashAccountsListCashAccountsSuccess>, SharesightReqwestError> {
        let mut index = NameIndex::default();

        let account_params = CashAccountsListParameters { date: None };
        let CashAccountsListSuccess { cash_accounts, .. } =
            self.execute::<CashAccountsList, _>(&account_params).await?;
        let cash_accounts = cash_accounts
            .into_iter()
            .filter(|a| a.portfolio_id == portfolio.id);

        index.extend(cash_accounts);

        Ok(index)
    }
}

impl AsRef<reqwest::Client> for Client {
    fn as_ref(&self) -> &reqwest::Client {
        &self.client
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SharesightReqwestError {
    #[error("Http request returned non-success status code\n{0} {1}\n{2}")]
    Http(reqwest::Url, reqwest::StatusCode, String),
    #[error("Http error occurred\n{0:?}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Deserialize error occurred\n{0:?}")]
    Deserialize(#[from] serde_json::Error),
}

#[derive(Debug)]
pub struct NameIndex<T>(Vec<T>);

impl<T> Default for NameIndex<T> {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl<T> NameIndex<T> {
    fn extend(&mut self, portfolios: impl IntoIterator<Item = T>) {
        for portfolio in portfolios {
            self.push(portfolio);
        }
    }
    fn push(&mut self, portfolio: T) {
        self.0.push(portfolio);
    }
}

impl<T: NameIndexItem> NameIndex<T> {
    pub fn find<'a>(&'a self, name: &str) -> Option<&'a T> {
        self.0.iter().find(|p| p.name() == name)
    }

    pub fn log_error_for(&self, name: &str) {
        eprint!("Unknown {}: {}, ", T::TYPE_NAME_SINGULAR, name);

        let mut names = self.0.iter().map(|p| p.name());

        match (names.next(), names.next_back()) {
            (Some(name_start), Some(name_end)) => {
                eprint!("the {} are: {}", T::TYPE_NAME_PLURAL, name_start);
                for name in names {
                    eprint!(", {}", name);
                }
                eprintln!(" or {}", name_end);
            }
            (Some(name), None) => {
                eprintln!("the only {} is: {}", T::TYPE_NAME_SINGULAR, name);
            }
            (None, None) => {
                eprintln!("there are no {}", T::TYPE_NAME_PLURAL);
            }
            _ => unreachable!(),
        }
    }
}

pub trait NameIndexItem {
    const TYPE_NAME_SINGULAR: &'static str;
    const TYPE_NAME_PLURAL: &'static str;

    fn name(&self) -> &str;
}

impl NameIndexItem for PortfolioListPortfoliosSuccess {
    const TYPE_NAME_SINGULAR: &'static str = "portfolio";
    const TYPE_NAME_PLURAL: &'static str = "portfolios";

    fn name(&self) -> &str {
        &self.name
    }
}

impl NameIndexItem for CashAccountsListCashAccountsSuccess {
    const TYPE_NAME_SINGULAR: &'static str = "cash account";
    const TYPE_NAME_PLURAL: &'static str = "cash accounts";

    fn name(&self) -> &str {
        &self.name
    }
}
