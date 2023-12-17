use crate::types_prelude::*;

/// Creates a new cash account within a portfolio.
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
    /// The new cash account's name.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub name: String,
    /// The new cash account's currency.
    pub currency: Currency,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CashAccountCreateSuccess {
    pub cash_account: CashAccountCreateCashAccountSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CashAccountCreateCashAccountSuccess {
    /// The cash account ID.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// The cash account name.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub name: String,
    /// The cash account currency code.
    pub currency: Currency,
    /// The cash accounts portfolio currency code.
    pub portfolio_currency: Currency,
    /// The portfolio ID
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub portfolio_id: i64,
    /// The cash accounts balance on date (format `YYYY-MM-DD`).
    #[serde_as(as = "DeserializeDate")]
    pub date: NaiveDate,
    /// The cash account balance on a given date (rounded to 2 decimal places).
    pub balance: Number,
    /// The cash account balance converted into the portfolios currency (rounded to 2 decimal places).
    pub balance_in_portfolio_currency: Number,
    /// List of links for this cash account
    pub links: CashAccountCreateCashAccountLinksSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CashAccountCreateCashAccountLinksSuccess {
    /// Url of the portfolio of this cash account
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub portfolio: String,
}

/// Deletes an existing cash account.
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
    /// ID of the cash account to delete.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
}

/// Returns details about a cash account including its balance on a specified date.
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
    /// ID of the cash account to show.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// Cash Account balance on date (format `YYYY-MM-DD`).
    #[serde_as(as = "Option<DeserializeDate>")]
    #[serde(default)]
    pub date: Option<NaiveDate>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CashAccountShowSuccess {
    /// The cash account ID.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// The cash account name.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub name: String,
    /// The cash account currency code.
    pub currency: Currency,
    /// The cash accounts portfolio currency code.
    pub portfolio_currency: Currency,
    /// The portfolio ID
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub portfolio_id: i64,
    /// The cash accounts balance on date (format `YYYY-MM-DD`).
    #[serde_as(as = "DeserializeDate")]
    pub date: NaiveDate,
    /// The cash account balance on a given date (rounded to 2 decimal places).
    pub balance: Number,
    /// The cash account balance converted into the portfolios currency (rounded to 2 decimal places).
    pub balance_in_portfolio_currency: Number,
    /// List of links for this cash account
    pub links: CashAccountShowLinksSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CashAccountShowLinksSuccess {
    /// Url of the portfolio of this cash account
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub portfolio: String,
    /// Url to the current cash account
    #[serde(rename = "self")]
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub itself: String,
}

/// Creates a new cash transaction within a cash account.
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
    /// ID of the cash account to list transactions for.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub cash_account_id: i64,
    /// The new transaction description.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub description: String,
    /// The new transaction amount.
    pub amount: Number,
    /// Transaction types may be any string. For example: `"OPENING BALANCE"`, `"DEPOSIT"`, `"WITHDRAWAL"`, `"INTEREST_PAYMENT"`, `"FEE"`, `"FEE_REIMBURSEMENT"`. The transaction type `"OPENING BALANCE"` has a rule to create an opening balance transaction, the others are all treated the same.
    pub type_name: CashAccountTransactionTypeName,
    /// The new transaction date and time (format `YYYY-MM-DDThh:mm:ss`, see <a href="https://en.wikipedia.org/wiki/ISO_8601">ISO 8601</a>).
    pub date_time: DateTime<FixedOffset>,
    /// The new transaction foreign-identifier.
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
    /// The transaction ID.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// The transaction description.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub description: String,
    /// The transaction date time (format `YYYY-MM-DDThh:mm:ss`, see <a href="https://en.wikipedia.org/wiki/ISO_8601">ISO 8601</a>).
    pub date_time: DateTime<FixedOffset>,
    /// The transaction amount (rounded to 2 decimal places).
    pub amount: Number,
    /// The transaction balance (rounded to 2 decimal places).
    pub balance: Number,
    /// ID of the cash account to list transactions for.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub cash_account_id: i64,
    /// The transaction foreign_identifier.
    #[serde(default)]
    pub foreign_identifier: Option<String>,
    /// Whenever the transaction was generated through a trade sync, this is the ID for the holding it belongs to.
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub holding_id: Option<i64>,
    /// Whenever the transaction was generated through a trade sync, this is the ID for the trade it belongs to.
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub trade_id: Option<i64>,
    /// The transaction type.
    pub cash_account_transaction_type: CashAccountTransactionType,
    /// List of links for this cash account transaction
    pub links: CashAccountTransactionCreateCashAccountTransactionLinksSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CashAccountTransactionCreateCashAccountTransactionLinksSuccess {
    /// Url of the portfolio of this cash account transaction
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub portfolio: String,
}

/// Deletes an existing cash account transaction.
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
    /// ID of the cash account transaction to delete.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
}

/// Update an existing cash account transaction.
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
    /// The transaction ID.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// The transaction description.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub description: String,
    /// The transaction amount.
    pub amount: Number,
    /// Transaction types may be any string. For example: `"OPENING BALANCE"`, `"DEPOSIT"`, `"WITHDRAWAL"`, `"INTEREST_PAYMENT"`, `"FEE"`, `"FEE_REIMBURSEMENT"`. The transaction type `"OPENING BALANCE"` has a rule to create an opening balance transaction, the others are all treated the same.
    pub type_name: CashAccountTransactionTypeName,
    /// The transaction date and time (format `YYYY-MM-DDThh:mm:ss`, see <a href="https://en.wikipedia.org/wiki/ISO_8601">ISO 8601</a>).
    pub date_time: DateTime<FixedOffset>,
    /// The transaction foreign-identifier.
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
    /// The transaction ID.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// The transaction description.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub description: String,
    /// The transaction date time (format `YYYY-MM-DDThh:mm:ss`, see <a href="https://en.wikipedia.org/wiki/ISO_8601">ISO 8601</a>).
    pub date_time: DateTime<FixedOffset>,
    /// The transaction amount (rounded to 2 decimal places).
    pub amount: Number,
    /// The transaction balance (rounded to 2 decimal places).
    pub balance: Number,
    /// ID of the cash account to list transactions for.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub cash_account_id: i64,
    /// The transaction foreign_identifier.
    #[serde(default)]
    pub foreign_identifier: Option<String>,
    /// Whenever the transaction was generated through a trade sync, this is the ID for the holding it belongs to.
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub holding_id: Option<i64>,
    /// Whenever the transaction was generated through a trade sync, this is the ID for the trade it belongs to.
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub trade_id: Option<i64>,
    /// The transaction type.
    pub cash_account_transaction_type: CashAccountTransactionType,
    /// List of links for this cash account transaction
    pub links: CashAccountTransactionUpdateCashAccountTransactionLinksSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CashAccountTransactionUpdateCashAccountTransactionLinksSuccess {
    /// Url of the portfolio of this cash account transaction
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub portfolio: String,
}

/// Returns list of transactions for a cash account.
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
    /// ID of the cash account to list transactions for.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub cash_account_id: i64,
    /// The transaction from date (format `YYYY-MM-DD`).
    #[serde_as(as = "Option<DeserializeDate>")]
    #[serde(default)]
    pub from: Option<NaiveDate>,
    /// The transaction to date (format `YYYY-MM-DD`).
    #[serde_as(as = "Option<DeserializeDate>")]
    #[serde(default)]
    pub to: Option<NaiveDate>,
    /// The transaction description
    #[serde(default)]
    pub description: Option<String>,
    /// The transaction identifier
    #[serde(default)]
    pub foreign_identifier: Option<String>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CashAccountTransactionsListSuccess {
    /// List of cash accounts.
    pub cash_account_transactions: Vec<CashAccountTransactionsListCashAccountTransactionsSuccess>,
    /// List of links for this resource
    pub links: CashAccountTransactionsListLinksSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CashAccountTransactionsListCashAccountTransactionsSuccess {
    /// The transaction ID.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// The transaction date and time.
    pub date_time: DateTime<FixedOffset>,
    /// The transaction amount (rounded to 2 decimal places).
    pub amount: Number,
    /// The transaction balance (rounded to 2 decimal places).
    pub balance: Number,
    /// ID of the cash account to list transactions for.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub cash_account_id: i64,
    /// The transaction foreign_identifier.
    #[serde(default)]
    pub foreign_identifier: Option<String>,
    /// Whenever the transaction was generated through a trade or payout sync, this is the ID for the holding it belongs to.
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub holding_id: Option<i64>,
    /// Whenever the transaction was generated through a trade sync, this is the ID for the trade it belongs to.
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub trade_id: Option<i64>,
    /// Whenever the transaction was generated through a payout sync, this is the ID for the payout it belongs to.
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub payout_id: Option<i64>,
    /// The transaction type.
    pub cash_account_transaction_type: CashAccountTransactionType,
    /// List of links for this cash account transaction
    pub links: CashAccountTransactionsListCashAccountTransactionsLinksSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CashAccountTransactionsListCashAccountTransactionsLinksSuccess {
    /// Url of the portfolio of this cash account transaction
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub portfolio: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CashAccountTransactionsListLinksSuccess {
    /// Url to list of cash account transactions
    #[serde(rename = "self")]
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub itself: String,
}

/// Updates attributes on an existing cash account.
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
    /// ID of the cash account to update.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// The cash account's name.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub name: String,
    /// The cash account's currency.
    pub currency: Currency,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CashAccountUpdateSuccess {
    /// The cash account ID.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// The cash account name.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub name: String,
    /// The cash account currency code.
    pub currency: Currency,
    /// The cash accounts portfolio currency code.
    pub portfolio_currency: Currency,
    /// The portfolio ID
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub portfolio_id: i64,
    /// The cash accounts balance on date (format `YYYY-MM-DD`).
    #[serde_as(as = "DeserializeDate")]
    pub date: NaiveDate,
    /// The cash account balance on a given date (rounded to 2 decimal places).
    pub balance: Number,
    /// The cash account balance converted into the portfolios currency (rounded to 2 decimal places).
    pub balance_in_portfolio_currency: Number,
    /// List of links for this cash account
    pub links: CashAccountUpdateLinksSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CashAccountUpdateLinksSuccess {
    /// Url of the portfolio of this cash account
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub portfolio: String,
}

/// Returns list of cash accounts for a portfolio. If you want to only access the cash account of one particular portfolios, use https://api.sharesight.com/api/v2/portfolios/:id/cash_accounts.json. The response will be the same.
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
    /// Cash Account balance on date (format `YYYY-MM-DD`).
    #[serde_as(as = "Option<DeserializeDate>")]
    #[serde(default)]
    pub date: Option<NaiveDate>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CashAccountsListSuccess {
    /// List of cash accounts.
    pub cash_accounts: Vec<CashAccountsListCashAccountsSuccess>,
    /// List of links for this resource
    pub links: CashAccountsListLinksSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CashAccountsListCashAccountsSuccess {
    /// The cash account ID.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// The cash account name.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub name: String,
    /// The cash account currency code.
    pub currency: Currency,
    /// The portfolio ID
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub portfolio_id: i64,
    /// The portfolio currency
    pub portfolio_currency: Currency,
    /// The cash accounts balance on date (format `YYYY-MM-DD`).
    #[serde_as(as = "DeserializeDate")]
    pub date: NaiveDate,
    /// The cash account balance on a given date (rounded to 2 decimal places).
    pub balance: Number,
    /// The cash account balance converted into the portfolios currency (rounded to 2 decimal places).
    pub balance_in_portfolio_currency: Number,
    /// List of links for this cash account
    pub links: CashAccountsListCashAccountsLinksSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CashAccountsListCashAccountsLinksSuccess {
    /// Url of the portfolio of this cash account
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub portfolio: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CashAccountsListLinksSuccess {
    /// Url to list of cash accounts
    #[serde(rename = "self")]
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub itself: String,
}

/// Returns a specific document.
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
    /// The document ID.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct DocumentShowSuccess {
    /// The file itself
    pub file: (),
}

/// List all custom (and regular) groups which a user has defined
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
    /// List of custom groups.
    pub groups: Vec<GroupsListGroupsSuccess>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct GroupsListGroupsSuccess {
    /// A unique id identifying the custom or regular group.
    pub id: IdOrName,
    /// The name of the group (not reliably unique).
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub name: String,
    /// True if this is a custom group.
    pub custom: bool,
    /// A list of portfolio ids a custom group can be used with (Custom groups can only be used on their creators portfolios)
    #[serde(default)]
    pub portfolio_ids: Option<Vec<i64>>,
}

/// The Create a holding merge endpoint is designed to record a company merger in the   situation where you hold shares in a listed company that is wholly acquired by another   listed company.
///
/// It creates a merge (cancel-trade) transaction against the cancelled holding.   This transaction is similar to a sell trade.   <br>   It also creates a merge (buy-trade) transaction within the new holding and   contains both the cost base and market value from the cancelled holding.
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
    /// The portfolio ID.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub portfolio_id: i64,
    /// Tho holding ID that is to cancel (sell)
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub holding_id: i64,
    /// The holding-merge date
    #[serde_as(as = "DeserializeDate")]
    pub merge_date: NaiveDate,
    /// The quantity
    pub quantity: Number,
    /// The instrument symbol for the new holding (buy)
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub symbol: String,
    /// The market code for the new holding
    pub market: Market,
    /// The cancelled price
    #[serde(default)]
    pub cancelled_price: Option<Number>,
    /// Your comments against the trade
    #[serde(default)]
    pub comments: Option<String>,
    /// This is a string of up to 255 characters used to identify duplicate trades. Generate an identifier for each trade and Sharesight will check on upload that this trade has not already been loaded.
    #[serde(default)]
    pub unique_identifier: Option<String>,
    /// Base64 encoded file to be attached to both trades (cancel- and buy-trade)
    #[serde(default)]
    pub attachment: Option<String>,
    /// File name for the attachment. This parameter is required if attachment is set.
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
    /// The ID. Equal to the cancel-trade ID.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// List of both trades created, cancel- and buy-trade
    pub trades: Vec<HoldingMergesCreateHoldingMergeTradesSuccess>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct HoldingMergesCreateHoldingMergeTradesSuccess {
    /// The trade ID. Maybe nil if the trade is based on an adjustment and has not yet been confirmed by the user.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// The trade type (BUY, SELL, SPLIT, etc).
    pub transaction_type: TradeDescription,
    /// The trade date.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub transaction_date: String,
    /// The market (ASX, NZX, etc).
    pub market: Market,
    /// The instrument code/symbol.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub symbol: String,
    /// Number of shares sold/bought.
    pub quantity: Number,
    /// Price paid/received.
    pub price: Number,
    /// The transfer's exchange rate.
    pub exchange_rate: Number,
    /// The transfer's brokerage.
    pub brokerage: Number,
    /// The brokerage currency.
    pub brokerage_currency_code: Currency,
    /// The value for the trade as displayed in the 'value' column of the UI. For a merge buy this is the cost of the transaction. The value displayed in the UI for the merge buy is the market value of the cancelled holding at the time of the merge event.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub value: String,
    /// The company event linked to the transaction.
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub company_event_id: Option<i64>,
    /// Your unique identifier for this trade, if given
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub unique_identifier: String,
    /// Any comments for that trade.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub comments: String,
    /// Portfolio ID of the trade.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub portfolio_id: i64,
    /// Holding ID of the trade.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub holding_id: i64,
    /// Instrument ID of the related Holding.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub instrument_id: i64,
    /// The state of the trade, can be any of "confirmed", "unconfirmed" or "rejected".
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub state: String,
    /// Filename of attachmented file, if present.
    #[serde(default)]
    pub attachment_filename: Option<String>,
    /// Id of the attachment, if present. Use the documents endpoint to get a copy of the file.
    #[serde(default)]
    pub attachment_id: Option<String>,
}

/// Update a holding merge.
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
    /// The portfolio ID.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub portfolio_id: i64,
    /// The ID of the cancel-trade
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// The holding-merge date
    #[serde_as(as = "Option<DeserializeDate>")]
    #[serde(default)]
    pub merge_date: Option<NaiveDate>,
    /// The quantity
    #[serde(default)]
    pub quantity: Option<Number>,
    /// The instrument symbol for the new holding (buy)
    #[serde(default)]
    pub symbol: Option<String>,
    /// The market code for the new holding
    #[serde(default)]
    pub market: Option<Market>,
    /// The cancelled price
    #[serde(default)]
    pub cancelled_price: Option<Number>,
    /// Your comments against the trade
    #[serde(default)]
    pub comments: Option<String>,
    /// This is a string of up to 255 characters used to identify duplicate trades. Generate an identifier for each trade and Sharesight will check on upload that this trade has not already been loaded.
    #[serde(default)]
    pub unique_identifier: Option<String>,
    /// Base64 encoded file to be attached to both trades (cancel- and buy-trade)
    #[serde(default)]
    pub attachment: Option<String>,
    /// File name for the attachment. This parameter is required if attachment is set.
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
    /// The ID. Equal to the cancel-trade ID.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// List of both trades created, cancel- and buy-trade
    pub trades: Vec<HoldingMergesUpdateHoldingMergeTradesSuccess>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct HoldingMergesUpdateHoldingMergeTradesSuccess {
    /// The trade ID. Maybe nil if the trade is based on an adjustment and has not yet been confirmed by the user.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// The trade type (BUY, SELL, SPLIT, etc).
    pub transaction_type: TradeDescription,
    /// The trade date.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub transaction_date: String,
    /// The market (ASX, NZX, etc).
    pub market: Market,
    /// The instrument code/symbol.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub symbol: String,
    /// Number of shares sold/bought.
    pub quantity: Number,
    /// Price paid/received.
    pub price: Number,
    /// The transfer's exchange rate.
    pub exchange_rate: Number,
    /// The transfer's brokerage.
    pub brokerage: Number,
    /// The brokerage currency.
    pub brokerage_currency_code: Currency,
    /// The value for the trade as displayed in the 'value' column of the UI. For a merge buy this is the cost of the transaction. The value displayed in the UI for the merge buy is the market value of the cancelled holding at the time of the merge event.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub value: String,
    /// The company event linked to the transaction.
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub company_event_id: Option<i64>,
    /// Your unique identifier for this trade, if given
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub unique_identifier: String,
    /// Any comments for that trade.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub comments: String,
    /// Portfolio ID of the trade.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub portfolio_id: i64,
    /// Holding ID of the trade.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub holding_id: i64,
    /// Instrument ID of the related Holding.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub instrument_id: i64,
    /// The state of the trade, can be any of "confirmed", "unconfirmed" or "rejected".
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub state: String,
    /// Filename of attachmented file, if present.
    #[serde(default)]
    pub attachment_filename: Option<String>,
    /// Id of the attachment, if present. Use the documents endpoint to get a copy of the file.
    #[serde(default)]
    pub attachment_id: Option<String>,
}

/// Returns trade transactions for a holding.
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
    /// The holding id(to show trades for).
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub holding_id: String,
    /// Search for trade with the given unique identifier.
    #[serde(default)]
    pub unique_identifier: Option<String>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct HoldingTradesSuccess {
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub trades: Vec<String>,
    /// The current API Transaction.
    pub api_transaction: HoldingTradesApiTransactionSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct HoldingTradesTradesSuccess {
    /// The trade ID. Maybe nil if the trade is based on an adjustment and has not yet been confirmed by the user.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub id: String,
    /// A unique identifier associated with this trade
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub unique_identifier: String,
    /// The trade date (format YYYY-MM-DD).
    #[serde_as(as = "DeserializeDate")]
    pub transaction_date: NaiveDate,
    /// Number of shares sold/bought.
    pub quantity: Number,
    /// Price paid/received.
    pub price: Number,
    /// For an opening balance, the cost base of the trade. Always returned in the portfolio currency
    pub cost_base: Number,
    /// The trade's exchange rate as portfolio currency / instrument currency.
    pub exchange_rate: Number,
    /// The trade's brokerage.
    pub brokerage: Number,
    /// The ISO code of the brokerage currency, must be either Portfolio or Instrument currency. If the instrument is a cryptocurrency, any valid brokerage currency is supported.
    pub brokerage_currency_code: Currency,
    /// The value for the trade as displayed in the 'value' column of the UI. For a return of capital, this will be the (signed) capital return value. For a capital call, this will be the (positive) capital return value. For a cost base adjustment, this will be the value of the adjustment. For an opening balance, this will be the market value: the market price x quantity at the opening balance date In each case this is in portfolio currency (rounded to 2 decimal places).
    pub value: Number,
    /// For a CAPITAL_RETURN or other trade with a linked payout, this is the paid on date
    #[serde_as(as = "DeserializeDate")]
    pub paid_on: NaiveDate,
    /// The company event linked to the transaction.
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub company_event_id: Option<i64>,
    /// Any comments for that trade.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub comments: String,
    /// Portfolio ID of the trade.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub portfolio_id: i64,
    /// Holding ID of the trade.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub holding_id: i64,
    /// The state of the trade, can be any of "confirmed", "unconfirmed" or "rejected".
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub state: String,
    /// The trade type ('BUY','SELL','SPLIT','BONUS','CONSOLD','CANCEL','CAPITAL_RETURN','OPENING_BALANCE','ADJUST_COST_BASE','CAPITAL_CALL').
    pub transaction_type: TradeDescription,
    /// Instrument ID of the related Holding.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub instrument_id: i64,
    /// The instrument code/symbol
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub symbol: String,
    /// The market code (eg. `ASX`, `NZX`, etc).
    pub market: Market,
    /// The filename of any attachment
    #[serde(default)]
    pub attachment_filename: Option<String>,
    /// The document id of any attachment, for use with the Show Document API (v2)
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub attachment_id: Option<i64>,
    /// Returns 'true' if trade is confirmed.  DEPRECATED: Use the state field to determine 'confirmed' vs. 'rejected' vs. 'unconfirmed' instead.
    pub confirmed: bool,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct HoldingTradesApiTransactionSuccess {
    /// The unique API Transaction id.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// The API version you called.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub version: i64,
    /// The path executed.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub action: String,
    /// When the transaction was executed.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub timestamp: String,
}

/// Returns reject trade transactions for the holding.
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
    /// The holding id(to show trades for).
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub holding_id: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct HoldingTradesRejectedSuccess {
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub trades: Vec<String>,
    /// The current API Transaction.
    pub api_transaction: HoldingTradesRejectedApiTransactionSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct HoldingTradesRejectedTradesSuccess {
    /// The trade ID. Maybe nil if the trade is based on an adjustment and has not yet been confirmed by the user.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub id: String,
    /// A unique identifier associated with this trade
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub unique_identifier: String,
    /// The trade date (format YYYY-MM-DD).
    #[serde_as(as = "DeserializeDate")]
    pub transaction_date: NaiveDate,
    /// Number of shares sold/bought.
    pub quantity: Number,
    /// Price paid/received.
    pub price: Number,
    /// For an opening balance, the cost base of the trade. Always returned in the portfolio currency
    pub cost_base: Number,
    /// The trade's exchange rate as portfolio currency / instrument currency.
    pub exchange_rate: Number,
    /// The trade's brokerage.
    pub brokerage: Number,
    /// The ISO code of the brokerage currency, must be either Portfolio or Instrument currency. If the instrument is a cryptocurrency, any valid brokerage currency is supported.
    pub brokerage_currency_code: Currency,
    /// The value for the trade as displayed in the 'value' column of the UI. For a return of capital, this will be the (signed) capital return value. For a capital call, this will be the (positive) capital return value. For a cost base adjustment, this will be the value of the adjustment. For an opening balance, this will be the market value: the market price x quantity at the opening balance date In each case this is in portfolio currency (rounded to 2 decimal places).
    pub value: Number,
    /// For a CAPITAL_RETURN or other trade with a linked payout, this is the paid on date
    #[serde_as(as = "DeserializeDate")]
    pub paid_on: NaiveDate,
    /// The company event linked to the transaction.
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub company_event_id: Option<i64>,
    /// Any comments for that trade.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub comments: String,
    /// Portfolio ID of the trade.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub portfolio_id: i64,
    /// Holding ID of the trade.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub holding_id: i64,
    /// The state of the trade, can be any of "confirmed", "unconfirmed" or "rejected".
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub state: String,
    /// The trade type ('BUY','SELL','SPLIT','BONUS','CONSOLD','CANCEL','CAPITAL_RETURN','OPENING_BALANCE','ADJUST_COST_BASE','CAPITAL_CALL').
    pub transaction_type: TradeDescription,
    /// Instrument ID of the related Holding.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub instrument_id: i64,
    /// The instrument code/symbol
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub symbol: String,
    /// The market code (eg. `ASX`, `NZX`, etc).
    pub market: Market,
    /// The filename of any attachment
    #[serde(default)]
    pub attachment_filename: Option<String>,
    /// The document id of any attachment, for use with the Show Document API (v2)
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub attachment_id: Option<i64>,
    /// Returns 'true' if trade is confirmed.  DEPRECATED: Use the state field to determine 'confirmed' vs. 'rejected' vs. 'unconfirmed' instead.
    pub confirmed: bool,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct HoldingTradesRejectedApiTransactionSuccess {
    /// The unique API Transaction id.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// The API version you called.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub version: i64,
    /// The path executed.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub action: String,
    /// When the transaction was executed.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub timestamp: String,
}

/// This takes a Google id token, from an approved mobile app. The token will be validated. If the email matches one registered with Sharesight (as the primary or Google ID email) then an access_token will be returned. Otherwise, a 404 error will be generated.
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
    /// The ID token as sent by Google
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub id_token: String,
    /// The client id assigned to your application
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub client_id: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct IdentityByTokenSuccess {
    /// An access_token to access Sharesight
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub access_token: String,
    /// An OAuth 2.0 refresh token
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub refresh_token: String,
    /// The number of seconds the token will be valid
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub expires_in: i64,
    /// The token type
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub token_type: String,
}

/// This takes a Google id token, from an approved mobile app. The token will be validated. A new Sharesight account (with no holdings) will be created. If the email matches one registered with Sharesight (as the primary or Google ID email) then an error will be returned. The access and refresh tokens will be returned. (Note that the user must create a holding before the account becomes useful).
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
    /// The ID token as sent by Google
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub id_token: String,
    /// The client id assigned to your application
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub client_id: String,
    /// The country code for the users first portfolio tax residence
    pub country_code: Country,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct IdentitySignupByTokenSuccess {
    /// An access_token to access Sharesight
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub access_token: String,
    /// An OAuth 2.0 refresh token
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub refresh_token: String,
    /// The number of seconds the token will be valid
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub expires_in: i64,
    /// The token type
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub token_type: String,
}

/// Returns a list of instruments in the current user's portfolios.
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
    /// List of instruments.
    pub instruments: Vec<ListUserInstrumentsInstrumentsSuccess>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct ListUserInstrumentsInstrumentsSuccess {
    /// Identifier for this instrument.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// The instrument's code (on the associated market).
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub code: String,
    /// The market this instrument is listed on.
    pub market_code: Market,
    /// The instrument name.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub name: String,
    /// The currency this instrument is listed in, or default market currency if nothing specified for the instrument.
    pub currency_code: Currency,
    /// The price-earnings ratio for this instrument.
    #[serde(default)]
    pub pe_ratio: Option<Number>,
    /// The net tangible assets for this instrument displayed on it's currency.
    #[serde(default)]
    pub nta: Option<Number>,
    /// The earnings per share for this instrument displayed on it's currency.
    #[serde(default)]
    pub eps: Option<Number>,
    /// The current price for this instrument displayed on it's currency.
    #[serde(default)]
    pub current_price: Option<Number>,
    /// The date and time the current price was loaded (format YYYY-MM-DDThh:mm:ss, see <a href="https://en.wikipedia.org/wiki/ISO_8601">ISO 8601</a>).
    #[serde(default)]
    pub current_price_updated_at: Option<DateTime<FixedOffset>>,
    /// The instrument sector.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub sector_classification_name: String,
    /// The instrument industry.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub industry_classification_name: String,
    /// The instrument security type. [DEPRECATED, use friendly_instrument_description]
    #[serde(default)]
    pub security_type: Option<String>,
    /// A normalised description of the instrument.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub friendly_instrument_description: String,
    /// The instrument registry.
    #[serde(default)]
    pub registry_name: Option<String>,
}

/// Create a new membership for a user's portfolio, using an existing user id or by creating a new user (providing email, etc.).
///
/// You may only use this endpoint if the user is on the required plan (not free, etc.).
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
    /// Details about the new user to create. You have to provide the user_id or a user hash.
    #[serde(default)]
    pub user: Option<MembershipCreateUserParameters>,
    /// Details about the invitation created together with the membership
    pub invitation: MembershipCreateInvitationParameters,
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct MembershipCreateMembershipParameters {
    /// Portfolio ID to create the membership for.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub portfolio_id: i64,
    /// Access level (one of NONE, READ, EDIT, ADMIN)
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub access_code: String,
    /// User ID to use for the new membership. You have to provide the user_id or a user hash.
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub user_id: Option<i64>,
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct MembershipCreateUserParameters {
    /// The email address of the new user.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub email: String,
    /// The first name of the new user.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub first_name: String,
    /// The last name of the new user.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub last_name: String,
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct MembershipCreateInvitationParameters {
    /// Invitation text sent to the new member. No HTML tags are allowed here but line-breaks are converted into paragraphs and HTML line breaks (e.g. "\n" into `\>br /\<` and "\n\n" into wrapping `\>p\<...\>/p\<` elements).
    #[serde(default)]
    pub text: Option<String>,
    /// Set to true if you don't want an invitation email being sent to the new member.
    #[serde(default)]
    pub no_email: Option<bool>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct MembershipCreateSuccess {
    /// The membership ID.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// Access level (one of NONE, READ, EDIT, ADMIN)
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub access_code: String,
    /// The portfolio ID.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub portfolio_id: i64,
    /// If present, organisation name associated with this membership.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub organisation_name: String,
    /// If true, this membership is for a Sharesight Pro organisation and the portfolio has been shared to the whole organisation. The user details returned will be those of the organisation's main account, so may differ from the invited email address.
    pub shared_with_organisation: bool,
    pub user: MembershipCreateUserSuccess,
    /// True if the user enabled portfolio alerts.
    pub alerts_enabled: bool,
    /// True if the user enabled company event alerts.
    pub company_event_alerts_enabled: bool,
    /// True if the user enabled price alerts.
    pub price_alerts_enabled: bool,
    pub invitation: MembershipCreateInvitationSuccess,
    /// List of links for this membership
    pub links: MembershipCreateLinksSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct MembershipCreateUserSuccess {
    /// The user ID.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// The user's first name.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub first_name: i64,
    /// The user's last name.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub last_name: i64,
    /// The user's email.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub email: i64,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct MembershipCreateInvitationSuccess {
    /// The invitation ID.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// The invitation text.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub text: String,
    /// 'accepted' or 'pending'
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub status: String,
    /// URL used for the invitation. Note that you can append a "redirect_to" parameter to the invitation URL in case you want the user to be redirected to the specified path after the invitation is accepted; ex. https://portfolio.sharesight.com/invitations?code=code&amp;redirect_to=/portfolios/1
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub url: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct MembershipCreateLinksSuccess {
    /// Url of the portfolio of this membership
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub portfolio: String,
}

/// Delete an existing user membership.
///
/// You may only use this endpoint if the user is on the required plan (not free, etc.).
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
    /// ID of the membership to delete.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
}

/// Lists memberships for the current user's portfolios.
///
/// If you want to only access the memberships of one particular portfolio, use `https://api.sharesight.com/api/v2/portfolios/:id/memberships.json`. The response will be the same.
///
/// You may only use this endpoint if the user is on the required plan (not free, etc.).
///
/// To access the memberships of all the user's portfolios, use this common endpoint:
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
    /// List of memberships.
    pub memberships: Vec<MembershipListMembershipsSuccess>,
    /// If present, organisation name associated with this membership.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub organisation_name: String,
    /// If true, this membership is for a Sharesight Pro organisation and the portfolio has been shared to the whole organisation.
    pub shared_with_organisation: bool,
    /// List of links for this resource
    pub links: MembershipListLinksSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct MembershipListMembershipsSuccess {
    /// The membership ID.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// Access level (one of NONE, READ, EDIT, ADMIN, OWNER)
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub access_code: String,
    /// The portfolio ID.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub portfolio_id: i64,
    /// The user details.
    pub user: MembershipListMembershipsUserSuccess,
    /// True if the user enabled portfolio alerts.
    pub alerts_enabled: bool,
    /// True if the user enabled company event alerts.
    pub company_event_alerts_enabled: bool,
    /// True if the user enabled price alerts.
    pub price_alerts_enabled: bool,
    pub invitation: MembershipListMembershipsInvitationSuccess,
    /// List of links for this membership
    pub links: MembershipListMembershipsLinksSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct MembershipListMembershipsUserSuccess {
    /// The user id.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// The user first name.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub first_name: String,
    /// The user last name.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub last_name: String,
    /// The user email.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub email: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct MembershipListPortfoliosSuccess {
    /// Alerts sent on: one of 0 (no emails), 1 (all emails), 2 (email errors only).
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub api_email_notification: i64,
    /// Notifications for Trade Confirmation Emails.  One of: 0 (no emails), 1 (all emails), 2 (email errors only).
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub contract_note_email_notification: i64,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct MembershipListMembershipsInvitationSuccess {
    /// The invitation ID.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// The invitation text.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub text: String,
    /// URL used for the invitation.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub url: String,
    /// 'accepted' or 'pending'
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub status: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct MembershipListMembershipsLinksSuccess {
    /// Url of the portfolio of this membership
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub portfolio: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct MembershipListLinksSuccess {
    /// Url to list of memberships
    #[serde(rename = "self")]
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub itself: String,
}

/// Update an existing user membership.
///
/// You may only use this endpoint if the user is on the required plan (not free, etc.).
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
    /// ID of the membership to update.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// Access level (one of NONE, READ, EDIT, ADMIN)
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub access_code: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct MembershipUpdateSuccess {
    /// The membership ID.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// Access level (one of NONE, READ, EDIT, ADMIN)
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub access_code: String,
    /// The portfolio ID.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub portfolio_id: i64,
    /// The user details.
    pub user: MembershipUpdateUserSuccess,
    /// True if the user enabled portfolio alerts.
    pub alerts_enabled: bool,
    /// True if the user enabled company event alerts.
    pub company_event_alerts_enabled: bool,
    /// True if the user enabled price alerts.
    pub price_alerts_enabled: bool,
    pub invitation: MembershipUpdateInvitationSuccess,
    /// List of links for this membership
    pub links: MembershipUpdateLinksSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct MembershipUpdateUserSuccess {
    /// The user id.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// The user first name.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub first_name: String,
    /// The user last name.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub last_name: String,
    /// The user email.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub email: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct MembershipUpdateInvitationSuccess {
    /// The invitation ID.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// The invitation text.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub text: String,
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub url: String,
    /// 'accepted' or 'pending'
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub status: String,
    /// URL used for the invitation.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub invitation_path: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct MembershipUpdateLinksSuccess {
    /// Url of the portfolio of this membership
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub portfolio: String,
}

/// Access currency definitions
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
    /// List of currency definitions
    pub currencies: Vec<CurrenciesCurrenciesSuccess>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CurrenciesCurrenciesSuccess {
    /// The ISO currency code
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub code: String,
    /// An integer currency id
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub id: String,
    /// The currency description
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub description: String,
    /// The date the currency came into use
    #[serde_as(as = "DeserializeDate")]
    pub in_use_from: NaiveDate,
    /// The date the currency came into use
    #[serde_as(as = "DeserializeDate")]
    pub in_use_until: NaiveDate,
    /// The source data feeds with symbol when not ISO and date ranges if applicable
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub source_feeds: String,
}

/// Returns payouts for the user’s holding between the two supplied dates.
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
    /// The holding ID (to show payouts for).
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub holding_id: i64,
    /// Show trades from this date on (format `YYYY-MM-DD`).
    #[serde_as(as = "Option<DeserializeDate>")]
    #[serde(default)]
    pub start_date: Option<NaiveDate>,
    /// Show trades until this date (format `YYYY-MM-DD`).
    #[serde_as(as = "Option<DeserializeDate>")]
    #[serde(default)]
    pub end_date: Option<NaiveDate>,
    /// Which payout date to use when filtering. Allowed values: `"paid_on"`, `"ex_date"`.
    #[serde(default)]
    pub use_date: Option<String>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct ListHoldingPayoutsSuccess {
    /// List of payouts.
    pub payouts: Vec<ListHoldingPayoutsPayoutsSuccess>,
    /// List of links for this resource
    pub links: ListHoldingPayoutsLinksSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct ListHoldingPayoutsPayoutsSuccess {
    /// The payout ID.
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub id: Option<i64>,
    /// Payout company/instrument symbol on the market.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub symbol: String,
    /// Market code.
    pub market: Market,
    /// The payout date (format `YYYY-MM-DD`).
    #[serde_as(as = "DeserializeDate")]
    pub paid_on: NaiveDate,
    /// The payout ex date (format `YYYY-MM-DD`).
    #[serde_as(as = "Option<DeserializeDate>")]
    #[serde(default)]
    pub ex_date: Option<NaiveDate>,
    /// The payout amount.
    pub amount: Number,
    /// The calculated gross amount
    pub gross_amount: Number,
    /// The payout type: DIV (Dividend), REP (Capital replayment), INT (Interest), or DIS (Distribution)
    pub transaction_description: PayoutDescription,
    /// The payout resident withholding tax amount. Always returned in the portfolio currency.
    #[serde(default)]
    pub resident_withholding_tax: Option<Number>,
    /// The payout non-resident withholding tax amount.
    #[serde(default)]
    pub non_resident_withholding_tax: Option<Number>,
    /// The payout tax credit amount. Always returned in the portfolio currency.
    #[serde(default)]
    pub tax_credit: Option<Number>,
    /// Currency code of the payout, using 3-letter ISO 4217 code.
    pub currency: Currency,
    /// The payout's exchange rate.
    pub exchange_rate: Number,
    /// If `true`, payout is non taxable.
    pub non_taxable: bool,
    /// Any comments for that payout.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub comments: String,
    /// Other net foreign source income.
    #[serde(default)]
    pub other_net_fsi: Option<Number>,
    /// The amount of an LIC dividend that is attributable to an LIC capital gain.
    pub lic_capital_gain: Number,
    /// ID of the company event the given payout is based on (nil if not based on any).
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub company_event_id: Option<i64>,
    /// The state of the payout, can be any of `"confirmed"`, `"unconfirmed"` or `"rejected"`.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub state: String,
    /// Parameters when the payout is reinvested.
    #[serde(default)]
    pub drp_trade_attributes: Option<ListHoldingPayoutsPayoutsDrpTradeAttributesSuccess>,
    /// Franked amount in the payout. (Australia only)
    #[serde(default)]
    pub franked_amount: Option<Number>,
    /// Unfranked amount in the payout (Australia only)
    #[serde(default)]
    pub unfranked_amount: Option<Number>,
    /// `true` if this payout is for a trust. (Australia only)
    #[serde(default)]
    pub trust: Option<bool>,
    /// Extra interest amount in this payout. (Australia only)
    #[serde(default)]
    pub extra_interest_payment_amount: Option<Number>,
    /// Capital gain amount in this payout. (Australia only)
    #[serde(default)]
    pub capital_gains: Option<Number>,
    /// Discounted capital gain amount in this payout. (Australia only)
    #[serde(default)]
    pub discounted_capital_gains: Option<Number>,
    /// Interest payment amount in this payout. (Australia only)
    #[serde(default)]
    pub interest_payment: Option<Number>,
    /// Amount of foreign income in this payout. (Australia only)
    #[serde(default)]
    pub foreign_source_income: Option<Number>,
    /// Value of deferred income in this payout. (Australia only)
    #[serde(default)]
    pub deferred_income: Option<Number>,
    /// Any non-tax assessable amount.
    #[serde(default)]
    pub non_assessable: Option<Number>,
    /// Value of CGT concession in this payout. (Australia only)
    #[serde(default)]
    pub cgt_concession_amount: Option<Number>,
    /// Relevant for attribution managed investment trusts (AMIT) when the taxable income attributed to you is less than the cash distribution you received. This amount is non-assessable and is used to decrease your cost base for cgt purposes (Australia only)
    #[serde(default)]
    pub amit_decrease_amount: Option<Number>,
    /// Relevant for attribution managed investment trusts (AMIT) when the taxable income attributed to you is more than the cash distribution you received. This amount is non-assessable and is used to increase your cost base for cgt purposes (Australia only)
    #[serde(default)]
    pub amit_increase_amount: Option<Number>,
    /// List of links for this payout
    pub links: ListHoldingPayoutsPayoutsLinksSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct ListHoldingPayoutsPayoutsDrpTradeAttributesSuccess {
    /// `true` for a reinvested payout.
    pub dividend_reinvested: bool,
    /// How many units are reinvested.
    pub quantity: Number,
    /// Price per reinvested unit.
    pub price: Number,
    /// ID of any source adjustment for the reinvested amount.
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub source_adjustment_id: Option<i64>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct ListHoldingPayoutsPayoutsLinksSuccess {
    /// Url of the portfolio of this payout
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub portfolio: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct ListHoldingPayoutsLinksSuccess {
    /// Url to list of payouts
    #[serde(rename = "self")]
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub itself: String,
}

/// Returns payouts for the user’s portfolio between the two supplied dates.
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
    /// The portfolio ID (to show payouts for).
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub portfolio_id: i64,
    /// Show payouts from this date on (format `YYYY-MM-DD`).
    #[serde_as(as = "Option<DeserializeDate>")]
    #[serde(default)]
    pub start_date: Option<NaiveDate>,
    /// Show payouts until this date (format `YYYY-MM-DD`).
    #[serde_as(as = "Option<DeserializeDate>")]
    #[serde(default)]
    pub end_date: Option<NaiveDate>,
    /// Which payout date to use when filtering. Allowed values: `"paid_on"`, `"ex_date"`.
    #[serde(default)]
    pub use_date: Option<String>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct ListPortfolioPayoutsSuccess {
    /// List of payouts.
    pub payouts: Vec<ListPortfolioPayoutsPayoutsSuccess>,
    /// List of links for this resource
    pub links: ListPortfolioPayoutsLinksSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct ListPortfolioPayoutsPayoutsSuccess {
    /// The payout ID.
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub id: Option<i64>,
    /// The portfolio ID.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub portfolio_id: i64,
    /// The holding ID.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub holding_id: i64,
    /// The instrument ID.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub instrument_id: i64,
    /// Payout company/instrument symbol on the market.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub symbol: String,
    /// Market code.
    pub market: Market,
    /// The payout date (format `YYYY-MM-DD`).
    #[serde_as(as = "DeserializeDate")]
    pub paid_on: NaiveDate,
    /// The payout ex date (format `YYYY-MM-DD`).
    #[serde_as(as = "Option<DeserializeDate>")]
    #[serde(default)]
    pub ex_date: Option<NaiveDate>,
    /// The payout amount.
    pub amount: Number,
    /// The calculated gross amount
    pub gross_amount: Number,
    /// The payout resident withholding tax amount. Always returned in the portfolio currency.
    #[serde(default)]
    pub resident_withholding_tax: Option<Number>,
    /// The payout non-resident withholding tax amount.
    #[serde(default)]
    pub non_resident_withholding_tax: Option<Number>,
    /// The payout tax credit amount. Always returned in the portfolio currency.
    #[serde(default)]
    pub tax_credit: Option<Number>,
    /// Currency code of the payout, using 3-letter ISO 4217 code.
    pub currency: Currency,
    /// The payout's exchange rate.
    pub exchange_rate: Number,
    /// If true, payout is non taxable.
    pub non_taxable: bool,
    /// Any comments for that payout.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub comments: String,
    /// Other net foreign source income.
    #[serde(default)]
    pub other_net_fsi: Option<Number>,
    /// ID of the company event the given payout is based on (nil if not based on any).
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub company_event_id: Option<i64>,
    /// The state of the payout, can be any of `"confirmed"`, `"unconfirmed"` or `"rejected"`.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub state: String,
    /// Parameters when the payout is reinvested.
    #[serde(default)]
    pub drp_trade_attributes: Option<ListPortfolioPayoutsPayoutsDrpTradeAttributesSuccess>,
    /// Franked amount in the payout. (Australia only)
    #[serde(default)]
    pub franked_amount: Option<Number>,
    /// Unfranked amount in the payout (Australia only)
    #[serde(default)]
    pub unfranked_amount: Option<Number>,
    /// `true` if this payout is for a trust. (Australia only)
    #[serde(default)]
    pub trust: Option<bool>,
    /// Extra interest amount in this payout. (Australia only)
    #[serde(default)]
    pub extra_interest_payment_amount: Option<Number>,
    /// Capital gain amount in this payout. (Australia only)
    #[serde(default)]
    pub capital_gains: Option<Number>,
    /// Discounted capital gain amount in this payout. (Australia only)
    #[serde(default)]
    pub discounted_capital_gains: Option<Number>,
    /// Interest payment amount in this payout. (Australia only)
    #[serde(default)]
    pub interest_payment: Option<Number>,
    /// Amount of foreign income in this payout. (Australia only)
    #[serde(default)]
    pub foreign_source_income: Option<Number>,
    /// Value of deferred income in this payout. (Australia only)
    #[serde(default)]
    pub deferred_income: Option<Number>,
    /// Any non-tax assessable amount.
    #[serde(default)]
    pub non_assessable: Option<Number>,
    /// Relevant for attribution managed investment trusts (AMIT) when the taxable income attributed to you is less than the cash distribution you received. This amount is non-assessable and is used to decrease your cost base for cgt purposes (Australia only)
    #[serde(default)]
    pub amit_decrease_amount: Option<Number>,
    /// Relevant for attribution managed investment trusts (AMIT) when the taxable income attributed to you is more than the cash distribution you received. This amount is non-assessable and is used to increase your cost base for cgt purposes (Australia only)
    #[serde(default)]
    pub amit_increase_amount: Option<Number>,
    /// List of links for this payout
    pub links: ListPortfolioPayoutsPayoutsLinksSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct ListPortfolioPayoutsPayoutsDrpTradeAttributesSuccess {
    /// True for a reinvested payout.
    pub dividend_reinvested: bool,
    /// How many units are reinvested.
    pub quantity: Number,
    /// Price per reinvested unit.
    pub price: Number,
    /// ID of any source adjustment for the reinvested amount.
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub source_adjustment_id: Option<i64>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct ListPortfolioPayoutsPayoutsLinksSuccess {
    /// Url of the portfolio of this payout
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub portfolio: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct ListPortfolioPayoutsLinksSuccess {
    /// Url to list of payouts
    #[serde(rename = "self")]
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub itself: String,
}

/// Confirm a payout. Tnis can be either a payout based on a company event or a payout based on the payment date of an interest payment.
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
    /// The ID of the holding.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub holding_id: i64,
    /// ID used to identify the company event the unconfirmed payout is based on.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub company_event_id: i64,
    /// Date used to identify the payout based on a payment date of an interest payment (format `YYYY-MM-DD`).
    #[serde_as(as = "DeserializeDate")]
    pub paid_on: NaiveDate,
    /// The new state of the payout: `"confirmed"`.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub state: String,
    /// Parameters when the payout is reinvested.
    #[serde(default)]
    pub drp_trade_attributes: Option<PayoutConfirmPayoutDrpTradeAttributesParameters>,
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct PayoutConfirmPayoutDrpTradeAttributesParameters {
    /// `true` for a reinvested payout.
    #[serde(default)]
    pub dividend_reinvested: Option<bool>,
    /// How many units are reinvested.
    #[serde(default)]
    pub quantity: Option<Number>,
    /// Price per reinvested unit.
    #[serde(default)]
    pub price: Option<Number>,
    /// ID of any source adjustment for the reinvested amount (same as `company_event_id`).
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
    /// The confirmed payout.
    pub payout: PayoutConfirmPayoutSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PayoutConfirmPayoutSuccess {
    /// The payout ID.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// The portfolio ID.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub portfolio_id: i64,
    /// The holding ID.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub holding_id: i64,
    /// The instrument ID.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub instrument_id: i64,
    /// Payout company/instrument symbol on the market.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub symbol: String,
    /// Market code.
    pub market: Market,
    /// The payout date (format `YYYY-MM-DD`).
    #[serde_as(as = "DeserializeDate")]
    pub paid_on: NaiveDate,
    /// The payout ex date (format `YYYY-MM-DD`).
    #[serde_as(as = "Option<DeserializeDate>")]
    #[serde(default)]
    pub ex_date: Option<NaiveDate>,
    /// The payout amount.
    pub amount: Number,
    /// The calculated gross amount
    pub gross_amount: Number,
    /// The payout resident withholding tax amount. Always returned in the portfolio currency.
    #[serde(default)]
    pub resident_withholding_tax: Option<Number>,
    /// The payout non-resident withholding tax amount.
    #[serde(default)]
    pub non_resident_withholding_tax: Option<Number>,
    /// The payout tax credit amount. Always returned in the portfolio currency.
    #[serde(default)]
    pub tax_credit: Option<Number>,
    /// Payout currency code, using 3-letter ISO 4217 code.
    pub currency: Currency,
    /// The payout's exchange rate.
    pub exchange_rate: Number,
    /// If `true`, payout is non taxable.
    pub non_taxable: bool,
    /// Any comments for that payout.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub comments: String,
    /// ID of the company event the given payout is based on (nil if not based on any).
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub company_event_id: Option<i64>,
    /// The state of the payout, can be any of `"confirmed"`, `"unconfirmed"` or `"rejected"`.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub state: String,
}

/// Creates a new payout for this portfolio.
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
    /// Portfolio ID to create the payout for. This needs to be specified together with market and symbol, unless you specify a holding_id (for an existing Holding).
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub portfolio_id: Option<i64>,
    /// Holding ID to create the trade for. If you include this, you do not need to specify portfolio_id, market and symbol parameters.
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub holding_id: Option<i64>,
    /// Payout company/instrument symbol on the market. This is not mandatory if holding_id for an existing Holding is specified.
    #[serde(default)]
    pub symbol: Option<String>,
    /// Market code (like `"NZX"` or `"ASX"`). This is not mandatory if holding_id for an existing Holding is specified.
    #[serde(default)]
    pub market: Option<Market>,
    /// The date of the payout (format `YYYY-MM-DD`).
    #[serde_as(as = "DeserializeDate")]
    pub paid_on: NaiveDate,
    /// Payout amount.
    pub amount: Number,
    /// Code for the payout currency_code, using 3-letter ISO 4217 code.
    pub currency_code: Currency,
    /// The ex date for the payout (format `YYYY-MM-DD`).
    #[serde_as(as = "Option<DeserializeDate>")]
    #[serde(default)]
    pub goes_ex_on: Option<NaiveDate>,
    /// Resident withholding tax for the payout.
    #[serde(default)]
    pub resident_withholding_tax: Option<Number>,
    /// Non-resident withholding tax for the payout
    #[serde(default)]
    pub non_resident_withholding_tax: Option<Number>,
    /// Tax credit for the payout.
    #[serde(default)]
    pub tax_credit: Option<Number>,
    /// Exchange rate for other currency payout.
    #[serde(default)]
    pub exchange_rate: Option<Number>,
    /// The ID of any adjustment.
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub adjustment_id: Option<i64>,
    /// Any comments to be saved on the payout.
    #[serde(default)]
    pub comments: Option<String>,
    /// `true` if this payout is non-taxable.
    #[serde(default)]
    pub non_taxable: Option<bool>,
    /// Date of the source payment
    #[serde(default)]
    pub source_payment_date: Option<String>,
    /// If `true`, sync this payout to xero
    #[serde(default)]
    pub send_to_xero: Option<bool>,
    /// Banked amount for this payout.
    #[serde(default)]
    pub banked_amount: Option<Number>,
    /// Parameters when the payout is reinvested.
    #[serde(default)]
    pub drp_trade_attributes: Option<PayoutCreatePayoutDrpTradeAttributesParameters>,
    /// Franked amount in the payout. (Australia only)
    #[serde(default)]
    pub franked_amount: Option<Number>,
    /// Unfranked amount in the payout (Australia only)
    #[serde(default)]
    pub unfranked_amount: Option<Number>,
    /// `true` if this payout is for a trust. (Australia only)
    #[serde(default)]
    pub trust: Option<bool>,
    /// Extra interest amount in this payout. (Australia only)
    #[serde(default)]
    pub extra_interest_payment_amount: Option<Number>,
    /// Capital gain amount in this payout. (Australia only)
    #[serde(default)]
    pub capital_gains: Option<Number>,
    /// Discounted capital gain amount in this payout. (Australia only)
    #[serde(default)]
    pub discounted_capital_gains: Option<Number>,
    /// Amount of foreign income in this payout. (Australia only)
    #[serde(default)]
    pub foreign_source_income: Option<Number>,
    /// The amount of an LIC dividend that is attributable to an LIC capital gain. (Australia only)
    #[serde(default)]
    pub lic_capital_gain: Option<Number>,
    /// Any non-tax assessable amount.
    #[serde(default)]
    pub non_assessable: Option<Number>,
    /// Value of deferred income in this payout. (Australia only)
    #[serde(default)]
    pub deferred_income: Option<Number>,
    /// Value of CGT concession in this payout. (Australia only)
    #[serde(default)]
    pub cgt_concession_amount: Option<Number>,
    /// Relevant for attribution managed investment trusts (AMIT) when the taxable income attributed to you is less than the cash distribution you received. This amount is non-assessable and is used to decrease your cost base for cgt purposes (Australia only)
    #[serde(default)]
    pub amit_decrease_amount: Option<Number>,
    /// Relevant for attribution managed investment trusts (AMIT) when the taxable income attributed to you is more than the cash distribution you received. This amount is non-assessable and is used to increase your cost base for cgt purposes (Australia only)
    #[serde(default)]
    pub amit_increase_amount: Option<Number>,
    /// File name for the attachment. This parameter is required if attachment is set.
    #[serde(default)]
    pub file_name: Option<String>,
    /// Base64 encoded attachment file to save against the payout.
    #[serde(default)]
    pub file_attachment: Option<String>,
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct PayoutCreatePayoutDrpTradeAttributesParameters {
    /// `true` for a reinvested payout.
    #[serde(default)]
    pub dividend_reinvested: Option<bool>,
    /// How many units are reinvested.
    #[serde(default)]
    pub quantity: Option<Number>,
    /// Price per reinvested unit.
    #[serde(default)]
    pub price: Option<Number>,
    /// ID of any source adjustment for the reinvested amount.
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub source_adjustment_id: Option<i64>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PayoutCreateSuccess {
    /// The new payout
    pub payout: PayoutCreatePayoutSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PayoutCreatePayoutSuccess {
    /// The payout ID.
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub id: Option<i64>,
    /// The portfolio ID.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub portfolio_id: i64,
    /// The holding ID.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub holding_id: i64,
    /// The instrument ID.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub instrument_id: i64,
    /// Payout company/instrument symbol on the market.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub symbol: String,
    /// Market code.
    pub market: Market,
    /// The payout date (format `YYYY-MM-DD`).
    #[serde_as(as = "DeserializeDate")]
    pub paid_on: NaiveDate,
    /// The payout ex date (format `YYYY-MM-DD`).
    #[serde_as(as = "Option<DeserializeDate>")]
    #[serde(default)]
    pub ex_date: Option<NaiveDate>,
    /// The payout amount.
    pub amount: Number,
    /// The calculated gross amount
    pub gross_amount: Number,
    /// The payout resident withholding tax amount. Always returned in the portfolio currency.
    #[serde(default)]
    pub resident_withholding_tax: Option<Number>,
    /// The payout non-resident withholding tax amount.
    #[serde(default)]
    pub non_resident_withholding_tax: Option<Number>,
    /// The payout tax credit amount. Always returned in the portfolio currency.
    #[serde(default)]
    pub tax_credit: Option<Number>,
    /// Currency code of the payout, using 3-letter ISO 4217 code.
    pub currency: Currency,
    /// The payout's exchange rate.
    pub exchange_rate: Number,
    /// If true, payout is non taxable.
    pub non_taxable: bool,
    /// Any comments for that payout.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub comments: String,
    /// Other net foreign source income.
    #[serde(default)]
    pub other_net_fsi: Option<Number>,
    /// ID of the company event the given payout is based on (nil if not based on any).
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub company_event_id: Option<i64>,
    /// The state of the payout, can be any of `"confirmed"`, `"unconfirmed"` or `"rejected"`.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub state: String,
    /// Parameters when the payout is reinvested.
    #[serde(default)]
    pub drp_trade_attributes: Option<PayoutCreatePayoutDrpTradeAttributesSuccess>,
    /// Franked amount in the payout. (Australia only)
    #[serde(default)]
    pub franked_amount: Option<Number>,
    /// Unfranked amount in the payout (Australia only)
    #[serde(default)]
    pub unfranked_amount: Option<Number>,
    /// `true` if this payout is for a trust. (Australia only)
    #[serde(default)]
    pub trust: Option<bool>,
    /// Extra interest amount in this payout. (Australia only)
    #[serde(default)]
    pub extra_interest_payment_amount: Option<Number>,
    /// Capital gain amount in this payout. (Australia only)
    #[serde(default)]
    pub capital_gains: Option<Number>,
    /// Discounted capital gain amount in this payout. (Australia only)
    #[serde(default)]
    pub discounted_capital_gains: Option<Number>,
    /// Interest payment amount in this payout. (Australia only)
    #[serde(default)]
    pub interest_payment: Option<Number>,
    /// Amount of foreign income in this payout. (Australia only)
    #[serde(default)]
    pub foreign_source_income: Option<Number>,
    /// Value of deferred income in this payout. (Australia only)
    #[serde(default)]
    pub deferred_income: Option<Number>,
    /// True if this payout is not assessed for tax. (Australia only)
    #[serde(default)]
    pub non_assessable: Option<bool>,
    /// Relevant for attribution managed investment trusts (AMIT) when the taxable income attributed to you is less than the cash distribution you received. This amount is non-assessable and is used to decrease your cost base for cgt purposes (Australia only)
    #[serde(default)]
    pub amit_decrease_amount: Option<Number>,
    /// Relevant for attribution managed investment trusts (AMIT) when the taxable income attributed to you is more than the cash distribution you received. This amount is non-assessable and is used to increase your cost base for cgt purposes (Australia only)
    #[serde(default)]
    pub amit_increase_amount: Option<Number>,
    /// Filename of payout attachment, if present.
    #[serde(default)]
    pub attachment_filename: Option<String>,
    /// Id of payout attachment, if present. Use the attachments endpoint to get a copy of the file.
    #[serde(default)]
    pub attachment_id: Option<String>,
    /// List of links for this payout
    pub links: PayoutCreatePayoutLinksSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PayoutCreatePayoutDrpTradeAttributesSuccess {
    /// True for a reinvested payout.
    pub dividend_reinvested: bool,
    /// How many units are reinvested.
    pub quantity: Number,
    /// Price per reinvested unit.
    pub price: Number,
    /// ID of any source adjustment for the reinvested amount.
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub source_adjustment_id: Option<i64>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PayoutCreatePayoutLinksSuccess {
    /// Url of the portfolio of this payout
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub portfolio: String,
}

/// Deletes a confirmed payout, leaving an unconfirmed payout derived from the linked corporate action. (This is similar to the 'Reset this payout') button in the UI). See Reject Payout in order to hide the corporate action altogether.
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
    /// The ID of the payout to be deleted.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PayoutDeleteSuccess {
    /// The payout has been deleted.
    pub deleted: bool,
}

/// Rejects an unconfirmed payout (based on a corporate action event). A payout derived from the linked corporate action will no longer appear in the users portfolio. (This is similar to the 'Reject this Payout' button in the UI). See Delete Payout in order to delete a confirmed payout.
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
    /// The ID of the holding.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub holding_id: i64,
    /// ID used to identify the company event the unconfirmed payout is based on.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub company_event_id: i64,
    /// The new state of the payout: `"rejected"`.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub state: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PayoutRejectSuccess {
    /// The rejected payout.
    pub payout: PayoutRejectPayoutSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PayoutRejectPayoutSuccess {
    /// The payout ID.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// The portfolio ID.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub portfolio_id: i64,
    /// The holding ID.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub holding_id: i64,
    /// The instrument ID.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub instrument_id: i64,
    /// Payout company/instrument symbol on the market.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub symbol: String,
    /// Market code.
    pub market: Market,
    /// The payout date (format `YYYY-MM-DD`).
    #[serde_as(as = "DeserializeDate")]
    pub paid_on: NaiveDate,
    /// The payout ex date (format `YYYY-MM-DD`).
    #[serde_as(as = "Option<DeserializeDate>")]
    #[serde(default)]
    pub ex_date: Option<NaiveDate>,
    /// The payout amount.
    pub amount: Number,
    /// The calculated gross amount
    pub gross_amount: Number,
    /// The payout resident withholding tax amount. Always returned in the portfolio currency.
    #[serde(default)]
    pub resident_withholding_tax: Option<Number>,
    /// The payout non-resident withholding tax amount.
    #[serde(default)]
    pub non_resident_withholding_tax: Option<Number>,
    /// The payout tax credit amount. Always returned in the portfolio currency.
    #[serde(default)]
    pub tax_credit: Option<Number>,
    /// Payout currency code, using 3-letter ISO 4217 code.
    pub currency: Currency,
    /// The payout's exchange rate.
    pub exchange_rate: Number,
    /// If `true`, payout is non taxable.
    pub non_taxable: bool,
    /// Any comments for that payout.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub comments: String,
    /// ID of the company event the given payout is based on (nil if not based on any).
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub company_event_id: Option<i64>,
    /// The state of the payout, can be any of "confirmed", "unconfirmed" or "rejected".
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub state: String,
}

/// Returns a specific payout.
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
    /// The payout ID.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PayoutShowSuccess {
    /// The payout ID.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// The portfolio ID.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub portfolio_id: i64,
    /// The holding ID.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub holding_id: i64,
    /// The instrument ID.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub instrument_id: i64,
    /// Payout company/instrument symbol on the market.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub symbol: String,
    /// Market code.
    pub market: Market,
    /// The payout date (format `YYYY-MM-DD`).
    #[serde_as(as = "DeserializeDate")]
    pub paid_on: NaiveDate,
    /// The payout ex date (format `YYYY-MM-DD`).
    #[serde_as(as = "Option<DeserializeDate>")]
    #[serde(default)]
    pub ex_date: Option<NaiveDate>,
    /// The payout amount.
    pub amount: Number,
    /// The calculated gross amount
    pub gross_amount: Number,
    /// The payout resident withholding tax amount. Always returned in the portfolio currency.
    #[serde(default)]
    pub resident_withholding_tax: Option<Number>,
    /// The payout non-resident withholding tax amount.
    #[serde(default)]
    pub non_resident_withholding_tax: Option<Number>,
    /// The payout tax credit amount. Always returned in the portfolio currency.
    #[serde(default)]
    pub tax_credit: Option<Number>,
    /// Payout currency code, using 3-letter ISO 4217 code.
    pub currency: Currency,
    /// The payout's exchange rate.
    pub exchange_rate: Number,
    /// If `true`, payout is non taxable.
    pub non_taxable: bool,
    /// Any comments for that payout.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub comments: String,
    /// Other net foreign source income.
    #[serde(default)]
    pub other_net_fsi: Option<Number>,
    /// The amount of an LIC dividend that is attributable to an LIC capital gain.
    pub lic_capital_gain: Number,
    /// ID of the company event the given payout is based on (nil if not based on any).
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub company_event_id: Option<i64>,
    /// The state of the payout, can be any of "confirmed", "unconfirmed" or "rejected".
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub state: String,
    /// Parameters when the payout is reinvested.
    #[serde(default)]
    pub drp_trade_attributes: Option<PayoutShowDrpTradeAttributesSuccess>,
    /// Franked amount in the payout. (Australia only)
    #[serde(default)]
    pub franked_amount: Option<Number>,
    /// Unfranked amount in the payout (Australia only)
    #[serde(default)]
    pub unfranked_amount: Option<Number>,
    /// `true` if this payout is for a trust. (Australia only)
    #[serde(default)]
    pub trust: Option<bool>,
    /// Extra interest amount in this payout. (Australia only)
    #[serde(default)]
    pub extra_interest_payment_amount: Option<Number>,
    /// Capital gain amount in this payout. (Australia only)
    #[serde(default)]
    pub capital_gains: Option<Number>,
    /// Discounted capital gain amount in this payout. (Australia only)
    #[serde(default)]
    pub discounted_capital_gains: Option<Number>,
    /// Amount of foreign income in this payout. (Australia only)
    #[serde(default)]
    pub foreign_source_income: Option<Number>,
    /// Any non-tax assessable amount.
    #[serde(default)]
    pub non_assessable: Option<Number>,
    /// Value of deferred income in this payout. (Australia only)
    #[serde(default)]
    pub deferred_income: Option<Number>,
    /// Value of CGT concession in this payout. (Australia only)
    #[serde(default)]
    pub cgt_concession_amount: Option<Number>,
    /// Relevant for attribution managed investment trusts (AMIT) when the taxable income attributed to you is less than the cash distribution you received. This amount is non-assessable and is used to decrease your cost base for cgt purposes (Australia only)
    #[serde(default)]
    pub amit_decrease_amount: Option<Number>,
    /// Relevant for attribution managed investment trusts (AMIT) when the taxable income attributed to you is more than the cash distribution you received. This amount is non-assessable and is used to increase your cost base for cgt purposes (Australia only)
    #[serde(default)]
    pub amit_increase_amount: Option<Number>,
    /// Filename of payout attachment, if present.
    #[serde(default)]
    pub attachment_filename: Option<String>,
    /// Id of payout attachment, if present. Use the attachments endpoint to get a copy of the file.
    #[serde(default)]
    pub attachment_id: Option<String>,
    /// List of links for this payout
    pub links: PayoutShowLinksSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PayoutShowDrpTradeAttributesSuccess {
    /// `true` for a reinvested payout.
    pub dividend_reinvested: bool,
    /// How many units are reinvested.
    pub quantity: Number,
    /// Price per reinvested unit.
    pub price: Number,
    /// ID of any source adjustment for the reinvested amount.
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub source_adjustment_id: Option<i64>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PayoutShowLinksSuccess {
    /// Url of this payout
    #[serde(rename = "self")]
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub itself: String,
    /// Url of the portfolio of this payout
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub portfolio: String,
}

/// Updates a payout with the provided parameters
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
    /// The ID of the payout to be updated.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// The payout
    pub payout: PayoutUpdatePayoutParameters,
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct PayoutUpdatePayoutParameters {
    /// The date of the payout (format `YYYY-MM-DD`).
    #[serde_as(as = "DeserializeDate")]
    pub paid_on: NaiveDate,
    /// The ex date for the payout (format `YYYY-MM-DD`).
    #[serde_as(as = "Option<DeserializeDate>")]
    #[serde(default)]
    pub goes_ex_on: Option<NaiveDate>,
    /// Resident withholding tax for the payout.
    #[serde(default)]
    pub resident_withholding_tax: Option<Number>,
    /// Non-resident withholding tax for the payout
    #[serde(default)]
    pub non_resident_withholding_tax: Option<Number>,
    /// Tax credit for the payout.
    #[serde(default)]
    pub tax_credit: Option<Number>,
    /// Exchange rate for other currency payout.
    #[serde(default)]
    pub exchange_rate: Option<Number>,
    /// Payout amount. (All except Australia)
    #[serde(default)]
    pub amount: Option<Number>,
    /// The ID of any adjustment.
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub adjustment_id: Option<i64>,
    /// Any comments to be saved on the payout.
    #[serde(default)]
    pub comments: Option<String>,
    /// `true` if this payout is non-taxable.
    #[serde(default)]
    pub non_taxable: Option<bool>,
    /// Code for the payout currency, using 3-letter ISO 4217 code.
    #[serde(default)]
    pub currency_code: Option<Currency>,
    /// Date of the source payment (format `YYYY-MM-DD`).
    #[serde_as(as = "Option<DeserializeDate>")]
    #[serde(default)]
    pub source_payment_date: Option<NaiveDate>,
    /// If `true`, sync this payout to xero
    #[serde(default)]
    pub send_to_xero: Option<bool>,
    /// Banked amount for this payout.
    #[serde(default)]
    pub banked_amount: Option<Number>,
    /// ID of any source adjustment.
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub source_adjustment_id: Option<i64>,
    /// Parameters when the payout is reinvested.
    #[serde(default)]
    pub drp_trade_attributes: Option<PayoutUpdatePayoutDrpTradeAttributesParameters>,
    /// [Franked amount in the payout] (Australia only)
    #[serde(default)]
    pub franked_amount: Option<Number>,
    /// Unfranked amount in the payout (Australia only)
    #[serde(default)]
    pub unfranked_amount: Option<Number>,
    /// `true` if this payout is for a trust. (Australia only)
    #[serde(default)]
    pub trust: Option<bool>,
    /// Extra interest amount in this payout. (Australia only)
    #[serde(default)]
    pub extra_interest_payment_amount: Option<Number>,
    /// Capital gain amount in this payout. (Australia only)
    #[serde(default)]
    pub capital_gains: Option<Number>,
    /// Discounted capital gain amount in this payout. (Australia only)
    #[serde(default)]
    pub discounted_capital_gains: Option<Number>,
    /// Amount of foreign income in this payout. (Australia only)
    #[serde(default)]
    pub foreign_source_income: Option<Number>,
    /// The amount of an LIC dividend that is attributable to an LIC capital gain. (Australia only)
    #[serde(default)]
    pub lic_capital_gain: Option<Number>,
    /// Any non-tax assessable amount.
    #[serde(default)]
    pub non_assessable: Option<Number>,
    /// Value of deferred income in this payout. (Australia only)
    #[serde(default)]
    pub deferred_income: Option<Number>,
    /// Value of CGT concession in this payout. (Australia only)
    #[serde(default)]
    pub cgt_concession_amount: Option<Number>,
    /// Relevant for attribution managed investment trusts (AMIT) when the taxable income attributed to you is less than the cash distribution you received. This amount is non-assessable and is used to decrease your cost base for cgt purposes (Australia only)
    #[serde(default)]
    pub amit_decrease_amount: Option<Number>,
    /// Relevant for attribution managed investment trusts (AMIT) when the taxable income attributed to you is more than the cash distribution you received. This amount is non-assessable and is used to increase your cost base for cgt purposes (Australia only)
    #[serde(default)]
    pub amit_increase_amount: Option<Number>,
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct PayoutUpdatePayoutDrpTradeAttributesParameters {
    /// True for a reinvested payout.
    #[serde(default)]
    pub dividend_reinvested: Option<bool>,
    /// How many units are reinvested.
    #[serde(default)]
    pub quantity: Option<Number>,
    /// Price per reinvested unit.
    #[serde(default)]
    pub price: Option<Number>,
    /// ID of any source adjustment for the reinvested amount.
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub source_adjustment_id: Option<i64>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PayoutUpdateSuccess {
    /// The payout ID.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// Payout company/instrument symbol on the market.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub symbol: String,
    /// Market code.
    pub market: Market,
    /// The payout date (format `YYYY-MM-DD`).
    #[serde_as(as = "DeserializeDate")]
    pub paid_on: NaiveDate,
    /// The payout ex date (format `YYYY-MM-DD`).
    #[serde_as(as = "Option<DeserializeDate>")]
    #[serde(default)]
    pub ex_date: Option<NaiveDate>,
    /// The payout amount.
    pub amount: Number,
    /// The calculated gross amount
    pub gross_amount: Number,
    /// The payout resident withholding tax amount. Always returned in the portfolio currency.
    #[serde(default)]
    pub resident_withholding_tax: Option<Number>,
    /// The payout non-resident withholding tax amount.
    #[serde(default)]
    pub non_resident_withholding_tax: Option<Number>,
    /// The payout tax credit amount. Always returned in the portfolio currency.
    #[serde(default)]
    pub tax_credit: Option<Number>,
    /// Payout currency code, using 3-letter ISO 4217 code.
    pub currency: Currency,
    /// The payout's exchange rate.
    pub exchange_rate: Number,
    /// If true, payout is non taxable.
    pub non_taxable: bool,
    /// Any comments for that payout.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub comments: String,
    /// ID of the company event the given payout is based on (nil if not based on any).
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub company_event_id: Option<i64>,
    /// The state of the payout, can be any of `"confirmed"`, `"unconfirmed"` or `"rejected"`.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub state: String,
    /// Parameters when the payout is reinvested.
    #[serde(default)]
    pub drp_trade_attributes: Option<PayoutUpdateDrpTradeAttributesSuccess>,
    /// Franked amount in the payout. (Australia only)
    #[serde(default)]
    pub franked_amount: Option<Number>,
    /// Unfranked amount in the payout (Australia only)
    #[serde(default)]
    pub unfranked_amount: Option<Number>,
    /// `true` if this payout is for a trust. (Australia only)
    #[serde(default)]
    pub trust: Option<bool>,
    /// Extra interest amount in this payout. (Australia only)
    #[serde(default)]
    pub extra_interest_payment_amount: Option<Number>,
    /// Capital gain amount in this payout. (Australia only)
    #[serde(default)]
    pub capital_gains: Option<Number>,
    /// Discounted capital gain amount in this payout. (Australia only)
    #[serde(default)]
    pub discounted_capital_gains: Option<Number>,
    /// Amount of foreign income in this payout. (Australia only)
    #[serde(default)]
    pub foreign_source_income: Option<Number>,
    /// Any non-tax assessable amount.
    #[serde(default)]
    pub non_assessable: Option<Number>,
    /// Value of deferred income in this payout. (Australia only)
    #[serde(default)]
    pub deferred_income: Option<Number>,
    /// Value of CGT concession in this payout. (Australia only)
    #[serde(default)]
    pub cgt_concession_amount: Option<Number>,
    /// Relevant for attribution managed investment trusts (AMIT) when the taxable income attributed to you is less than the cash distribution you received. This amount is non-assessable and is used to decrease your cost base for cgt purposes (Australia only)
    #[serde(default)]
    pub amit_decrease_amount: Option<Number>,
    /// Relevant for attribution managed investment trusts (AMIT) when the taxable income attributed to you is more than the cash distribution you received. This amount is non-assessable and is used to increase your cost base for cgt purposes (Australia only)
    #[serde(default)]
    pub amit_increase_amount: Option<Number>,
    /// Filename of payout attachment, if present.
    #[serde(default)]
    pub attachment_filename: Option<String>,
    /// Id of payout attachment, if present. Use the attachments endpoint to get a copy of the file.
    #[serde(default)]
    pub attachment_id: Option<String>,
    /// List of links for this payout
    pub links: PayoutUpdateLinksSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PayoutUpdateDrpTradeAttributesSuccess {
    /// `true` for a reinvested payout.
    pub dividend_reinvested: bool,
    /// How many units are reinvested.
    pub quantity: Number,
    /// Price per reinvested unit.
    pub price: Number,
    /// ID of any source adjustment for the reinvested amount.
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub source_adjustment_id: Option<i64>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PayoutUpdateLinksSuccess {
    /// Url of the portfolio of this payout
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub portfolio: String,
}

/// Create a new portfolio for the current user.
pub struct PortfolioCreate;

impl<'a> ApiEndpoint<'a> for PortfolioCreate {
    const URL_PATH: &'static str = "/portfolios.json";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Post;

    type UrlDisplay = &'static str;
    type Parameters = PortfolioCreateParameters;
    type Success = PortfolioCreateSuccess;

    fn url_path(_parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        "/portfolios.json"
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct PortfolioCreateParameters {
    pub portfolio: PortfolioCreatePortfolioParameters,
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct PortfolioCreatePortfolioParameters {
    /// The new portfolio's name.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub name: String,
    /// `true` to enable broker import via Trade Confirmation Emails.
    pub broker_email_api_enabled: bool,
    /// (deprecated) Financial year end month (`1` for January, ..., `12` for December).
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub financial_year_end_month_id: i64,
    /// Financial year end MM-DD.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub financial_year_end: String,
    /// Default sale allocation method. See <a href="/api/2/codes#sale_allocation_method">valid methods by country</a>.
    pub default_sale_allocation_method: SaleAllocationMethod,
    /// Interest method calculation: `"simple"` or `"compound"`.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub interest_method: String,
    /// Tax Status (`true`: Trader, `false`: Investor)
    pub trader: bool,
    /// For Canadian portfolios, the type of tax processing (`"non_registered"`, `"rrsp"` or `"rrif"`)
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub tax_entity_type: String,
    /// Disable Automatic Transactions for this portfolio (`true`: disable, `false`: enable)
    #[serde(default)]
    pub disable_automatic_transactions: Option<bool>,
    /// `1`: Individuals / Trust, `2`: Self Managed Super Fund, `3`: Company. Defaults to Individuals / Trust for AU portfolios.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub cg_discount_id: i64,
    /// New Zealand Resident Withholding Tax Rate. As a percentage to 1 decimal place (for example, 33.1% comes as `33.1`).
    pub rwtr_rate: Number,
    /// ISO code of the portfolio country (see <a href="https://en.wikipedia.org/wiki/ISO_3166-1">ISO 3166-1</a>). Defaults to the portfolio owner's default country.
    #[serde(default)]
    pub country_code: Option<Country>,
    /// `true` in order to calculate accrual adjustments against any portfolio cash accounts to allow for unsettled trades and unpaid dividends
    #[serde(default)]
    pub apply_cash_account_adjustments: Option<bool>,
    /// Specifies the number of working days between the buy trade date and settlement in the cash account
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub buy_trade_settlement_delay: Option<i64>,
    /// Specifies the number of working days between the sell trade date and settlement in the cash account
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub sell_trade_settlement_delay: Option<i64>,
    /// `true` in order to account for the fact that bank statement data is delayed by a day due to overnight processing
    #[serde(default)]
    pub account_for_delayed_cash_transactions: Option<bool>,
    /// Typically used by professionals to identify the tax entity owner of the portfolio
    #[serde(default)]
    pub external_identifier: Option<String>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PortfolioCreateSuccess {
    /// The portfolio ID.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// The portfolio name.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub name: String,
    /// The default sale allocation method for tax reporting.
    pub default_sale_allocation_method: SaleAllocationMethod,
    /// Discount for Capital Gains Tax.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub cg_discount: String,
    /// New Zealand Resident Withholding Tax Rate. As a percentage to 1 decimal place (for example, 33.1% comes as `33.1`).
    pub rwtr_rate: Number,
    /// Tax Status (`true`: Trade, `false`: Investor). Can be `null`.
    pub trader: bool,
    /// Automatic Transactions are disabled (`true`) or enabled (`false`).
    pub disable_automatic_transactions: bool,
    /// For Canadian portfolios, the type of tax processing (`"non_registered"`, `"rrsp"` or `"rrif"`). Can be `null`.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub tax_entity_type: String,
    /// `true` if the broker import email is enabled. Present for portfolio admins
    pub broker_email_api_enabled: bool,
    /// Email prefix of the broker import email address. Present for portfolio admins
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub broker_email_key: String,
    /// (deprecated) Financial Year end month (`1`: Jan, `2`: Feb, etc.).
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub financial_year_end_month_id: i64,
    /// Financial Year end date MM-DD.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub financial_year_end: String,
    /// Performance Calculation Method
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub interest_method: String,
    /// ISO code of the portfolio country (see <a href="https://en.wikipedia.org/wiki/ISO_3166-1">ISO 3166-1</a>)
    pub country_code: Country,
    /// ISO code of the portfolio currency (see <a href="https://en.wikipedia.org/wiki/ISO_4217">ISO 4217</a>)
    pub currency_code: Currency,
    /// Portfolio's inception date (first trade record). Format: dd mmm yyyy
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub inception_date: String,
    /// Time zone name
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub tz_name: String,
    /// Calculates accrual adjustments against any portfolio cash accounts to allow for unsettled trades and unpaid dividends
    pub apply_cash_account_adjustments: bool,
    /// Specifies the number of working days between the buy trade date and settlement in the cash account. Can be `null`.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub buy_trade_settlement_delay: i64,
    /// Specifies the number of working days between the sell trade date and settlement in the cash account. Can be `null`.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub sell_trade_settlement_delay: i64,
    /// Accounts for the fact that bank statement data is delayed by a day due to overnight processing
    pub account_for_delayed_cash_transactions: bool,
    /// All buys and sells will generate a corresponding deposit/withdrawal in the selected 'trading' account. Can be `null`.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub trade_sync_cash_account_id: i64,
    /// All payouts will generate a corresponding deposit in the selected 'payout' account. Can be `null`.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub payout_sync_cash_account_id: i64,
    /// Typically used by professionals to identify the tax entity owner of the portfolio
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub external_identifier: String,
    /// List of links for this portfolio
    pub links: PortfolioCreateLinksSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PortfolioCreateLinksSuccess {
    /// Url of this portfolio
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub portfolio: String,
    /// Url of this portfolio
    #[serde(rename = "self")]
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub itself: String,
}

/// Delete an existing user portfolio
pub struct PortfolioDelete;

impl<'a> ApiEndpoint<'a> for PortfolioDelete {
    const URL_PATH: &'static str = "/portfolios/{id}.json";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Delete;

    type UrlDisplay = &'static str;
    type Parameters = ();
    type Success = PortfolioDeleteSuccess;

    fn url_path(_parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        "/portfolios/{id}.json"
    }
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PortfolioDeleteSuccess {
    /// The portfolio was successfully deleted.
    pub status: (),
}

/// Returns list of portfolios for the user. If the user owns the portfolio, all the info related to it will be displayed. Otherwise only basic info is returned.
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
    /// List of links for this resource
    pub links: PortfolioListLinksSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PortfolioListPortfoliosSuccess {
    /// The portfolio ID.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// The portfolio name.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub name: String,
    /// The default sale allocation method for tax reporting.
    pub default_sale_allocation_method: SaleAllocationMethod,
    /// Discount for Capital Gains Tax.
    #[serde(default)]
    pub cg_discount: Option<String>,
    /// New Zealand Resident Withholding Tax Rate. As a percentage to 1 decimal place (for example, 33.1% comes as `33.1`).
    pub rwtr_rate: Number,
    /// Tax Status (`true`: Trade, `false`: Investor). Can be `null`.
    #[serde(default)]
    pub trader: Option<bool>,
    /// Automatic Transactions are disabled (`true`) or enabled (`false`).
    pub disable_automatic_transactions: bool,
    /// For Canadian portfolios, the type of tax processing (`"non_registered"`, `"rrsp"` or `"rrif"`). Can be `null`.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub tax_entity_type: String,
    /// `true` if the broker import email is enabled. Present for portfolio admins
    pub broker_email_api_enabled: bool,
    /// Email prefix of the broker import email address. Present for portfolio admins
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub broker_email_key: String,
    /// (deprecated) Financial Year end month (`1`: Jan, `2`: Feb, etc.).
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub financial_year_end_month_id: i64,
    /// Financial Year end date MM-DD.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub financial_year_end: String,
    /// Performance Calculation Method
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub interest_method: String,
    /// ISO code of the portfolio country (see <a href="https://en.wikipedia.org/wiki/ISO_3166-1">ISO 3166-1</a>)
    pub country_code: Country,
    /// ISO code of the portfolio currency (see <a href="https://en.wikipedia.org/wiki/ISO_4217">ISO 4217</a>)
    pub currency_code: Currency,
    /// Portfolio's inception date (first trade record). Format: dd mmm yyyy
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub inception_date: String,
    /// Time zone name
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub tz_name: String,
    /// Calculates accrual adjustments against any portfolio cash accounts to allow for unsettled trades and unpaid dividends
    pub apply_cash_account_adjustments: bool,
    /// Specifies the number of working days between the buy trade date and settlement in the cash account. Can be `null`.
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub buy_trade_settlement_delay: Option<i64>,
    /// Specifies the number of working days between the sell trade date and settlement in the cash account. Can be `null`.
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub sell_trade_settlement_delay: Option<i64>,
    /// Accounts for the fact that bank statement data is delayed by a day due to overnight processing
    pub account_for_delayed_cash_transactions: bool,
    /// All buys and sells will generate a corresponding deposit/withdrawal in the selected 'trading' account. Can be `null`.
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub trade_sync_cash_account_id: Option<i64>,
    /// All payouts will generate a corresponding deposit in the selected 'payout' account. Can be `null`.
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub payout_sync_cash_account_id: Option<i64>,
    /// Typically used by professionals to identify the tax entity owner of the portfolio
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub external_identifier: String,
    /// List of links for this portfolio
    pub links: PortfolioListPortfoliosLinksSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PortfolioListPortfoliosLinksSuccess {
    /// Url of this portfolio
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub portfolio: String,
    /// Url of this portfolio
    #[serde(rename = "self")]
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub itself: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PortfolioListLinksSuccess {
    /// Url to list of portfolios
    #[serde(rename = "self")]
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub itself: String,
}

/// Returns a single portfolio with settings for the user.
pub struct PortfolioShow;

impl<'a> ApiEndpoint<'a> for PortfolioShow {
    const URL_PATH: &'static str = "/portfolios/{id}.json";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Get;

    type UrlDisplay = &'static str;
    type Parameters = ();
    type Success = PortfolioShowSuccess;

    fn url_path(_parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        "/portfolios/{id}.json"
    }
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PortfolioShowSuccess {
    /// The portfolio ID.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// The portfolio name.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub name: String,
    /// The default sale allocation method for tax reporting.
    pub default_sale_allocation_method: SaleAllocationMethod,
    /// Discount for Capital Gains Tax.
    #[serde(default)]
    pub cg_discount: Option<String>,
    /// New Zealand Resident Withholding Tax Rate. As a percentage to 1 decimal place (for example, 33.1% comes as `33.1`).
    pub rwtr_rate: Number,
    /// Tax Status (`true`: Trade, `false`: Investor). Can be `null`.
    pub trader: bool,
    /// Automatic Transactions are disabled (`true`) or enabled (`false`).
    pub disable_automatic_transactions: bool,
    /// For Canadian portfolios, the type of tax processing (`"non_registered"`, `"rrsp"` or `"rrif"`). Can be `null`.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub tax_entity_type: String,
    /// `true` if the broker import email is enabled. Present for portfolio admins
    pub broker_email_api_enabled: bool,
    /// Email prefix of the broker import email address. Present for portfolio admins
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub broker_email_key: String,
    /// (deprecated) Financial Year end month (`1`: Jan, `2`: Feb, etc.).
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub financial_year_end_month_id: i64,
    /// Financial Year end date MM-DD.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub financial_year_end: String,
    /// Performance Calculation Method
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub interest_method: String,
    /// ISO code of the portfolio country (see <a href="https://en.wikipedia.org/wiki/ISO_3166-1">ISO 3166-1</a>)
    pub country_code: Country,
    /// ISO code of the portfolio currency (see <a href="https://en.wikipedia.org/wiki/ISO_4217">ISO 4217</a>)
    pub currency_code: Currency,
    /// Portfolio's inception date (first trade record). Format: dd mmm yyyy
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub inception_date: String,
    /// Time zone name
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub tz_name: String,
    /// Calculates accrual adjustments against any portfolio cash accounts to allow for unsettled trades and unpaid dividends
    pub apply_cash_account_adjustments: bool,
    /// Specifies the number of working days between the buy trade date and settlement in the cash account. Can be `null`.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub buy_trade_settlement_delay: i64,
    /// Specifies the number of working days between the sell trade date and settlement in the cash account. Can be `null`.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub sell_trade_settlement_delay: i64,
    /// Accounts for the fact that bank statement data is delayed by a day due to overnight processing
    pub account_for_delayed_cash_transactions: bool,
    /// All buys and sells will generate a corresponding deposit/withdrawal in the selected 'trading' account. Can be `null`.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub trade_sync_cash_account_id: i64,
    /// All payouts will generate a corresponding deposit in the selected 'payout' account. Can be `null`.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub payout_sync_cash_account_id: i64,
    /// Typically used by professionals to identify the tax entity owner of the portfolio
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub external_identifier: String,
    /// List of links for this portfolio
    pub links: PortfolioShowLinksSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PortfolioShowLinksSuccess {
    /// Url of this portfolio
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub portfolio: String,
    /// Url of this portfolio
    #[serde(rename = "self")]
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub itself: String,
}

/// Update an existing portfolio for the user.
pub struct PortfolioUpdate;

impl<'a> ApiEndpoint<'a> for PortfolioUpdate {
    const URL_PATH: &'static str = "/portfolios/{id}.json";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Put;

    type UrlDisplay = &'static str;
    type Parameters = PortfolioUpdateParameters;
    type Success = PortfolioUpdateSuccess;

    fn url_path(_parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        "/portfolios/{id}.json"
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct PortfolioUpdateParameters {
    pub portfolio: PortfolioUpdatePortfolioParameters,
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct PortfolioUpdatePortfolioParameters {
    /// The new portfolio's name.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub name: String,
    /// `true` to enable broker import via Trade Confirmation Emails.
    pub broker_email_api_enabled: bool,
    /// (deprecated) Financial year end month (`1` for January, ..., `12` for December).
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub financial_year_end_month_id: i64,
    /// Financial year end MM-DD.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub financial_year_end: String,
    /// Default sale allocation method. See <a href="/api/2/codes#sale_allocation_method">valid methods by country</a>.
    pub default_sale_allocation_method: SaleAllocationMethod,
    /// Interest method calculation: `"simple"` or `"compound"`.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub interest_method: String,
    /// Tax Status (`true`: Trader, `false`: Investor)
    pub trader: bool,
    /// For Canadian portfolios, the type of tax processing (`"non_registered"`, `"rrsp"` or `"rrif"`)
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub tax_entity_type: String,
    /// Disable Automatic Transactions for this portfolio (`true`: disable, `false`: enable)
    #[serde(default)]
    pub disable_automatic_transactions: Option<bool>,
    /// `1`: Individuals / Trust, `2`: Self Managed Super Fund, `3`: Company. Defaults to Individuals / Trust for AU portfolios.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub cg_discount_id: i64,
    /// New Zealand Resident Withholding Tax Rate. As a percentage to 1 decimal place (for example, 33.1% comes as `33.1`).
    pub rwtr_rate: Number,
    /// ISO code of the portfolio country (see <a href="https://en.wikipedia.org/wiki/ISO_3166-1">ISO 3166-1</a>). Defaults to the portfolio owner's default country.
    #[serde(default)]
    pub country_code: Option<Country>,
    /// `true` in order to calculate accrual adjustments against any portfolio cash accounts to allow for unsettled trades and unpaid dividends
    #[serde(default)]
    pub apply_cash_account_adjustments: Option<bool>,
    /// Specifies the number of working days between the buy trade date and settlement in the cash account
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub buy_trade_settlement_delay: Option<i64>,
    /// Specifies the number of working days between the sell trade date and settlement in the cash account
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub sell_trade_settlement_delay: Option<i64>,
    /// `true` in order to account for the fact that bank statement data is delayed by a day due to overnight processing
    #[serde(default)]
    pub account_for_delayed_cash_transactions: Option<bool>,
    /// All buys and sells will generate a corresponding deposit/withdrawal in the selected 'trading' account. You can only select a trading cash account if it has the same currency as the portfolio currency.
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub trade_sync_cash_account_id: Option<i64>,
    /// All payouts will generate a corresponding deposit in the selected 'payout' account. Can be `null`.
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub payout_sync_cash_account_id: Option<i64>,
    /// Typically used by professionals to identify the tax entity owner of the portfolio
    #[serde(default)]
    pub external_identifier: Option<String>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PortfolioUpdateSuccess {
    /// The portfolio ID.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// The portfolio name.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub name: String,
    /// The default sale allocation method for tax reporting.
    pub default_sale_allocation_method: SaleAllocationMethod,
    /// Discount for Capital Gains Tax.
    #[serde(default)]
    pub cg_discount: Option<String>,
    /// New Zealand Resident Withholding Tax Rate. As a percentage to 1 decimal place (for example, 33.1% comes as `33.1`).
    pub rwtr_rate: Number,
    /// Tax Status (`true`: Trade, `false`: Investor). Can be `null`.
    pub trader: bool,
    /// Automatic Transactions are disabled (`true`) or enabled (`false`).
    pub disable_automatic_transactions: bool,
    /// For Canadian portfolios, the type of tax processing (`"non_registered"`, `"rrsp"` or `"rrif"`). Can be `null`.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub tax_entity_type: String,
    /// `true` if the broker import email is enabled. Present for portfolio admins
    pub broker_email_api_enabled: bool,
    /// Email prefix of the broker import email address. Present for portfolio admins
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub broker_email_key: String,
    /// (deprecated) Financial Year end month (`1`: Jan, `2`: Feb, etc.).
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub financial_year_end_month_id: i64,
    /// Financial Year end date MM-DD.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub financial_year_end: String,
    /// Performance Calculation Method
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub interest_method: String,
    /// ISO code of the portfolio country (see <a href="https://en.wikipedia.org/wiki/ISO_3166-1">ISO 3166-1</a>)
    pub country_code: Country,
    /// ISO code of the portfolio currency (see <a href="https://en.wikipedia.org/wiki/ISO_4217">ISO 4217</a>)
    pub currency_code: Currency,
    /// Portfolio's inception date (first trade record). Format: dd mmm yyyy
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub inception_date: String,
    /// Time zone name
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub tz_name: String,
    /// Calculates accrual adjustments against any portfolio cash accounts to allow for unsettled trades and unpaid dividends
    pub apply_cash_account_adjustments: bool,
    /// Specifies the number of working days between the buy trade date and settlement in the cash account. Can be `null`.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub buy_trade_settlement_delay: i64,
    /// Specifies the number of working days between the sell trade date and settlement in the cash account. Can be `null`.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub sell_trade_settlement_delay: i64,
    /// Accounts for the fact that bank statement data is delayed by a day due to overnight processing
    pub account_for_delayed_cash_transactions: bool,
    /// All buys and sells will generate a corresponding deposit/withdrawal in the selected 'trading' account. Can be `null`.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub trade_sync_cash_account_id: i64,
    /// All payouts will generate a corresponding deposit in the selected 'payout' account. Can be `null`.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub payout_sync_cash_account_id: i64,
    /// Typically used by professionals to identify the tax entity owner of the portfolio
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub external_identifier: String,
    /// List of links for this portfolio
    pub links: PortfolioUpdateLinksSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PortfolioUpdateLinksSuccess {
    /// Url of this portfolio
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub portfolio: String,
    /// Url of this portfolio
    #[serde(rename = "self")]
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub itself: String,
}

/// Return a report on capital gains tax (for Australian portfolios only)
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
    /// The id of the (Australian) portfolio to report on
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub portfolio_id: i64,
    /// inception date] Show report from this date on (`YYYY-MM-DD`).
    #[serde_as(as = "Option<DeserializeDate>")]
    #[serde(default)]
    pub start_date: Option<NaiveDate>,
    /// Show report until this date (`YYYY-MM-DD`).
    #[serde_as(as = "Option<DeserializeDate>")]
    #[serde(default)]
    pub end_date: Option<NaiveDate>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CapitalGainsSuccess {
    /// The total of short term gains (less than one year, rounded to 2 decimal places).
    pub short_term_gains: Number,
    /// The total of long term gains (over one year, rounded to 2 decimal places).
    pub long_term_gains: Number,
    /// The total of losses (rounded to 2 decimal places).
    pub losses: Number,
    /// The total of short term capital losses available to be offset (rounded to 2 decimal places).
    pub short_term_losses: Number,
    /// The total of capital losses available to be offset after deducting short term losses (rounded to 2 decimal places).
    pub long_term_losses: Number,
    /// The total of discounted capital gain distributions (grossed up, rounded to 2 decimal places).
    pub total_discounted_capital_gain_distributions: Number,
    /// The total of non discounted capital gain distributions (rounded to 2 decimal places).
    pub total_non_discounted_capital_gain_distributions: Number,
    /// The rate of CGT concession on long term gains applied (rounded to 2 decimal places).
    pub cgt_concession_rate: Number,
    /// The amount of the CGT concession applied (rounded to 2 decimal places).
    pub cgt_concession_amount: Number,
    /// The market value of the portfolio (rounded to 2 decimal places).
    pub market_value: Number,
    /// The gain (or loss, if negative, rounded to 2 decimal places).
    pub tax_gain_loss: Number,
    /// Discounted capital gain distributions
    pub discounted_capital_gain_distributions:
        Vec<CapitalGainsDiscountedCapitalGainDistributionsSuccess>,
    /// Non-discounted capital gain distributions
    pub non_discounted_capital_gain_distributions:
        Vec<CapitalGainsNonDiscountedCapitalGainDistributionsSuccess>,
    /// Stock parcels (separated by date) held for less than one year and subject to tax without concession
    pub short_term_parcels: Vec<CapitalGainsShortTermParcelsSuccess>,
    /// Stock parcels (separated by date)  held for over a year and eligible for tax concession
    pub long_term_parcels: Vec<CapitalGainsLongTermParcelsSuccess>,
    /// Holdings showing a loss (which will be expressed as a negative value)
    pub loss_parcels: Vec<CapitalGainsLossParcelsSuccess>,
    /// The start date gains are calculated from (format `YYYY-MM-DD`).
    #[serde_as(as = "DeserializeDate")]
    pub start_date: NaiveDate,
    /// The end date gains are calculated to (format `YYYY-MM-DD`).
    #[serde_as(as = "DeserializeDate")]
    pub end_date: NaiveDate,
    /// The portfolio id, as requested
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub portfolio_id: i64,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CapitalGainsDiscountedCapitalGainDistributionsSuccess {
    /// The market symbol
    pub market: Market,
    /// The instrument symbol
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub symbol: String,
    /// The name of the instrument
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub name: String,
    /// The capital gain (negative for losses, rounded to 2 decimal places).
    pub gain: Number,
    /// The date the gain is calculated for (format `YYYY-MM-DD`)
    #[serde_as(as = "DeserializeDate")]
    pub gain_date: NaiveDate,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CapitalGainsNonDiscountedCapitalGainDistributionsSuccess {
    /// The market symbol
    pub market: Market,
    /// The instrument symbol
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub symbol: String,
    /// The name of the instrument
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub name: String,
    /// The capital gain (negative for losses, rounded to 2 decimal places).
    pub gain: Number,
    /// The date the gain is calculated for (format `YYYY-MM-DD`)
    #[serde_as(as = "DeserializeDate")]
    pub gain_date: NaiveDate,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CapitalGainsShortTermParcelsSuccess {
    /// The market symbol
    pub market: Market,
    /// The instrument symbol
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub symbol: String,
    /// The name of the instrument
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub name: String,
    /// The allocation method used: see <a href="/api/2/codes#sale_allocation_method">valid methods by country</a>
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub allocation_method: String,
    /// The purchase date for this parcel (format `YYYY-MM-DD`).
    #[serde_as(as = "DeserializeDate")]
    pub purchase_date: NaiveDate,
    /// The quantity of stock held
    pub quantity: Number,
    /// The adjusted total cost of the parcel (rounded to 2 decimal places).
    pub cost_base: Number,
    /// The market value of the parcel at the balance date (rounded to 2 decimal places).
    pub market_value: Number,
    /// The capital gain (negative for losses, rounded to 2 decimal places).
    pub gain: Number,
    /// The date the gain is calculated for (format `YYYY-MM-DD`)
    #[serde_as(as = "DeserializeDate")]
    pub gain_date: NaiveDate,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CapitalGainsLongTermParcelsSuccess {
    /// The market symbol
    pub market: Market,
    /// The instrument symbol
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub symbol: String,
    /// The name of the instrument
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub name: String,
    /// The allocation method used: see <a href="/api/2/codes#sale_allocation_method">valid methods by country</a>
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub allocation_method: String,
    /// The purchase date for this parcel (format `YYYY-MM-DD`).
    #[serde_as(as = "DeserializeDate")]
    pub purchase_date: NaiveDate,
    /// The quantity of stock held
    pub quantity: Number,
    /// The adjusted total cost of the parcel (rounded to 2 decimal places).
    pub cost_base: Number,
    /// The market value of the parcel at the balance date (rounded to 2 decimal places).
    pub market_value: Number,
    /// The capital gain (negative for losses, rounded to 2 decimal places).
    pub gain: Number,
    /// The date the gain is calculated for (format `YYYY-MM-DD`)
    #[serde_as(as = "DeserializeDate")]
    pub gain_date: NaiveDate,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CapitalGainsLossParcelsSuccess {
    /// The market symbol
    pub market: Market,
    /// The instrument symbol
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub symbol: String,
    /// The name of the instrument
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub name: String,
    /// The allocation method used: see <a href="/api/2/codes#sale_allocation_method">valid methods by country</a>
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub allocation_method: String,
    /// The purchase date for this parcel (format `YYYY-MM-DD`).
    #[serde_as(as = "DeserializeDate")]
    pub purchase_date: NaiveDate,
    /// The quantity of stock held
    pub quantity: Number,
    /// The adjusted total cost of the parcel (rounded to 2 decimal places).
    pub cost_base: Number,
    /// The market value of the parcel at the balance date (rounded to 2 decimal places).
    pub market_value: Number,
    /// The capital gain (negative for losses, rounded to 2 decimal places).
    pub gain: Number,
    /// The date the gain is calculated for (format `YYYY-MM-DD`)
    #[serde_as(as = "DeserializeDate")]
    pub gain_date: NaiveDate,
}

/// Retrieves the Diversity Report for the underlying portfolio
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
    /// The portfolio ID.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub portfolio_id: i64,
    /// true if a consolidated view is requested.
    #[serde(default)]
    pub consolidated: Option<bool>,
    /// Show report from this date on (`YYYY-MM-DD`).
    #[serde_as(as = "Option<DeserializeDate>")]
    #[serde(default)]
    pub date: Option<NaiveDate>,
    /// One of: market(s), industry_classification(s), sector_classification(s), investment_type(s), countries/country, portfolio, ungrouped or custom_group.
    #[serde(default)]
    pub grouping: Option<String>,
    /// If present, the custom group id to group by, as an integer id returned from the CustomGroupsList endpoint. When this is used, the 'grouping' parameter must be set to 'custom_group'.
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub custom_group_id: Option<i64>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct DiversitySuccess {
    /// Each group in the report, keyed by the group name
    pub groups: Vec<DiversityGroupsSuccess>,
    /// The total percentage across the portfolio (always 100%)
    pub percentage: Number,
    /// The total value of the portfolio (rounded to 2 decimal places).
    pub value: Number,
    /// The date this report was generated for (format `YYYY-MM-DD`).
    #[serde_as(as = "DeserializeDate")]
    pub date: NaiveDate,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct DiversityGroupsSuccess {
    /// The group's contents
    pub group: DiversityGroupsGroupSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct DiversityGroupsGroupSuccess {
    /// A holding or cash account
    pub elements: Vec<DiversityGroupsGroupElementsSuccess>,
    /// The percentage of the portfolio in this group (rounded to 2 decimal places). For example, 33% comes as `33.0`.
    pub percentage: Number,
    /// The value of the portfolio components in this group (rounded to 2 decimal places).
    pub value: Number,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct DiversityGroupsGroupElementsSuccess {
    /// The name of this element
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub name: String,
    /// If a holding, the instrument symbol of this element
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub code: String,
    /// If a holding, the market code of this element
    pub market: Market,
    /// The percentage of the portfolio this element represents (rounded to 2 decimal places). For example, 33% comes as `33.0`.
    pub percentage: Number,
    /// The value of the holding or cash account (rounded to 2 decimal places).
    pub value: Number,
}

/// Retrieves the Performance Report for the underlying portfolio.
///
/// ## Holding Limit
///
/// Depending on user's plan the number of holdings shown in the report is limited. In that case   the following response headers will be set:   <ul>     <li>`X-HoldingLimit-Limit`: The plan's holding limit.</li>     <li>`X-HoldingLimit-Total`: The number of holding in the requested portfolio.</li>     <li>`X-HoldingLimit-Reason`: A human-readable string describing why the number of holdings is limited.</li>   </ul>
///
/// ## Remarks
///
/// <sup>1</sup> Infinity is represented by string values "Infinity" or "-Infinity"
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
    /// inception date] Show report from this date on (YYYY-MM-DD).
    #[serde_as(as = "Option<DeserializeDate>")]
    #[serde(default)]
    pub start_date: Option<NaiveDate>,
    /// Show report until this date (YYYY-MM-DD).
    #[serde_as(as = "Option<DeserializeDate>")]
    #[serde(default)]
    pub end_date: Option<NaiveDate>,
    /// The Portfolio id
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub portfolio_id: i64,
    /// true if a consolidated view is requested.
    #[serde(default)]
    pub consolidated: Option<bool>,
    /// true: include sales, false: do not include sales.
    #[serde(default)]
    pub include_sales: Option<bool>,
    /// One of: market(s), industry_classification(s), sector_classification(s), investment_type(s), countries/country, portfolio, ungrouped, or custom_group.
    #[serde(default)]
    pub grouping: Option<String>,
    /// If present, the custom group id to group by, as an integer id returned from the CustomGroupsList endpoint. When this is used, the 'grouping' parameter must be set to 'custom_group'.
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub custom_group_id: Option<i64>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PerformanceSuccess {
    /// A unique id identifying this report instance
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub id: String,
    /// The portfolio id
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub portfolio_id: i64,
    /// Grouping id or name
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub grouping: String,
    /// The id of the custom group, if any, otherwise nil if a built-in group was selected
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub custom_group_id: String,
    /// The total value of the portfolio
    pub value: Number,
    /// Capital Gain<sup>1</sup> (rounded to 2 decimal places).
    pub capital_gain: Number,
    /// Capital Gain (percentage, rounded to 2 decimal places, 33% as `33.0`)<sup>1</sup>
    pub capital_gain_percent: Number,
    /// Payout Gain<sup>1</sup> (rounded to 2 decimal places).
    pub payout_gain: Number,
    /// Payout Gain (percentage, rounded to 2 decimal places, 33% as `33.0`)<sup>1</sup>
    pub payout_gain_percent: Number,
    /// Currency Gain<sup>1</sup> (rounded to 2 decimal places).
    pub currency_gain: Number,
    /// Currency Gain (percentage, rounded to 2 decimal places, 33% as `33.0`)<sup>1</sup>
    pub currency_gain_percent: Number,
    /// Total Gain<sup>1</sup> (rounded to 2 decimal places).
    pub total_gain: Number,
    /// Total Gain (percentage, rounded to 2 decimal places, 33% as ``33.0``)<sup>1</sup>
    pub total_gain_percent: Number,
    /// Start date (format `YYYY-MM-DD`)
    #[serde_as(as = "DeserializeDate")]
    pub start_date: NaiveDate,
    /// End date (format `YYYY-MM-DD`)
    #[serde_as(as = "DeserializeDate")]
    pub end_date: NaiveDate,
    /// Include sales
    pub include_sales: bool,
    /// List of holdings.
    pub holdings: Vec<PerformanceHoldingsSuccess>,
    /// List of cash accounts. This includes Unsettled Trades and Unpaid Payout Adjustments.
    pub cash_accounts: Vec<PerformanceCashAccountsSuccess>,
    /// List of sub-totals for each group.
    pub sub_totals: Vec<PerformanceSubTotalsSuccess>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PerformanceHoldingsSuccess {
    /// The id of this holding
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// The Sharesight symbol for the held instrument
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub symbol: String,
    /// A unique id identifying the instrument
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub instrument_id: i64,
    /// The code for the market the held instrument is listed on
    pub market: Market,
    /// The group value this instrument has been placed in - note that the field name will be the group type
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub _group_type_: String,
    /// The name of the selected grouping for the report
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub grouping: String,
    /// The name of the instrument
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub name: String,
    /// The current value of the holding.
    pub value: Number,
    /// The quantity of shares or other instruments in the holding
    pub quantity: Number,
    /// Capital Gain<sup>1</sup> on the holding (rounded to 2 decimal places).
    pub capital_gain: Number,
    /// Capital Gain (percentage, rounded to 2 decimal places, 33% as `33.0`)<sup>1</sup> on the holding
    pub capital_gain_percent: Number,
    /// Payout Gain<sup>1</sup> on the holding (rounded to 2 decimal places).
    pub payout_gain: Number,
    /// Payout Gain (percentage, rounded to 2 decimal places, 33% as `33.0`)<sup>1</sup> on the holding
    pub payout_gain_percent: Number,
    /// Currency Gain<sup>1</sup> on the holding (rounded to 2 decimal places).
    pub currency_gain: Number,
    /// Currency Gain (percentage, rounded to 2 decimal places, 33% as `33.0`)<sup>1</sup> on the holding
    pub currency_gain_percent: Number,
    /// Total Gain<sup>1</sup> on the holding (rounded to 2 decimal places).
    pub total_gain: Number,
    /// Total Gain (percentage, rounded to 2 decimal places, 33% as `33.0`)<sup>1</sup> on the holding
    pub total_gain_percent: Number,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PerformanceCashAccountsSuccess {
    /// A unique key for each cash account
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub id: String,
    /// The id of the cash account, null for adjustments
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub cash_account_id: String,
    /// The name of the cash account
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub name: String,
    /// The value of the cash account
    pub value: Number,
    /// The currency symbol (e.g. AU$) of the cash account
    pub currency: Currency,
    /// The ISO currency code (e.g. AUD) of the portfolio
    pub currency_code: Currency,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PerformanceSubTotalsSuccess {
    /// The group value - note that the field name will be the group type
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub _group_type_: String,
    /// The total value of the holdings in this group
    pub value: Number,
    /// Capital Gain<sup>1</sup> on the holdings in this group (rounded to 2 decimal places).
    pub capital_gain: Number,
    /// Capital Gain (percentage, rounded to 2 decimal places, 33% as `33.0`)<sup>1</sup>
    pub capital_gain_percent: Number,
    /// Payout Gain<sup>1</sup> on the holdings in this group (rounded to 2 decimal places).
    pub payout_gain: Number,
    /// Payout Gain (percentage, rounded to 2 decimal places, 33% as `33.0`)<sup>1</sup> on the holdings in this group
    pub payout_gain_percent: Number,
    /// Currency Gain<sup>1</sup> on the holdings in this group (rounded to 2 decimal places).
    pub currency_gain: Number,
    /// Currency Gain (percentage, rounded to 2 decimal places, 33% as `33.0`)<sup>1</sup> on the holdings in this group
    pub currency_gain_percent: Number,
    /// Total Gain<sup>1</sup> on the holdings in this group (rounded to 2 decimal places).
    pub total_gain: Number,
    /// Total Gain (percentage, rounded to 2 decimal places, 33% as `33.0`)<sup>1</sup> on the holdings in this group
    pub total_gain_percent: Number,
}

/// Return a report on unrealised capital gains tax (for Australian portfolios only)
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
    /// The id of the (Australian) portfolio to report on
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub portfolio_id: i64,
    /// The reference date to price the holdings on (format `YYYY-MM-DD`)
    #[serde_as(as = "DeserializeDate")]
    pub balance_date: NaiveDate,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct UnrealisedCgtSuccess {
    /// The total of unrealized short term gains (less than one year, rounded to 2 decimal places).
    pub unrealised_short_term_gains: Number,
    /// The total of unrealized long term gains (over one year, rounded to 2 decimal places).
    pub unrealised_long_term_gains: Number,
    /// The total of unrealised lossses (rounded to 2 decimal places).
    pub unrealised_losses: Number,
    /// The rate of CGT concession on long term gains applied (rounded to 2 decimal places).
    pub cgt_concession_rate: Number,
    /// The amount of the CGT concession applied (rounded to 2 decimal places).
    pub unrealised_cgt_concession_amount: Number,
    /// The market value of the portfolio (rounded to 2 decimal places).
    pub market_value: Number,
    /// The unrealised gain (or loss, if negative, rounded to 2 decimal places).
    pub unrealised_tax_gain_loss: Number,
    /// Stock parcels (separated by date) held for less than one year and subject to tax without concession
    pub short_term_parcels: Vec<UnrealisedCgtShortTermParcelsSuccess>,
    /// Stock parcels (separated by date)  held for over a year and eligible for tax concession
    pub long_term_parcels: Vec<UnrealisedCgtLongTermParcelsSuccess>,
    /// Holdings showing a loss (which will be expressed as a negative value)
    pub losses: Vec<UnrealisedCgtLossesSuccess>,
    /// The balance date capital gains are calculated for, as requested (format `YYYY-MM-DD`)
    #[serde_as(as = "DeserializeDate")]
    pub balance_date: NaiveDate,
    /// The portfolio id, as requested
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub portfolio_id: i64,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct UnrealisedCgtShortTermParcelsSuccess {
    /// The market symbol
    pub market: Market,
    /// The instrument symbol
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub symbol: String,
    /// The name of the instrument
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub name: String,
    /// The allocation method used: see <a href="/api/2/codes#sale_allocation_method">valid methods by country</a>
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub allocation_method: String,
    /// The purchase date for this parcel (format `YYYY-MM-DD`)
    #[serde_as(as = "DeserializeDate")]
    pub purchase_date: NaiveDate,
    /// The quantity of stock held
    pub quantity: Number,
    /// The adjusted total cost of the parcel (rounded to 2 decimal places).
    pub cost_base: Number,
    /// The market value of the parcel at the balance date (rounded to 2 decimal places).
    pub market_value: Number,
    /// The unrealised capital gain (negative for losses, rounded to 2 decimal places).
    pub unrealised_gain: Number,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct UnrealisedCgtLongTermParcelsSuccess {
    /// The market symbol
    pub market: Market,
    /// The instrument symbol
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub symbol: String,
    /// The name of the instrument
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub name: String,
    /// The allocation method used: see <a href="/api/2/codes#sale_allocation_method">valid methods by country</a>
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub allocation_method: String,
    /// The purchase date for this parcel (format `YYYY-MM-DD`)
    #[serde_as(as = "DeserializeDate")]
    pub purchase_date: NaiveDate,
    /// The quantity of stock held
    pub quantity: Number,
    /// The adjusted total cost of the parcel (rounded to 2 decimal places).
    pub cost_base: Number,
    /// The market value of the parcel at the balance date (rounded to 2 decimal places).
    pub market_value: Number,
    /// The unrealised capital gain (negative for losses, rounded to 2 decimal places).
    pub unrealised_gain: Number,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct UnrealisedCgtLossesSuccess {
    /// The market symbol
    pub market: Market,
    /// The instrument symbol
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub symbol: String,
    /// The name of the instrument
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub name: String,
    /// The allocation method used: see <a href="/api/2/codes#sale_allocation_method">valid methods by country</a>
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub allocation_method: String,
    /// The purchase date for this parcel (format `YYYY-MM-DD`)
    #[serde_as(as = "DeserializeDate")]
    pub purchase_date: NaiveDate,
    /// The quantity of stock held
    pub quantity: Number,
    /// The adjusted total cost of the parcel (rounded to 2 decimal places).
    pub cost_base: Number,
    /// The market value of the parcel at the balance date (rounded to 2 decimal places).
    pub market_value: Number,
    /// The unrealised capital gain (negative for losses, rounded to 2 decimal places).
    pub unrealised_gain: Number,
}

/// Retrieves the Valuation Report for the underlying portfolio.
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
    /// Show report as of this date (`YYYY-MM-DD`).
    #[serde_as(as = "Option<DeserializeDate>")]
    #[serde(default)]
    pub balance_date: Option<NaiveDate>,
    /// The Portfolio id
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub portfolio_id: i64,
    /// true if a consolidated view is requested.
    #[serde(default)]
    pub consolidated: Option<bool>,
    /// true: include sales, false: do not include sales.
    #[serde(default)]
    pub include_sales: Option<bool>,
    /// One of: market(s), industry_classification(s), sector_classification(s), investment_type(s), countries/country, portfolio, ungrouped, or custom_group.
    #[serde(default)]
    pub grouping: Option<String>,
    /// If present, the custom group id to group by, as an integer id returned from the CustomGroupsList endpoint. When this is used, the 'grouping' parameter must be set to 'custom_group'.
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub custom_group_id: Option<i64>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct ValuationSuccess {
    /// A unique id identifying this report instance
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub id: String,
    /// Report is as of this date.
    #[serde_as(as = "DeserializeDate")]
    pub balance_date: NaiveDate,
    /// The portfolio id
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub portfolio_id: i64,
    /// Grouping id or name
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub grouping: String,
    /// The id of the custom group, if any, otherwise nil if a built-in group was selected
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub custom_group_id: String,
    /// The total value of the portfolio
    pub value: Number,
    /// List of holdings.
    pub holdings: Vec<ValuationHoldingsSuccess>,
    /// List of cash accounts. This includes Unsettled Trades and Unpaid Payout Adjustments.
    pub cash_accounts: Vec<ValuationCashAccountsSuccess>,
    /// List of sub-totals for each group.
    pub sub_totals: Vec<ValuationSubTotalsSuccess>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct ValuationHoldingsSuccess {
    /// The id of this holding
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// The Sharesight symbol for the held instrument
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub symbol: String,
    /// A unique id identifying the instrument
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub instrument_id: i64,
    /// The code for the market the held instrument is listed on
    pub market: Market,
    /// The group value this instrument has been placed in - note that the field name will be the group type
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub _group_type_: String,
    /// The name of the selected grouping for the report
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub grouping: String,
    /// The name of the instrument
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub name: String,
    /// The current value of the holding.
    pub value: Number,
    /// The quantity of shares or other instruments in the holding
    pub quantity: Number,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct ValuationCashAccountsSuccess {
    /// A unique key for each cash account
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub id: String,
    /// The id of the cash account, null for adjustments
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub cash_account_id: String,
    /// The name of the cash account
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub name: String,
    /// The value of the cash account
    pub value: Number,
    /// The currency symbol (e.g. AU$) of the cash account
    pub currency: Currency,
    /// The ISO currency code (e.g. AUD) of the portfolio
    pub currency_code: Currency,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct ValuationSubTotalsSuccess {
    /// The group value - note that the field name will be the group type
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub _group_type_: String,
    /// The total value of the holdings in this group
    pub value: Number,
}

/// Single sign on authorisation. The single sign-on operation returns a URL that will allow the user to login to their Sharesight account without the need to enter their email address and password. The URL is valid for one minute. A single sign-on link or button in your application should be implemented so that the user click initiates this API call and then the URL returned is launched in the user’s browser. A "redirect_to" parameter can be appended to the login url. After successfully been logged in, the user will be then redirected to the specified redirect_to path; example: https://api.sharesight.com/users/sign_in?signon-token=token&amp;redirect_to=/portfolios/1
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
    /// Single-sign-on link
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub login_url: String,
}

/// Confirm a trade. This can be either a trade based on a company event or a sell trade based on the maturity date of an interest instrument.
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
    /// The ID of the holding.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub holding_id: i64,
    /// ID used to identify the company event the unconfirmed trade is based on.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub company_event_id: i64,
    /// Date used to identify the sell trade based on a maturity date of an interest instrument (format `YYYY-MM-DD`).
    #[serde_as(as = "DeserializeDate")]
    pub transaction_date: NaiveDate,
    /// The new state of the trade: `"confirmed"`.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub state: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct TradeConfirmSuccess {
    /// The confirmed trade.
    pub trade: TradeConfirmTradeSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct TradeConfirmTradeSuccess {
    /// The confirmed trade's ID.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// The trade type (`"BUY"`, `"SELL"`, `"SPLIT"`, etc).
    pub transaction_type: TradeDescription,
    /// The trade date (matches the maturity date for interest instruments, format `YYYY-MM-DD`).
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub transaction_date: String,
    /// The market code (`"ASX"`, `"NZX"`, etc).
    pub market: Market,
    /// The instrument code/symbol.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub symbol: String,
    /// Number of shares sold/bought.
    pub quantity: Number,
    /// Price paid/received.
    pub price: Number,
    /// The transfer's exchange rate.
    pub exchange_rate: Number,
    /// The transfer's brokerage.
    pub brokerage: Number,
    /// The brokerage currency.
    pub brokerage_currency_code: Currency,
    /// The value for the trade as displayed in the 'value' column of the UI. For a return of capital, this will be the (signed) capital return value. For a cost base adjustment, this will be the value of the adjustment. For an opening balance, this will be the market value. In each case this is in portfolio currency (rounded to 2 decimal places).
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub value: String,
    /// Any comments for that trade.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub comments: String,
    /// Portfolio ID of the trade.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub portfolio_id: i64,
    /// Holding ID of the trade.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub holding_id: i64,
    /// Instrument ID of the related Holding.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub instrument_id: i64,
    /// ID of the company event the given trade is based on (nil if not based on any).
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub company_event_id: Option<i64>,
    /// The state of the trade, can be any of `"confirmed"`, `"unconfirmed"` or `"rejected"`.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub state: String,
}

/// Reject a trade. This can be either a trade based on a company event or a sell trade based on the maturity date of an interest instrument.
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
    /// The ID of the holding.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub holding_id: i64,
    /// ID used to identify the company event the unconfirmed trade is based on.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub company_event_id: i64,
    /// Date used to identify the sell trade based on a maturity date of an interest instrument (format `YYYY-MM-DD`).
    #[serde_as(as = "DeserializeDate")]
    pub transaction_date: NaiveDate,
    /// The new state of the trade: `"rejected"`.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub state: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct TradeRejectSuccess {
    /// The rejected trade.
    pub trade: TradeRejectTradeSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct TradeRejectTradeSuccess {
    /// The rejected trade's ID.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// The trade type (`"BUY"`, `"SELL"`, `"SPLIT"`, etc).
    pub transaction_type: TradeDescription,
    /// The trade date (matches the maturity date for interest instruments, format `YYYY-MM-DD`).
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub transaction_date: String,
    /// The market code (`"ASX"`, `"NZX"`, etc).
    pub market: Market,
    /// The instrument code/symbol.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub symbol: String,
    /// Number of shares sold/bought.
    pub quantity: Number,
    /// Price paid/received.
    pub price: Number,
    /// The transfer's exchange rate.
    pub exchange_rate: Number,
    /// The transfer's brokerage.
    pub brokerage: Number,
    /// The brokerage currency.
    pub brokerage_currency_code: Currency,
    /// The value for the trade as displayed in the 'value' column of the UI. For a return of capital, this will be the (signed) capital return value. For a cost base adjustment, this will be the value of the adjustment. For an opening balance, this will be the market value. In each case this is in portfolio currency (rounded to 2 decimal places).
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub value: String,
    /// Any comments for that trade.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub comments: String,
    /// Portfolio ID of the trade.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub portfolio_id: i64,
    /// Holding ID of the trade.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub holding_id: i64,
    /// Instrument ID of the related Holding.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub instrument_id: i64,
    /// ID of the company event the given trade is based on (nil if not based on any).
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub company_event_id: Option<i64>,
    /// The state of the trade, can be any of `"confirmed"`, `"unconfirmed"` or `"rejected"`.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub state: String,
}

/// Returns trade transactions for the user’s portfolio between the two supplied dates. If you want to only access the trades of a particular holding, use `https://api.sharesight.com/api/v2/holdings/:holding_id/trades.json`. The response will be the same.
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
    /// The portfolio ID (to show trades for).
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub portfolio_id: String,
    /// Show trades from this date on. Defaults to portfolio inception date. Default value: inception_date
    #[serde_as(as = "Option<DeserializeDate>")]
    #[serde(default)]
    pub start_date: Option<NaiveDate>,
    /// Show trades until this date (format `YYYY-MM-DD`). Default value: `Today`
    #[serde_as(as = "Option<DeserializeDate>")]
    #[serde(default)]
    pub end_date: Option<NaiveDate>,
    /// Search for trade with the given unique identifier.
    #[serde(default)]
    pub unique_identifier: Option<String>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct TradesSuccess {
    pub trades: Vec<TradesTradesSuccess>,
    /// The current API Transaction.
    pub api_transaction: TradesApiTransactionSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct TradesTradesSuccess {
    /// The trade ID. Maybe nil if the trade is based on an adjustment and has not yet been confirmed by the user.
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub id: Option<i64>,
    /// A unique identifier associated with this trade
    #[serde(default)]
    pub unique_identifier: Option<String>,
    /// The trade date (format YYYY-MM-DD).
    #[serde_as(as = "DeserializeDate")]
    pub transaction_date: NaiveDate,
    /// Number of shares sold/bought.
    pub quantity: Number,
    /// Price paid/received.
    pub price: Number,
    /// For an opening balance, the cost base of the trade. Always returned in the portfolio currency
    #[serde(default)]
    pub cost_base: Option<Number>,
    /// The trade's exchange rate as portfolio currency / instrument currency.
    pub exchange_rate: Number,
    /// The trade's brokerage.
    pub brokerage: Number,
    /// The ISO code of the brokerage currency, must be either Portfolio or Instrument currency. If the instrument is a cryptocurrency, any valid brokerage currency is supported.
    #[serde(default)]
    pub brokerage_currency_code: Option<Currency>,
    /// The value for the trade as displayed in the 'value' column of the UI. For a return of capital, this will be the (signed) capital return value. For a capital call, this will be the (positive) capital return value. For a cost base adjustment, this will be the value of the adjustment. For an opening balance, this will be the market value: the market price x quantity at the opening balance date In each case this is in portfolio currency (rounded to 2 decimal places).
    pub value: Number,
    /// For a CAPITAL_RETURN or other trade with a linked payout, this is the paid on date
    #[serde_as(as = "Option<DeserializeDate>")]
    #[serde(default)]
    pub paid_on: Option<NaiveDate>,
    /// The company event linked to the transaction.
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub company_event_id: Option<i64>,
    /// Any comments for that trade.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub comments: String,
    /// Portfolio ID of the trade.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub portfolio_id: i64,
    /// Holding ID of the trade.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub holding_id: i64,
    /// The state of the trade, can be any of "confirmed", "unconfirmed" or "rejected".
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub state: String,
    /// The trade type ('BUY','SELL','SPLIT','BONUS','CONSOLD','CANCEL','CAPITAL_RETURN','OPENING_BALANCE','ADJUST_COST_BASE','CAPITAL_CALL').
    pub transaction_type: TradeDescription,
    /// Instrument ID of the related Holding.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub instrument_id: i64,
    /// The instrument code/symbol
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub symbol: String,
    /// The market code (eg. `ASX`, `NZX`, etc).
    pub market: Market,
    /// The filename of any attachment
    #[serde(default)]
    pub attachment_filename: Option<String>,
    /// The document id of any attachment, for use with the Show Document API (v2)
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub attachment_id: Option<i64>,
    /// Returns 'true' if trade is confirmed.  DEPRECATED: Use the state field to determine 'confirmed' vs. 'rejected' vs. 'unconfirmed' instead.
    pub confirmed: bool,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct TradesApiTransactionSuccess {
    /// The unique API Transaction id.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// The API version you called.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub version: i64,
    /// The path executed.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub action: String,
    /// When the transaction was executed.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub timestamp: String,
}

/// Create a trade against a Portfolio or an existing Holding.
pub struct TradesCreate;

impl<'a> ApiEndpoint<'a> for TradesCreate {
    const URL_PATH: &'static str = "/trades.json";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Post;

    type UrlDisplay = &'static str;
    type Parameters = TradesCreateParameters;
    type Success = TradesCreateSuccess;

    fn url_path(_parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        "/trades.json"
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct TradesCreateParameters {
    pub trade: TradesCreateTradeParameters,
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct TradesCreateTradeParameters {
    /// Portfolio ID to create the trade against. Alternatively, you can pass a 'holding_id' and also avoid sending instrument lookup parameters.
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub portfolio_id: Option<i64>,
    /// Holding ID to create the trade for. If you specify this, you can avoid specifying 'portfolio_id' and instrument lookup parameters.
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub holding_id: Option<i64>,
    /// This is a string of up to 255 characters used to identify duplicate trades. Generate an identifier for each trade and Sharesight will check on upload that this trade has not already been loaded.
    #[serde(default)]
    pub unique_identifier: Option<String>,
    /// For an confirm trade, ID used to identify the company event the unconfirmed trade is based on.
    #[serde(default)]
    pub company_event_id: Option<String>,
    /// For an confirm trade, the new state of the trade.
    #[serde(default)]
    pub state: Option<String>,
    /// The transaction or trade date (format YYYY-MM-DD).
    #[serde(default)]
    pub transaction_date: Option<String>,
    /// Number of units in the transaction. Must be a whole number unless the market allows fractional quantities to be traded.
    #[serde(default)]
    pub quantity: Option<Number>,
    /// Currency value per unit.
    #[serde(default)]
    pub price: Option<Number>,
    /// For an opening balance, the cost base of the trade.
    #[serde(default)]
    pub cost_base: Option<Number>,
    /// The exchange rate used for the transaction as portfolio currency / instrument currency.
    #[serde(default)]
    pub exchange_rate: Option<Number>,
    /// The brokerage fee (currency value).
    #[serde(default)]
    pub brokerage: Option<Number>,
    /// The ISO code of the brokerage currency, must be either Portfolio or Instrument currency. If the instrument is a cryptocurrency, any valid brokerage currency is supported.
    #[serde(default)]
    pub brokerage_currency_code: Option<Currency>,
    /// When transaction_type "ADJUST_COST_BASE" was chosen, this is the required value
    #[serde(default)]
    pub adjust_cost_base_value: Option<Number>,
    /// When transaction_type "CAPITAL_RETURN" or "CAPITAL_CALL" is chosen, this is the required value
    #[serde(default)]
    pub capital_return_value: Option<Number>,
    /// When transaction_type "CAPITAL_RETURN" or "CAPITAL_CALL" is chosen, this is the paid on date
    #[serde_as(as = "Option<DeserializeDate>")]
    #[serde(default)]
    pub paid_on: Option<NaiveDate>,
    /// Comments against the trade.
    #[serde(default)]
    pub comments: Option<String>,
    /// An `instrument_id` to look up the Instrument.  Required unless you specify a `holding_id` or search by `instrument_code` and `instrument_market_code`.
    #[serde(default)]
    pub instrument_id: Option<String>,
    /// Code to look up the Instrument; must be accompanied by `market`. Required unless you specify a `holding_id`.
    #[serde(default)]
    pub symbol: Option<String>,
    /// Market Code to look up the Instrument; must be accompanied by `symbol`.  Required unless you specify a `holding_id`.
    #[serde(default)]
    pub market: Option<Market>,
    /// Market Country Code to look up the Instrument; must be accompanied by `symbol`.  Required unless you specify a `symbol` with `market`, or a `holding_id`.
    #[serde(default)]
    pub market_country_code: Option<String>,
    /// Supported transaction types: 'BUY','SELL','SPLIT','BONUS','CONSOLD','CANCEL','CAPITAL_RETURN','OPENING_BALANCE','ADJUST_COST_BASE','CAPITAL_CALL'.
    #[serde(default)]
    pub transaction_type: Option<TradeDescription>,
    /// Base64 encoded file to be attached to the trade.
    #[serde(default)]
    pub attachment: Option<String>,
    /// File name for the attachment. This parameter is required if attachment is set.
    #[serde(default)]
    pub attachment_filename: Option<String>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct TradesCreateSuccess {
    pub trade: TradesCreateTradeSuccess,
    /// The current API Transaction.
    pub api_transaction: TradesCreateApiTransactionSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct TradesCreateTradeSuccess {
    /// The trade ID. Maybe nil if the trade is based on an adjustment and has not yet been confirmed by the user.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub id: String,
    /// A unique identifier associated with this trade
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub unique_identifier: String,
    /// The trade date (format YYYY-MM-DD).
    #[serde_as(as = "DeserializeDate")]
    pub transaction_date: NaiveDate,
    /// Number of shares sold/bought.
    pub quantity: Number,
    /// Price paid/received.
    pub price: Number,
    /// For an opening balance, the cost base of the trade. Always returned in the portfolio currency
    pub cost_base: Number,
    /// The trade's exchange rate as portfolio currency / instrument currency.
    pub exchange_rate: Number,
    /// The trade's brokerage.
    pub brokerage: Number,
    /// The ISO code of the brokerage currency, must be either Portfolio or Instrument currency. If the instrument is a cryptocurrency, any valid brokerage currency is supported.
    pub brokerage_currency_code: Currency,
    /// The value for the trade as displayed in the 'value' column of the UI. For a return of capital, this will be the (signed) capital return value. For a capital call, this will be the (positive) capital return value. For a cost base adjustment, this will be the value of the adjustment. For an opening balance, this will be the market value: the market price x quantity at the opening balance date In each case this is in portfolio currency (rounded to 2 decimal places).
    pub value: Number,
    /// For a CAPITAL_RETURN or other trade with a linked payout, this is the paid on date
    #[serde_as(as = "DeserializeDate")]
    pub paid_on: NaiveDate,
    /// The company event linked to the transaction.
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub company_event_id: Option<i64>,
    /// Any comments for that trade.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub comments: String,
    /// Portfolio ID of the trade.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub portfolio_id: i64,
    /// Holding ID of the trade.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub holding_id: i64,
    /// The state of the trade, can be any of "confirmed", "unconfirmed" or "rejected".
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub state: String,
    /// The trade type ('BUY','SELL','SPLIT','BONUS','CONSOLD','CANCEL','CAPITAL_RETURN','OPENING_BALANCE','ADJUST_COST_BASE','CAPITAL_CALL').
    pub transaction_type: TradeDescription,
    /// Instrument ID of the related Holding.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub instrument_id: i64,
    /// The instrument code/symbol
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub symbol: String,
    /// The market code (eg. `ASX`, `NZX`, etc).
    pub market: Market,
    /// The filename of any attachment
    #[serde(default)]
    pub attachment_filename: Option<String>,
    /// The document id of any attachment, for use with the Show Document API (v2)
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub attachment_id: Option<i64>,
    /// Returns 'true' if trade is confirmed.  DEPRECATED: Use the state field to determine 'confirmed' vs. 'rejected' vs. 'unconfirmed' instead.
    pub confirmed: bool,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct TradesCreateApiTransactionSuccess {
    /// The unique API Transaction id.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// The API version you called.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub version: i64,
    /// The path executed.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub action: String,
    /// When the transaction was executed.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub timestamp: String,
}

/// Deletes a trade
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
    /// id of the trade to delete
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct TradesDestroySuccess {
    /// true if the record was deleted
    pub deleted: bool,
    /// The current API Transaction.
    pub api_transaction: TradesDestroyApiTransactionSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct TradesDestroyApiTransactionSuccess {
    /// The unique API Transaction id.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// The API version you called.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub version: i64,
    /// The path executed.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub action: String,
    /// When the transaction was executed.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub timestamp: String,
}

/// Returns a trade transaction.
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
    /// The trade ID (to show trades for).
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub id: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct TradesShowSuccess {
    /// The trade ID. Maybe nil if the trade is based on an adjustment and has not yet been confirmed by the user.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub id: String,
    /// A unique identifier associated with this trade
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub unique_identifier: String,
    /// The trade date (format YYYY-MM-DD).
    #[serde_as(as = "DeserializeDate")]
    pub transaction_date: NaiveDate,
    /// Number of shares sold/bought.
    pub quantity: Number,
    /// Price paid/received.
    pub price: Number,
    /// For an opening balance, the cost base of the trade. Always returned in the portfolio currency
    pub cost_base: Number,
    /// The trade's exchange rate as portfolio currency / instrument currency.
    pub exchange_rate: Number,
    /// The trade's brokerage.
    pub brokerage: Number,
    /// The ISO code of the brokerage currency, must be either Portfolio or Instrument currency. If the instrument is a cryptocurrency, any valid brokerage currency is supported.
    pub brokerage_currency_code: Currency,
    /// The value for the trade as displayed in the 'value' column of the UI. For a return of capital, this will be the (signed) capital return value. For a capital call, this will be the (positive) capital return value. For a cost base adjustment, this will be the value of the adjustment. For an opening balance, this will be the market value: the market price x quantity at the opening balance date In each case this is in portfolio currency (rounded to 2 decimal places).
    pub value: Number,
    /// For a CAPITAL_RETURN or other trade with a linked payout, this is the paid on date
    #[serde_as(as = "DeserializeDate")]
    pub paid_on: NaiveDate,
    /// The company event linked to the transaction.
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub company_event_id: Option<i64>,
    /// Any comments for that trade.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub comments: String,
    /// Portfolio ID of the trade.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub portfolio_id: i64,
    /// Holding ID of the trade.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub holding_id: i64,
    /// The state of the trade, can be any of "confirmed", "unconfirmed" or "rejected".
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub state: String,
    /// The trade type ('BUY','SELL','SPLIT','BONUS','CONSOLD','CANCEL','CAPITAL_RETURN','OPENING_BALANCE','ADJUST_COST_BASE','CAPITAL_CALL').
    pub transaction_type: TradeDescription,
    /// Instrument ID of the related Holding.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub instrument_id: i64,
    /// The instrument code/symbol
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub symbol: String,
    /// The market code (eg. `ASX`, `NZX`, etc).
    pub market: Market,
    /// The filename of any attachment
    #[serde(default)]
    pub attachment_filename: Option<String>,
    /// The document id of any attachment, for use with the Show Document API (v2)
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub attachment_id: Option<i64>,
    /// Returns 'true' if trade is confirmed.  DEPRECATED: Use the state field to determine 'confirmed' vs. 'rejected' vs. 'unconfirmed' instead.
    pub confirmed: bool,
    /// The current API Transaction.
    pub api_transaction: TradesShowApiTransactionSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct TradesShowApiTransactionSuccess {
    /// The unique API Transaction id.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// The API version you called.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub version: i64,
    /// The path executed.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub action: String,
    /// When the transaction was executed.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub timestamp: String,
}

/// Update an existing trade for the user. The trade is identified by the 'id'. Any parameter supplied will replace that field on the trade. Other fields will remain unchanged.
pub struct TradesUpdate;

impl<'a> ApiEndpoint<'a> for TradesUpdate {
    const URL_PATH: &'static str = "/trades/:id.json";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Put;

    type UrlDisplay = TradesUpdateUrlDisplay<'a>;
    type Parameters = TradesUpdateParameters;
    type Success = TradesUpdateSuccess;

    fn url_path(parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        TradesUpdateUrlDisplay(parameters)
    }
}

pub struct TradesUpdateUrlDisplay<'a>(&'a TradesUpdateParameters);

impl<'a> fmt::Display for TradesUpdateUrlDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parameters = self.0;

        write!(f, "/trades/{}.json", parameters.id)
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct TradesUpdateParameters {
    /// id of the trade to update.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    pub trade: TradesUpdateTradeParameters,
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct TradesUpdateTradeParameters {
    /// The transaction or trade date (format YYYY-MM-DD).
    #[serde(default)]
    pub transaction_date: Option<String>,
    /// Number of units in the transaction. Must be a whole number unless the market allows fractional quantities to be traded.
    #[serde(default)]
    pub quantity: Option<Number>,
    /// Currency value per unit.
    #[serde(default)]
    pub price: Option<Number>,
    /// For an opening balance, the cost base of the trade.
    #[serde(default)]
    pub cost_base: Option<Number>,
    /// The exchange rate used for the transaction as portfolio currency / instrument currency.
    #[serde(default)]
    pub exchange_rate: Option<Number>,
    /// The brokerage fee (currency value).
    #[serde(default)]
    pub brokerage: Option<Number>,
    /// The ISO code of the brokerage currency, must be either Portfolio or Instrument currency. If the instrument is a cryptocurrency, any valid brokerage currency is supported.
    #[serde(default)]
    pub brokerage_currency_code: Option<Currency>,
    /// When transaction_type "ADJUST_COST_BASE" was chosen, this is the required value
    #[serde(default)]
    pub adjust_cost_base_value: Option<Number>,
    /// When transaction_type "CAPITAL_RETURN" or "CAPITAL_CALL" is chosen, this is the required value
    #[serde(default)]
    pub capital_return_value: Option<Number>,
    /// When transaction_type "CAPITAL_RETURN" or "CAPITAL_CALL" is chosen, this is the paid on date
    #[serde_as(as = "Option<DeserializeDate>")]
    #[serde(default)]
    pub paid_on: Option<NaiveDate>,
    /// Comments against the trade.
    #[serde(default)]
    pub comments: Option<String>,
    /// An `instrument_id` to look up the Instrument.  Required unless you specify a `holding_id` or search by `instrument_code` and `instrument_market_code`.
    #[serde(default)]
    pub instrument_id: Option<String>,
    /// Code to look up the Instrument; must be accompanied by `market`. Required unless you specify a `holding_id`.
    #[serde(default)]
    pub symbol: Option<String>,
    /// Market Code to look up the Instrument; must be accompanied by `symbol`.  Required unless you specify a `holding_id`.
    #[serde(default)]
    pub market: Option<Market>,
    /// Market Country Code to look up the Instrument; must be accompanied by `symbol`.  Required unless you specify a `symbol` with `market`, or a `holding_id`.
    #[serde(default)]
    pub market_country_code: Option<String>,
    /// Supported transaction types: 'BUY','SELL','SPLIT','BONUS','CONSOLD','CANCEL','CAPITAL_RETURN','OPENING_BALANCE','ADJUST_COST_BASE','CAPITAL_CALL'.
    #[serde(default)]
    pub transaction_type: Option<TradeDescription>,
    /// Base64 encoded file to be attached to the trade.
    #[serde(default)]
    pub attachment: Option<String>,
    /// File name for the attachment. This parameter is required if attachment is set.
    #[serde(default)]
    pub attachment_filename: Option<String>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct TradesUpdateSuccess {
    pub trade: TradesUpdateTradeSuccess,
    /// The current API Transaction.
    pub api_transaction: TradesUpdateApiTransactionSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct TradesUpdateTradeSuccess {
    /// The trade ID. Maybe nil if the trade is based on an adjustment and has not yet been confirmed by the user.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub id: String,
    /// A unique identifier associated with this trade
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub unique_identifier: String,
    /// The trade date (format YYYY-MM-DD).
    #[serde_as(as = "DeserializeDate")]
    pub transaction_date: NaiveDate,
    /// Number of shares sold/bought.
    pub quantity: Number,
    /// Price paid/received.
    pub price: Number,
    /// For an opening balance, the cost base of the trade. Always returned in the portfolio currency
    pub cost_base: Number,
    /// The trade's exchange rate as portfolio currency / instrument currency.
    pub exchange_rate: Number,
    /// The trade's brokerage.
    pub brokerage: Number,
    /// The ISO code of the brokerage currency, must be either Portfolio or Instrument currency. If the instrument is a cryptocurrency, any valid brokerage currency is supported.
    pub brokerage_currency_code: Currency,
    /// The value for the trade as displayed in the 'value' column of the UI. For a return of capital, this will be the (signed) capital return value. For a capital call, this will be the (positive) capital return value. For a cost base adjustment, this will be the value of the adjustment. For an opening balance, this will be the market value: the market price x quantity at the opening balance date In each case this is in portfolio currency (rounded to 2 decimal places).
    pub value: Number,
    /// For a CAPITAL_RETURN or other trade with a linked payout, this is the paid on date
    #[serde_as(as = "DeserializeDate")]
    pub paid_on: NaiveDate,
    /// The company event linked to the transaction.
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub company_event_id: Option<i64>,
    /// Any comments for that trade.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub comments: String,
    /// Portfolio ID of the trade.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub portfolio_id: i64,
    /// Holding ID of the trade.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub holding_id: i64,
    /// The state of the trade, can be any of "confirmed", "unconfirmed" or "rejected".
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub state: String,
    /// The trade type ('BUY','SELL','SPLIT','BONUS','CONSOLD','CANCEL','CAPITAL_RETURN','OPENING_BALANCE','ADJUST_COST_BASE','CAPITAL_CALL').
    pub transaction_type: TradeDescription,
    /// Instrument ID of the related Holding.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub instrument_id: i64,
    /// The instrument code/symbol
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub symbol: String,
    /// The market code (eg. `ASX`, `NZX`, etc).
    pub market: Market,
    /// The filename of any attachment
    #[serde(default)]
    pub attachment_filename: Option<String>,
    /// The document id of any attachment, for use with the Show Document API (v2)
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub attachment_id: Option<i64>,
    /// Returns 'true' if trade is confirmed.  DEPRECATED: Use the state field to determine 'confirmed' vs. 'rejected' vs. 'unconfirmed' instead.
    pub confirmed: bool,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct TradesUpdateApiTransactionSuccess {
    /// The unique API Transaction id.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// The API version you called.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub version: i64,
    /// The path executed.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub action: String,
    /// When the transaction was executed.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub timestamp: String,
}

/// Access information on the current user
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
    /// Information on the current user
    pub user: MyUserUserSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct MyUserUserSuccess {
    /// The user id
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// The full name of the user
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub name: String,
    /// The first name of the user
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub first_name: String,
    /// The last name of the user
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub last_name: String,
    /// The email address of the user
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub email: String,
    /// The Sharesight plan code of the user
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub plan_code: String,
    /// Is the user activated (by confirming email address)
    pub is_activated: bool,
    /// Is the user on a free plan
    pub is_free: bool,
    /// Is the user a beta tester
    pub is_beta: bool,
    /// Is the user a guest
    pub is_guest: bool,
    /// Is the user a staff member
    pub is_staff: bool,
    /// Is the user on a professional plan
    pub is_professional: bool,
    /// Has the user cancelled
    pub is_cancelled: bool,
    /// Has the user account expired
    pub is_expired: bool,
    /// The date the user signed up to Sharesight in ISO8601 format (YYYY-MM-DDT00:00:00.000Z)
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub signed_up_at: String,
    /// True if the user signed up for Sharesight via your application
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub signup_via_your_integration: String,
}
