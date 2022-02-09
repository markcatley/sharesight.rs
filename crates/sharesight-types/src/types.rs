use crate::types_prelude::*;

pub struct CashAccountCreate;

impl<'a> ApiEndpoint<'a> for CashAccountCreate {
    const URL_PATH: &'static str = "/portfolios/:portfolio_id/cash_accounts.json";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Post;

    type UrlDisplay = CashAccountCreateUrlDisplay<'a>;
    type Parameters = CashAccountCreateParameters;
    type Success = CashAccountCreateSuccess;

    fn url_path(parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        CashAccountCreateUrlDisplay(parameters)
    }
}

pub struct CashAccountCreateUrlDisplay<'a>(&'a CashAccountCreateParameters);

impl<'a> fmt::Display for CashAccountCreateUrlDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parameters = self.0;

        write!(
            f,
            "/portfolios/{}/cash_accounts.json",
            parameters.portfolio_id
        )
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct CashAccountCreateParameters {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub portfolio_id: i64,
    pub cash_account: CashAccountCreateCashAccountParameters,
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct CashAccountCreateCashAccountParameters {
    pub name: String,
    pub currency: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CashAccountCreateSuccess {
    pub cash_account: CashAccountCreateCashAccountSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CashAccountCreateCashAccountSuccess {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    pub name: String,
    pub currency: String,
    pub portfolio_currency: String,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub portfolio_id: i64,
    #[serde_as(as = "DeserializeDate")]
    pub date: NaiveDate,
    pub balance: f64,
    pub balance_in_portfolio_currency: f64,
    pub links: CashAccountCreateCashAccountLinksSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CashAccountCreateCashAccountLinksSuccess {
    pub portfolio: String,
}

pub struct CashAccountDelete;

impl<'a> ApiEndpoint<'a> for CashAccountDelete {
    const URL_PATH: &'static str = "/cash_accounts/:id.json";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Delete;

    type UrlDisplay = CashAccountDeleteUrlDisplay<'a>;
    type Parameters = CashAccountDeleteParameters;
    type Success = ();

    fn url_path(parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        CashAccountDeleteUrlDisplay(parameters)
    }
}

pub struct CashAccountDeleteUrlDisplay<'a>(&'a CashAccountDeleteParameters);

impl<'a> fmt::Display for CashAccountDeleteUrlDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parameters = self.0;

        write!(f, "/cash_accounts/{}.json", parameters.id)
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct CashAccountDeleteParameters {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
}

pub struct CashAccountShow;

impl<'a> ApiEndpoint<'a> for CashAccountShow {
    const URL_PATH: &'static str = "/cash_accounts/:id.json";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Get;

    type UrlDisplay = CashAccountShowUrlDisplay<'a>;
    type Parameters = CashAccountShowParameters;
    type Success = CashAccountShowSuccess;

    fn url_path(parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        CashAccountShowUrlDisplay(parameters)
    }
}

pub struct CashAccountShowUrlDisplay<'a>(&'a CashAccountShowParameters);

impl<'a> fmt::Display for CashAccountShowUrlDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parameters = self.0;

        write!(f, "/cash_accounts/{}.json", parameters.id)
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct CashAccountShowParameters {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    #[serde_as(as = "Option<DeserializeDate>")]
    #[serde(default)]
    pub date: Option<NaiveDate>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CashAccountShowSuccess {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    pub name: String,
    pub currency: String,
    pub portfolio_currency: String,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub portfolio_id: i64,
    #[serde_as(as = "DeserializeDate")]
    pub date: NaiveDate,
    pub balance: f64,
    pub balance_in_portfolio_currency: f64,
    pub links: CashAccountShowLinksSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CashAccountShowLinksSuccess {
    pub portfolio: String,
    #[serde(rename = "self")]
    pub itself: String,
}

pub struct CashAccountTransactionCreate;

impl<'a> ApiEndpoint<'a> for CashAccountTransactionCreate {
    const URL_PATH: &'static str = "/cash_accounts/:cash_account_id/cash_account_transactions.json";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Post;

    type UrlDisplay = CashAccountTransactionCreateUrlDisplay<'a>;
    type Parameters = CashAccountTransactionCreateParameters;
    type Success = CashAccountTransactionCreateSuccess;

    fn url_path(parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        CashAccountTransactionCreateUrlDisplay(parameters)
    }
}

pub struct CashAccountTransactionCreateUrlDisplay<'a>(&'a CashAccountTransactionCreateParameters);

impl<'a> fmt::Display for CashAccountTransactionCreateUrlDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parameters = self.0;

        write!(
            f,
            "/cash_accounts/{}/cash_account_transactions.json",
            parameters.cash_account_id
        )
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct CashAccountTransactionCreateParameters {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub cash_account_id: i64,
    pub description: String,
    pub amount: f64,
    pub type_name: String,
    pub date_time: NaiveDateTime,
    #[serde(default)]
    pub foreign_identifier: Option<String>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CashAccountTransactionCreateSuccess {
    pub cash_account_transaction: CashAccountTransactionCreateCashAccountTransactionSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CashAccountTransactionCreateCashAccountTransactionSuccess {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    pub description: String,
    pub date_time: NaiveDateTime,
    pub amount: f64,
    pub balance: f64,
    pub cash_account_id: String,
    pub foreign_identifier: String,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub holding_id: i64,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub trade_id: i64,
    pub cash_account_transaction_type: String,
    pub links: CashAccountTransactionCreateCashAccountTransactionLinksSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CashAccountTransactionCreateCashAccountTransactionLinksSuccess {
    pub portfolio: String,
}

pub struct CashAccountTransactionDelete;

impl<'a> ApiEndpoint<'a> for CashAccountTransactionDelete {
    const URL_PATH: &'static str = "/cash_account_transactions/:id.json";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Delete;

    type UrlDisplay = CashAccountTransactionDeleteUrlDisplay<'a>;
    type Parameters = CashAccountTransactionDeleteParameters;
    type Success = ();

    fn url_path(parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        CashAccountTransactionDeleteUrlDisplay(parameters)
    }
}

pub struct CashAccountTransactionDeleteUrlDisplay<'a>(&'a CashAccountTransactionDeleteParameters);

impl<'a> fmt::Display for CashAccountTransactionDeleteUrlDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parameters = self.0;

        write!(f, "/cash_account_transactions/{}.json", parameters.id)
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct CashAccountTransactionDeleteParameters {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
}

pub struct CashAccountTransactionUpdate;

impl<'a> ApiEndpoint<'a> for CashAccountTransactionUpdate {
    const URL_PATH: &'static str = "/cash_account_transactions/:id.json";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Put;

    type UrlDisplay = CashAccountTransactionUpdateUrlDisplay<'a>;
    type Parameters = CashAccountTransactionUpdateParameters;
    type Success = CashAccountTransactionUpdateSuccess;

    fn url_path(parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        CashAccountTransactionUpdateUrlDisplay(parameters)
    }
}

pub struct CashAccountTransactionUpdateUrlDisplay<'a>(&'a CashAccountTransactionUpdateParameters);

impl<'a> fmt::Display for CashAccountTransactionUpdateUrlDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parameters = self.0;

        write!(f, "/cash_account_transactions/{}.json", parameters.id)
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct CashAccountTransactionUpdateParameters {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    pub description: String,
    pub amount: f64,
    pub type_name: String,
    pub date_time: NaiveDateTime,
    #[serde(default)]
    pub foreign_identifier: Option<String>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CashAccountTransactionUpdateSuccess {
    pub cash_account_transaction: CashAccountTransactionUpdateCashAccountTransactionSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CashAccountTransactionUpdateCashAccountTransactionSuccess {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    pub description: String,
    pub date_time: NaiveDateTime,
    pub amount: f64,
    pub balance: f64,
    pub cash_account_id: String,
    pub foreign_identifier: String,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub holding_id: i64,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub trade_id: i64,
    pub cash_account_transaction_type: String,
    pub links: CashAccountTransactionUpdateCashAccountTransactionLinksSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CashAccountTransactionUpdateCashAccountTransactionLinksSuccess {
    pub portfolio: String,
}

pub struct CashAccountTransactionsList;

impl<'a> ApiEndpoint<'a> for CashAccountTransactionsList {
    const URL_PATH: &'static str = "/cash_accounts/:cash_account_id/cash_account_transactions.json";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Get;

    type UrlDisplay = CashAccountTransactionsListUrlDisplay<'a>;
    type Parameters = CashAccountTransactionsListParameters;
    type Success = CashAccountTransactionsListSuccess;

    fn url_path(parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        CashAccountTransactionsListUrlDisplay(parameters)
    }
}

pub struct CashAccountTransactionsListUrlDisplay<'a>(&'a CashAccountTransactionsListParameters);

impl<'a> fmt::Display for CashAccountTransactionsListUrlDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parameters = self.0;

        write!(
            f,
            "/cash_accounts/{}/cash_account_transactions.json",
            parameters.cash_account_id
        )
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct CashAccountTransactionsListParameters {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub cash_account_id: i64,
    #[serde_as(as = "Option<DeserializeDate>")]
    #[serde(default)]
    pub from: Option<NaiveDate>,
    #[serde_as(as = "Option<DeserializeDate>")]
    #[serde(default)]
    pub to: Option<NaiveDate>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub foreign_identifier: Option<String>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CashAccountTransactionsListSuccess {
    pub cash_account_transactions: Vec<()>,
    pub links: CashAccountTransactionsListLinksSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CashAccountTransactionsListCashAccountTransactionSuccess {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    pub date_time: NaiveDateTime,
    pub amount: f64,
    pub balance: f64,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub cash_account_id: i64,
    pub foreign_identifier: String,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub holding_id: i64,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub trade_id: i64,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub payout_id: i64,
    pub cash_account_transaction_type: String,
    pub links: CashAccountTransactionsListCashAccountTransactionLinksSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CashAccountTransactionsListCashAccountTransactionLinksSuccess {
    pub portfolio: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CashAccountTransactionsListLinksSuccess {
    #[serde(rename = "self")]
    pub itself: String,
}

pub struct CashAccountUpdate;

impl<'a> ApiEndpoint<'a> for CashAccountUpdate {
    const URL_PATH: &'static str = "/cash_accounts/:id.json";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Put;

    type UrlDisplay = CashAccountUpdateUrlDisplay<'a>;
    type Parameters = CashAccountUpdateParameters;
    type Success = CashAccountUpdateSuccess;

    fn url_path(parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        CashAccountUpdateUrlDisplay(parameters)
    }
}

pub struct CashAccountUpdateUrlDisplay<'a>(&'a CashAccountUpdateParameters);

impl<'a> fmt::Display for CashAccountUpdateUrlDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parameters = self.0;

        write!(f, "/cash_accounts/{}.json", parameters.id)
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct CashAccountUpdateParameters {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    pub name: String,
    pub currency: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CashAccountUpdateSuccess {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    pub name: String,
    pub currency: String,
    pub portfolio_currency: String,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub portfolio_id: i64,
    #[serde_as(as = "DeserializeDate")]
    pub date: NaiveDate,
    pub balance: f64,
    pub balance_in_portfolio_currency: f64,
    pub links: CashAccountUpdateLinksSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CashAccountUpdateLinksSuccess {
    pub portfolio: String,
}

pub struct CashAccountsList;

impl<'a> ApiEndpoint<'a> for CashAccountsList {
    const URL_PATH: &'static str = "/cash_accounts.json";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Get;

    type UrlDisplay = &'static str;
    type Parameters = CashAccountsListParameters;
    type Success = CashAccountsListSuccess;

    fn url_path(_parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        "/cash_accounts.json"
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct CashAccountsListParameters {
    #[serde_as(as = "Option<DeserializeDate>")]
    #[serde(default)]
    pub date: Option<NaiveDate>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CashAccountsListSuccess {
    pub cash_accounts: Vec<()>,
    pub links: CashAccountsListLinksSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CashAccountsListCashAccountsSuccess {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    pub name: String,
    pub currency: String,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub portfolio_id: i64,
    pub portfolio_currency: String,
    #[serde_as(as = "DeserializeDate")]
    pub date: NaiveDate,
    pub balance: f64,
    pub balance_in_portfolio_currency: f64,
    pub links: CashAccountsListCashAccountsLinksSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CashAccountsListCashAccountsLinksSuccess {
    pub portfolio: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CashAccountsListLinksSuccess {
    #[serde(rename = "self")]
    pub itself: String,
}

pub struct DocumentShow;

impl<'a> ApiEndpoint<'a> for DocumentShow {
    const URL_PATH: &'static str = "/documents/:id.json";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Get;

    type UrlDisplay = DocumentShowUrlDisplay<'a>;
    type Parameters = DocumentShowParameters;
    type Success = DocumentShowSuccess;

    fn url_path(parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        DocumentShowUrlDisplay(parameters)
    }
}

pub struct DocumentShowUrlDisplay<'a>(&'a DocumentShowParameters);

impl<'a> fmt::Display for DocumentShowUrlDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parameters = self.0;

        write!(f, "/documents/{}.json", parameters.id)
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct DocumentShowParameters {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct DocumentShowSuccess {
    pub file: (),
}

pub struct GroupsList;

impl<'a> ApiEndpoint<'a> for GroupsList {
    const URL_PATH: &'static str = "/groups.json";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Get;

    type UrlDisplay = &'static str;
    type Parameters = ();
    type Success = GroupsListSuccess;

    fn url_path(_parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        "/groups.json"
    }
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct GroupsListSuccess {
    pub groups: Vec<()>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct GroupsListGroupsSuccess {
    pub id: String,
    pub name: String,
    pub custom: String,
    pub portfolio_ids: Vec<i64>,
}

pub struct HoldingMergesCreate;

impl<'a> ApiEndpoint<'a> for HoldingMergesCreate {
    const URL_PATH: &'static str = "/portfolios/:portfolio_id/holding_merges.json";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Post;

    type UrlDisplay = HoldingMergesCreateUrlDisplay<'a>;
    type Parameters = HoldingMergesCreateParameters;
    type Success = HoldingMergesCreateSuccess;

    fn url_path(parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        HoldingMergesCreateUrlDisplay(parameters)
    }
}

pub struct HoldingMergesCreateUrlDisplay<'a>(&'a HoldingMergesCreateParameters);

impl<'a> fmt::Display for HoldingMergesCreateUrlDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parameters = self.0;

        write!(
            f,
            "/portfolios/{}/holding_merges.json",
            parameters.portfolio_id
        )
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct HoldingMergesCreateParameters {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub portfolio_id: i64,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub holding_id: i64,
    #[serde_as(as = "DeserializeDate")]
    pub merge_date: NaiveDate,
    pub quantity: f64,
    pub symbol: String,
    pub market: String,
    #[serde(default)]
    pub cancelled_price: Option<f64>,
    #[serde(default)]
    pub comments: Option<String>,
    #[serde(default)]
    pub unique_identifier: Option<String>,
    #[serde(default)]
    pub attachment: Option<String>,
    #[serde(default)]
    pub attachment_filename: Option<String>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct HoldingMergesCreateSuccess {
    pub holding_merge: HoldingMergesCreateHoldingMergeSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct HoldingMergesCreateHoldingMergeSuccess {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    pub trades: Vec<()>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct HoldingMergesCreateHoldingMergeTradesSuccess {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    pub transaction_type: String,
    pub transaction_date: String,
    pub market: String,
    pub symbol: String,
    pub quantity: f64,
    pub price: f64,
    pub exchange_rate: f64,
    pub brokerage: f64,
    pub brokerage_currency_code: String,
    pub value: String,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub company_event_id: i64,
    pub unique_identifier: String,
    pub comments: String,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub portfolio_id: i64,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub holding_id: i64,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub instrument_id: i64,
    pub state: String,
    pub attachment_filename: String,
    pub attachment_id: String,
}

pub struct HoldingMergesUpdate;

impl<'a> ApiEndpoint<'a> for HoldingMergesUpdate {
    const URL_PATH: &'static str = "/portfolios/:portfolio_id/holding_merges/:id.json";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Put;

    type UrlDisplay = HoldingMergesUpdateUrlDisplay<'a>;
    type Parameters = HoldingMergesUpdateParameters;
    type Success = HoldingMergesUpdateSuccess;

    fn url_path(parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        HoldingMergesUpdateUrlDisplay(parameters)
    }
}

pub struct HoldingMergesUpdateUrlDisplay<'a>(&'a HoldingMergesUpdateParameters);

impl<'a> fmt::Display for HoldingMergesUpdateUrlDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parameters = self.0;

        write!(
            f,
            "/portfolios/{}/holding_merges/{}.json",
            parameters.portfolio_id, parameters.id
        )
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct HoldingMergesUpdateParameters {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub portfolio_id: i64,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    #[serde_as(as = "Option<DeserializeDate>")]
    #[serde(default)]
    pub merge_date: Option<NaiveDate>,
    #[serde(default)]
    pub quantity: Option<f64>,
    #[serde(default)]
    pub symbol: Option<String>,
    #[serde(default)]
    pub market: Option<String>,
    #[serde(default)]
    pub cancelled_price: Option<f64>,
    #[serde(default)]
    pub comments: Option<String>,
    #[serde(default)]
    pub unique_identifier: Option<String>,
    #[serde(default)]
    pub attachment: Option<String>,
    #[serde(default)]
    pub attachment_filename: Option<String>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct HoldingMergesUpdateSuccess {
    pub holding_merge: HoldingMergesUpdateHoldingMergeSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct HoldingMergesUpdateHoldingMergeSuccess {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    pub trades: Vec<()>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct HoldingMergesUpdateHoldingMergeTradesSuccess {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    pub transaction_type: String,
    pub transaction_date: String,
    pub market: String,
    pub symbol: String,
    pub quantity: f64,
    pub price: f64,
    pub exchange_rate: f64,
    pub brokerage: f64,
    pub brokerage_currency_code: String,
    pub value: String,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub company_event_id: i64,
    pub unique_identifier: String,
    pub comments: String,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub portfolio_id: i64,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub holding_id: i64,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub instrument_id: i64,
    pub state: String,
    pub attachment_filename: String,
    pub attachment_id: String,
}

pub struct HoldingTrades;

impl<'a> ApiEndpoint<'a> for HoldingTrades {
    const URL_PATH: &'static str = "/holdings/:holding_id/trades.json";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Get;

    type UrlDisplay = HoldingTradesUrlDisplay<'a>;
    type Parameters = HoldingTradesParameters;
    type Success = HoldingTradesSuccess;

    fn url_path(parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        HoldingTradesUrlDisplay(parameters)
    }
}

pub struct HoldingTradesUrlDisplay<'a>(&'a HoldingTradesParameters);

impl<'a> fmt::Display for HoldingTradesUrlDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parameters = self.0;

        write!(f, "/holdings/{}/trades.json", parameters.holding_id)
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct HoldingTradesParameters {
    pub holding_id: String,
    #[serde(default)]
    pub unique_identifier: Option<String>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct HoldingTradesSuccess {
    pub trades: Vec<String>,
    pub api_transaction: HoldingTradesApiTransactionSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct HoldingTradesTradesSuccess {
    pub id: String,
    pub unique_identifier: String,
    #[serde_as(as = "DeserializeDate")]
    pub transaction_date: NaiveDate,
    pub quantity: f64,
    pub price: f64,
    pub cost_base: f64,
    pub exchange_rate: f64,
    pub brokerage: f64,
    pub brokerage_currency_code: String,
    pub value: f64,
    #[serde_as(as = "DeserializeDate")]
    pub paid_on: NaiveDate,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub company_event_id: i64,
    pub comments: String,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub portfolio_id: i64,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub holding_id: i64,
    pub state: String,
    pub transaction_type: String,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub instrument_id: i64,
    pub symbol: String,
    pub market: String,
    pub attachment_filename: String,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub attachment_id: i64,
    pub confirmed: bool,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct HoldingTradesApiTransactionSuccess {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub version: i64,
    pub action: String,
    pub timestamp: String,
}

pub struct HoldingTradesRejected;

impl<'a> ApiEndpoint<'a> for HoldingTradesRejected {
    const URL_PATH: &'static str = "/holdings/:holding_id/rejected_trades.json";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Get;

    type UrlDisplay = HoldingTradesRejectedUrlDisplay<'a>;
    type Parameters = HoldingTradesRejectedParameters;
    type Success = HoldingTradesRejectedSuccess;

    fn url_path(parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        HoldingTradesRejectedUrlDisplay(parameters)
    }
}

pub struct HoldingTradesRejectedUrlDisplay<'a>(&'a HoldingTradesRejectedParameters);

impl<'a> fmt::Display for HoldingTradesRejectedUrlDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parameters = self.0;

        write!(
            f,
            "/holdings/{}/rejected_trades.json",
            parameters.holding_id
        )
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct HoldingTradesRejectedParameters {
    pub holding_id: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct HoldingTradesRejectedSuccess {
    pub trades: Vec<String>,
    pub api_transaction: HoldingTradesRejectedApiTransactionSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct HoldingTradesRejectedTradesSuccess {
    pub id: String,
    pub unique_identifier: String,
    #[serde_as(as = "DeserializeDate")]
    pub transaction_date: NaiveDate,
    pub quantity: f64,
    pub price: f64,
    pub cost_base: f64,
    pub exchange_rate: f64,
    pub brokerage: f64,
    pub brokerage_currency_code: String,
    pub value: f64,
    #[serde_as(as = "DeserializeDate")]
    pub paid_on: NaiveDate,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub company_event_id: i64,
    pub comments: String,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub portfolio_id: i64,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub holding_id: i64,
    pub state: String,
    pub transaction_type: String,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub instrument_id: i64,
    pub symbol: String,
    pub market: String,
    pub attachment_filename: String,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub attachment_id: i64,
    pub confirmed: bool,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct HoldingTradesRejectedApiTransactionSuccess {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub version: i64,
    pub action: String,
    pub timestamp: String,
}

pub struct IdentityByToken;

impl<'a> ApiEndpoint<'a> for IdentityByToken {
    const URL_PATH: &'static str = ".1-mobile/identity/by_token.json";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Get;

    type UrlDisplay = &'static str;
    type Parameters = IdentityByTokenParameters;
    type Success = IdentityByTokenSuccess;

    fn url_path(_parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        "/.1-mobile/identity/by_token.json"
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct IdentityByTokenParameters {
    pub id_token: String,
    pub client_id: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct IdentityByTokenSuccess {
    pub access_token: String,
    pub refresh_token: String,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub expires_in: i64,
    pub token_type: String,
}

pub struct IdentitySignupByToken;

impl<'a> ApiEndpoint<'a> for IdentitySignupByToken {
    const URL_PATH: &'static str = ".1-mobile/identity/signup_by_token.json";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Get;

    type UrlDisplay = &'static str;
    type Parameters = IdentitySignupByTokenParameters;
    type Success = IdentitySignupByTokenSuccess;

    fn url_path(_parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        "/.1-mobile/identity/signup_by_token.json"
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct IdentitySignupByTokenParameters {
    pub id_token: String,
    pub client_id: String,
    pub country_code: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct IdentitySignupByTokenSuccess {
    pub access_token: String,
    pub refresh_token: String,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub expires_in: i64,
    pub token_type: String,
}

pub struct ListUserInstruments;

impl<'a> ApiEndpoint<'a> for ListUserInstruments {
    const URL_PATH: &'static str = "/user_instruments.json";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Get;

    type UrlDisplay = &'static str;
    type Parameters = ();
    type Success = ListUserInstrumentsSuccess;

    fn url_path(_parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        "/user_instruments.json"
    }
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct ListUserInstrumentsSuccess {
    pub instruments: Vec<()>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct ListUserInstrumentsInstrumentsSuccess {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    pub code: String,
    pub market_code: String,
    pub name: String,
    pub currency_code: String,
    pub pe_ratio: f64,
    pub nta: f64,
    pub eps: f64,
    pub current_price: f64,
    pub current_price_updated_at: NaiveDateTime,
    pub sector_classification_name: String,
    pub industry_classification_name: String,
    pub security_type: String,
    pub friendly_instrument_description: String,
    pub registry_name: String,
}

pub struct MembershipCreate;

impl<'a> ApiEndpoint<'a> for MembershipCreate {
    const URL_PATH: &'static str = "/memberships.json";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Post;

    type UrlDisplay = &'static str;
    type Parameters = MembershipCreateParameters;
    type Success = MembershipCreateSuccess;

    fn url_path(_parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        "/memberships.json"
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct MembershipCreateParameters {
    pub membership: MembershipCreateMembershipParameters,
    #[serde(default)]
    pub user: Option<MembershipCreateUserParameters>,
    pub invitation: MembershipCreateInvitationParameters,
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct MembershipCreateMembershipParameters {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub portfolio_id: i64,
    pub access_code: String,
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub user_id: Option<i64>,
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct MembershipCreateUserParameters {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct MembershipCreateInvitationParameters {
    #[serde(default)]
    pub text: Option<String>,
    #[serde(default)]
    pub no_email: Option<bool>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct MembershipCreateSuccess {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    pub access_code: String,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub portfolio_id: i64,
    pub organisation_name: String,
    pub shared_with_organisation: bool,
    pub user: MembershipCreateUserSuccess,
    pub alerts_enabled: bool,
    pub company_event_alerts_enabled: bool,
    pub price_alerts_enabled: bool,
    pub invitation: MembershipCreateInvitationSuccess,
    pub links: MembershipCreateLinksSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct MembershipCreateUserSuccess {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub first_name: i64,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub last_name: i64,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct MembershipCreateInvitationSuccess {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    pub text: String,
    pub status: String,
    pub url: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct MembershipCreateLinksSuccess {
    pub portfolio: String,
}

pub struct MembershipDelete;

impl<'a> ApiEndpoint<'a> for MembershipDelete {
    const URL_PATH: &'static str = "/memberships/:id.json";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Delete;

    type UrlDisplay = MembershipDeleteUrlDisplay<'a>;
    type Parameters = MembershipDeleteParameters;
    type Success = ();

    fn url_path(parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        MembershipDeleteUrlDisplay(parameters)
    }
}

pub struct MembershipDeleteUrlDisplay<'a>(&'a MembershipDeleteParameters);

impl<'a> fmt::Display for MembershipDeleteUrlDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parameters = self.0;

        write!(f, "/memberships/{}.json", parameters.id)
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct MembershipDeleteParameters {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
}

pub struct MembershipList;

impl<'a> ApiEndpoint<'a> for MembershipList {
    const URL_PATH: &'static str = "/memberships.json";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Get;

    type UrlDisplay = &'static str;
    type Parameters = ();
    type Success = MembershipListSuccess;

    fn url_path(_parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        "/memberships.json"
    }
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct MembershipListSuccess {
    pub memberships: Vec<()>,
    pub organisation_name: String,
    pub shared_with_organisation: bool,
    pub links: MembershipListLinksSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct MembershipListMembershipsSuccess {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    pub access_code: String,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub portfolio_id: i64,
    pub user: MembershipListMembershipsUserSuccess,
    pub alerts_enabled: bool,
    pub company_event_alerts_enabled: bool,
    pub price_alerts_enabled: bool,
    pub invitation: MembershipListMembershipsInvitationSuccess,
    pub links: MembershipListMembershipsLinksSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct MembershipListMembershipsUserSuccess {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct MembershipListPortfoliosSuccess {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub api_email_notification: i64,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub contract_note_email_notification: i64,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct MembershipListMembershipsInvitationSuccess {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    pub text: String,
    pub url: String,
    pub status: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct MembershipListMembershipsLinksSuccess {
    pub portfolio: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct MembershipListLinksSuccess {
    #[serde(rename = "self")]
    pub itself: String,
}

pub struct MembershipUpdate;

impl<'a> ApiEndpoint<'a> for MembershipUpdate {
    const URL_PATH: &'static str = "/memberships/:id.json";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Put;

    type UrlDisplay = MembershipUpdateUrlDisplay<'a>;
    type Parameters = MembershipUpdateParameters;
    type Success = MembershipUpdateSuccess;

    fn url_path(parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        MembershipUpdateUrlDisplay(parameters)
    }
}

pub struct MembershipUpdateUrlDisplay<'a>(&'a MembershipUpdateParameters);

impl<'a> fmt::Display for MembershipUpdateUrlDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parameters = self.0;

        write!(f, "/memberships/{}.json", parameters.id)
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct MembershipUpdateParameters {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    pub access_code: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct MembershipUpdateSuccess {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    pub access_code: String,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub portfolio_id: i64,
    pub user: MembershipUpdateUserSuccess,
    pub alerts_enabled: bool,
    pub company_event_alerts_enabled: bool,
    pub price_alerts_enabled: bool,
    pub invitation: MembershipUpdateInvitationSuccess,
    pub links: MembershipUpdateLinksSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct MembershipUpdateUserSuccess {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct MembershipUpdateInvitationSuccess {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    pub text: String,
    pub url: String,
    pub status: String,
    pub invitation_path: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct MembershipUpdateLinksSuccess {
    pub portfolio: String,
}

pub struct Currencies;

impl<'a> ApiEndpoint<'a> for Currencies {
    const URL_PATH: &'static str = "/currencies.json";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Get;

    type UrlDisplay = &'static str;
    type Parameters = ();
    type Success = CurrenciesSuccess;

    fn url_path(_parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        "/currencies.json"
    }
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CurrenciesSuccess {
    pub currencies: Vec<()>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CurrenciesCurrenciesSuccess {
    pub code: String,
    pub id: String,
    pub description: String,
    #[serde_as(as = "DeserializeDate")]
    pub in_use_from: NaiveDate,
    #[serde_as(as = "DeserializeDate")]
    pub in_use_until: NaiveDate,
    pub source_feeds: String,
}

pub struct ListHoldingPayouts;

impl<'a> ApiEndpoint<'a> for ListHoldingPayouts {
    const URL_PATH: &'static str = "/holdings/:holding_id/payouts.json";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Get;

    type UrlDisplay = ListHoldingPayoutsUrlDisplay<'a>;
    type Parameters = ListHoldingPayoutsParameters;
    type Success = ListHoldingPayoutsSuccess;

    fn url_path(parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        ListHoldingPayoutsUrlDisplay(parameters)
    }
}

pub struct ListHoldingPayoutsUrlDisplay<'a>(&'a ListHoldingPayoutsParameters);

impl<'a> fmt::Display for ListHoldingPayoutsUrlDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parameters = self.0;

        write!(f, "/holdings/{}/payouts.json", parameters.holding_id)
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct ListHoldingPayoutsParameters {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub holding_id: i64,
    #[serde_as(as = "Option<DeserializeDate>")]
    #[serde(default)]
    pub start_date: Option<NaiveDate>,
    #[serde_as(as = "Option<DeserializeDate>")]
    #[serde(default)]
    pub end_date: Option<NaiveDate>,
    #[serde(default)]
    pub use_date: Option<String>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct ListHoldingPayoutsSuccess {
    pub payouts: Vec<()>,
    pub links: ListHoldingPayoutsLinksSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct ListHoldingPayoutsPayoutsSuccess {
    pub id: String,
    pub symbol: String,
    pub market: String,
    pub paid_on: String,
    pub ex_date: String,
    pub amount: f64,
    pub gross_amount: f64,
    pub transaction_description: String,
    pub resident_withholding_tax: f64,
    pub non_resident_withholding_tax: f64,
    pub tax_credit: f64,
    pub currency: String,
    pub exchange_rate: f64,
    pub non_taxable: bool,
    pub comments: String,
    pub other_net_fsi: f64,
    pub lic_capital_gain: f64,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub company_event_id: i64,
    pub state: String,
    pub drp_trade_attributes: (),
    pub franked_amount: f64,
    pub unfranked_amount: f64,
    pub trust: bool,
    pub extra_interest_payment_amount: f64,
    pub capital_gains: f64,
    pub discounted_capital_gains: f64,
    pub interest_payment: f64,
    pub foreign_source_income: f64,
    pub deferred_income: f64,
    pub non_assessable: bool,
    pub cgt_concession_amount: f64,
    pub amit_decrease_amount: f64,
    pub amit_increase_amount: f64,
    pub links: ListHoldingPayoutsPayoutsLinksSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct ListHoldingPayoutsPayoutsDrpTradeAttributesSuccess {
    pub dividend_reinvested: bool,
    pub quantity: f64,
    pub price: f64,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub source_adjustment_id: i64,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct ListHoldingPayoutsPayoutsLinksSuccess {
    pub portfolio: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct ListHoldingPayoutsLinksSuccess {
    #[serde(rename = "self")]
    pub itself: String,
}

pub struct ListPortfolioPayouts;

impl<'a> ApiEndpoint<'a> for ListPortfolioPayouts {
    const URL_PATH: &'static str = "/portfolios/:portfolio_id/payouts.json";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Get;

    type UrlDisplay = ListPortfolioPayoutsUrlDisplay<'a>;
    type Parameters = ListPortfolioPayoutsParameters;
    type Success = ListPortfolioPayoutsSuccess;

    fn url_path(parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        ListPortfolioPayoutsUrlDisplay(parameters)
    }
}

pub struct ListPortfolioPayoutsUrlDisplay<'a>(&'a ListPortfolioPayoutsParameters);

impl<'a> fmt::Display for ListPortfolioPayoutsUrlDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parameters = self.0;

        write!(f, "/portfolios/{}/payouts.json", parameters.portfolio_id)
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct ListPortfolioPayoutsParameters {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub portfolio_id: i64,
    #[serde_as(as = "Option<DeserializeDate>")]
    #[serde(default)]
    pub start_date: Option<NaiveDate>,
    #[serde_as(as = "Option<DeserializeDate>")]
    #[serde(default)]
    pub end_date: Option<NaiveDate>,
    #[serde(default)]
    pub use_date: Option<String>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct ListPortfolioPayoutsSuccess {
    pub payouts: Vec<()>,
    pub links: ListPortfolioPayoutsLinksSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct ListPortfolioPayoutsPayoutsSuccess {
    pub id: String,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub portfolio_id: i64,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub holding_id: i64,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub instrument_id: i64,
    pub symbol: String,
    pub market: String,
    pub paid_on: String,
    pub ex_date: String,
    pub amount: f64,
    pub gross_amount: f64,
    pub resident_withholding_tax: f64,
    pub non_resident_withholding_tax: f64,
    pub tax_credit: f64,
    pub currency: String,
    pub exchange_rate: f64,
    pub non_taxable: String,
    pub comments: String,
    pub other_net_fsi: f64,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub company_event_id: i64,
    pub state: String,
    pub drp_trade_attributes: (),
    pub franked_amount: f64,
    pub unfranked_amount: f64,
    pub trust: bool,
    pub extra_interest_payment_amount: f64,
    pub capital_gains: f64,
    pub discounted_capital_gains: f64,
    pub interest_payment: f64,
    pub foreign_source_income: f64,
    pub deferred_income: f64,
    pub non_assessable: bool,
    pub amit_decrease_amount: f64,
    pub amit_increase_amount: f64,
    pub links: ListPortfolioPayoutsPayoutsLinksSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct ListPortfolioPayoutsPayoutsDrpTradeAttributesSuccess {
    pub dividend_reinvested: bool,
    pub quantity: f64,
    pub price: f64,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub source_adjustment_id: i64,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct ListPortfolioPayoutsPayoutsLinksSuccess {
    pub portfolio: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct ListPortfolioPayoutsLinksSuccess {
    #[serde(rename = "self")]
    pub itself: String,
}

pub struct PayoutConfirm;

impl<'a> ApiEndpoint<'a> for PayoutConfirm {
    const URL_PATH: &'static str = "/payouts.json";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Post;

    type UrlDisplay = &'static str;
    type Parameters = PayoutConfirmParameters;
    type Success = PayoutConfirmSuccess;

    fn url_path(_parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        "/payouts.json"
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct PayoutConfirmPayoutParameters {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub holding_id: i64,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub company_event_id: i64,
    #[serde_as(as = "DeserializeDate")]
    pub paid_on: NaiveDate,
    pub state: String,
    #[serde(default)]
    pub drp_trade_attributes: Option<()>,
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct PayoutConfirmPayoutDrpTradeAttributesParameters {
    #[serde(default)]
    pub dividend_reinvested: Option<bool>,
    #[serde(default)]
    pub quantity: Option<f64>,
    #[serde(default)]
    pub price: Option<f64>,
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub source_adjustment_id: Option<i64>,
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct PayoutConfirmParameters {
    pub payout: PayoutConfirmPayoutParameters,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PayoutConfirmSuccess {
    pub payout: PayoutConfirmPayoutSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PayoutConfirmPayoutSuccess {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub portfolio_id: i64,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub holding_id: i64,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub instrument_id: i64,
    pub symbol: String,
    pub market: String,
    #[serde_as(as = "DeserializeDate")]
    pub paid_on: NaiveDate,
    #[serde_as(as = "DeserializeDate")]
    pub ex_date: NaiveDate,
    pub amount: f64,
    pub gross_amount: f64,
    pub resident_withholding_tax: f64,
    pub non_resident_withholding_tax: f64,
    pub tax_credit: f64,
    pub currency: String,
    pub exchange_rate: f64,
    pub non_taxable: String,
    pub comments: String,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub company_event_id: i64,
    pub state: String,
}

pub struct PayoutCreate;

impl<'a> ApiEndpoint<'a> for PayoutCreate {
    const URL_PATH: &'static str = "/payouts";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Post;

    type UrlDisplay = &'static str;
    type Parameters = PayoutCreateParameters;
    type Success = PayoutCreateSuccess;

    fn url_path(_parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        "/payouts"
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct PayoutCreateParameters {
    pub payout: PayoutCreatePayoutParameters,
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct PayoutCreatePayoutParameters {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub portfolio_id: i64,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub holding_id: i64,
    pub symbol: String,
    pub market: String,
    #[serde_as(as = "DeserializeDate")]
    pub paid_on: NaiveDate,
    pub amount: f64,
    pub currency_code: String,
    #[serde_as(as = "Option<DeserializeDate>")]
    #[serde(default)]
    pub goes_ex_on: Option<NaiveDate>,
    #[serde(default)]
    pub resident_withholding_tax: Option<f64>,
    #[serde(default)]
    pub non_resident_withholding_tax: Option<f64>,
    #[serde(default)]
    pub tax_credit: Option<f64>,
    #[serde(default)]
    pub exchange_rate: Option<f64>,
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub adjustment_id: Option<i64>,
    #[serde(default)]
    pub comments: Option<String>,
    #[serde(default)]
    pub non_taxable: Option<bool>,
    #[serde(default)]
    pub source_payment_date: Option<String>,
    #[serde(default)]
    pub send_to_xero: Option<bool>,
    #[serde(default)]
    pub banked_amount: Option<f64>,
    #[serde(default)]
    pub drp_trade_attributes: Option<()>,
    pub franked_amount: f64,
    pub unfranked_amount: f64,
    pub trust: bool,
    #[serde(default)]
    pub extra_interest_payment_amount: Option<f64>,
    #[serde(default)]
    pub capital_gains: Option<f64>,
    #[serde(default)]
    pub discounted_capital_gains: Option<f64>,
    #[serde(default)]
    pub foreign_source_income: Option<f64>,
    #[serde(default)]
    pub lic_capital_gain: Option<f64>,
    #[serde(default)]
    pub non_assessable: Option<bool>,
    pub deferred_income: f64,
    pub cgt_concession_amount: f64,
    pub amit_decrease_amount: f64,
    pub amit_increase_amount: f64,
    pub file_name: String,
    pub file_attachment: String,
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct PayoutCreatePayoutDrpTradeAttributesParameters {
    #[serde(default)]
    pub dividend_reinvested: Option<bool>,
    #[serde(default)]
    pub quantity: Option<f64>,
    #[serde(default)]
    pub price: Option<f64>,
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub source_adjustment_id: Option<i64>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PayoutCreateSuccess {
    pub payout: (),
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PayoutCreatePayoutSuccess {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    pub symbol: String,
    pub market: String,
    pub paid_on: String,
    pub ex_date: String,
    pub amount: f64,
    pub gross_amount: f64,
    pub resident_withholding_tax: f64,
    pub non_resident_withholding_tax: f64,
    pub tax_credit: f64,
    pub currency: String,
    pub exchange_rate: f64,
    pub non_taxable: bool,
    pub franked_amount: f64,
    pub unfranked_amount: f64,
    pub comments: String,
    pub interest_payment: f64,
    pub non_discounted_capital_gains: f64,
    pub discounted_capital_gains: f64,
    pub foreign_tax_income: f64,
    pub non_assessable: f64,
    pub trust: bool,
    pub drp_trade_attributes: (),
    pub extra_interest_payment_amount: f64,
    pub capital_gains: f64,
    pub foreign_source_income: f64,
    pub deferred_income: f64,
    pub cgt_concession_amount: f64,
    pub amit_decrease_amount: f64,
    pub amit_increase_amount: f64,
    pub attachment_filename: String,
    pub attachment_id: String,
    pub links: PayoutCreatePayoutLinksSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PayoutCreatePayoutDrpTradeAttributesSuccess {
    pub dividend_reinvested: bool,
    pub quantity: f64,
    pub price: f64,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub source_adjustment_id: i64,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PayoutCreatePayoutLinksSuccess {
    pub portfolio: String,
}

pub struct PayoutDelete;

impl<'a> ApiEndpoint<'a> for PayoutDelete {
    const URL_PATH: &'static str = "/payouts/:id.json";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Delete;

    type UrlDisplay = PayoutDeleteUrlDisplay<'a>;
    type Parameters = PayoutDeleteParameters;
    type Success = PayoutDeleteSuccess;

    fn url_path(parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        PayoutDeleteUrlDisplay(parameters)
    }
}

pub struct PayoutDeleteUrlDisplay<'a>(&'a PayoutDeleteParameters);

impl<'a> fmt::Display for PayoutDeleteUrlDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parameters = self.0;

        write!(f, "/payouts/{}.json", parameters.id)
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct PayoutDeleteParameters {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PayoutDeleteSuccess {
    pub deleted: bool,
}

pub struct PayoutReject;

impl<'a> ApiEndpoint<'a> for PayoutReject {
    const URL_PATH: &'static str = "/payouts.json";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Post;

    type UrlDisplay = &'static str;
    type Parameters = PayoutRejectParameters;
    type Success = PayoutRejectSuccess;

    fn url_path(_parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        "/payouts.json"
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct PayoutRejectParameters {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub holding_id: i64,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub company_event_id: i64,
    pub state: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PayoutRejectSuccess {
    pub payout: PayoutRejectPayoutSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PayoutRejectPayoutSuccess {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub portfolio_id: i64,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub holding_id: i64,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub instrument_id: i64,
    pub symbol: String,
    pub market: String,
    #[serde_as(as = "DeserializeDate")]
    pub paid_on: NaiveDate,
    #[serde_as(as = "DeserializeDate")]
    pub ex_date: NaiveDate,
    pub amount: f64,
    pub gross_amount: f64,
    pub resident_withholding_tax: f64,
    pub non_resident_withholding_tax: f64,
    pub tax_credit: f64,
    pub currency: String,
    pub exchange_rate: f64,
    pub non_taxable: bool,
    pub comments: String,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub company_event_id: i64,
    pub state: String,
}

pub struct PayoutShow;

impl<'a> ApiEndpoint<'a> for PayoutShow {
    const URL_PATH: &'static str = "/payouts/:id.json";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Get;

    type UrlDisplay = PayoutShowUrlDisplay<'a>;
    type Parameters = PayoutShowParameters;
    type Success = PayoutShowSuccess;

    fn url_path(parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        PayoutShowUrlDisplay(parameters)
    }
}

pub struct PayoutShowUrlDisplay<'a>(&'a PayoutShowParameters);

impl<'a> fmt::Display for PayoutShowUrlDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parameters = self.0;

        write!(f, "/payouts/{}.json", parameters.id)
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct PayoutShowParameters {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PayoutShowSuccess {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub portfolio_id: i64,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub holding_id: i64,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub instrument_id: i64,
    pub symbol: String,
    pub market: String,
    pub paid_on: String,
    pub ex_date: String,
    pub amount: f64,
    pub gross_amount: f64,
    pub resident_withholding_tax: f64,
    pub non_resident_withholding_tax: f64,
    pub tax_credit: f64,
    pub currency: String,
    pub exchange_rate: f64,
    pub non_taxable: String,
    pub comments: String,
    pub other_net_fsi: f64,
    pub lic_capital_gain: f64,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub company_event_id: i64,
    pub state: String,
    pub drp_trade_attributes: (),
    pub franked_amount: f64,
    pub unfranked_amount: f64,
    pub trust: bool,
    pub extra_interest_payment_amount: f64,
    pub capital_gains: f64,
    pub discounted_capital_gains: f64,
    pub foreign_source_income: f64,
    pub non_assessable: bool,
    pub deferred_income: f64,
    pub cgt_concession_amount: f64,
    pub amit_decrease_amount: f64,
    pub amit_increase_amount: f64,
    pub attachment_filename: String,
    pub attachment_id: String,
    pub links: PayoutShowLinksSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PayoutShowDrpTradeAttributesSuccess {
    pub dividend_reinvested: bool,
    pub quantity: f64,
    pub price: f64,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub source_adjustment_id: i64,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PayoutShowLinksSuccess {
    #[serde(rename = "self")]
    pub itself: String,
    pub portfolio: String,
}

pub struct PayoutUpdate;

impl<'a> ApiEndpoint<'a> for PayoutUpdate {
    const URL_PATH: &'static str = "/payouts/:id.json";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Put;

    type UrlDisplay = PayoutUpdateUrlDisplay<'a>;
    type Parameters = PayoutUpdateParameters;
    type Success = PayoutUpdateSuccess;

    fn url_path(parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        PayoutUpdateUrlDisplay(parameters)
    }
}

pub struct PayoutUpdateUrlDisplay<'a>(&'a PayoutUpdateParameters);

impl<'a> fmt::Display for PayoutUpdateUrlDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parameters = self.0;

        write!(f, "/payouts/{}.json", parameters.id)
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct PayoutUpdateParameters {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    pub payout: (),
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct PayoutUpdatePayoutParameters {
    #[serde_as(as = "Option<DeserializeDate>")]
    #[serde(default)]
    pub paid_on: Option<NaiveDate>,
    #[serde_as(as = "Option<DeserializeDate>")]
    #[serde(default)]
    pub goes_ex_on: Option<NaiveDate>,
    #[serde(default)]
    pub resident_withholding_tax: Option<f64>,
    #[serde(default)]
    pub non_resident_withholding_tax: Option<f64>,
    #[serde(default)]
    pub tax_credit: Option<f64>,
    #[serde(default)]
    pub exchange_rate: Option<f64>,
    #[serde(default)]
    pub amount: Option<f64>,
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub adjustment_id: Option<i64>,
    #[serde(default)]
    pub comments: Option<String>,
    #[serde(default)]
    pub non_taxable: Option<bool>,
    #[serde(default)]
    pub currency_code: Option<String>,
    #[serde_as(as = "Option<DeserializeDate>")]
    #[serde(default)]
    pub source_payment_date: Option<NaiveDate>,
    #[serde(default)]
    pub send_to_xero: Option<bool>,
    #[serde(default)]
    pub banked_amount: Option<f64>,
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub source_adjustment_id: Option<i64>,
    #[serde(default)]
    pub drp_trade_attributes: Option<()>,
    #[serde(default)]
    pub franked_amount: Option<f64>,
    #[serde(default)]
    pub unfranked_amount: Option<f64>,
    #[serde(default)]
    pub trust: Option<bool>,
    #[serde(default)]
    pub extra_interest_payment_amount: Option<f64>,
    #[serde(default)]
    pub capital_gains: Option<f64>,
    #[serde(default)]
    pub discounted_capital_gains: Option<f64>,
    #[serde(default)]
    pub foreign_source_income: Option<f64>,
    #[serde(default)]
    pub lic_capital_gain: Option<f64>,
    #[serde(default)]
    pub non_assessable: Option<bool>,
    pub deferred_income: f64,
    pub cgt_concession_amount: f64,
    pub amit_decrease_amount: f64,
    pub amit_increase_amount: f64,
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct PayoutUpdatePayoutDrpTradeAttributesParameters {
    #[serde(default)]
    pub dividend_reinvested: Option<bool>,
    #[serde(default)]
    pub quantity: Option<f64>,
    #[serde(default)]
    pub price: Option<f64>,
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub source_adjustment_id: Option<i64>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PayoutUpdateSuccess {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    pub symbol: String,
    pub market: String,
    #[serde_as(as = "DeserializeDate")]
    pub paid_on: NaiveDate,
    #[serde_as(as = "DeserializeDate")]
    pub ex_date: NaiveDate,
    pub amount: f64,
    pub gross_amount: f64,
    pub resident_withholding_tax: f64,
    pub non_resident_withholding_tax: f64,
    pub tax_credit: f64,
    pub currency: String,
    pub exchange_rate: f64,
    pub non_taxable: String,
    pub comments: String,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub company_event_id: i64,
    pub state: String,
    pub drp_trade_attributes: (),
    pub franked_amount: f64,
    pub unfranked_amount: f64,
    pub trust: bool,
    pub extra_interest_payment_amount: f64,
    pub capital_gains: f64,
    pub discounted_capital_gains: f64,
    pub foreign_source_income: f64,
    pub non_assessable: bool,
    pub deferred_income: f64,
    pub cgt_concession_amount: f64,
    pub amit_decrease_amount: f64,
    pub amit_increase_amount: f64,
    pub attachment_filename: String,
    pub attachment_id: String,
    pub links: PayoutUpdateLinksSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PayoutUpdateDrpTradeAttributesSuccess {
    pub dividend_reinvested: bool,
    pub quantity: f64,
    pub price: f64,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub source_adjustment_id: i64,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PayoutUpdateLinksSuccess {
    pub portfolio: String,
}

pub struct PortfolioCreate;

impl<'a> ApiEndpoint<'a> for PortfolioCreate {
    const URL_PATH: &'static str = "/portfolios.json";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Post;

    type UrlDisplay = &'static str;
    type Parameters = PortfolioCreateParameters;
    type Success = ();

    fn url_path(_parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        "/portfolios.json"
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct PortfolioCreateParameters {
    pub name: String,
    pub broker_email_api_enabled: bool,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub financial_year_end_month_id: i64,
    pub financial_year_end: String,
    pub default_sale_allocation_method: String,
    pub interest_method: String,
    pub trader: bool,
    pub tax_entity_type: String,
    #[serde(default)]
    pub disable_automatic_transactions: Option<bool>,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub cg_discount_id: i64,
    pub rwtr_rate: f64,
    #[serde(default)]
    pub country_code: Option<String>,
    #[serde(default)]
    pub apply_cash_account_adjustments: Option<bool>,
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub buy_trade_settlement_delay: Option<i64>,
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub sell_trade_settlement_delay: Option<i64>,
    #[serde(default)]
    pub account_for_delayed_cash_transactions: Option<bool>,
}

pub struct PortfolioDelete;

impl<'a> ApiEndpoint<'a> for PortfolioDelete {
    const URL_PATH: &'static str = "/portfolios/:id.json";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Delete;

    type UrlDisplay = PortfolioDeleteUrlDisplay<'a>;
    type Parameters = PortfolioDeleteParameters;
    type Success = PortfolioDeleteSuccess;

    fn url_path(parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        PortfolioDeleteUrlDisplay(parameters)
    }
}

pub struct PortfolioDeleteUrlDisplay<'a>(&'a PortfolioDeleteParameters);

impl<'a> fmt::Display for PortfolioDeleteUrlDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parameters = self.0;

        write!(f, "/portfolios/{}.json", parameters.id)
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct PortfolioDeleteParameters {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PortfolioDeleteSuccess {
    pub status: (),
}

pub struct PortfolioList;

impl<'a> ApiEndpoint<'a> for PortfolioList {
    const URL_PATH: &'static str = "/portfolios.json";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Get;

    type UrlDisplay = &'static str;
    type Parameters = ();
    type Success = PortfolioListSuccess;

    fn url_path(_parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        "/portfolios.json"
    }
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PortfolioListSuccess {
    pub portfolios: Vec<PortfolioListPortfoliosSuccess>,
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub trade_sync_cash_account_id: Option<i64>,
    pub links: PortfolioListLinksSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PortfolioListPortfoliosSuccess {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    pub name: String,
    pub default_sale_allocation_method: String,
    pub cg_discount: String,
    pub rwtr_rate: f64,
    #[serde(default)]
    pub trader: Option<bool>,
    pub disable_automatic_transactions: bool,
    pub broker_email_api_enabled: bool,
    pub broker_email_key: String,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub financial_year_end_month_id: i64,
    pub financial_year_end: String,
    pub interest_method: String,
    pub country_code: String,
    pub currency_code: String,
    #[serde_as(as = "DeserializeDate")]
    pub inception_date: NaiveDate,
    pub tz_name: String,
    pub apply_cash_account_adjustments: bool,
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub buy_trade_settlement_delay: Option<i64>,
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub sell_trade_settlement_delay: Option<i64>,
    pub account_for_delayed_cash_transactions: bool,
    pub links: PortfolioListPortfoliosLinksSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PortfolioListPortfoliosLinksSuccess {
    pub portfolio: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PortfolioListLinksSuccess {
    #[serde(rename = "self")]
    pub itself: String,
}

pub struct PortfolioShow;

impl<'a> ApiEndpoint<'a> for PortfolioShow {
    const URL_PATH: &'static str = "/portfolios/:id.json";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Get;

    type UrlDisplay = PortfolioShowUrlDisplay<'a>;
    type Parameters = PortfolioShowParameters;
    type Success = PortfolioShowSuccess;

    fn url_path(parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        PortfolioShowUrlDisplay(parameters)
    }
}

pub struct PortfolioShowUrlDisplay<'a>(&'a PortfolioShowParameters);

impl<'a> fmt::Display for PortfolioShowUrlDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parameters = self.0;

        write!(f, "/portfolios/{}.json", parameters.id)
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct PortfolioShowParameters {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PortfolioShowSuccess {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    pub name: String,
    pub default_sale_allocation_method: String,
    pub cg_discount: String,
    pub rwtr_rate: f64,
    pub trader: bool,
    pub disable_automatic_transactions: bool,
    pub tax_entity_type: String,
    pub broker_email_api_enabled: bool,
    pub broker_email_key: String,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub financial_year_end_month_id: i64,
    pub financial_year_end: String,
    pub interest_method: String,
    pub country_code: String,
    pub currency_code: String,
    #[serde_as(as = "DeserializeDate")]
    pub inception_date: NaiveDate,
    pub tz_name: String,
    pub apply_cash_account_adjustments: bool,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub buy_trade_settlement_delay: i64,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub sell_trade_settlement_delay: i64,
    pub account_for_delayed_cash_transactions: bool,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub trade_sync_cash_account_id: i64,
    pub links: PortfolioShowLinksSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PortfolioShowLinksSuccess {
    #[serde(rename = "self")]
    pub itself: String,
    pub portfolio: String,
}

pub struct PortfolioUpdate;

impl<'a> ApiEndpoint<'a> for PortfolioUpdate {
    const URL_PATH: &'static str = "/portfolios/:id.json";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Put;

    type UrlDisplay = PortfolioUpdateUrlDisplay<'a>;
    type Parameters = PortfolioUpdateParameters;
    type Success = PortfolioUpdateSuccess;

    fn url_path(parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        PortfolioUpdateUrlDisplay(parameters)
    }
}

pub struct PortfolioUpdateUrlDisplay<'a>(&'a PortfolioUpdateParameters);

impl<'a> fmt::Display for PortfolioUpdateUrlDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parameters = self.0;

        write!(f, "/portfolios/{}.json", parameters.id)
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct PortfolioUpdateParameters {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    pub name: String,
    pub broker_email_api_enabled: bool,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub financial_year_end_month_id: i64,
    pub financial_year_end: String,
    pub default_sale_allocation_method: String,
    pub interest_method: String,
    pub trader: bool,
    pub tax_entity_type: String,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub cg_discount_id: i64,
    pub rwtr_rate: f64,
    #[serde(default)]
    pub apply_cash_account_adjustments: Option<bool>,
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub buy_trade_settlement_delay: Option<i64>,
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub sell_trade_settlement_delay: Option<i64>,
    #[serde(default)]
    pub account_for_delayed_cash_transactions: Option<bool>,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub trade_sync_cash_account_id: i64,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PortfolioUpdateSuccess {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    pub name: String,
    pub default_sale_allocation_method: String,
    pub cg_discount: String,
    pub rwtr_rate: f64,
    pub trader: bool,
    pub tax_entity_type: String,
    pub broker_email_api_enabled: bool,
    pub broker_email_key: String,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub financial_year_end_month_id: i64,
    pub financial_year_end: String,
    pub interest_method: String,
    pub country_code: String,
    pub currency_code: String,
    pub apply_cash_account_adjustments: bool,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub buy_trade_settlement_delay: i64,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub sell_trade_settlement_delay: i64,
    pub account_for_delayed_cash_transactions: bool,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub trade_sync_cash_account_id: i64,
    pub links: PortfolioUpdateLinksSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PortfolioUpdateLinksSuccess {
    #[serde(rename = "self")]
    pub itself: String,
    pub portfolio: String,
}

pub struct CapitalGains;

impl<'a> ApiEndpoint<'a> for CapitalGains {
    const URL_PATH: &'static str = "/portfolios/:portfolio_id/capital_gains.json";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Get;

    type UrlDisplay = CapitalGainsUrlDisplay<'a>;
    type Parameters = CapitalGainsParameters;
    type Success = CapitalGainsSuccess;

    fn url_path(parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        CapitalGainsUrlDisplay(parameters)
    }
}

pub struct CapitalGainsUrlDisplay<'a>(&'a CapitalGainsParameters);

impl<'a> fmt::Display for CapitalGainsUrlDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parameters = self.0;

        write!(
            f,
            "/portfolios/{}/capital_gains.json",
            parameters.portfolio_id
        )
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct CapitalGainsParameters {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub portfolio_id: i64,
    #[serde_as(as = "Option<DeserializeDate>")]
    #[serde(default)]
    pub start_date: Option<NaiveDate>,
    #[serde_as(as = "Option<DeserializeDate>")]
    #[serde(default)]
    pub end_date: Option<NaiveDate>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CapitalGainsSuccess {
    pub short_term_gains: f64,
    pub long_term_gains: f64,
    pub losses: f64,
    pub short_term_losses: f64,
    pub long_term_losses: f64,
    pub total_discounted_capital_gain_distributions: f64,
    pub total_non_discounted_capital_gain_distributions: f64,
    pub cgt_concession_rate: f64,
    pub cgt_concession_amount: f64,
    pub market_value: f64,
    pub tax_gain_loss: f64,
    pub discounted_capital_gain_distributions: Vec<()>,
    pub non_discounted_capital_gain_distributions: Vec<()>,
    pub short_term_parcels: Vec<()>,
    pub long_term_parcels: Vec<()>,
    pub loss_parcels: Vec<()>,
    #[serde_as(as = "DeserializeDate")]
    pub start_date: NaiveDate,
    #[serde_as(as = "DeserializeDate")]
    pub end_date: NaiveDate,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub portfolio_id: i64,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CapitalGainsDiscountedCapitalGainDistributionsSuccess {
    pub market: String,
    pub symbol: String,
    pub name: String,
    pub gain: f64,
    #[serde_as(as = "DeserializeDate")]
    pub gain_date: NaiveDate,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CapitalGainsNonDiscountedCapitalGainDistributionsSuccess {
    pub market: String,
    pub symbol: String,
    pub name: String,
    pub gain: f64,
    #[serde_as(as = "DeserializeDate")]
    pub gain_date: NaiveDate,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CapitalGainsShortTermParcelsSuccess {
    pub market: String,
    pub symbol: String,
    pub name: String,
    pub allocation_method: String,
    #[serde_as(as = "DeserializeDate")]
    pub purchase_date: NaiveDate,
    pub quantity: f64,
    pub cost_base: f64,
    pub market_value: f64,
    pub gain: f64,
    #[serde_as(as = "DeserializeDate")]
    pub gain_date: NaiveDate,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CapitalGainsLongTermParcelsSuccess {
    pub market: String,
    pub symbol: String,
    pub name: String,
    pub allocation_method: String,
    #[serde_as(as = "DeserializeDate")]
    pub purchase_date: NaiveDate,
    pub quantity: f64,
    pub cost_base: f64,
    pub market_value: f64,
    pub gain: f64,
    #[serde_as(as = "DeserializeDate")]
    pub gain_date: NaiveDate,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CapitalGainsLossParcelsSuccess {
    pub market: String,
    pub symbol: String,
    pub name: String,
    pub allocation_method: String,
    #[serde_as(as = "DeserializeDate")]
    pub purchase_date: NaiveDate,
    pub quantity: f64,
    pub cost_base: f64,
    pub market_value: f64,
    pub gain: f64,
    #[serde_as(as = "DeserializeDate")]
    pub gain_date: NaiveDate,
}

pub struct Diversity;

impl<'a> ApiEndpoint<'a> for Diversity {
    const URL_PATH: &'static str = "/portfolios/:portfolio_id/diversity.json";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Get;

    type UrlDisplay = DiversityUrlDisplay<'a>;
    type Parameters = DiversityParameters;
    type Success = DiversitySuccess;

    fn url_path(parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        DiversityUrlDisplay(parameters)
    }
}

pub struct DiversityUrlDisplay<'a>(&'a DiversityParameters);

impl<'a> fmt::Display for DiversityUrlDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parameters = self.0;

        write!(f, "/portfolios/{}/diversity.json", parameters.portfolio_id)
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct DiversityParameters {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub portfolio_id: i64,
    #[serde(default)]
    pub consolidated: Option<bool>,
    #[serde_as(as = "Option<DeserializeDate>")]
    #[serde(default)]
    pub date: Option<NaiveDate>,
    #[serde(default)]
    pub grouping: Option<String>,
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub custom_group_id: Option<i64>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct DiversitySuccess {
    pub groups: Vec<()>,
    pub percentage: f64,
    pub value: f64,
    #[serde_as(as = "DeserializeDate")]
    pub date: NaiveDate,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct DiversityGroupsSuccess {
    pub group: (),
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct DiversityGroupsGroupSuccess {
    pub elements: Vec<()>,
    pub percentage: f64,
    pub value: f64,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct DiversityGroupsGroupElementsSuccess {
    pub name: String,
    pub code: String,
    pub market: String,
    pub percentage: f64,
    pub value: f64,
}

pub struct Performance;

impl<'a> ApiEndpoint<'a> for Performance {
    const URL_PATH: &'static str = "/portfolios/:portfolio_id/performance.json";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Get;

    type UrlDisplay = PerformanceUrlDisplay<'a>;
    type Parameters = PerformanceParameters;
    type Success = PerformanceSuccess;

    fn url_path(parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        PerformanceUrlDisplay(parameters)
    }
}

pub struct PerformanceUrlDisplay<'a>(&'a PerformanceParameters);

impl<'a> fmt::Display for PerformanceUrlDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parameters = self.0;

        write!(
            f,
            "/portfolios/{}/performance.json",
            parameters.portfolio_id
        )
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct PerformanceParameters {
    #[serde_as(as = "Option<DeserializeDate>")]
    #[serde(default)]
    pub start_date: Option<NaiveDate>,
    #[serde_as(as = "Option<DeserializeDate>")]
    #[serde(default)]
    pub end_date: Option<NaiveDate>,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub portfolio_id: i64,
    #[serde(default)]
    pub consolidated: Option<bool>,
    #[serde(default)]
    pub include_sales: Option<bool>,
    #[serde(default)]
    pub grouping: Option<String>,
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub custom_group_id: Option<i64>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PerformanceSuccess {
    pub id: String,
    pub portfolio_id: String,
    pub grouping: String,
    pub custom_group_id: String,
    pub value: f64,
    pub capital_gain: f64,
    pub capital_gain_percent: f64,
    pub payout_gain: f64,
    pub payout_gain_percent: f64,
    pub currency_gain: f64,
    pub currency_gain_percent: f64,
    pub total_gain: f64,
    pub total_gain_percent: f64,
    #[serde_as(as = "DeserializeDate")]
    pub start_date: NaiveDate,
    #[serde_as(as = "DeserializeDate")]
    pub end_date: NaiveDate,
    #[serde_as(as = "DeserializeDate")]
    pub include_sales: NaiveDate,
    pub holdings: Vec<()>,
    pub cash_accounts: Vec<()>,
    pub sub_totals: Vec<()>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PerformanceHoldingsSuccess {
    pub id: String,
    pub symbol: String,
    pub instrument_id: String,
    pub market: String,
    pub _group_type_: String,
    pub grouping: String,
    pub name: String,
    pub value: f64,
    pub quantity: f64,
    pub capital_gain: f64,
    pub capital_gain_percent: f64,
    pub payout_gain: f64,
    pub payout_gain_percent: f64,
    pub currency_gain: f64,
    pub currency_gain_percent: f64,
    pub total_gain: f64,
    pub total_gain_percent: f64,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PerformanceCashAccountsSuccess {
    pub id: String,
    pub cash_account_id: String,
    pub name: String,
    pub value: f64,
    pub currency: String,
    pub currency_code: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PerformanceSubTotalsSuccess {
    pub _group_type_: String,
    pub value: f64,
    pub capital_gain: f64,
    pub capital_gain_percent: f64,
    pub payout_gain: f64,
    pub payout_gain_percent: f64,
    pub currency_gain: f64,
    pub currency_gain_percent: f64,
    pub total_gain: f64,
    pub total_gain_percent: f64,
}

pub struct UnrealisedCgt;

impl<'a> ApiEndpoint<'a> for UnrealisedCgt {
    const URL_PATH: &'static str = "/portfolios/:portfolio_id/unrealised_cgt.json";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Get;

    type UrlDisplay = UnrealisedCgtUrlDisplay<'a>;
    type Parameters = UnrealisedCgtParameters;
    type Success = UnrealisedCgtSuccess;

    fn url_path(parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        UnrealisedCgtUrlDisplay(parameters)
    }
}

pub struct UnrealisedCgtUrlDisplay<'a>(&'a UnrealisedCgtParameters);

impl<'a> fmt::Display for UnrealisedCgtUrlDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parameters = self.0;

        write!(
            f,
            "/portfolios/{}/unrealised_cgt.json",
            parameters.portfolio_id
        )
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct UnrealisedCgtParameters {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub portfolio_id: i64,
    #[serde_as(as = "DeserializeDate")]
    pub balance_date: NaiveDate,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct UnrealisedCgtSuccess {
    pub unrealised_short_term_gains: f64,
    pub unrealised_long_term_gains: f64,
    pub unrealised_losses: f64,
    pub cgt_concession_rate: f64,
    pub unrealised_cgt_concession_amount: f64,
    pub market_value: f64,
    pub unrealised_tax_gain_loss: f64,
    pub short_term_parcels: Vec<()>,
    pub long_term_parcels: Vec<()>,
    pub losses: Vec<()>,
    #[serde_as(as = "DeserializeDate")]
    pub balance_date: NaiveDate,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub portfolio_id: i64,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct UnrealisedCgtShortTermParcelsSuccess {
    pub market: String,
    pub symbol: String,
    pub name: String,
    pub allocation_method: String,
    #[serde_as(as = "DeserializeDate")]
    pub purchase_date: NaiveDate,
    pub quantity: f64,
    pub cost_base: f64,
    pub market_value: f64,
    pub unrealised_gain: f64,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct UnrealisedCgtLongTermParcelsSuccess {
    pub market: String,
    pub symbol: String,
    pub name: String,
    pub allocation_method: String,
    #[serde_as(as = "DeserializeDate")]
    pub purchase_date: NaiveDate,
    pub quantity: f64,
    pub cost_base: f64,
    pub market_value: f64,
    pub unrealised_gain: f64,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct UnrealisedCgtLossesSuccess {
    pub market: String,
    pub symbol: String,
    pub name: String,
    pub allocation_method: String,
    #[serde_as(as = "DeserializeDate")]
    pub purchase_date: NaiveDate,
    pub quantity: f64,
    pub cost_base: f64,
    pub market_value: f64,
    pub unrealised_gain: f64,
}

pub struct Valuation;

impl<'a> ApiEndpoint<'a> for Valuation {
    const URL_PATH: &'static str = "/portfolios/:portfolio_id/valuation.json";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Get;

    type UrlDisplay = ValuationUrlDisplay<'a>;
    type Parameters = ValuationParameters;
    type Success = ValuationSuccess;

    fn url_path(parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        ValuationUrlDisplay(parameters)
    }
}

pub struct ValuationUrlDisplay<'a>(&'a ValuationParameters);

impl<'a> fmt::Display for ValuationUrlDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parameters = self.0;

        write!(f, "/portfolios/{}/valuation.json", parameters.portfolio_id)
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct ValuationParameters {
    #[serde_as(as = "Option<DeserializeDate>")]
    #[serde(default)]
    pub balance_date: Option<NaiveDate>,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub portfolio_id: i64,
    #[serde(default)]
    pub consolidated: Option<bool>,
    #[serde(default)]
    pub include_sales: Option<bool>,
    #[serde(default)]
    pub grouping: Option<String>,
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub custom_group_id: Option<i64>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct ValuationSuccess {
    pub id: String,
    #[serde_as(as = "DeserializeDate")]
    pub balance_date: NaiveDate,
    pub portfolio_id: String,
    pub grouping: String,
    pub custom_group_id: String,
    pub value: f64,
    pub holdings: Vec<()>,
    pub cash_accounts: Vec<()>,
    pub sub_totals: Vec<()>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct ValuationHoldingsSuccess {
    pub id: String,
    pub symbol: String,
    pub instrument_id: String,
    pub market: String,
    pub _group_type_: String,
    pub grouping: String,
    pub name: String,
    pub value: f64,
    pub quantity: f64,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct ValuationCashAccountsSuccess {
    pub id: String,
    pub cash_account_id: String,
    pub name: String,
    pub value: f64,
    pub currency: String,
    pub currency_code: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct ValuationSubTotalsSuccess {
    pub _group_type_: String,
    pub value: f64,
}

pub struct RequestSingleSignOn;

impl<'a> ApiEndpoint<'a> for RequestSingleSignOn {
    const URL_PATH: &'static str = "/single_sign_on.json";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Get;

    type UrlDisplay = &'static str;
    type Parameters = ();
    type Success = RequestSingleSignOnSuccess;

    fn url_path(_parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        "/single_sign_on.json"
    }
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct RequestSingleSignOnSuccess {
    pub login_url: String,
}

pub struct TradeConfirm;

impl<'a> ApiEndpoint<'a> for TradeConfirm {
    const URL_PATH: &'static str = "/trades.json";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Post;

    type UrlDisplay = &'static str;
    type Parameters = TradeConfirmParameters;
    type Success = TradeConfirmSuccess;

    fn url_path(_parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        "/trades.json"
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct TradeConfirmParameters {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub holding_id: i64,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub company_event_id: i64,
    #[serde_as(as = "DeserializeDate")]
    pub transaction_date: NaiveDate,
    pub state: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct TradeConfirmSuccess {
    pub trade: TradeConfirmTradeSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct TradeConfirmTradeSuccess {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    pub transaction_type: String,
    pub transaction_date: String,
    pub market: String,
    pub symbol: String,
    pub quantity: f64,
    pub price: f64,
    pub exchange_rate: f64,
    pub brokerage: f64,
    pub brokerage_currency_code: String,
    pub value: String,
    pub comments: String,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub portfolio_id: i64,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub holding_id: i64,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub instrument_id: i64,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub company_event_id: i64,
    pub state: String,
}

pub struct TradeReject;

impl<'a> ApiEndpoint<'a> for TradeReject {
    const URL_PATH: &'static str = "/trades.json";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Post;

    type UrlDisplay = &'static str;
    type Parameters = TradeRejectParameters;
    type Success = TradeRejectSuccess;

    fn url_path(_parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        "/trades.json"
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct TradeRejectParameters {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub holding_id: i64,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub company_event_id: i64,
    #[serde_as(as = "DeserializeDate")]
    pub transaction_date: NaiveDate,
    pub state: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct TradeRejectSuccess {
    pub trade: TradeRejectTradeSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct TradeRejectTradeSuccess {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    pub transaction_type: String,
    pub transaction_date: String,
    pub market: String,
    pub symbol: String,
    pub quantity: f64,
    pub price: f64,
    pub exchange_rate: f64,
    pub brokerage: f64,
    pub brokerage_currency_code: String,
    pub value: String,
    pub comments: String,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub portfolio_id: i64,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub holding_id: i64,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub instrument_id: i64,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub company_event_id: i64,
    pub state: String,
}

pub struct Trades;

impl<'a> ApiEndpoint<'a> for Trades {
    const URL_PATH: &'static str = "/portfolios/:portfolio_id/trades.json";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Get;

    type UrlDisplay = TradesUrlDisplay<'a>;
    type Parameters = TradesParameters;
    type Success = TradesSuccess;

    fn url_path(parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        TradesUrlDisplay(parameters)
    }
}

pub struct TradesUrlDisplay<'a>(&'a TradesParameters);

impl<'a> fmt::Display for TradesUrlDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parameters = self.0;

        write!(f, "/portfolios/{}/trades.json", parameters.portfolio_id)
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct TradesParameters {
    pub portfolio_id: String,
    #[serde_as(as = "Option<DeserializeDate>")]
    #[serde(default)]
    pub start_date: Option<NaiveDate>,
    #[serde_as(as = "Option<DeserializeDate>")]
    #[serde(default)]
    pub end_date: Option<NaiveDate>,
    #[serde(default)]
    pub unique_identifier: Option<String>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct TradesSuccess {
    pub trades: Vec<String>,
    pub api_transaction: TradesApiTransactionSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct TradesTradesSuccess {
    pub id: String,
    pub unique_identifier: String,
    #[serde_as(as = "DeserializeDate")]
    pub transaction_date: NaiveDate,
    pub quantity: f64,
    pub price: f64,
    pub cost_base: f64,
    pub exchange_rate: f64,
    pub brokerage: f64,
    pub brokerage_currency_code: String,
    pub value: f64,
    #[serde_as(as = "DeserializeDate")]
    pub paid_on: NaiveDate,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub company_event_id: i64,
    pub comments: String,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub portfolio_id: i64,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub holding_id: i64,
    pub state: String,
    pub transaction_type: String,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub instrument_id: i64,
    pub symbol: String,
    pub market: String,
    pub attachment_filename: String,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub attachment_id: i64,
    pub confirmed: bool,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct TradesApiTransactionSuccess {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub version: i64,
    pub action: String,
    pub timestamp: String,
}

pub struct TradesDestroy;

impl<'a> ApiEndpoint<'a> for TradesDestroy {
    const URL_PATH: &'static str = "/trades/:id.json";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Delete;

    type UrlDisplay = TradesDestroyUrlDisplay<'a>;
    type Parameters = TradesDestroyParameters;
    type Success = TradesDestroySuccess;

    fn url_path(parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        TradesDestroyUrlDisplay(parameters)
    }
}

pub struct TradesDestroyUrlDisplay<'a>(&'a TradesDestroyParameters);

impl<'a> fmt::Display for TradesDestroyUrlDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parameters = self.0;

        write!(f, "/trades/{}.json", parameters.id)
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct TradesDestroyParameters {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct TradesDestroySuccess {
    pub deleted: bool,
    pub api_transaction: TradesDestroyApiTransactionSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct TradesDestroyApiTransactionSuccess {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub version: i64,
    pub action: String,
    pub timestamp: String,
}

pub struct TradesShow;

impl<'a> ApiEndpoint<'a> for TradesShow {
    const URL_PATH: &'static str = "/trades/:id.json";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Get;

    type UrlDisplay = TradesShowUrlDisplay<'a>;
    type Parameters = TradesShowParameters;
    type Success = TradesShowSuccess;

    fn url_path(parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        TradesShowUrlDisplay(parameters)
    }
}

pub struct TradesShowUrlDisplay<'a>(&'a TradesShowParameters);

impl<'a> fmt::Display for TradesShowUrlDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parameters = self.0;

        write!(f, "/trades/{}.json", parameters.id)
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct TradesShowParameters {
    pub id: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct TradesShowSuccess {
    pub id: String,
    pub unique_identifier: String,
    #[serde_as(as = "DeserializeDate")]
    pub transaction_date: NaiveDate,
    pub quantity: f64,
    pub price: f64,
    pub cost_base: f64,
    pub exchange_rate: f64,
    pub brokerage: f64,
    pub brokerage_currency_code: String,
    pub value: f64,
    #[serde_as(as = "DeserializeDate")]
    pub paid_on: NaiveDate,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub company_event_id: i64,
    pub comments: String,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub portfolio_id: i64,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub holding_id: i64,
    pub state: String,
    pub transaction_type: String,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub instrument_id: i64,
    pub symbol: String,
    pub market: String,
    pub attachment_filename: String,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub attachment_id: i64,
    pub confirmed: bool,
    pub api_transaction: TradesShowApiTransactionSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct TradesShowApiTransactionSuccess {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub version: i64,
    pub action: String,
    pub timestamp: String,
}

pub struct MyUser;

impl<'a> ApiEndpoint<'a> for MyUser {
    const URL_PATH: &'static str = "/my_user.json";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Get;

    type UrlDisplay = &'static str;
    type Parameters = ();
    type Success = MyUserSuccess;

    fn url_path(_parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        "/my_user.json"
    }
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct MyUserSuccess {
    pub user: (),
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct MyUserUserSuccess {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    pub name: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub plan_code: String,
    pub is_activated: bool,
    pub is_free: bool,
    pub is_beta: bool,
    pub is_guest: bool,
    pub is_staff: bool,
    pub is_professional: bool,
    pub is_cancelled: bool,
    pub is_expired: bool,
}
