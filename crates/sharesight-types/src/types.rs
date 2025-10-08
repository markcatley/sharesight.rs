use crate::types_prelude::*;

/// Creates a new cash account within a portfolio.
pub struct CashAccountCreate;

impl<'a> ApiEndpoint<'a> for CashAccountCreate {
    const URL_PATH: &'static str = "/portfolios/:portfolio_id/cash_accounts.json";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Post;
    const VERSION: &'static str = "2.0.0";

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
    #[serde_as(as = "DeserializeNumber")]
    pub balance: Number,
    /// The cash account balance converted into the portfolios currency (rounded to 2 decimal places).
    #[serde_as(as = "DeserializeNumber")]
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
    const VERSION: &'static str = "2.0.0";

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
    const VERSION: &'static str = "2.0.0";

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
    #[serde_as(as = "DeserializeNumber")]
    pub balance: Number,
    /// The cash account balance converted into the portfolios currency (rounded to 2 decimal places).
    #[serde_as(as = "DeserializeNumber")]
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
    const VERSION: &'static str = "2.0.0";

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
    #[serde_as(as = "DeserializeNumber")]
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
    #[serde_as(as = "DeserializeNumber")]
    pub amount: Number,
    /// The transaction balance (rounded to 2 decimal places).
    #[serde_as(as = "DeserializeNumber")]
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
    const VERSION: &'static str = "2.0.0";

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
    const VERSION: &'static str = "2.0.0";

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
    #[serde_as(as = "DeserializeNumber")]
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
    #[serde_as(as = "DeserializeNumber")]
    pub amount: Number,
    /// The transaction balance (rounded to 2 decimal places).
    #[serde_as(as = "DeserializeNumber")]
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
    const VERSION: &'static str = "2.0.0";

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
    #[serde_as(as = "DeserializeNumber")]
    pub amount: Number,
    /// The transaction balance (rounded to 2 decimal places).
    #[serde_as(as = "DeserializeNumber")]
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
    const VERSION: &'static str = "2.0.0";

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
    #[serde_as(as = "DeserializeNumber")]
    pub balance: Number,
    /// The cash account balance converted into the portfolios currency (rounded to 2 decimal places).
    #[serde_as(as = "DeserializeNumber")]
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
    const VERSION: &'static str = "2.0.0";

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
    #[serde_as(as = "DeserializeNumber")]
    pub balance: Number,
    /// The cash account balance converted into the portfolios currency (rounded to 2 decimal places).
    #[serde_as(as = "DeserializeNumber")]
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
    const VERSION: &'static str = "2.0.0";

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
    const VERSION: &'static str = "2.0.0";

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
    const VERSION: &'static str = "2.0.0";

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
    #[serde_as(as = "DeserializeNumber")]
    pub quantity: Number,
    /// The instrument symbol for the new holding (buy)
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub symbol: String,
    /// The market code for the new holding
    pub market: Market,
    /// The cancelled price
    #[serde_as(as = "Option<DeserializeNumber>")]
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
    #[serde_as(as = "DeserializeDate")]
    pub transaction_date: NaiveDate,
    /// The market (ASX, NZX, etc).
    pub market: Market,
    /// The instrument code/symbol.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub symbol: String,
    /// Number of shares sold/bought.
    #[serde_as(as = "DeserializeNumber")]
    pub quantity: Number,
    /// Price paid/received.
    #[serde_as(as = "DeserializeNumber")]
    pub price: Number,
    /// The transfer's exchange rate.
    #[serde_as(as = "DeserializeNumber")]
    pub exchange_rate: Number,
    /// The transfer's brokerage.
    #[serde_as(as = "DeserializeNumber")]
    pub brokerage: Number,
    /// The brokerage currency.
    #[serde(default)]
    pub brokerage_currency_code: Option<Currency>,
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
    pub unique_identifier: Option<String>,
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
    const VERSION: &'static str = "2.0.0";

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
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub quantity: Option<Number>,
    /// The instrument symbol for the new holding (buy)
    #[serde(default)]
    pub symbol: Option<String>,
    /// The market code for the new holding
    #[serde(default)]
    pub market: Option<Market>,
    /// The cancelled price
    #[serde_as(as = "Option<DeserializeNumber>")]
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
    #[serde_as(as = "DeserializeDate")]
    pub transaction_date: NaiveDate,
    /// The market (ASX, NZX, etc).
    pub market: Market,
    /// The instrument code/symbol.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub symbol: String,
    /// Number of shares sold/bought.
    #[serde_as(as = "DeserializeNumber")]
    pub quantity: Number,
    /// Price paid/received.
    #[serde_as(as = "DeserializeNumber")]
    pub price: Number,
    /// The transfer's exchange rate.
    #[serde_as(as = "DeserializeNumber")]
    pub exchange_rate: Number,
    /// The transfer's brokerage.
    #[serde_as(as = "DeserializeNumber")]
    pub brokerage: Number,
    /// The brokerage currency.
    #[serde(default)]
    pub brokerage_currency_code: Option<Currency>,
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
    pub unique_identifier: Option<String>,
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
    const VERSION: &'static str = "2.0.0";

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
    pub unique_identifier: Option<String>,
    /// The trade date (format YYYY-MM-DD).
    #[serde_as(as = "DeserializeDate")]
    pub transaction_date: NaiveDate,
    /// Number of shares sold/bought.
    #[serde_as(as = "DeserializeNumber")]
    pub quantity: Number,
    /// Price paid/received.
    #[serde_as(as = "DeserializeNumber")]
    pub price: Number,
    /// For an opening balance, the cost base of the trade. Always returned in the portfolio currency
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub cost_base: Option<Number>,
    /// The trade's exchange rate as portfolio currency / instrument currency.
    #[serde_as(as = "DeserializeNumber")]
    pub exchange_rate: Number,
    /// The trade's brokerage.
    #[serde_as(as = "DeserializeNumber")]
    pub brokerage: Number,
    /// The ISO code of the brokerage currency, must be either Portfolio or Instrument currency. If the instrument is a cryptocurrency, any valid brokerage currency is supported.
    #[serde(default)]
    pub brokerage_currency_code: Option<Currency>,
    /// The value for the trade as displayed in the 'value' column of the UI. For a return of capital, this will be the (signed) capital return value. For a capital call, this will be the (positive) capital return value. For a cost base adjustment, this will be the value of the adjustment. For an opening balance, this will be the market value: the market price x quantity at the opening balance date In each case this is in portfolio currency (rounded to 2 decimal places).
    #[serde_as(as = "DeserializeNumber")]
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
    const VERSION: &'static str = "2.0.0";

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
    pub unique_identifier: Option<String>,
    /// The trade date (format YYYY-MM-DD).
    #[serde_as(as = "DeserializeDate")]
    pub transaction_date: NaiveDate,
    /// Number of shares sold/bought.
    #[serde_as(as = "DeserializeNumber")]
    pub quantity: Number,
    /// Price paid/received.
    #[serde_as(as = "DeserializeNumber")]
    pub price: Number,
    /// For an opening balance, the cost base of the trade. Always returned in the portfolio currency
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub cost_base: Option<Number>,
    /// The trade's exchange rate as portfolio currency / instrument currency.
    #[serde_as(as = "DeserializeNumber")]
    pub exchange_rate: Number,
    /// The trade's brokerage.
    #[serde_as(as = "DeserializeNumber")]
    pub brokerage: Number,
    /// The ISO code of the brokerage currency, must be either Portfolio or Instrument currency. If the instrument is a cryptocurrency, any valid brokerage currency is supported.
    #[serde(default)]
    pub brokerage_currency_code: Option<Currency>,
    /// The value for the trade as displayed in the 'value' column of the UI. For a return of capital, this will be the (signed) capital return value. For a capital call, this will be the (positive) capital return value. For a cost base adjustment, this will be the value of the adjustment. For an opening balance, this will be the market value: the market price x quantity at the opening balance date In each case this is in portfolio currency (rounded to 2 decimal places).
    #[serde_as(as = "DeserializeNumber")]
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
    const VERSION: &'static str = "2.0.0";

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
    const VERSION: &'static str = "2.0.0";

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
    const VERSION: &'static str = "2.0.0";

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
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub pe_ratio: Option<Number>,
    /// The net tangible assets for this instrument displayed on it's currency.
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub nta: Option<Number>,
    /// The earnings per share for this instrument displayed on it's currency.
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub eps: Option<Number>,
    /// The current price for this instrument displayed on it's currency.
    #[serde_as(as = "Option<DeserializeNumber>")]
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
    const VERSION: &'static str = "2.0.0";

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
    const VERSION: &'static str = "2.0.0";

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
    const VERSION: &'static str = "2.0.0";

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
    const VERSION: &'static str = "2.0.0";

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
    const VERSION: &'static str = "2.0.0";

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
    const VERSION: &'static str = "2.0.0";

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
    #[serde_as(as = "DeserializeNumber")]
    pub amount: Number,
    /// The calculated gross amount
    #[serde_as(as = "DeserializeNumber")]
    pub gross_amount: Number,
    /// The payout type: DIV (Dividend), REP (Capital replayment), INT (Interest), or DIS (Distribution)
    pub transaction_description: PayoutDescription,
    /// The payout resident withholding tax amount. Always returned in the portfolio currency.
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub resident_withholding_tax: Option<Number>,
    /// The payout non-resident withholding tax amount.
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub non_resident_withholding_tax: Option<Number>,
    /// The payout tax credit amount. Always returned in the portfolio currency.
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub tax_credit: Option<Number>,
    /// Currency code of the payout, using 3-letter ISO 4217 code.
    pub currency: Currency,
    /// The payout's exchange rate.
    #[serde_as(as = "DeserializeNumber")]
    pub exchange_rate: Number,
    /// If `true`, payout is non taxable.
    pub non_taxable: bool,
    /// Any comments for that payout.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub comments: String,
    /// Other net foreign source income.
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub other_net_fsi: Option<Number>,
    /// The amount of an LIC dividend that is attributable to an LIC capital gain.
    #[serde_as(as = "DeserializeNumber")]
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
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub franked_amount: Option<Number>,
    /// Unfranked amount in the payout (Australia only)
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub unfranked_amount: Option<Number>,
    /// `true` if this payout is for a trust. (Australia only)
    #[serde(default)]
    pub trust: Option<bool>,
    /// Extra interest amount in this payout. (Australia only)
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub extra_interest_payment_amount: Option<Number>,
    /// Capital gain amount in this payout. (Australia only)
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub capital_gains: Option<Number>,
    /// Discounted capital gain amount in this payout. (Australia only)
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub discounted_capital_gains: Option<Number>,
    /// Interest payment amount in this payout. (Australia only)
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub interest_payment: Option<Number>,
    /// Amount of foreign income in this payout. (Australia only)
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub foreign_source_income: Option<Number>,
    /// Value of deferred income in this payout. (Australia only)
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub deferred_income: Option<Number>,
    /// Any non-tax assessable amount.
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub non_assessable: Option<Number>,
    /// Value of CGT concession in this payout. (Australia only)
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub cgt_concession_amount: Option<Number>,
    /// Relevant for attribution managed investment trusts (AMIT) when the taxable income attributed to you is less than the cash distribution you received. This amount is non-assessable and is used to decrease your cost base for cgt purposes (Australia only)
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub amit_decrease_amount: Option<Number>,
    /// Relevant for attribution managed investment trusts (AMIT) when the taxable income attributed to you is more than the cash distribution you received. This amount is non-assessable and is used to increase your cost base for cgt purposes (Australia only)
    #[serde_as(as = "Option<DeserializeNumber>")]
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
    #[serde_as(as = "DeserializeNumber")]
    pub quantity: Number,
    /// Price per reinvested unit.
    #[serde_as(as = "DeserializeNumber")]
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
    const VERSION: &'static str = "2.0.0";

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
    #[serde_as(as = "DeserializeNumber")]
    pub amount: Number,
    /// The calculated gross amount
    #[serde_as(as = "DeserializeNumber")]
    pub gross_amount: Number,
    /// The payout resident withholding tax amount. Always returned in the portfolio currency.
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub resident_withholding_tax: Option<Number>,
    /// The payout non-resident withholding tax amount.
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub non_resident_withholding_tax: Option<Number>,
    /// The payout tax credit amount. Always returned in the portfolio currency.
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub tax_credit: Option<Number>,
    /// Currency code of the payout, using 3-letter ISO 4217 code.
    pub currency: Currency,
    /// The payout's exchange rate.
    #[serde_as(as = "DeserializeNumber")]
    pub exchange_rate: Number,
    /// If true, payout is non taxable.
    pub non_taxable: bool,
    /// Any comments for that payout.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub comments: String,
    /// Other net foreign source income.
    #[serde_as(as = "Option<DeserializeNumber>")]
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
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub franked_amount: Option<Number>,
    /// Unfranked amount in the payout (Australia only)
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub unfranked_amount: Option<Number>,
    /// `true` if this payout is for a trust. (Australia only)
    #[serde(default)]
    pub trust: Option<bool>,
    /// Extra interest amount in this payout. (Australia only)
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub extra_interest_payment_amount: Option<Number>,
    /// Capital gain amount in this payout. (Australia only)
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub capital_gains: Option<Number>,
    /// Discounted capital gain amount in this payout. (Australia only)
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub discounted_capital_gains: Option<Number>,
    /// Interest payment amount in this payout. (Australia only)
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub interest_payment: Option<Number>,
    /// Amount of foreign income in this payout. (Australia only)
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub foreign_source_income: Option<Number>,
    /// Value of deferred income in this payout. (Australia only)
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub deferred_income: Option<Number>,
    /// Any non-tax assessable amount.
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub non_assessable: Option<Number>,
    /// Relevant for attribution managed investment trusts (AMIT) when the taxable income attributed to you is less than the cash distribution you received. This amount is non-assessable and is used to decrease your cost base for cgt purposes (Australia only)
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub amit_decrease_amount: Option<Number>,
    /// Relevant for attribution managed investment trusts (AMIT) when the taxable income attributed to you is more than the cash distribution you received. This amount is non-assessable and is used to increase your cost base for cgt purposes (Australia only)
    #[serde_as(as = "Option<DeserializeNumber>")]
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
    #[serde_as(as = "DeserializeNumber")]
    pub quantity: Number,
    /// Price per reinvested unit.
    #[serde_as(as = "DeserializeNumber")]
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
    const VERSION: &'static str = "2.0.0";

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
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub company_event_id: Option<i64>,
    /// Date used to identify the payout based on a payment date of an interest payment (format `YYYY-MM-DD`).
    #[serde_as(as = "Option<DeserializeDate>")]
    #[serde(default)]
    pub paid_on: Option<NaiveDate>,
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
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub quantity: Option<Number>,
    /// Price per reinvested unit.
    #[serde_as(as = "Option<DeserializeNumber>")]
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
    #[serde_as(as = "DeserializeNumber")]
    pub amount: Number,
    /// The calculated gross amount
    #[serde_as(as = "DeserializeNumber")]
    pub gross_amount: Number,
    /// The payout resident withholding tax amount. Always returned in the portfolio currency.
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub resident_withholding_tax: Option<Number>,
    /// The payout non-resident withholding tax amount.
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub non_resident_withholding_tax: Option<Number>,
    /// The payout tax credit amount. Always returned in the portfolio currency.
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub tax_credit: Option<Number>,
    /// Payout currency code, using 3-letter ISO 4217 code.
    pub currency: Currency,
    /// The payout's exchange rate.
    #[serde_as(as = "DeserializeNumber")]
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
    const VERSION: &'static str = "2.0.0";

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
    #[serde_as(as = "DeserializeNumber")]
    pub amount: Number,
    /// Code for the payout currency_code, using 3-letter ISO 4217 code.
    pub currency_code: Currency,
    /// The ex date for the payout (format `YYYY-MM-DD`).
    #[serde_as(as = "Option<DeserializeDate>")]
    #[serde(default)]
    pub goes_ex_on: Option<NaiveDate>,
    /// Resident withholding tax for the payout.
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub resident_withholding_tax: Option<Number>,
    /// Non-resident withholding tax for the payout
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub non_resident_withholding_tax: Option<Number>,
    /// Tax credit for the payout.
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub tax_credit: Option<Number>,
    /// Exchange rate for other currency payout.
    #[serde_as(as = "Option<DeserializeNumber>")]
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
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub banked_amount: Option<Number>,
    /// Parameters when the payout is reinvested.
    #[serde(default)]
    pub drp_trade_attributes: Option<PayoutCreatePayoutDrpTradeAttributesParameters>,
    /// Franked amount in the payout. (Australia only)
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub franked_amount: Option<Number>,
    /// Unfranked amount in the payout (Australia only)
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub unfranked_amount: Option<Number>,
    /// `true` if this payout is for a trust. (Australia only)
    #[serde(default)]
    pub trust: Option<bool>,
    /// Extra interest amount in this payout. (Australia only)
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub extra_interest_payment_amount: Option<Number>,
    /// Capital gain amount in this payout. (Australia only)
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub capital_gains: Option<Number>,
    /// Discounted capital gain amount in this payout. (Australia only)
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub discounted_capital_gains: Option<Number>,
    /// Amount of foreign income in this payout. (Australia only)
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub foreign_source_income: Option<Number>,
    /// The amount of an LIC dividend that is attributable to an LIC capital gain. (Australia only)
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub lic_capital_gain: Option<Number>,
    /// Any non-tax assessable amount.
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub non_assessable: Option<Number>,
    /// Value of deferred income in this payout. (Australia only)
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub deferred_income: Option<Number>,
    /// Value of CGT concession in this payout. (Australia only)
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub cgt_concession_amount: Option<Number>,
    /// Relevant for attribution managed investment trusts (AMIT) when the taxable income attributed to you is less than the cash distribution you received. This amount is non-assessable and is used to decrease your cost base for cgt purposes (Australia only)
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub amit_decrease_amount: Option<Number>,
    /// Relevant for attribution managed investment trusts (AMIT) when the taxable income attributed to you is more than the cash distribution you received. This amount is non-assessable and is used to increase your cost base for cgt purposes (Australia only)
    #[serde_as(as = "Option<DeserializeNumber>")]
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
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub quantity: Option<Number>,
    /// Price per reinvested unit.
    #[serde_as(as = "Option<DeserializeNumber>")]
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
    #[serde_as(as = "DeserializeNumber")]
    pub amount: Number,
    /// The calculated gross amount
    #[serde_as(as = "DeserializeNumber")]
    pub gross_amount: Number,
    /// The payout resident withholding tax amount. Always returned in the portfolio currency.
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub resident_withholding_tax: Option<Number>,
    /// The payout non-resident withholding tax amount.
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub non_resident_withholding_tax: Option<Number>,
    /// The payout tax credit amount. Always returned in the portfolio currency.
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub tax_credit: Option<Number>,
    /// Currency code of the payout, using 3-letter ISO 4217 code.
    pub currency: Currency,
    /// The payout's exchange rate.
    #[serde_as(as = "DeserializeNumber")]
    pub exchange_rate: Number,
    /// If true, payout is non taxable.
    pub non_taxable: bool,
    /// Any comments for that payout.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub comments: String,
    /// Other net foreign source income.
    #[serde_as(as = "Option<DeserializeNumber>")]
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
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub franked_amount: Option<Number>,
    /// Unfranked amount in the payout (Australia only)
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub unfranked_amount: Option<Number>,
    /// `true` if this payout is for a trust. (Australia only)
    #[serde(default)]
    pub trust: Option<bool>,
    /// Extra interest amount in this payout. (Australia only)
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub extra_interest_payment_amount: Option<Number>,
    /// Capital gain amount in this payout. (Australia only)
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub capital_gains: Option<Number>,
    /// Discounted capital gain amount in this payout. (Australia only)
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub discounted_capital_gains: Option<Number>,
    /// Interest payment amount in this payout. (Australia only)
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub interest_payment: Option<Number>,
    /// Amount of foreign income in this payout. (Australia only)
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub foreign_source_income: Option<Number>,
    /// Value of deferred income in this payout. (Australia only)
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub deferred_income: Option<Number>,
    /// True if this payout is not assessed for tax. (Australia only)
    #[serde(default)]
    pub non_assessable: Option<bool>,
    /// Relevant for attribution managed investment trusts (AMIT) when the taxable income attributed to you is less than the cash distribution you received. This amount is non-assessable and is used to decrease your cost base for cgt purposes (Australia only)
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub amit_decrease_amount: Option<Number>,
    /// Relevant for attribution managed investment trusts (AMIT) when the taxable income attributed to you is more than the cash distribution you received. This amount is non-assessable and is used to increase your cost base for cgt purposes (Australia only)
    #[serde_as(as = "Option<DeserializeNumber>")]
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
    #[serde_as(as = "DeserializeNumber")]
    pub quantity: Number,
    /// Price per reinvested unit.
    #[serde_as(as = "DeserializeNumber")]
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
    const VERSION: &'static str = "2.0.0";

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
    const VERSION: &'static str = "2.0.0";

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
    #[serde_as(as = "DeserializeNumber")]
    pub amount: Number,
    /// The calculated gross amount
    #[serde_as(as = "DeserializeNumber")]
    pub gross_amount: Number,
    /// The payout resident withholding tax amount. Always returned in the portfolio currency.
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub resident_withholding_tax: Option<Number>,
    /// The payout non-resident withholding tax amount.
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub non_resident_withholding_tax: Option<Number>,
    /// The payout tax credit amount. Always returned in the portfolio currency.
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub tax_credit: Option<Number>,
    /// Payout currency code, using 3-letter ISO 4217 code.
    pub currency: Currency,
    /// The payout's exchange rate.
    #[serde_as(as = "DeserializeNumber")]
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
    const VERSION: &'static str = "2.0.0";

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
    #[serde_as(as = "DeserializeNumber")]
    pub amount: Number,
    /// The calculated gross amount
    #[serde_as(as = "DeserializeNumber")]
    pub gross_amount: Number,
    /// The payout resident withholding tax amount. Always returned in the portfolio currency.
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub resident_withholding_tax: Option<Number>,
    /// The payout non-resident withholding tax amount.
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub non_resident_withholding_tax: Option<Number>,
    /// The payout tax credit amount. Always returned in the portfolio currency.
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub tax_credit: Option<Number>,
    /// Payout currency code, using 3-letter ISO 4217 code.
    pub currency: Currency,
    /// The payout's exchange rate.
    #[serde_as(as = "DeserializeNumber")]
    pub exchange_rate: Number,
    /// If `true`, payout is non taxable.
    pub non_taxable: bool,
    /// Any comments for that payout.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub comments: String,
    /// Other net foreign source income.
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub other_net_fsi: Option<Number>,
    /// The amount of an LIC dividend that is attributable to an LIC capital gain.
    #[serde_as(as = "DeserializeNumber")]
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
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub franked_amount: Option<Number>,
    /// Unfranked amount in the payout (Australia only)
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub unfranked_amount: Option<Number>,
    /// `true` if this payout is for a trust. (Australia only)
    #[serde(default)]
    pub trust: Option<bool>,
    /// Extra interest amount in this payout. (Australia only)
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub extra_interest_payment_amount: Option<Number>,
    /// Capital gain amount in this payout. (Australia only)
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub capital_gains: Option<Number>,
    /// Discounted capital gain amount in this payout. (Australia only)
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub discounted_capital_gains: Option<Number>,
    /// Amount of foreign income in this payout. (Australia only)
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub foreign_source_income: Option<Number>,
    /// Any non-tax assessable amount.
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub non_assessable: Option<Number>,
    /// Value of deferred income in this payout. (Australia only)
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub deferred_income: Option<Number>,
    /// Value of CGT concession in this payout. (Australia only)
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub cgt_concession_amount: Option<Number>,
    /// Relevant for attribution managed investment trusts (AMIT) when the taxable income attributed to you is less than the cash distribution you received. This amount is non-assessable and is used to decrease your cost base for cgt purposes (Australia only)
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub amit_decrease_amount: Option<Number>,
    /// Relevant for attribution managed investment trusts (AMIT) when the taxable income attributed to you is more than the cash distribution you received. This amount is non-assessable and is used to increase your cost base for cgt purposes (Australia only)
    #[serde_as(as = "Option<DeserializeNumber>")]
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
    #[serde_as(as = "DeserializeNumber")]
    pub quantity: Number,
    /// Price per reinvested unit.
    #[serde_as(as = "DeserializeNumber")]
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
    const VERSION: &'static str = "2.0.0";

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
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub resident_withholding_tax: Option<Number>,
    /// Non-resident withholding tax for the payout
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub non_resident_withholding_tax: Option<Number>,
    /// Tax credit for the payout.
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub tax_credit: Option<Number>,
    /// Exchange rate for other currency payout.
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub exchange_rate: Option<Number>,
    /// Payout amount. (All except Australia)
    #[serde_as(as = "Option<DeserializeNumber>")]
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
    #[serde_as(as = "Option<DeserializeNumber>")]
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
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub franked_amount: Option<Number>,
    /// Unfranked amount in the payout (Australia only)
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub unfranked_amount: Option<Number>,
    /// `true` if this payout is for a trust. (Australia only)
    #[serde(default)]
    pub trust: Option<bool>,
    /// Extra interest amount in this payout. (Australia only)
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub extra_interest_payment_amount: Option<Number>,
    /// Capital gain amount in this payout. (Australia only)
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub capital_gains: Option<Number>,
    /// Discounted capital gain amount in this payout. (Australia only)
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub discounted_capital_gains: Option<Number>,
    /// Amount of foreign income in this payout. (Australia only)
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub foreign_source_income: Option<Number>,
    /// The amount of an LIC dividend that is attributable to an LIC capital gain. (Australia only)
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub lic_capital_gain: Option<Number>,
    /// Any non-tax assessable amount.
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub non_assessable: Option<Number>,
    /// Value of deferred income in this payout. (Australia only)
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub deferred_income: Option<Number>,
    /// Value of CGT concession in this payout. (Australia only)
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub cgt_concession_amount: Option<Number>,
    /// Relevant for attribution managed investment trusts (AMIT) when the taxable income attributed to you is less than the cash distribution you received. This amount is non-assessable and is used to decrease your cost base for cgt purposes (Australia only)
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub amit_decrease_amount: Option<Number>,
    /// Relevant for attribution managed investment trusts (AMIT) when the taxable income attributed to you is more than the cash distribution you received. This amount is non-assessable and is used to increase your cost base for cgt purposes (Australia only)
    #[serde_as(as = "Option<DeserializeNumber>")]
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
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub quantity: Option<Number>,
    /// Price per reinvested unit.
    #[serde_as(as = "Option<DeserializeNumber>")]
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
    #[serde_as(as = "DeserializeNumber")]
    pub amount: Number,
    /// The calculated gross amount
    #[serde_as(as = "DeserializeNumber")]
    pub gross_amount: Number,
    /// The payout resident withholding tax amount. Always returned in the portfolio currency.
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub resident_withholding_tax: Option<Number>,
    /// The payout non-resident withholding tax amount.
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub non_resident_withholding_tax: Option<Number>,
    /// The payout tax credit amount. Always returned in the portfolio currency.
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub tax_credit: Option<Number>,
    /// Payout currency code, using 3-letter ISO 4217 code.
    pub currency: Currency,
    /// The payout's exchange rate.
    #[serde_as(as = "DeserializeNumber")]
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
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub franked_amount: Option<Number>,
    /// Unfranked amount in the payout (Australia only)
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub unfranked_amount: Option<Number>,
    /// `true` if this payout is for a trust. (Australia only)
    #[serde(default)]
    pub trust: Option<bool>,
    /// Extra interest amount in this payout. (Australia only)
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub extra_interest_payment_amount: Option<Number>,
    /// Capital gain amount in this payout. (Australia only)
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub capital_gains: Option<Number>,
    /// Discounted capital gain amount in this payout. (Australia only)
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub discounted_capital_gains: Option<Number>,
    /// Amount of foreign income in this payout. (Australia only)
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub foreign_source_income: Option<Number>,
    /// Any non-tax assessable amount.
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub non_assessable: Option<Number>,
    /// Value of deferred income in this payout. (Australia only)
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub deferred_income: Option<Number>,
    /// Value of CGT concession in this payout. (Australia only)
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub cgt_concession_amount: Option<Number>,
    /// Relevant for attribution managed investment trusts (AMIT) when the taxable income attributed to you is less than the cash distribution you received. This amount is non-assessable and is used to decrease your cost base for cgt purposes (Australia only)
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub amit_decrease_amount: Option<Number>,
    /// Relevant for attribution managed investment trusts (AMIT) when the taxable income attributed to you is more than the cash distribution you received. This amount is non-assessable and is used to increase your cost base for cgt purposes (Australia only)
    #[serde_as(as = "Option<DeserializeNumber>")]
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
    #[serde_as(as = "DeserializeNumber")]
    pub quantity: Number,
    /// Price per reinvested unit.
    #[serde_as(as = "DeserializeNumber")]
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
    const VERSION: &'static str = "2.0.0";

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
    #[serde_as(as = "DeserializeNumber")]
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
    #[serde_as(as = "DeserializeNumber")]
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
    #[serde_as(as = "DeserializeDate")]
    pub inception_date: NaiveDate,
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
    const VERSION: &'static str = "2.0.0";

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
    /// The portfolio was successfully deleted.
    pub status: (),
}

/// Retrieves a list of a user's portfolios
pub struct PortfolioList;

impl<'a> ApiEndpoint<'a> for PortfolioList {
    const URL_PATH: &'static str = "/portfolios";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Get;
    const VERSION: &'static str = "3.0.0";

    type UrlDisplay = &'static str;
    type Parameters = PortfolioListParameters;
    type Success = PortfolioListSuccess;

    fn url_path(_parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        "/portfolios"
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct PortfolioListParameters {
    /// Set to true to see consolidated portfolio views<br>Default value: `false`
    #[serde(default)]
    pub consolidated: Option<bool>,
    /// The instrument ID, where populated consolidated will always default to false.
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub instrument_id: Option<i64>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PortfolioListSuccess {
    /// List of Portfolios associated with this model
    pub portfolios: Vec<PortfolioListPortfoliosSuccess>,
    /// The current API Transaction.
    #[serde(default)]
    pub api_transaction: Option<PortfolioListApiTransactionSuccess>,
    pub links: PortfolioListLinksSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PortfolioListPortfoliosSuccess {
    /// The unique id identifying the portfolio
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// Whether or not this is a consolidated view portfolio
    #[serde(default)]
    pub consolidated: Option<bool>,
    /// The name of the portfolio
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub name: String,
    /// Typically used by professionals to identify the tax entity owner of the portfolio
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub external_identifier: String,
    /// The id of the holding in this portfolio
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub holding_id: Option<i64>,
    /// The timezone name applicable to the portfolio country
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub tz_name: String,
    /// For an Australian portfolio, the default sale allocation method for capital gains purposes. One of: fifo, lifo, maximise_cr, minimise_cr, ss_minimise, average, default
    pub default_sale_allocation_method: SaleAllocationMethod,
    /// The CGT discount rate for Australian portfolios
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub cg_discount: String,
    /// The end of the financial year (MM-DD)
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub financial_year_end: String,
    /// The interest method: 'simple' or 'compound'
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub interest_method: String,
    /// The ISO country code of the (tax) country of this portfolio
    pub country_code: Country,
    /// The ISO currency code of this portfolio
    pub currency_code: Currency,
    /// The date your portfolio was started on or the oldest portfolio if a consolidated view
    #[serde_as(as = "DeserializeDate")]
    pub inception_date: NaiveDate,
    /// The current user's access level to this portfolio, one of: OWNER, STAFF, ADMIN, EDIT, READ
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub access_level: String,
    /// The unique identifier of the portfolio owner
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub user_id: Option<i64>,
    /// The name of the portfolio owner. A first and last name will be returned if available, otherwise the owners organisation name
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub owner_name: String,
    /// For NZ portfolios, the rate of resident witholding tax to be applied
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub rwtr_rate: Option<Number>,
    /// For NZ portfolios, true if the owner is taxed as a trader
    #[serde(default)]
    pub trader: Option<bool>,
    /// If set, transactions such as company events are not automatically applied to the portfolio holdings
    #[serde(default)]
    pub disable_automatic_transactions: Option<bool>,
    /// For Canadian portfolios, the type of tax entity: non_registered, rrsp, rrif, tfsa
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub tax_entity_type: String,
    /// All buys and sells will generate a corresponding deposit/withdrawal in the selected 'trading' account
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub trade_sync_cash_account_id: Option<i64>,
    /// All payouts will generate a corresponding deposit in the selected 'payout' account
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub payout_sync_cash_account_id: Option<i64>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PortfolioListApiTransactionSuccess {
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

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PortfolioListLinksSuccess {
    /// URL to a list of requested resources.
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
    const VERSION: &'static str = "2.0.0";

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
    #[serde_as(as = "DeserializeNumber")]
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
    #[serde_as(as = "DeserializeDate")]
    pub inception_date: NaiveDate,
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
    const VERSION: &'static str = "2.0.0";

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
    pub portfolio: PortfolioUpdatePortfolioParameters,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
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
    #[serde_as(as = "DeserializeNumber")]
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
    #[serde_as(as = "DeserializeNumber")]
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
    #[serde_as(as = "DeserializeDate")]
    pub inception_date: NaiveDate,
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
    const VERSION: &'static str = "2.0.0";

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
    #[serde_as(as = "DeserializeNumber")]
    pub short_term_gains: Number,
    /// The total of long term gains (over one year, rounded to 2 decimal places).
    #[serde_as(as = "DeserializeNumber")]
    pub long_term_gains: Number,
    /// The total of losses (rounded to 2 decimal places).
    #[serde_as(as = "DeserializeNumber")]
    pub losses: Number,
    /// The total of short term capital losses available to be offset (rounded to 2 decimal places).
    #[serde_as(as = "DeserializeNumber")]
    pub short_term_losses: Number,
    /// The total of capital losses available to be offset after deducting short term losses (rounded to 2 decimal places).
    #[serde_as(as = "DeserializeNumber")]
    pub long_term_losses: Number,
    /// The total of discounted capital gain distributions (grossed up, rounded to 2 decimal places).
    #[serde_as(as = "DeserializeNumber")]
    pub total_discounted_capital_gain_distributions: Number,
    /// The total of non discounted capital gain distributions (rounded to 2 decimal places).
    #[serde_as(as = "DeserializeNumber")]
    pub total_non_discounted_capital_gain_distributions: Number,
    /// The rate of CGT concession on long term gains applied (rounded to 2 decimal places).
    #[serde_as(as = "DeserializeNumber")]
    pub cgt_concession_rate: Number,
    /// The amount of the CGT concession applied (rounded to 2 decimal places).
    #[serde_as(as = "DeserializeNumber")]
    pub cgt_concession_amount: Number,
    /// The market value of the portfolio (rounded to 2 decimal places).
    #[serde_as(as = "DeserializeNumber")]
    pub market_value: Number,
    /// The gain (or loss, if negative, rounded to 2 decimal places).
    #[serde_as(as = "DeserializeNumber")]
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
    #[serde_as(as = "DeserializeNumber")]
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
    #[serde_as(as = "DeserializeNumber")]
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
    #[serde_as(as = "DeserializeNumber")]
    pub quantity: Number,
    /// The adjusted total cost of the parcel (rounded to 2 decimal places).
    #[serde_as(as = "DeserializeNumber")]
    pub cost_base: Number,
    /// The market value of the parcel at the balance date (rounded to 2 decimal places).
    #[serde_as(as = "DeserializeNumber")]
    pub market_value: Number,
    /// The capital gain (negative for losses, rounded to 2 decimal places).
    #[serde_as(as = "DeserializeNumber")]
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
    #[serde_as(as = "DeserializeNumber")]
    pub quantity: Number,
    /// The adjusted total cost of the parcel (rounded to 2 decimal places).
    #[serde_as(as = "DeserializeNumber")]
    pub cost_base: Number,
    /// The market value of the parcel at the balance date (rounded to 2 decimal places).
    #[serde_as(as = "DeserializeNumber")]
    pub market_value: Number,
    /// The capital gain (negative for losses, rounded to 2 decimal places).
    #[serde_as(as = "DeserializeNumber")]
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
    #[serde_as(as = "DeserializeNumber")]
    pub quantity: Number,
    /// The adjusted total cost of the parcel (rounded to 2 decimal places).
    #[serde_as(as = "DeserializeNumber")]
    pub cost_base: Number,
    /// The market value of the parcel at the balance date (rounded to 2 decimal places).
    #[serde_as(as = "DeserializeNumber")]
    pub market_value: Number,
    /// The capital gain (negative for losses, rounded to 2 decimal places).
    #[serde_as(as = "DeserializeNumber")]
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
    const VERSION: &'static str = "2.0.0";

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
    #[serde_as(as = "DeserializeNumber")]
    pub percentage: Number,
    /// The total value of the portfolio (rounded to 2 decimal places).
    #[serde_as(as = "DeserializeNumber")]
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
    #[serde_as(as = "DeserializeNumber")]
    pub percentage: Number,
    /// The value of the portfolio components in this group (rounded to 2 decimal places).
    #[serde_as(as = "DeserializeNumber")]
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
    #[serde_as(as = "DeserializeNumber")]
    pub percentage: Number,
    /// The value of the holding or cash account (rounded to 2 decimal places).
    #[serde_as(as = "DeserializeNumber")]
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
    const VERSION: &'static str = "2.0.0";

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
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub custom_group_id: i64,
    /// The total value of the portfolio
    #[serde_as(as = "DeserializeNumber")]
    pub value: Number,
    /// Capital Gain<sup>1</sup> (rounded to 2 decimal places).
    #[serde_as(as = "DeserializeNumber")]
    pub capital_gain: Number,
    /// Capital Gain (percentage, rounded to 2 decimal places, 33% as `33.0`)<sup>1</sup>
    #[serde_as(as = "DeserializeNumber")]
    pub capital_gain_percent: Number,
    /// Payout Gain<sup>1</sup> (rounded to 2 decimal places).
    #[serde_as(as = "DeserializeNumber")]
    pub payout_gain: Number,
    /// Payout Gain (percentage, rounded to 2 decimal places, 33% as `33.0`)<sup>1</sup>
    #[serde_as(as = "DeserializeNumber")]
    pub payout_gain_percent: Number,
    /// Currency Gain<sup>1</sup> (rounded to 2 decimal places).
    #[serde_as(as = "DeserializeNumber")]
    pub currency_gain: Number,
    /// Currency Gain (percentage, rounded to 2 decimal places, 33% as `33.0`)<sup>1</sup>
    #[serde_as(as = "DeserializeNumber")]
    pub currency_gain_percent: Number,
    /// Total Gain<sup>1</sup> (rounded to 2 decimal places).
    #[serde_as(as = "DeserializeNumber")]
    pub total_gain: Number,
    /// Total Gain (percentage, rounded to 2 decimal places, 33% as ``33.0``)<sup>1</sup>
    #[serde_as(as = "DeserializeNumber")]
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
    /// The name of the selected grouping for the report
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub grouping: String,
    /// The name of the instrument
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub name: String,
    /// The current value of the holding.
    #[serde_as(as = "DeserializeNumber")]
    pub value: Number,
    /// The quantity of shares or other instruments in the holding
    #[serde_as(as = "DeserializeNumber")]
    pub quantity: Number,
    /// Capital Gain<sup>1</sup> on the holding (rounded to 2 decimal places).
    #[serde_as(as = "DeserializeNumber")]
    pub capital_gain: Number,
    /// Capital Gain (percentage, rounded to 2 decimal places, 33% as `33.0`)<sup>1</sup> on the holding
    #[serde_as(as = "DeserializeNumber")]
    pub capital_gain_percent: Number,
    /// Payout Gain<sup>1</sup> on the holding (rounded to 2 decimal places).
    #[serde_as(as = "DeserializeNumber")]
    pub payout_gain: Number,
    /// Payout Gain (percentage, rounded to 2 decimal places, 33% as `33.0`)<sup>1</sup> on the holding
    #[serde_as(as = "DeserializeNumber")]
    pub payout_gain_percent: Number,
    /// Currency Gain<sup>1</sup> on the holding (rounded to 2 decimal places).
    #[serde_as(as = "DeserializeNumber")]
    pub currency_gain: Number,
    /// Currency Gain (percentage, rounded to 2 decimal places, 33% as `33.0`)<sup>1</sup> on the holding
    #[serde_as(as = "DeserializeNumber")]
    pub currency_gain_percent: Number,
    /// Total Gain<sup>1</sup> on the holding (rounded to 2 decimal places).
    #[serde_as(as = "DeserializeNumber")]
    pub total_gain: Number,
    /// Total Gain (percentage, rounded to 2 decimal places, 33% as `33.0`)<sup>1</sup> on the holding
    #[serde_as(as = "DeserializeNumber")]
    pub total_gain_percent: Number,
    /// The group value this instrument has been placed in - note that the field name will be the group type
    #[serde(flatten)]
    pub grouping_information: HashMap<String, String>,
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
    #[serde_as(as = "DeserializeNumber")]
    pub value: Number,
    /// The currency symbol (e.g. AU$) of the cash account
    pub currency: Currency,
    /// The ISO currency code (e.g. AUD) of the portfolio
    pub currency_code: Currency,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PerformanceSubTotalsSuccess {
    /// The total value of the holdings in this group
    #[serde_as(as = "DeserializeNumber")]
    pub value: Number,
    /// Capital Gain<sup>1</sup> on the holdings in this group (rounded to 2 decimal places).
    #[serde_as(as = "DeserializeNumber")]
    pub capital_gain: Number,
    /// Capital Gain (percentage, rounded to 2 decimal places, 33% as `33.0`)<sup>1</sup>
    #[serde_as(as = "DeserializeNumber")]
    pub capital_gain_percent: Number,
    /// Payout Gain<sup>1</sup> on the holdings in this group (rounded to 2 decimal places).
    #[serde_as(as = "DeserializeNumber")]
    pub payout_gain: Number,
    /// Payout Gain (percentage, rounded to 2 decimal places, 33% as `33.0`)<sup>1</sup> on the holdings in this group
    #[serde_as(as = "DeserializeNumber")]
    pub payout_gain_percent: Number,
    /// Currency Gain<sup>1</sup> on the holdings in this group (rounded to 2 decimal places).
    #[serde_as(as = "DeserializeNumber")]
    pub currency_gain: Number,
    /// Currency Gain (percentage, rounded to 2 decimal places, 33% as `33.0`)<sup>1</sup> on the holdings in this group
    #[serde_as(as = "DeserializeNumber")]
    pub currency_gain_percent: Number,
    /// Total Gain<sup>1</sup> on the holdings in this group (rounded to 2 decimal places).
    #[serde_as(as = "DeserializeNumber")]
    pub total_gain: Number,
    /// Total Gain (percentage, rounded to 2 decimal places, 33% as `33.0`)<sup>1</sup> on the holdings in this group
    #[serde_as(as = "DeserializeNumber")]
    pub total_gain_percent: Number,
    /// The group value - note that the field name will be the group type
    #[serde(flatten)]
    pub grouping_information: HashMap<String, String>,
}

/// Return a report on unrealised capital gains tax (for Australian portfolios only)
pub struct UnrealisedCgt;

impl<'a> ApiEndpoint<'a> for UnrealisedCgt {
    const URL_PATH: &'static str = "/portfolios/:portfolio_id/unrealised_cgt.json";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Get;
    const VERSION: &'static str = "2.0.0";

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
    #[serde_as(as = "DeserializeNumber")]
    pub unrealised_short_term_gains: Number,
    /// The total of unrealized long term gains (over one year, rounded to 2 decimal places).
    #[serde_as(as = "DeserializeNumber")]
    pub unrealised_long_term_gains: Number,
    /// The total of unrealised lossses (rounded to 2 decimal places).
    #[serde_as(as = "DeserializeNumber")]
    pub unrealised_losses: Number,
    /// The rate of CGT concession on long term gains applied (rounded to 2 decimal places).
    #[serde_as(as = "DeserializeNumber")]
    pub cgt_concession_rate: Number,
    /// The amount of the CGT concession applied (rounded to 2 decimal places).
    #[serde_as(as = "DeserializeNumber")]
    pub unrealised_cgt_concession_amount: Number,
    /// The market value of the portfolio (rounded to 2 decimal places).
    #[serde_as(as = "DeserializeNumber")]
    pub market_value: Number,
    /// The unrealised gain (or loss, if negative, rounded to 2 decimal places).
    #[serde_as(as = "DeserializeNumber")]
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
    #[serde_as(as = "DeserializeNumber")]
    pub quantity: Number,
    /// The adjusted total cost of the parcel (rounded to 2 decimal places).
    #[serde_as(as = "DeserializeNumber")]
    pub cost_base: Number,
    /// The market value of the parcel at the balance date (rounded to 2 decimal places).
    #[serde_as(as = "DeserializeNumber")]
    pub market_value: Number,
    /// The unrealised capital gain (negative for losses, rounded to 2 decimal places).
    #[serde_as(as = "DeserializeNumber")]
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
    #[serde_as(as = "DeserializeNumber")]
    pub quantity: Number,
    /// The adjusted total cost of the parcel (rounded to 2 decimal places).
    #[serde_as(as = "DeserializeNumber")]
    pub cost_base: Number,
    /// The market value of the parcel at the balance date (rounded to 2 decimal places).
    #[serde_as(as = "DeserializeNumber")]
    pub market_value: Number,
    /// The unrealised capital gain (negative for losses, rounded to 2 decimal places).
    #[serde_as(as = "DeserializeNumber")]
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
    #[serde_as(as = "DeserializeNumber")]
    pub quantity: Number,
    /// The adjusted total cost of the parcel (rounded to 2 decimal places).
    #[serde_as(as = "DeserializeNumber")]
    pub cost_base: Number,
    /// The market value of the parcel at the balance date (rounded to 2 decimal places).
    #[serde_as(as = "DeserializeNumber")]
    pub market_value: Number,
    /// The unrealised capital gain (negative for losses, rounded to 2 decimal places).
    #[serde_as(as = "DeserializeNumber")]
    pub unrealised_gain: Number,
}

/// Retrieves the Valuation Report for the underlying portfolio.
pub struct Valuation;

impl<'a> ApiEndpoint<'a> for Valuation {
    const URL_PATH: &'static str = "/portfolios/:portfolio_id/valuation.json";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Get;
    const VERSION: &'static str = "2.0.0";

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
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub custom_group_id: i64,
    /// The total value of the portfolio
    #[serde_as(as = "DeserializeNumber")]
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
    /// The name of the selected grouping for the report
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub grouping: String,
    /// The name of the instrument
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub name: String,
    /// The current value of the holding.
    #[serde_as(as = "DeserializeNumber")]
    pub value: Number,
    /// The quantity of shares or other instruments in the holding
    #[serde_as(as = "DeserializeNumber")]
    pub quantity: Number,
    /// The group value this instrument has been placed in - note that the field name will be the group type
    #[serde(flatten)]
    pub grouping_information: HashMap<String, String>,
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
    #[serde_as(as = "DeserializeNumber")]
    pub value: Number,
    /// The currency symbol (e.g. AU$) of the cash account
    pub currency: Currency,
    /// The ISO currency code (e.g. AUD) of the portfolio
    pub currency_code: Currency,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct ValuationSubTotalsSuccess {
    /// The total value of the holdings in this group
    #[serde_as(as = "DeserializeNumber")]
    pub value: Number,
    /// The group value - note that the field name will be the group type
    #[serde(flatten)]
    pub grouping_information: HashMap<String, String>,
}

/// Single sign on authorisation. The single sign-on operation returns a URL that will allow the user to login to their Sharesight account without the need to enter their email address and password. The URL is valid for one minute. A single sign-on link or button in your application should be implemented so that the user click initiates this API call and then the URL returned is launched in the user’s browser. A "redirect_to" parameter can be appended to the login url. After successfully been logged in, the user will be then redirected to the specified redirect_to path; example: https://api.sharesight.com/users/sign_in?signon-token=token&amp;redirect_to=/portfolios/1
pub struct RequestSingleSignOn;

impl<'a> ApiEndpoint<'a> for RequestSingleSignOn {
    const URL_PATH: &'static str = "/single_sign_on.json";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Get;
    const VERSION: &'static str = "2.0.0";

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
    const VERSION: &'static str = "2.0.0";

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
    #[serde_as(as = "DeserializeDate")]
    pub transaction_date: NaiveDate,
    /// The market code (`"ASX"`, `"NZX"`, etc).
    pub market: Market,
    /// The instrument code/symbol.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub symbol: String,
    /// Number of shares sold/bought.
    #[serde_as(as = "DeserializeNumber")]
    pub quantity: Number,
    /// Price paid/received.
    #[serde_as(as = "DeserializeNumber")]
    pub price: Number,
    /// The transfer's exchange rate.
    #[serde_as(as = "DeserializeNumber")]
    pub exchange_rate: Number,
    /// The transfer's brokerage.
    #[serde_as(as = "DeserializeNumber")]
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
    const VERSION: &'static str = "2.0.0";

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
    #[serde_as(as = "DeserializeDate")]
    pub transaction_date: NaiveDate,
    /// The market code (`"ASX"`, `"NZX"`, etc).
    pub market: Market,
    /// The instrument code/symbol.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub symbol: String,
    /// Number of shares sold/bought.
    #[serde_as(as = "DeserializeNumber")]
    pub quantity: Number,
    /// Price paid/received.
    #[serde_as(as = "DeserializeNumber")]
    pub price: Number,
    /// The transfer's exchange rate.
    #[serde_as(as = "DeserializeNumber")]
    pub exchange_rate: Number,
    /// The transfer's brokerage.
    #[serde_as(as = "DeserializeNumber")]
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
    const VERSION: &'static str = "2.0.0";

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
    #[serde_as(as = "DeserializeNumber")]
    pub quantity: Number,
    /// Price paid/received.
    #[serde_as(as = "DeserializeNumber")]
    pub price: Number,
    /// For an opening balance, the cost base of the trade. Always returned in the portfolio currency
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub cost_base: Option<Number>,
    /// The trade's exchange rate as portfolio currency / instrument currency.
    #[serde_as(as = "DeserializeNumber")]
    pub exchange_rate: Number,
    /// The trade's brokerage.
    #[serde_as(as = "DeserializeNumber")]
    pub brokerage: Number,
    /// The ISO code of the brokerage currency, must be either Portfolio or Instrument currency. If the instrument is a cryptocurrency, any valid brokerage currency is supported.
    #[serde(default)]
    pub brokerage_currency_code: Option<Currency>,
    /// The value for the trade as displayed in the 'value' column of the UI. For a return of capital, this will be the (signed) capital return value. For a capital call, this will be the (positive) capital return value. For a cost base adjustment, this will be the value of the adjustment. For an opening balance, this will be the market value: the market price x quantity at the opening balance date In each case this is in portfolio currency (rounded to 2 decimal places).
    #[serde_as(as = "DeserializeNumber")]
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
    const VERSION: &'static str = "2.0.0";

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
    #[serde_as(as = "Option<DeserializeDate>")]
    #[serde(default)]
    pub transaction_date: Option<NaiveDate>,
    /// Number of units in the transaction. Must be a whole number unless the market allows fractional quantities to be traded.
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub quantity: Option<Number>,
    /// Currency value per unit.
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub price: Option<Number>,
    /// For an opening balance, the cost base of the trade.
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub cost_base: Option<Number>,
    /// The exchange rate used for the transaction as portfolio currency / instrument currency.
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub exchange_rate: Option<Number>,
    /// The brokerage fee (currency value).
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub brokerage: Option<Number>,
    /// The ISO code of the brokerage currency, must be either Portfolio or Instrument currency. If the instrument is a cryptocurrency, any valid brokerage currency is supported.
    #[serde(default)]
    pub brokerage_currency_code: Option<Currency>,
    /// When transaction_type "ADJUST_COST_BASE" was chosen, this is the required value
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub adjust_cost_base_value: Option<Number>,
    /// When transaction_type "CAPITAL_RETURN" or "CAPITAL_CALL" is chosen, this is the required value
    #[serde_as(as = "Option<DeserializeNumber>")]
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
    #[serde_as(as = "DeserializeNumber")]
    pub quantity: Number,
    /// Price paid/received.
    #[serde_as(as = "DeserializeNumber")]
    pub price: Number,
    /// For an opening balance, the cost base of the trade. Always returned in the portfolio currency
    #[serde_as(as = "DeserializeNumber")]
    pub cost_base: Number,
    /// The trade's exchange rate as portfolio currency / instrument currency.
    #[serde_as(as = "DeserializeNumber")]
    pub exchange_rate: Number,
    /// The trade's brokerage.
    #[serde_as(as = "DeserializeNumber")]
    pub brokerage: Number,
    /// The ISO code of the brokerage currency, must be either Portfolio or Instrument currency. If the instrument is a cryptocurrency, any valid brokerage currency is supported.
    pub brokerage_currency_code: Currency,
    /// The value for the trade as displayed in the 'value' column of the UI. For a return of capital, this will be the (signed) capital return value. For a capital call, this will be the (positive) capital return value. For a cost base adjustment, this will be the value of the adjustment. For an opening balance, this will be the market value: the market price x quantity at the opening balance date In each case this is in portfolio currency (rounded to 2 decimal places).
    #[serde_as(as = "DeserializeNumber")]
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
    const VERSION: &'static str = "2.0.0";

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
    const VERSION: &'static str = "2.0.0";

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
    #[serde_as(as = "DeserializeNumber")]
    pub quantity: Number,
    /// Price paid/received.
    #[serde_as(as = "DeserializeNumber")]
    pub price: Number,
    /// For an opening balance, the cost base of the trade. Always returned in the portfolio currency
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub cost_base: Option<Number>,
    /// The trade's exchange rate as portfolio currency / instrument currency.
    #[serde_as(as = "DeserializeNumber")]
    pub exchange_rate: Number,
    /// The trade's brokerage.
    #[serde_as(as = "DeserializeNumber")]
    pub brokerage: Number,
    /// The ISO code of the brokerage currency, must be either Portfolio or Instrument currency. If the instrument is a cryptocurrency, any valid brokerage currency is supported.
    pub brokerage_currency_code: Currency,
    /// The value for the trade as displayed in the 'value' column of the UI. For a return of capital, this will be the (signed) capital return value. For a capital call, this will be the (positive) capital return value. For a cost base adjustment, this will be the value of the adjustment. For an opening balance, this will be the market value: the market price x quantity at the opening balance date In each case this is in portfolio currency (rounded to 2 decimal places).
    #[serde_as(as = "DeserializeNumber")]
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
    const VERSION: &'static str = "2.0.0";

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
    #[serde_as(as = "Option<DeserializeDate>")]
    #[serde(default)]
    pub transaction_date: Option<NaiveDate>,
    /// Number of units in the transaction. Must be a whole number unless the market allows fractional quantities to be traded.
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub quantity: Option<Number>,
    /// Currency value per unit.
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub price: Option<Number>,
    /// For an opening balance, the cost base of the trade.
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub cost_base: Option<Number>,
    /// The exchange rate used for the transaction as portfolio currency / instrument currency.
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub exchange_rate: Option<Number>,
    /// The brokerage fee (currency value).
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub brokerage: Option<Number>,
    /// The ISO code of the brokerage currency, must be either Portfolio or Instrument currency. If the instrument is a cryptocurrency, any valid brokerage currency is supported.
    #[serde(default)]
    pub brokerage_currency_code: Option<Currency>,
    /// When transaction_type "ADJUST_COST_BASE" was chosen, this is the required value
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub adjust_cost_base_value: Option<Number>,
    /// When transaction_type "CAPITAL_RETURN" or "CAPITAL_CALL" is chosen, this is the required value
    #[serde_as(as = "Option<DeserializeNumber>")]
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
    #[serde_as(as = "DeserializeNumber")]
    pub quantity: Number,
    /// Price paid/received.
    #[serde_as(as = "DeserializeNumber")]
    pub price: Number,
    /// For an opening balance, the cost base of the trade. Always returned in the portfolio currency
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub cost_base: Option<Number>,
    /// The trade's exchange rate as portfolio currency / instrument currency.
    #[serde_as(as = "DeserializeNumber")]
    pub exchange_rate: Number,
    /// The trade's brokerage.
    #[serde_as(as = "DeserializeNumber")]
    pub brokerage: Number,
    /// The ISO code of the brokerage currency, must be either Portfolio or Instrument currency. If the instrument is a cryptocurrency, any valid brokerage currency is supported.
    pub brokerage_currency_code: Currency,
    /// The value for the trade as displayed in the 'value' column of the UI. For a return of capital, this will be the (signed) capital return value. For a capital call, this will be the (positive) capital return value. For a cost base adjustment, this will be the value of the adjustment. For an opening balance, this will be the market value: the market price x quantity at the opening balance date In each case this is in portfolio currency (rounded to 2 decimal places).
    #[serde_as(as = "DeserializeNumber")]
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
    const VERSION: &'static str = "2.0.0";

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
    pub signed_up_at: DateTime<FixedOffset>,
    /// True if the user signed up for Sharesight via your application
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub signup_via_your_integration: String,
}

/// Apply a coupon code to the current user
pub struct CouponCodeCreate;

impl<'a> ApiEndpoint<'a> for CouponCodeCreate {
    const URL_PATH: &'static str = "/coupon_code";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Post;
    const VERSION: &'static str = "3.0.0";

    type UrlDisplay = &'static str;
    type Parameters = CouponCodeCreateParameters;
    type Success = CouponCodeCreateSuccess;

    fn url_path(_parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        "/coupon_code"
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct CouponCodeCreateParameters {
    /// Code to be applied
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub code: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CouponCodeCreateSuccess {
    /// The coupon code applied to this user
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub code: String,
    /// The current API Transaction.
    pub api_transaction: CouponCodeCreateApiTransactionSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CouponCodeCreateApiTransactionSuccess {
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

/// Delete a coupon code from the current user
pub struct CouponCodeDelete;

impl<'a> ApiEndpoint<'a> for CouponCodeDelete {
    const URL_PATH: &'static str = "/coupon_code";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Delete;
    const VERSION: &'static str = "3.0.0";

    type UrlDisplay = &'static str;
    type Parameters = ();
    type Success = CouponCodeDeleteSuccess;

    fn url_path(_parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        "/coupon_code"
    }
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CouponCodeDeleteSuccess {
    /// The current API Transaction.
    pub api_transaction: CouponCodeDeleteApiTransactionSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CouponCodeDeleteApiTransactionSuccess {
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

/// Return the coupon code for the current user
pub struct CouponCodeList;

impl<'a> ApiEndpoint<'a> for CouponCodeList {
    const URL_PATH: &'static str = "/coupon_code";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Get;
    const VERSION: &'static str = "3.0.0";

    type UrlDisplay = &'static str;
    type Parameters = ();
    type Success = CouponCodeListSuccess;

    fn url_path(_parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        "/coupon_code"
    }
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CouponCodeListSuccess {
    /// The coupon code applied to this user
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub code: String,
    /// True if this coupon is only enabled for API use
    pub api_only: bool,
    /// The current API Transaction.
    pub api_transaction: CouponCodeListApiTransactionSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CouponCodeListApiTransactionSuccess {
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

/// Creates coupon rates for a custom investment
pub struct CouponRateCreate;

impl<'a> ApiEndpoint<'a> for CouponRateCreate {
    const URL_PATH: &'static str = "/custom_investments/{instrument_id}/coupon_rates";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Post;
    const VERSION: &'static str = "3.0.0";

    type UrlDisplay = CouponRateCreateUrlDisplay<'a>;
    type Parameters = CouponRateCreateParameters;
    type Success = CouponRateCreateSuccess;

    fn url_path(parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        CouponRateCreateUrlDisplay(parameters)
    }
}

pub struct CouponRateCreateUrlDisplay<'a>(&'a CouponRateCreateParameters);

impl<'a> fmt::Display for CouponRateCreateUrlDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parameters = self.0;

        write!(
            f,
            "/custom_investments/{}/coupon_rates",
            parameters.instrument_id
        )
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct CouponRateCreateParameters {
    /// The interest rate as a percentage
    #[serde_as(as = "DeserializeNumber")]
    pub interest_rate: Number,
    /// The date from which the interest rate applies (YYYY-MM-DD)
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub date: String,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub instrument_id: i64,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CouponRateCreateSuccess {
    pub coupon_rate: CouponRateCreateCouponRateSuccess,
    /// The current API Transaction.
    pub api_transaction: CouponRateCreateApiTransactionSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CouponRateCreateCouponRateSuccess {
    /// Identifier of the coupon rate
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// The interest rate as a percentage
    #[serde_as(as = "DeserializeNumber")]
    pub interest_rate: Number,
    /// The date from which the interest rate applies (YYYY-MM-DD)
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub date: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CouponRateCreateApiTransactionSuccess {
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

/// Deletes custom investment coupon rates
pub struct CouponRateDelete;

impl<'a> ApiEndpoint<'a> for CouponRateDelete {
    const URL_PATH: &'static str = "/coupon_rates/{id}";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Delete;
    const VERSION: &'static str = "3.0.0";

    type UrlDisplay = CouponRateDeleteUrlDisplay<'a>;
    type Parameters = CouponRateDeleteParameters;
    type Success = CouponRateDeleteSuccess;

    fn url_path(parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        CouponRateDeleteUrlDisplay(parameters)
    }
}

pub struct CouponRateDeleteUrlDisplay<'a>(&'a CouponRateDeleteParameters);

impl<'a> fmt::Display for CouponRateDeleteUrlDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parameters = self.0;

        write!(f, "/coupon_rates/{}", parameters.id)
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct CouponRateDeleteParameters {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CouponRateDeleteSuccess {
    /// The current API Transaction.
    pub api_transaction: CouponRateDeleteApiTransactionSuccess,
    pub links: CouponRateDeleteLinksSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CouponRateDeleteApiTransactionSuccess {
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

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CouponRateDeleteLinksSuccess {
    /// URL to a list of requested resources.
    #[serde(rename = "self")]
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub itself: String,
}

/// Retrieves coupon rates for a fixed interest custom investment
pub struct CouponRateList;

impl<'a> ApiEndpoint<'a> for CouponRateList {
    const URL_PATH: &'static str = "/custom_investments/{instrument_id}/coupon_rates";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Get;
    const VERSION: &'static str = "3.0.0";

    type UrlDisplay = CouponRateListUrlDisplay<'a>;
    type Parameters = CouponRateListParameters;
    type Success = CouponRateListSuccess;

    fn url_path(parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        CouponRateListUrlDisplay(parameters)
    }
}

pub struct CouponRateListUrlDisplay<'a>(&'a CouponRateListParameters);

impl<'a> fmt::Display for CouponRateListUrlDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parameters = self.0;

        write!(
            f,
            "/custom_investments/{}/coupon_rates",
            parameters.instrument_id
        )
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct CouponRateListParameters {
    /// Retrieve coupon rates from this date in (YYYY-MM-DD)
    #[serde_as(as = "Option<DeserializeDate>")]
    #[serde(default)]
    pub start_date: Option<NaiveDate>,
    /// Retrieve prices until this date in (YYYY-MM-DD)
    #[serde_as(as = "Option<DeserializeDate>")]
    #[serde(default)]
    pub end_date: Option<NaiveDate>,
    /// Pointer to the next page/set of items. This will be the page returned from the previous response.
    #[serde(default)]
    pub page: Option<String>,
    /// Items returned per page. If not given, the default is 50. Maximum is 100.
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub per_page: Option<i64>,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub instrument_id: i64,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CouponRateListSuccess {
    /// List of coupon rates
    pub coupon_rates: Vec<CouponRateListCouponRatesSuccess>,
    pub pagination: CouponRateListPaginationSuccess,
    /// The current API Transaction.
    pub api_transaction: CouponRateListApiTransactionSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CouponRateListCouponRatesSuccess {
    /// Identifier of the coupon rate
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// The interest rate as a percentage
    #[serde_as(as = "DeserializeNumber")]
    pub interest_rate: Number,
    /// The date from which the interest rate applies (YYYY-MM-DD)
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub date: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CouponRateListPaginationSuccess {
    /// Pointer to the next page/set of items. This will be the page returned from the previous response.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub page: String,
    /// Items returned per page. If not given, the default is 50. Maximum is 100.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub per_page: i64,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CouponRateListApiTransactionSuccess {
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

/// Updates custom investment coupon rates
pub struct CouponRateUpdate;

impl<'a> ApiEndpoint<'a> for CouponRateUpdate {
    const URL_PATH: &'static str = "/coupon_rates/{id}";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Put;
    const VERSION: &'static str = "3.0.0";

    type UrlDisplay = CouponRateUpdateUrlDisplay<'a>;
    type Parameters = CouponRateUpdateParameters;
    type Success = CouponRateUpdateSuccess;

    fn url_path(parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        CouponRateUpdateUrlDisplay(parameters)
    }
}

pub struct CouponRateUpdateUrlDisplay<'a>(&'a CouponRateUpdateParameters);

impl<'a> fmt::Display for CouponRateUpdateUrlDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parameters = self.0;

        write!(f, "/coupon_rates/{}", parameters.id)
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct CouponRateUpdateParameters {
    /// The interest rate as a percentage
    #[serde_as(as = "DeserializeNumber")]
    pub interest_rate: Number,
    /// The date from which the interest rate applies (YYYY-MM-DD)
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub date: String,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CouponRateUpdateSuccess {
    pub coupon_rate: CouponRateUpdateCouponRateSuccess,
    /// The current API Transaction.
    pub api_transaction: CouponRateUpdateApiTransactionSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CouponRateUpdateCouponRateSuccess {
    /// Identifier of the coupon rate
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// The interest rate as a percentage
    #[serde_as(as = "DeserializeNumber")]
    pub interest_rate: Number,
    /// The date from which the interest rate applies (YYYY-MM-DD)
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub date: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CouponRateUpdateApiTransactionSuccess {
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

/// Create a price entry for a custom investment
pub struct CustomInvestmentPriceCreate;

impl<'a> ApiEndpoint<'a> for CustomInvestmentPriceCreate {
    const URL_PATH: &'static str = "/custom_investment/{id}/prices.json";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Post;
    const VERSION: &'static str = "3.0.0";

    type UrlDisplay = CustomInvestmentPriceCreateUrlDisplay<'a>;
    type Parameters = CustomInvestmentPriceCreateParameters;
    type Success = CustomInvestmentPriceCreateSuccess;

    fn url_path(parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        CustomInvestmentPriceCreateUrlDisplay(parameters)
    }
}

pub struct CustomInvestmentPriceCreateUrlDisplay<'a>(&'a CustomInvestmentPriceCreateParameters);

impl<'a> fmt::Display for CustomInvestmentPriceCreateUrlDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parameters = self.0;

        write!(f, "/custom_investment/{}/prices.json", parameters.id)
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct CustomInvestmentPriceCreateParameters {
    /// The price in the instrument currency
    #[serde_as(as = "DeserializeNumber")]
    pub last_traded_price: Number,
    /// The date of the instrument price in (YYYY-MM-DD) format.
    #[serde_as(as = "DeserializeDate")]
    pub last_traded_on: NaiveDate,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CustomInvestmentPriceCreateSuccess {
    /// Identifier for the custom investment
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub custom_investment_id: i64,
    /// Id of the created price
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// The price in the instrument currency
    #[serde_as(as = "DeserializeNumber")]
    pub last_traded_price: Number,
    /// The date of the instrument price
    #[serde_as(as = "DeserializeDate")]
    pub last_traded_on: NaiveDate,
    /// The date and time of the instrument price
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub last_traded_at: String,
    /// The current API Transaction.
    pub api_transaction: CustomInvestmentPriceCreateApiTransactionSuccess,
    pub links: CustomInvestmentPriceCreateLinksSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CustomInvestmentPriceCreateApiTransactionSuccess {
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

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CustomInvestmentPriceCreateLinksSuccess {
    /// URL to a list of requested resources.
    #[serde(rename = "self")]
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub itself: String,
}

/// Delete a price for a custom investment
pub struct CustomInvestmentPriceDelete;

impl<'a> ApiEndpoint<'a> for CustomInvestmentPriceDelete {
    const URL_PATH: &'static str = "/prices/{id}.json";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Delete;
    const VERSION: &'static str = "3.0.0";

    type UrlDisplay = CustomInvestmentPriceDeleteUrlDisplay<'a>;
    type Parameters = CustomInvestmentPriceDeleteParameters;
    type Success = CustomInvestmentPriceDeleteSuccess;

    fn url_path(parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        CustomInvestmentPriceDeleteUrlDisplay(parameters)
    }
}

pub struct CustomInvestmentPriceDeleteUrlDisplay<'a>(&'a CustomInvestmentPriceDeleteParameters);

impl<'a> fmt::Display for CustomInvestmentPriceDeleteUrlDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parameters = self.0;

        write!(f, "/prices/{}.json", parameters.id)
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct CustomInvestmentPriceDeleteParameters {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CustomInvestmentPriceDeleteSuccess {
    /// The current API Transaction.
    pub api_transaction: CustomInvestmentPriceDeleteApiTransactionSuccess,
    pub links: CustomInvestmentPriceDeleteLinksSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CustomInvestmentPriceDeleteApiTransactionSuccess {
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

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CustomInvestmentPriceDeleteLinksSuccess {
    /// URL to a list of requested resources.
    #[serde(rename = "self")]
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub itself: String,
}

/// Update a price for a custom investment
pub struct CustomInvestmentPriceUpdate;

impl<'a> ApiEndpoint<'a> for CustomInvestmentPriceUpdate {
    const URL_PATH: &'static str = "/prices/{id}.json";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Put;
    const VERSION: &'static str = "3.0.0";

    type UrlDisplay = CustomInvestmentPriceUpdateUrlDisplay<'a>;
    type Parameters = CustomInvestmentPriceUpdateParameters;
    type Success = CustomInvestmentPriceUpdateSuccess;

    fn url_path(parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        CustomInvestmentPriceUpdateUrlDisplay(parameters)
    }
}

pub struct CustomInvestmentPriceUpdateUrlDisplay<'a>(&'a CustomInvestmentPriceUpdateParameters);

impl<'a> fmt::Display for CustomInvestmentPriceUpdateUrlDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parameters = self.0;

        write!(f, "/prices/{}.json", parameters.id)
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct CustomInvestmentPriceUpdateParameters {
    /// The price in the instrument currency
    #[serde(default)]
    pub last_traded_price: Option<String>,
    /// The date of the instrument price in (YYYY-MM-DD) format.
    #[serde_as(as = "Option<DeserializeDate>")]
    #[serde(default)]
    pub last_traded_on: Option<NaiveDate>,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CustomInvestmentPriceUpdateSuccess {
    /// Identifier for the custom investment
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub custom_investment_id: i64,
    /// Id of the created price
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// The price in the instrument currency
    #[serde_as(as = "DeserializeNumber")]
    pub last_traded_price: Number,
    /// The date of the instrument price
    #[serde_as(as = "DeserializeDate")]
    pub last_traded_on: NaiveDate,
    /// The date and time of the instrument price
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub last_traded_at: String,
    /// The current API Transaction.
    pub api_transaction: CustomInvestmentPriceUpdateApiTransactionSuccess,
    pub links: CustomInvestmentPriceUpdateLinksSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CustomInvestmentPriceUpdateApiTransactionSuccess {
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

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CustomInvestmentPriceUpdateLinksSuccess {
    /// URL to a list of requested resources.
    #[serde(rename = "self")]
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub itself: String,
}

/// Create a Custom Investment
pub struct CustomInvestmentCreate;

impl<'a> ApiEndpoint<'a> for CustomInvestmentCreate {
    const URL_PATH: &'static str = "/custom_investments";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Post;
    const VERSION: &'static str = "3.0.0";

    type UrlDisplay = &'static str;
    type Parameters = CustomInvestmentCreateParameters;
    type Success = CustomInvestmentCreateSuccess;

    fn url_path(_parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        "/custom_investments"
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct CustomInvestmentCreateParameters {
    /// The ID of the portfolio this instrument is associated with. If not provided, the instrument will be linked to the owner
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub portfolio_id: Option<i64>,
    /// The investment code
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub code: String,
    /// The name of the custom investment
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub name: String,
    /// The code identifier of the (tax) country of this custom investment. Refer to the `/api/v3/codes#country` documentation for a list of country codes
    pub country_code: Country,
    /// Supported investment types: ORDINARY, WARRANT, SHAREFUND, PROPFUND, PREFERENCE, STAPLEDSEC, OPTIONS, RIGHTS, MANAGED_FUND, FIXED_INTEREST, PIE
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub investment_type: String,
    /// Face value per unit. Only applicable to 'FIXED_INTEREST'. Required for a 'FIXED_INTEREST' instrument
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub face_value: Option<Number>,
    /// Initial interest rate. Only applicable to 'FIXED_INTEREST'. Required for a 'FIXED_INTEREST' instrument when `auto_calc_income` is set. (When coupon rates are defined for specific time periods, these will override this value)
    #[serde_as(as = "Option<DeserializeNumber>")]
    #[serde(default)]
    pub interest_rate: Option<Number>,
    /// 'DIVIDEND' or 'INTEREST' only. Only applicable to 'FIXED_INTEREST'
    #[serde(default)]
    pub income_type: Option<String>,
    /// 'ON_MATURITY','YEARLY','TWICE_YEARLY','QUARTERLY','MONTHLY'. Only applicable to 'FIXED_INTEREST'. Only applicable to 'FIXED_INTEREST'. Required for a 'FIXED_INTEREST' instrument when `auto_calc_income` is set
    #[serde(default)]
    pub payment_frequency: Option<String>,
    /// Date of first interest payment. Only applicable to 'FIXED_INTEREST'. Required for a 'FIXED_INTEREST' instrument when `auto_calc_income` is set. Format is: (YYYY-MM-DD)
    #[serde_as(as = "Option<DeserializeDate>")]
    #[serde(default)]
    pub first_payment_date: Option<NaiveDate>,
    /// If a maturity date is set, a sell trade will be recorded on the maturity date to reflect the repayment of the principal. Only applicable to 'FIXED_INTEREST'. Required if 'payment_frequency' = 'ON_MATURITY'. Format is: (YYYY-MM-DD)
    #[serde_as(as = "Option<DeserializeDate>")]
    #[serde(default)]
    pub maturity_date: Option<NaiveDate>,
    /// Autopopulate income payments and maturity trades based on selected dates. Only applicable to 'FIXED_INTEREST'<br>Default value: `false`
    #[serde(default)]
    pub auto_calc_income: Option<bool>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CustomInvestmentCreateSuccess {
    pub custom_investment: CustomInvestmentCreateCustomInvestmentSuccess,
    /// The current API Transaction.
    pub api_transaction: CustomInvestmentCreateApiTransactionSuccess,
    pub links: CustomInvestmentCreateLinksSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CustomInvestmentCreateCustomInvestmentSuccess {
    /// The ID of this custom investment
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// The portfolio associated with this model
    pub portfolio: CustomInvestmentCreateCustomInvestmentPortfolioSuccess,
    /// The investment code
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub code: String,
    /// Market code. It will always be OTHER.
    pub market_code: Market,
    /// The name of the custom investment
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub name: String,
    /// The code identifier of the (tax) country of this custom investment. Refer to the `/api/v3/codes#country` documentation for a list of country codes
    pub country_code: Country,
    /// The internal id of the (tax) country of this custom investment. Refer to the `/api/v3/countries` endpoint to look up country details by id.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub country_id: i64,
    /// The ISO-4217 currency code (https://www.currency-iso.org/en/home/amendments/secretariat.html) (e.g. AUD) of this custom investment
    pub currency_code: Currency,
    /// A normalised description of the custom investment.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub investment_type: String,
    /// The Timezone for the investment.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub tz_name: String,
    /// 'DIVIDEND' or 'INTEREST' only. Only applicable to 'FIXED_INTEREST'
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub income_type: String,
    /// 'ON_MATURITY','YEARLY','TWICE_YEARLY','QUARTERLY','MONTHLY'
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub payment_frequency: String,
    /// Date of first interest payment. Only applicable to 'FIXED_INTEREST'. This is in the timezone of the instrument. Format is: (YYYY-MM-DD)
    #[serde_as(as = "DeserializeDate")]
    pub first_payment_date: NaiveDate,
    /// If a maturity date is set a sell trade will be recorded on the maturity date to reflect the repayment of the principal. Only applicable to 'FIXED_INTEREST'. This is in the timezone of the instrument. Format is: (YYYY-MM-DD)
    #[serde_as(as = "DeserializeDate")]
    pub maturity_date: NaiveDate,
    /// Autopopulate income payments based on selected dates. Only applicable to 'FIXED_INTEREST'.
    pub auto_calc_income: bool,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CustomInvestmentCreateCustomInvestmentPortfolioSuccess {
    /// The unique id identifying the portfolio
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// Whether or not this is a consolidated view portfolio
    pub consolidated: bool,
    /// The name of the portfolio
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub name: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CustomInvestmentCreateApiTransactionSuccess {
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

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CustomInvestmentCreateLinksSuccess {
    /// URL to a list of requested resources.
    #[serde(rename = "self")]
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub itself: String,
}

/// Delete a Custom Investment
pub struct CustomInvestmentDelete;

impl<'a> ApiEndpoint<'a> for CustomInvestmentDelete {
    const URL_PATH: &'static str = "/custom_investments/{id}";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Delete;
    const VERSION: &'static str = "3.0.0";

    type UrlDisplay = CustomInvestmentDeleteUrlDisplay<'a>;
    type Parameters = CustomInvestmentDeleteParameters;
    type Success = CustomInvestmentDeleteSuccess;

    fn url_path(parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        CustomInvestmentDeleteUrlDisplay(parameters)
    }
}

pub struct CustomInvestmentDeleteUrlDisplay<'a>(&'a CustomInvestmentDeleteParameters);

impl<'a> fmt::Display for CustomInvestmentDeleteUrlDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parameters = self.0;

        write!(f, "/custom_investments/{}", parameters.id)
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct CustomInvestmentDeleteParameters {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CustomInvestmentDeleteSuccess {
    /// True if the record was deleted
    pub deleted: bool,
    /// The current API Transaction.
    pub api_transaction: CustomInvestmentDeleteApiTransactionSuccess,
    pub links: CustomInvestmentDeleteLinksSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CustomInvestmentDeleteApiTransactionSuccess {
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

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CustomInvestmentDeleteLinksSuccess {
    /// URL to a list of requested resources.
    #[serde(rename = "self")]
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub itself: String,
}

/// Retrieves a list of Custom Investments
pub struct CustomInvestmentList;

impl<'a> ApiEndpoint<'a> for CustomInvestmentList {
    const URL_PATH: &'static str = "/custom_investments";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Get;
    const VERSION: &'static str = "3.0.0";

    type UrlDisplay = &'static str;
    type Parameters = CustomInvestmentListParameters;
    type Success = CustomInvestmentListSuccess;

    fn url_path(_parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        "/custom_investments"
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct CustomInvestmentListParameters {
    /// The portfolio id to List all Custom Investments for
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub portfolio_id: Option<i64>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CustomInvestmentListSuccess {
    /// The custom investments
    pub custom_investments: Vec<CustomInvestmentListCustomInvestmentsSuccess>,
    /// The current API Transaction.
    pub api_transaction: CustomInvestmentListApiTransactionSuccess,
    pub links: CustomInvestmentListLinksSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CustomInvestmentListCustomInvestmentsSuccess {
    /// The ID of this custom investment
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// The portfolio associated with this model
    pub portfolio: CustomInvestmentListCustomInvestmentsPortfolioSuccess,
    /// The investment code
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub code: String,
    /// Market code. It will always be OTHER.
    pub market_code: Market,
    /// The name of the custom investment
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub name: String,
    /// The code identifier of the (tax) country of this custom investment. Refer to the `/api/v3/codes#country` documentation for a list of country codes
    pub country_code: Country,
    /// The internal id of the (tax) country of this custom investment. Refer to the `/api/v3/countries` endpoint to look up country details by id.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub country_id: i64,
    /// The ISO-4217 currency code (https://www.currency-iso.org/en/home/amendments/secretariat.html) (e.g. AUD) of this custom investment
    pub currency_code: Currency,
    /// A normalised description of the custom investment.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub investment_type: String,
    /// The Timezone for the investment.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub tz_name: String,
    /// 'DIVIDEND' or 'INTEREST' only. Only applicable to 'FIXED_INTEREST'
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub income_type: String,
    /// 'ON_MATURITY','YEARLY','TWICE_YEARLY','QUARTERLY','MONTHLY'
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub payment_frequency: String,
    /// Date of first interest payment. Only applicable to 'FIXED_INTEREST'. This is in the timezone of the instrument. Format is: (YYYY-MM-DD)
    #[serde_as(as = "DeserializeDate")]
    pub first_payment_date: NaiveDate,
    /// If a maturity date is set a sell trade will be recorded on the maturity date to reflect the repayment of the principal. Only applicable to 'FIXED_INTEREST'. This is in the timezone of the instrument. Format is: (YYYY-MM-DD)
    #[serde_as(as = "DeserializeDate")]
    pub maturity_date: NaiveDate,
    /// Autopopulate income payments based on selected dates. Only applicable to 'FIXED_INTEREST'.
    pub auto_calc_income: bool,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CustomInvestmentListCustomInvestmentsPortfolioSuccess {
    /// The unique id identifying the portfolio
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// Whether or not this is a consolidated view portfolio
    pub consolidated: bool,
    /// The name of the portfolio
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub name: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CustomInvestmentListApiTransactionSuccess {
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

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CustomInvestmentListLinksSuccess {
    /// URL to a list of requested resources.
    #[serde(rename = "self")]
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub itself: String,
}

/// Retrieves a single Custom Investment
pub struct CustomInvestmentShow;

impl<'a> ApiEndpoint<'a> for CustomInvestmentShow {
    const URL_PATH: &'static str = "/custom_investments/{id}";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Get;
    const VERSION: &'static str = "3.0.0";

    type UrlDisplay = CustomInvestmentShowUrlDisplay<'a>;
    type Parameters = CustomInvestmentShowParameters;
    type Success = CustomInvestmentShowSuccess;

    fn url_path(parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        CustomInvestmentShowUrlDisplay(parameters)
    }
}

pub struct CustomInvestmentShowUrlDisplay<'a>(&'a CustomInvestmentShowParameters);

impl<'a> fmt::Display for CustomInvestmentShowUrlDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parameters = self.0;

        write!(f, "/custom_investments/{}", parameters.id)
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct CustomInvestmentShowParameters {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CustomInvestmentShowSuccess {
    pub custom_investment: CustomInvestmentShowCustomInvestmentSuccess,
    /// The current API Transaction.
    pub api_transaction: CustomInvestmentShowApiTransactionSuccess,
    pub links: CustomInvestmentShowLinksSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CustomInvestmentShowCustomInvestmentSuccess {
    /// The ID of this custom investment
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// The ID of the portfolio this instrument is associated with. If not provided, the instrument will be linked to the owner.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub portfolio_id: i64,
    /// The portfolio associated with this model
    pub portfolio: CustomInvestmentShowCustomInvestmentPortfolioSuccess,
    /// The investment code
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub code: String,
    /// Market code. It will always be OTHER.
    pub market_code: Market,
    /// The name of the custom investment
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub name: String,
    /// The code identifier of the (tax) country of this custom investment. Refer to the `/api/v3/codes#country` documentation for a list of country codes
    pub country_code: Country,
    /// The internal id of the (tax) country of this custom investment. Refer to the `/api/v3/countries` endpoint to look up country details by id.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub country_id: i64,
    /// The ISO-4217 currency code (https://www.currency-iso.org/en/home/amendments/secretariat.html) (e.g. AUD) of this custom investment
    pub currency_code: Currency,
    /// A normalised description of the custom investment.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub investment_type: String,
    /// The Timezone for the investment.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub tz_name: String,
    /// 'DIVIDEND' or 'INTEREST' only. Only applicable to 'FIXED_INTEREST'.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub income_type: String,
    /// 'ON_MATURITY','YEARLY','TWICE_YEARLY','QUARTERLY','MONTHLY'
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub payment_frequency: String,
    /// Date of first interest payment. Only applicable to 'FIXED_INTEREST'. This is in the timezone of the instrument. Format is: (YYYY-MM-DD)
    #[serde_as(as = "DeserializeDate")]
    pub first_payment_date: NaiveDate,
    /// If a maturity date is set a sell trade will be recorded on the maturity date to reflect the repayment of the principal. Only applicable to 'FIXED_INTEREST'. This is in the timezone of the instrument. Format is: (YYYY-MM-DD)
    #[serde_as(as = "DeserializeDate")]
    pub maturity_date: NaiveDate,
    /// Autopopulate income payments based on selected dates. Only applicable to 'FIXED_INTEREST'.
    pub auto_calc_income: bool,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CustomInvestmentShowCustomInvestmentPortfolioSuccess {
    /// The unique id identifying the portfolio
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// Whether or not this is a consolidated view portfolio
    pub consolidated: bool,
    /// The name of the portfolio
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub name: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CustomInvestmentShowApiTransactionSuccess {
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

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CustomInvestmentShowLinksSuccess {
    /// URL to a list of requested resources.
    #[serde(rename = "self")]
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub itself: String,
}

/// Retrieves a list of Holdings
pub struct HoldingList;

impl<'a> ApiEndpoint<'a> for HoldingList {
    const URL_PATH: &'static str = "/holdings";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Get;
    const VERSION: &'static str = "3.0.0";

    type UrlDisplay = &'static str;
    type Parameters = ();
    type Success = HoldingListSuccess;

    fn url_path(_parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        "/holdings"
    }
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct HoldingListSuccess {
    /// A list of holdings
    pub holdings: Vec<HoldingListHoldingsSuccess>,
    /// The current API Transaction.
    pub api_transaction: HoldingListApiTransactionSuccess,
    pub links: HoldingListLinksSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct HoldingListHoldingsSuccess {
    /// The unique id of this holding
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// An instrument object for the Holding
    pub instrument: HoldingListHoldingsInstrumentSuccess,
    /// A currency object
    pub instrument_currency: HoldingListHoldingsInstrumentCurrencySuccess,
    /// The Sharesight code for the instrument. This field is a deprecated alias for 'instrument.code', please use that instead.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub symbol: String,
    /// True if the holding has a valid position (e.g. positive number of shares)
    pub valid_position: bool,
    /// The portfolio associated with this model
    pub portfolio: HoldingListHoldingsPortfolioSuccess,
    /// Documents associated with this Holding
    pub documents: Vec<HoldingListHoldingsDocumentsSuccess>,
    /// Attachments associated with this Holding
    pub attachments: Vec<HoldingListHoldingsAttachmentsSuccess>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct HoldingListHoldingsInstrumentSuccess {
    /// The Sharesight code for the instrument
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub code: String,
    /// The Sharesight country identifier associated with this instrument
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub country_id: i64,
    /// Whether or not this instrument is a cryptocurency
    pub crypto: bool,
    /// The Sharesight currency code associated with this instrument
    pub currency_code: Currency,
    /// The date the instrument will be or has expired on
    #[serde_as(as = "DeserializeDate")]
    pub expires_on: NaiveDate,
    /// Whether or not this instrument is currently expired
    pub expired: bool,
    /// The unique Sharesight identifier for this instrument
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// The code of the market the instrument is listed on
    pub market_code: Market,
    /// The name of the instrument
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub name: String,
    /// The timezone name associated with this instrument
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub tz_name: String,
    /// The instrument sector according to FactSet's global security classification scheme. FactSet's default classification can be overridden at the holding level.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub industry_classification_name: String,
    /// The instrument industry according to FactSet's global security classification scheme. FactSet's default classification can be overridden at the holding level.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub sector_classification_name: String,
    /// The type of investment. This determines holding functionality, e.g. Ordinary Shares or Fixed Interest
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub friendly_instrument_description: String,
    /// The type of investment (as a code). This determines holding functionality, e.g. ordinary_shares
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub friendly_instrument_description_code: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct HoldingListHoldingsInstrumentCurrencySuccess {
    /// The 3-letter ISO 4217 currency code
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub code: String,
    /// The unique Sharesight identifier for this currency
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// The symbol for this currency (eg. $, ¥, £)
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub symbol: String,
    /// The qualified currency symbol when the symbol is not specific enough, eg. 'US$', '¥', or the currency code
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub qualified_symbol: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct HoldingListHoldingsPortfolioSuccess {
    /// The unique id identifying the portfolio
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// Whether or not this is a consolidated view portfolio
    pub consolidated: bool,
    /// The name of the portfolio
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub name: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct HoldingListHoldingsDocumentsSuccess {
    /// The unique id of this document
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// The name of the document
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub file_name: String,
    /// The mime type of the document
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub content_type: String,
    /// The size of the document, in bytes
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub file_size: i64,
    /// Date the document was created on
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub created_at: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct HoldingListHoldingsAttachmentsSuccess {
    /// The ID of this attachment
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// The type of attachment, either 'DOCUMENT' or 'CONTRACT_NOTE'
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub r#type: String,
    /// The file name for the attachment, including extension
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub file_name: String,
    /// The size of the file
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub file_size: i64,
    /// The mimetype of the attached file
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub content_type: String,
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub created_at: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct HoldingListApiTransactionSuccess {
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

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct HoldingListLinksSuccess {
    /// URL to a list of requested resources.
    #[serde(rename = "self")]
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub itself: String,
}

/// Retrieves the details of a holding
pub struct HoldingShow;

impl<'a> ApiEndpoint<'a> for HoldingShow {
    const URL_PATH: &'static str = "/holdings/{id}";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Get;
    const VERSION: &'static str = "3.0.0";

    type UrlDisplay = HoldingShowUrlDisplay<'a>;
    type Parameters = HoldingShowParameters;
    type Success = HoldingShowSuccess;

    fn url_path(parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        HoldingShowUrlDisplay(parameters)
    }
}

pub struct HoldingShowUrlDisplay<'a>(&'a HoldingShowParameters);

impl<'a> fmt::Display for HoldingShowUrlDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parameters = self.0;

        write!(f, "/holdings/{}", parameters.id)
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct HoldingShowParameters {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct HoldingShowSuccess {
    /// A holding object
    pub holding: HoldingShowHoldingSuccess,
    /// The current API Transaction.
    pub api_transaction: HoldingShowApiTransactionSuccess,
    pub links: HoldingShowLinksSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct HoldingShowHoldingSuccess {
    /// The unique id of this holding
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// The drp mode setting of this holding
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub drp_setting: String,
    /// The payout type for the holding
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub payout_type: String,
    /// True if foreign tax credits are supported
    pub foreign_tax_credits_supported: bool,
    /// True if the payments are classed as trust income for tax purposes. Only relevant to Australian local dividends.
    pub trust_income: bool,
    /// An instrument object for the Holding
    pub instrument: HoldingShowHoldingInstrumentSuccess,
    /// A currency object
    pub instrument_currency: HoldingShowHoldingInstrumentCurrencySuccess,
    /// A currency object
    pub payout_currency: HoldingShowHoldingPayoutCurrencySuccess,
    /// The Sharesight code for the instrument. This field is a deprecated alias for 'instrument.code', please use that instead.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub symbol: String,
    /// True if the holding has a valid position (e.g. positive number of shares)
    pub valid_position: bool,
    /// The portfolio associated with this model
    pub portfolio: HoldingShowHoldingPortfolioSuccess,
    /// DEPRECATED - please use attachments. A list of holding documents
    pub documents: Vec<HoldingShowHoldingDocumentsSuccess>,
    /// A list of holding attachments
    pub attachments: Vec<HoldingShowHoldingAttachmentsSuccess>,
    /// The date your holding was started on typically the date of the first trade
    #[serde_as(as = "DeserializeDate")]
    pub inception_date: NaiveDate,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct HoldingShowHoldingInstrumentSuccess {
    /// The Sharesight code for the instrument
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub code: String,
    /// The Sharesight country identifier associated with this instrument
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub country_id: i64,
    /// Whether or not this instrument is a cryptocurency
    pub crypto: bool,
    /// The Sharesight currency code associated with this instrument
    pub currency_code: Currency,
    /// The date the instrument will be or has expired on
    #[serde_as(as = "DeserializeDate")]
    pub expires_on: NaiveDate,
    /// Whether or not this instrument is currently expired
    pub expired: bool,
    /// The unique Sharesight identifier for this instrument
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// The code of the market the instrument is listed on
    pub market_code: Market,
    /// The name of the instrument
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub name: String,
    /// The timezone name associated with this instrument
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub tz_name: String,
    /// The instrument sector according to FactSet's global security classification scheme. FactSet's default classification can be overridden at the holding level.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub industry_classification_name: String,
    /// The instrument industry according to FactSet's global security classification scheme. FactSet's default classification can be overridden at the holding level.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub sector_classification_name: String,
    /// The type of investment. This determines holding functionality, e.g. Ordinary Shares or Fixed Interest
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub friendly_instrument_description: String,
    /// The type of investment (as a code). This determines holding functionality, e.g. ordinary_shares
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub friendly_instrument_description_code: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct HoldingShowHoldingInstrumentCurrencySuccess {
    /// The 3-letter ISO 4217 currency code
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub code: String,
    /// The unique Sharesight identifier for this currency
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// The symbol for this currency (eg. $, ¥, £)
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub symbol: String,
    /// The qualified currency symbol when the symbol is not specific enough, eg. 'US$', '¥', or the currency code
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub qualified_symbol: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct HoldingShowHoldingPayoutCurrencySuccess {
    /// The 3-letter ISO 4217 currency code
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub code: String,
    /// The unique Sharesight identifier for this currency
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// The symbol for this currency (eg. $, ¥, £)
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub symbol: String,
    /// The qualified currency symbol when the symbol is not specific enough, eg. 'US$', '¥', or the currency code
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub qualified_symbol: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct HoldingShowHoldingPortfolioSuccess {
    /// The unique id identifying the portfolio
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// Whether or not this is a consolidated view portfolio
    pub consolidated: bool,
    /// The name of the portfolio
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub name: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct HoldingShowHoldingDocumentsSuccess {
    /// The unique id of this document
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// The name of the document
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub file_name: String,
    /// The mime type of the document
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub content_type: String,
    /// The size of the document, in bytes
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub file_size: i64,
    /// Date the document was created on
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub created_at: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct HoldingShowHoldingAttachmentsSuccess {
    /// The ID of this attachment
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// The type of attachment, either 'DOCUMENT' or 'CONTRACT_NOTE'
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub r#type: String,
    /// The file name for the attachment, including extension
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub file_name: String,
    /// The size of the file
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub file_size: i64,
    /// The mimetype of the attached file
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub content_type: String,
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub created_at: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct HoldingShowApiTransactionSuccess {
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

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct HoldingShowLinksSuccess {
    /// URL to a list of requested resources.
    #[serde(rename = "self")]
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub itself: String,
}

/// Updates a holding; currently we're only enabling/disabling drp
pub struct HoldingUpdate;

impl<'a> ApiEndpoint<'a> for HoldingUpdate {
    const URL_PATH: &'static str = "/holdings/{id}";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Put;
    const VERSION: &'static str = "3.0.0";

    type UrlDisplay = HoldingUpdateUrlDisplay<'a>;
    type Parameters = HoldingUpdateParameters;
    type Success = HoldingUpdateSuccess;

    fn url_path(parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        HoldingUpdateUrlDisplay(parameters)
    }
}

pub struct HoldingUpdateUrlDisplay<'a>(&'a HoldingUpdateParameters);

impl<'a> fmt::Display for HoldingUpdateUrlDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parameters = self.0;

        write!(f, "/holdings/{}", parameters.id)
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct HoldingUpdateParameters {
    /// Set to true to enable drp. Set to false to disable drp.
    #[serde(default)]
    pub enable_drp: Option<bool>,
    /// The drp mode setting, can be up, down, half, down_track. Default: down_track
    #[serde(default)]
    pub drp_mode_setting: Option<String>,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct HoldingUpdateSuccess {
    /// A holding object
    pub holding: HoldingUpdateHoldingSuccess,
    /// The current API Transaction.
    pub api_transaction: HoldingUpdateApiTransactionSuccess,
    pub links: HoldingUpdateLinksSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct HoldingUpdateHoldingSuccess {
    /// The unique id of this holding
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// The drp mode setting.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub drp_mode_setting: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct HoldingUpdateApiTransactionSuccess {
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

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct HoldingUpdateLinksSuccess {
    /// URL to a list of requested resources.
    #[serde(rename = "self")]
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub itself: String,
}

/// Access Sharesight country defintions
pub struct CountryList;

impl<'a> ApiEndpoint<'a> for CountryList {
    const URL_PATH: &'static str = "/countries";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Get;
    const VERSION: &'static str = "3.0.0";

    type UrlDisplay = &'static str;
    type Parameters = CountryListParameters;
    type Success = CountryListSuccess;

    fn url_path(_parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        "/countries"
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct CountryListParameters {
    /// To filter based on the supported field, leave blank or don't include for all results.
    #[serde(default)]
    pub supported: Option<bool>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CountryListSuccess {
    /// List of country definitions.
    pub countries: Vec<CountryListCountriesSuccess>,
    /// The current API Transaction.
    pub api_transaction: CountryListApiTransactionSuccess,
    pub links: CountryListLinksSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CountryListCountriesSuccess {
    /// The ISO country code.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub code: String,
    /// Sharesight country identifier.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// Sharesight naming of this country – please use the ISO Code as some things are non-standard, eg. we have countries tied to old currencies and cyrptocurrencies like Latvia (pre-Euro) with a currency of 81 (XLV, being the pre-Euro Latvian Dollar)
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub name: String,
    /// Sharesight associated currency identifier.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub currency_id: i64,
    /// Abbreviated name of final month of financial year, eg. dec, jun, or mar.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub financial_year_end: String,
    /// Whether or not Sharesight fully supports this country.
    pub supported: bool,
    /// The primary timezone for this country.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub tz_name: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CountryListApiTransactionSuccess {
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

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CountryListLinksSuccess {
    /// URL to a list of requested resources.
    #[serde(rename = "self")]
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub itself: String,
}

/// Retrieves a list of a Portfolio's Holdings
pub struct HoldingPortfolioList;

impl<'a> ApiEndpoint<'a> for HoldingPortfolioList {
    const URL_PATH: &'static str = "/portfolios/{portfolio_id}/holdings";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Get;
    const VERSION: &'static str = "3.0.0";

    type UrlDisplay = HoldingPortfolioListUrlDisplay<'a>;
    type Parameters = HoldingPortfolioListParameters;
    type Success = HoldingPortfolioListSuccess;

    fn url_path(parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        HoldingPortfolioListUrlDisplay(parameters)
    }
}

pub struct HoldingPortfolioListUrlDisplay<'a>(&'a HoldingPortfolioListParameters);

impl<'a> fmt::Display for HoldingPortfolioListUrlDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parameters = self.0;

        write!(f, "/portfolios/{}/holdings", parameters.portfolio_id)
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct HoldingPortfolioListParameters {
    /// True if a consolidated view is requested<br>Default value: `false`
    #[serde(default)]
    pub consolidated: Option<bool>,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub portfolio_id: i64,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct HoldingPortfolioListSuccess {
    /// A list of Holdings
    pub holdings: Vec<HoldingPortfolioListHoldingsSuccess>,
    /// The current API Transaction.
    pub api_transaction: HoldingPortfolioListApiTransactionSuccess,
    pub links: HoldingPortfolioListLinksSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct HoldingPortfolioListHoldingsSuccess {
    /// The unique id of this holding
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// An instrument object for the Holding
    pub instrument: HoldingPortfolioListHoldingsInstrumentSuccess,
    /// A currency object
    pub instrument_currency: HoldingPortfolioListHoldingsInstrumentCurrencySuccess,
    /// The Sharesight code for the instrument. This field is a deprecated alias for 'instrument.code', please use that instead.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub symbol: String,
    /// True if the holding has a valid position (e.g. positive number of shares)
    pub valid_position: bool,
    /// The portfolio associated with this model
    pub portfolio: HoldingPortfolioListHoldingsPortfolioSuccess,
    /// The group ID this holding belongs to
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub group_id: i64,
    /// The group name this holding belongs to - coming from the grouping parameter
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub group_name: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct HoldingPortfolioListHoldingsInstrumentSuccess {
    /// The Sharesight code for the instrument
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub code: String,
    /// The Sharesight country identifier associated with this instrument
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub country_id: i64,
    /// Whether or not this instrument is a cryptocurency
    pub crypto: bool,
    /// The Sharesight currency code associated with this instrument
    pub currency_code: Currency,
    /// The date the instrument will be or has expired on
    #[serde_as(as = "DeserializeDate")]
    pub expires_on: NaiveDate,
    /// Whether or not this instrument is currently expired
    pub expired: bool,
    /// The unique Sharesight identifier for this instrument
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// The code of the market the instrument is listed on
    pub market_code: Market,
    /// The name of the instrument
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub name: String,
    /// The timezone name associated with this instrument
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub tz_name: String,
    /// The instrument sector according to FactSet's global security classification scheme. FactSet's default classification can be overridden at the holding level.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub industry_classification_name: String,
    /// The instrument industry according to FactSet's global security classification scheme. FactSet's default classification can be overridden at the holding level.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub sector_classification_name: String,
    /// The type of investment. This determines holding functionality, e.g. Ordinary Shares or Fixed Interest
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub friendly_instrument_description: String,
    /// The type of investment (as a code). This determines holding functionality, e.g. ordinary_shares
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub friendly_instrument_description_code: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct HoldingPortfolioListHoldingsInstrumentCurrencySuccess {
    /// The 3-letter ISO 4217 currency code
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub code: String,
    /// The unique Sharesight identifier for this currency
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// The symbol for this currency (eg. $, ¥, £)
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub symbol: String,
    /// The qualified currency symbol when the symbol is not specific enough, eg. 'US$', '¥', or the currency code
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub qualified_symbol: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct HoldingPortfolioListHoldingsPortfolioSuccess {
    /// The unique id identifying the portfolio
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// Whether or not this is a consolidated view portfolio
    pub consolidated: bool,
    /// The name of the portfolio
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub name: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct HoldingPortfolioListApiTransactionSuccess {
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

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct HoldingPortfolioListLinksSuccess {
    #[serde(rename = "self")]
    pub itself: HoldingPortfolioListLinksSelfSuccess,
    pub portfolio: HoldingPortfolioListLinksPortfolioSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct HoldingPortfolioListLinksSelfSuccess {
    /// URL to a list of requested resources.
    #[serde(rename = "self")]
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub itself: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct HoldingPortfolioListLinksPortfolioSuccess {
    /// URL to a portfolio.
    #[serde(rename = "self")]
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub itself: String,
}

/// Retrieves a portfolio by id
pub struct Portfolio;

impl<'a> ApiEndpoint<'a> for Portfolio {
    const URL_PATH: &'static str = "/portfolios/{portfolio_id}";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Get;
    const VERSION: &'static str = "3.0.0";

    type UrlDisplay = PortfolioUrlDisplay<'a>;
    type Parameters = PortfolioParameters;
    type Success = PortfolioSuccess;

    fn url_path(parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        PortfolioUrlDisplay(parameters)
    }
}

pub struct PortfolioUrlDisplay<'a>(&'a PortfolioParameters);

impl<'a> fmt::Display for PortfolioUrlDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parameters = self.0;

        write!(f, "/portfolios/{}", parameters.portfolio_id)
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct PortfolioParameters {
    /// Set to true if the referenced portfolio is consolidated<br>Default value: `false`
    #[serde(default)]
    pub consolidated: Option<bool>,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub portfolio_id: i64,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PortfolioSuccess {
    /// The portfolio associated with this model
    pub portfolio: PortfolioPortfolioSuccess,
    /// The current API Transaction.
    pub api_transaction: PortfolioApiTransactionSuccess,
    pub links: PortfolioLinksSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PortfolioPortfolioSuccess {
    /// The unique id identifying the portfolio
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// Whether or not this is a consolidated view portfolio
    pub consolidated: bool,
    /// The name of the portfolio
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub name: String,
    /// Typically used by professionals to identify the tax entity owner of the portfolio
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub external_identifier: String,
    /// The timezone name applicable to the portfolio country
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub tz_name: String,
    /// For an Australian portfolio, the default sale allocation method for capital gains purposes. One of: fifo, lifo, maximise_cr, minimise_cr, ss_minimise, average, default
    pub default_sale_allocation_method: SaleAllocationMethod,
    /// The CGT discount rate for Australian portfolios
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub cg_discount: String,
    /// The end of the financial year (MM-DD)
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub financial_year_end: String,
    /// The interest method: 'simple' or 'compound'
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub interest_method: String,
    /// The ISO country code of the (tax) country of this portfolio
    pub country_code: Country,
    /// The ISO currency code of this portfolio
    pub currency_code: Currency,
    /// The date your portfolio was started on or the oldest portfolio if a consolidated view
    #[serde_as(as = "DeserializeDate")]
    pub inception_date: NaiveDate,
    /// The current user's access level to this portfolio, one of: OWNER, STAFF, ADMIN, EDIT, READ
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub access_level: String,
    /// The unique identifier of the portfolio owner
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub user_id: i64,
    /// The name of the portfolio owner. A first and last name will be returned if available, otherwise the owners organisation name
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub owner_name: String,
    /// For NZ portfolios, the rate of resident witholding tax to be applied
    #[serde_as(as = "DeserializeNumber")]
    pub rwtr_rate: Number,
    /// For NZ portfolios, true if the owner is taxed as a trader
    pub trader: bool,
    /// If set, transactions such as company events are not automatically applied to the portfolio holdings
    pub disable_automatic_transactions: bool,
    /// For Canadian portfolios, the type of tax entity: non_registered, rrsp, rrif, tfsa
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub tax_entity_type: String,
    /// All buys and sells will generate a corresponding deposit/withdrawal in the selected 'trading' account
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub trade_sync_cash_account_id: i64,
    /// All payouts will generate a corresponding deposit in the selected 'payout' account
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub payout_sync_cash_account_id: i64,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PortfolioApiTransactionSuccess {
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

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PortfolioLinksSuccess {
    /// URL to a list of requested resources.
    #[serde(rename = "self")]
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub itself: String,
}

/// Retrieves the user settings for the portfolio and current user. These are persisted in the database such that a user will have these settings retained between views of a page on any session
pub struct UserSettingList;

impl<'a> ApiEndpoint<'a> for UserSettingList {
    const URL_PATH: &'static str = "/portfolios/{portfolio_id}/user_setting";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Get;
    const VERSION: &'static str = "3.0.0";

    type UrlDisplay = UserSettingListUrlDisplay<'a>;
    type Parameters = UserSettingListParameters;
    type Success = UserSettingListSuccess;

    fn url_path(parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        UserSettingListUrlDisplay(parameters)
    }
}

pub struct UserSettingListUrlDisplay<'a>(&'a UserSettingListParameters);

impl<'a> fmt::Display for UserSettingListUrlDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parameters = self.0;

        write!(f, "/portfolios/{}/user_setting", parameters.portfolio_id)
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct UserSettingListParameters {
    /// Set to true for consolidated portfolio views<br>Default value: `false`
    #[serde(default)]
    pub consolidated: Option<bool>,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub portfolio_id: i64,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct UserSettingListSuccess {
    pub portfolio_user_setting: UserSettingListPortfolioUserSettingSuccess,
    /// The current API Transaction.
    pub api_transaction: UserSettingListApiTransactionSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct UserSettingListPortfolioUserSettingSuccess {
    /// The chart type to use for the portfolio: VALUE, VALUELINE, GROWTH, BENCHMARK, HIDE
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub portfolio_chart: String,
    /// The chart type to use for holdings in the portfolio: PRICE, HOLDING_VALUE, BENCHMARK, HIDE
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub holding_chart: String,
    /// True to combine holdings in consolidated portfolios
    pub combined: bool,
    /// True to combine holdings in consolidated portfolios when showing reports
    pub report_combined: bool,
    /// Grouping to use, as a string or integer custom group id
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub grouping: String,
    /// Grouping to use for reports
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub report_grouping: String,
    /// Currency to use for reports
    pub report_currency: Currency,
    /// True to include sold shares in calculations
    pub include_sold_shares: bool,
    /// True to include sold shares in reports
    pub report_include_sold_shares: bool,
    /// Instrument ID for benchmark
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub benchmark_instrument_id: String,
    /// True to show comments on taxable income report
    pub taxable_show_comments: bool,
    /// True to show holding totals on taxable income report
    pub taxable_grouped_by_holding: bool,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct UserSettingListApiTransactionSuccess {
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

/// Updates the user settings for the portfolio and current user. These are persisted in the database such that a user will have these settings retained between views of a page on any session.
pub struct UserSettingUpdate;

impl<'a> ApiEndpoint<'a> for UserSettingUpdate {
    const URL_PATH: &'static str = "/portfolios/{portfolio_id}/user_setting";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Patch;
    const VERSION: &'static str = "3.0.0";

    type UrlDisplay = UserSettingUpdateUrlDisplay<'a>;
    type Parameters = UserSettingUpdateParameters;
    type Success = UserSettingUpdateSuccess;

    fn url_path(parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        UserSettingUpdateUrlDisplay(parameters)
    }
}

pub struct UserSettingUpdateUrlDisplay<'a>(&'a UserSettingUpdateParameters);

impl<'a> fmt::Display for UserSettingUpdateUrlDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parameters = self.0;

        write!(f, "/portfolios/{}/user_setting", parameters.portfolio_id)
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct UserSettingUpdateParameters {
    /// Set to true for consolidated portfolio views<br>Default value: `false`
    #[serde(default)]
    pub consolidated: Option<bool>,
    #[serde(default)]
    pub portfolio_user_settings: Option<UserSettingUpdatePortfolioUserSettingsParameters>,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub portfolio_id: i64,
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct UserSettingUpdatePortfolioUserSettingsParameters {
    /// The chart type to use for the portfolio: VALUE, VALUELINE, GROWTH, BENCHMARK, HIDE
    #[serde(default)]
    pub portfolio_chart: Option<String>,
    /// The chart type to use for holdings in the portfolio: PRICE, HOLDING_VALUE, BENCHMARK, HIDE
    #[serde(default)]
    pub holding_chart: Option<String>,
    /// True to combine holdings in consolidated portfolios
    #[serde(default)]
    pub combined: Option<bool>,
    /// True to combine holdings in consolidated portfolios when showing reports
    #[serde(default)]
    pub report_combined: Option<bool>,
    /// Grouping to use, as a string or integer custom group id
    #[serde(default)]
    pub grouping: Option<String>,
    /// Grouping to use for reports
    #[serde(default)]
    pub report_grouping: Option<String>,
    /// Currency to use for reports
    #[serde(default)]
    pub report_currency: Option<Currency>,
    /// True to include sold shares in calculations
    #[serde(default)]
    pub include_sold_shares: Option<bool>,
    /// True to include sold shares in reports
    #[serde(default)]
    pub report_include_sold_shares: Option<bool>,
    /// Instrument ID for benchmark
    #[serde(default)]
    pub benchmark_instrument_id: Option<String>,
    /// True to show comments on taxable income report
    #[serde(default)]
    pub taxable_show_comments: Option<bool>,
    /// True to show holding totals on taxable income report
    #[serde(default)]
    pub taxable_grouped_by_holding: Option<bool>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct UserSettingUpdateSuccess {
    pub portfolio_user_setting: UserSettingUpdatePortfolioUserSettingSuccess,
    /// The current API Transaction.
    pub api_transaction: UserSettingUpdateApiTransactionSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct UserSettingUpdatePortfolioUserSettingSuccess {
    /// The chart type to use for the portfolio: VALUE, VALUELINE, GROWTH, BENCHMARK, HIDE
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub portfolio_chart: String,
    /// The chart type to use for holdings in the portfolio: PRICE, HOLDING_VALUE, BENCHMARK, HIDE
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub holding_chart: String,
    /// True to combine holdings in consolidated portfolios
    pub combined: bool,
    /// True to combine holdings in consolidated portfolios when showing reports
    pub report_combined: bool,
    /// Grouping to use, as a string or integer custom group id
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub grouping: String,
    /// Grouping to use for reports
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub report_grouping: String,
    /// Currency to use for reports
    pub report_currency: Currency,
    /// True to include sold shares in calculations
    pub include_sold_shares: bool,
    /// True to include sold shares in reports
    pub report_include_sold_shares: bool,
    /// Instrument ID for benchmark
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub benchmark_instrument_id: String,
    /// True to show comments on taxable income report
    pub taxable_show_comments: bool,
    /// True to show holding totals on taxable income report
    pub taxable_grouped_by_holding: bool,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct UserSettingUpdateApiTransactionSuccess {
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

/// Retrieves the Performance Report for the underlying portfolio<br><h3>Holding Limit</h3>Depending on user's plan the number of holdings shown in the report is limited. In that case the following response headers will be set:<br><li>`X-HoldingLimit-Limit`: The plan's holding limit.</li><li>`X-HoldingLimit-Total`: The number of holding in the requested portfolio.</li><li>`X-HoldingLimit-Reason`: A human-readable string describing why the number of holdings is limited.</li><h3>Remarks</h3>• Infinity is represented by string values 'Infinity' or '-Infinity'
pub struct PerformanceShow;

impl<'a> ApiEndpoint<'a> for PerformanceShow {
    const URL_PATH: &'static str = "/portfolios/{portfolio_id}/performance";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Get;
    const VERSION: &'static str = "3.0.0";

    type UrlDisplay = PerformanceShowUrlDisplay<'a>;
    type Parameters = PerformanceShowParameters;
    type Success = PerformanceShowSuccess;

    fn url_path(parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        PerformanceShowUrlDisplay(parameters)
    }
}

pub struct PerformanceShowUrlDisplay<'a>(&'a PerformanceShowParameters);

impl<'a> fmt::Display for PerformanceShowUrlDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parameters = self.0;

        write!(f, "/portfolios/{}/performance", parameters.portfolio_id)
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct PerformanceShowParameters {
    /// Show report from this date on (YYYY-MM-DD). In timezone specified by portfolio_tz_name<br>Default value: `portfolio`
    #[serde_as(as = "Option<DeserializeDate>")]
    #[serde(default)]
    pub start_date: Option<NaiveDate>,
    /// Show report until this date (YYYY-MM-DD). In timezone specified by portfolio_tz_name<br>Default value: `today`
    #[serde_as(as = "Option<DeserializeDate>")]
    #[serde(default)]
    pub end_date: Option<NaiveDate>,
    /// The Portfolio id
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub portfolio_id: i64,
    /// Set to true for consolidated portfolio views<br>Default value: `false`
    #[serde(default)]
    pub consolidated: Option<bool>,
    /// pass `true` to include or `false` to exclude sales<br>Default value: `false`
    #[serde(default)]
    pub include_sales: Option<bool>,
    /// To receive totals from holdings combined by instrument in addition to grouping.  This is only applicable when you have multiple holdings for the same instrument (a consolidated portfolio)<br>Default value: `false`
    #[serde(default)]
    pub report_combined: Option<bool>,
    /// An array of labels (by name) to filter on.  You can request multiple by passing multiple unindexed params, eg: `?labels[]=aud&amp;labels[]=usd`
    #[serde(default)]
    pub labels: Option<Vec<String>>,
    /// Group instruments by an attribute.  Valid values: [country, currency, custom_group, industry_classification, investment_type, market, portfolio, sector_classification, ungrouped]<br>Default value: `market`
    #[serde(default)]
    pub grouping: Option<String>,
    /// If present, the custom group id to group by, as an integer id returned from the CustomGroupsList endpoint. When this is used, the 'grouping' parameter must be set to 'custom_group'
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub custom_group_id: Option<i64>,
    /// Pass `true` to include holdings limited by the user plan in the results. Only identifiying data will be returned for these rows<br>Default value: `false`
    #[serde(default)]
    pub include_limited: Option<bool>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PerformanceShowSuccess {
    pub report: PerformanceShowReportSuccess,
    /// The current API Transaction.
    pub api_transaction: PerformanceShowApiTransactionSuccess,
    pub links: PerformanceShowLinksSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PerformanceShowReportSuccess {
    /// A unique id identifying this report instance
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub id: String,
    /// The portfolio id
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub portfolio_id: i64,
    /// The portfolio timezone name
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub portfolio_tz_name: String,
    /// The total value of the portfolio
    #[serde_as(as = "DeserializeNumber")]
    pub value: Number,
    /// The grouping being used for this report – coming from the grouping parameter
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub grouping: String,
    /// A currency object
    pub currency: PerformanceShowReportCurrencySuccess,
    /// Start date (format `YYYY-MM-DD`)
    #[serde_as(as = "DeserializeDate")]
    pub start_date: NaiveDate,
    /// End date (format `YYYY-MM-DD`)
    #[serde_as(as = "DeserializeDate")]
    pub end_date: NaiveDate,
    /// Whether or not sales are included
    pub include_sales: bool,
    /// Capital Gain (rounded to 2 decimal places)
    #[serde_as(as = "DeserializeNumber")]
    pub capital_gain: Number,
    /// Capital Gain Percentage (rounded to 2 decimal places, 33% as `33.00`)
    #[serde_as(as = "DeserializeNumber")]
    pub capital_gain_percent: Number,
    /// Payout Gain (rounded to 2 decimal places)
    #[serde_as(as = "DeserializeNumber")]
    pub payout_gain: Number,
    /// Payout Gain Percentage (rounded to 2 decimal places, 33% as `33.00`)
    #[serde_as(as = "DeserializeNumber")]
    pub payout_gain_percent: Number,
    /// Currency Gain (rounded to 2 decimal places)
    #[serde_as(as = "DeserializeNumber")]
    pub currency_gain: Number,
    /// Currency Gain Percentage (rounded to 2 decimal places, 33% as `33.00`)
    #[serde_as(as = "DeserializeNumber")]
    pub currency_gain_percent: Number,
    /// Total Gain (rounded to 2 decimal places)
    #[serde_as(as = "DeserializeNumber")]
    pub total_gain: Number,
    /// Total Gain Percentage (rounded to 2 decimal places, 33% as `33.00`)
    #[serde_as(as = "DeserializeNumber")]
    pub total_gain_percent: Number,
    /// True when percentages are per annum
    pub percentages_annualised: bool,
    /// List of Holdings
    pub holdings: Vec<PerformanceShowReportHoldingsSuccess>,
    /// List of sub-totals for each group
    pub sub_totals: Vec<PerformanceShowReportSubTotalsSuccess>,
    /// When you have multiple holdings of the same instruments and pass `report_combined=true`, this returns combined stats for those instruments. You may need to combine this with the holdings collection to get a full picture
    #[serde(default)]
    pub combined_holdings: Option<Vec<PerformanceShowReportCombinedHoldingsSuccess>>,
    pub cash_accounts: Vec<PerformanceShowReportCashAccountsSuccess>,
    /// The custom group used for this report; not included if custom grouping is not used
    #[serde(default)]
    pub custom_group: Option<PerformanceShowReportCustomGroupSuccess>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PerformanceShowReportCurrencySuccess {
    /// The 3-letter ISO 4217 currency code
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub code: String,
    /// The unique Sharesight identifier for this currency
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// The symbol for this currency (eg. $, ¥, £)
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub symbol: String,
    /// The qualified currency symbol when the symbol is not specific enough, eg. 'US$', '¥', or the currency code
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub qualified_symbol: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PerformanceShowReportHoldingsSuccess {
    /// The unique id of this holding
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// An instrument object for the Holding
    pub instrument: PerformanceShowReportHoldingsInstrumentSuccess,
    /// A currency object
    pub instrument_currency: PerformanceShowReportHoldingsInstrumentCurrencySuccess,
    /// True if this holding will not be shown due to subscription restrictions
    #[serde(default)]
    pub limited: Option<bool>,
    /// True if the holding has a valid position (e.g. positive number of shares)
    pub valid_position: bool,
    /// This price is in the instrument currency, except for when instrument.market_code equals 'FX' (a forex instrument), then the price is in the Portfolio Currency
    #[serde_as(as = "DeserializeNumber")]
    pub instrument_price: Number,
    /// The portfolio associated with this model
    pub portfolio: PerformanceShowReportHoldingsPortfolioSuccess,
    /// Labels assigned to this holding
    pub labels: Vec<PerformanceShowReportHoldingsLabelsSuccess>,
    /// The group id this holding belongs to
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub group_id: i64,
    /// The group name this holding belongs to – coming from the the grouping parameter
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub group_name: String,
    /// Capital Gain (rounded to 2 decimal places)
    #[serde_as(as = "DeserializeNumber")]
    pub capital_gain: Number,
    /// Capital Gain Percentage (rounded to 2 decimal places, 33% as `33.00`)
    #[serde_as(as = "DeserializeNumber")]
    pub capital_gain_percent: Number,
    /// Payout Gain (rounded to 2 decimal places)
    #[serde_as(as = "DeserializeNumber")]
    pub payout_gain: Number,
    /// Payout Gain Percentage (rounded to 2 decimal places, 33% as `33.00`)
    #[serde_as(as = "DeserializeNumber")]
    pub payout_gain_percent: Number,
    /// Currency Gain (rounded to 2 decimal places)
    #[serde_as(as = "DeserializeNumber")]
    pub currency_gain: Number,
    /// Currency Gain Percentage (rounded to 2 decimal places, 33% as `33.00`)
    #[serde_as(as = "DeserializeNumber")]
    pub currency_gain_percent: Number,
    /// Total Gain (rounded to 2 decimal places)
    #[serde_as(as = "DeserializeNumber")]
    pub total_gain: Number,
    /// Total Gain Percentage (rounded to 2 decimal places, 33% as `33.00`)
    #[serde_as(as = "DeserializeNumber")]
    pub total_gain_percent: Number,
    /// The number of unconfirmed payouts and trades
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub number_of_unconfirmed_transactions: i64,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PerformanceShowReportHoldingsInstrumentSuccess {
    /// The Sharesight code for the instrument
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub code: String,
    /// The Sharesight country identifier associated with this instrument
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub country_id: i64,
    /// Whether or not this instrument is a cryptocurency
    pub crypto: bool,
    /// The Sharesight currency code associated with this instrument
    pub currency_code: Currency,
    /// The date the instrument will be or has expired on
    #[serde_as(as = "Option<DeserializeDate>")]
    #[serde(default)]
    pub expires_on: Option<NaiveDate>,
    /// Whether or not this instrument is currently expired
    #[serde(default)]
    pub expired: Option<bool>,
    /// The unique Sharesight identifier for this instrument
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// The code of the market the instrument is listed on
    pub market_code: Market,
    /// The name of the instrument
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub name: String,
    /// The timezone name associated with this instrument
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub tz_name: String,
    /// The instrument sector according to FactSet's global security classification scheme. FactSet's default classification can be overridden at the holding level.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub industry_classification_name: String,
    /// The instrument industry according to FactSet's global security classification scheme. FactSet's default classification can be overridden at the holding level.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub sector_classification_name: String,
    /// The type of investment. This determines holding functionality, e.g. Ordinary Shares or Fixed Interest
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub friendly_instrument_description: String,
    /// The type of investment (as a code). This determines holding functionality, e.g. ordinary_shares
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub friendly_instrument_description_code: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PerformanceShowReportHoldingsInstrumentCurrencySuccess {
    /// The 3-letter ISO 4217 currency code
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub code: String,
    /// The unique Sharesight identifier for this currency
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// The symbol for this currency (eg. $, ¥, £)
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub symbol: String,
    /// The qualified currency symbol when the symbol is not specific enough, eg. 'US$', '¥', or the currency code
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub qualified_symbol: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PerformanceShowReportHoldingsPortfolioSuccess {
    /// The unique id identifying the portfolio
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// Whether or not this is a consolidated view portfolio
    pub consolidated: bool,
    /// The name of the portfolio
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub name: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PerformanceShowReportHoldingsLabelsSuccess {
    /// The unique id of the label
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// The name of the label – used in most API queries
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub name: String,
    /// The label color (to be used in HTML).  One of 14 assigned during creation: BlueViolet, MediumSeaGreen, Gold, Cyan, Crimson, DarkCyan, OrangeRed, LightGreen, MediumVioletRed, Indigo, SkyBlue, Khaki, LawnGreen
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub color: String,
    /// An array of holding ids assigned to this label
    pub holding_ids: Vec<i64>,
    /// An array of portfolio ids assigned to this label
    pub portfolio_ids: Vec<i64>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PerformanceShowReportSubTotalsSuccess {
    /// The total value of the holdings in this group
    #[serde_as(as = "DeserializeNumber")]
    pub value: Number,
    /// The group id this total belongs to
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub group_id: i64,
    /// The group name this total belongs to – coming from the the grouping parameter
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub group_name: String,
    /// Capital Gain (rounded to 2 decimal places)
    #[serde_as(as = "DeserializeNumber")]
    pub capital_gain: Number,
    /// Capital Gain Percentage (rounded to 2 decimal places, 33% as `33.00`)
    #[serde_as(as = "DeserializeNumber")]
    pub capital_gain_percent: Number,
    /// Payout Gain (rounded to 2 decimal places)
    #[serde_as(as = "DeserializeNumber")]
    pub payout_gain: Number,
    /// Payout Gain Percentage (rounded to 2 decimal places, 33% as `33.00`)
    #[serde_as(as = "DeserializeNumber")]
    pub payout_gain_percent: Number,
    /// Currency Gain (rounded to 2 decimal places)
    #[serde_as(as = "DeserializeNumber")]
    pub currency_gain: Number,
    /// Currency Gain Percentage (rounded to 2 decimal places, 33% as `33.00`)
    #[serde_as(as = "DeserializeNumber")]
    pub currency_gain_percent: Number,
    /// Total Gain (rounded to 2 decimal places)
    #[serde_as(as = "DeserializeNumber")]
    pub total_gain: Number,
    /// Total Gain Percentage (rounded to 2 decimal places, 33% as `33.00`)
    #[serde_as(as = "DeserializeNumber")]
    pub total_gain_percent: Number,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PerformanceShowReportCombinedHoldingsSuccess {
    /// The unique id of this holding
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// An instrument object for the Holding
    pub instrument: PerformanceShowReportCombinedHoldingsInstrumentSuccess,
    /// A currency object
    pub instrument_currency: PerformanceShowReportCombinedHoldingsInstrumentCurrencySuccess,
    /// true if this holding will not be shown due to subscription restrictions
    #[serde(default)]
    pub limited: Option<String>,
    /// True if the holding has a valid position (e.g. positive number of shares)
    pub valid_position: bool,
    /// This is in the instrument currency, except for when instrument.market_code equals 'FX' (a forex instrument), then the price is in the Portfolio Currency
    #[serde_as(as = "DeserializeNumber")]
    pub instrument_price: Number,
    /// List of Portfolios associated with this model
    pub portfolios: Vec<PerformanceShowReportCombinedHoldingsPortfoliosSuccess>,
    /// The group id this holding belings to
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub group_id: i64,
    /// The group name this holding belongs to - coming from the grouping parameter
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub group_name: String,
    /// Capital Gain (rounded to 2 decimal places)
    #[serde_as(as = "DeserializeNumber")]
    pub capital_gain: Number,
    /// Capital Gain Percentage (rounded to 2 decimal places, 33% as `33.00`)
    #[serde_as(as = "DeserializeNumber")]
    pub capital_gain_percent: Number,
    /// Payout Gain (rounded to 2 decimal places)
    #[serde_as(as = "DeserializeNumber")]
    pub payout_gain: Number,
    /// Payout Gain Percentage (rounded to 2 decimal places, 33% as `33.00`)
    #[serde_as(as = "DeserializeNumber")]
    pub payout_gain_percentage: Number,
    /// Currency Gain (rounded to 2 decimal places)
    #[serde_as(as = "DeserializeNumber")]
    pub currency_gain: Number,
    /// Currency Gain Percentage (rounded to 2 decimal places, 33% as `33.00`)
    #[serde_as(as = "DeserializeNumber")]
    pub currency_gain_percentage: Number,
    /// Total Gain (rounded to 2 decimal places
    #[serde_as(as = "DeserializeNumber")]
    pub total_gain: Number,
    /// Total Gain Percentage (rounded to 2 decimal places, 33% as `33.00`)
    #[serde_as(as = "DeserializeNumber")]
    pub total_gain_percentage: Number,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PerformanceShowReportCombinedHoldingsInstrumentSuccess {
    /// The Sharesight code for the instrument
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub code: String,
    /// The Sharesight country identifier associated with this instrument
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub country_id: i64,
    /// Whether or not this instrument is a cryptocurency
    pub crypto: bool,
    /// The Sharesight currency code associated with this instrument
    pub currency_code: Currency,
    /// The date the instrument will be or has expired on
    #[serde_as(as = "DeserializeDate")]
    pub expires_on: NaiveDate,
    /// Whether or not this instrument is currently expired
    pub expired: bool,
    /// The unique Sharesight identifier for this instrument
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// The code of the market the instrument is listed on
    pub market_code: Market,
    /// The name of the instrument
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub name: String,
    /// The timezone name associated with this instrument
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub tz_name: String,
    /// The instrument sector according to FactSet's global security classification scheme. FactSet's default classification can be overridden at the holding level.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub industry_classification_name: String,
    /// The instrument industry according to FactSet's global security classification scheme. FactSet's default classification can be overridden at the holding level.
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub sector_classification_name: String,
    /// The type of investment. This determines holding functionality, e.g. Ordinary Shares or Fixed Interest
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub friendly_instrument_description: String,
    /// The type of investment (as a code). This determines holding functionality, e.g. ordinary_shares
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub friendly_instrument_description_code: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PerformanceShowReportCombinedHoldingsInstrumentCurrencySuccess {
    /// The 3-letter ISO 4217 currency code
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub code: String,
    /// The unique Sharesight identifier for this currency
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// The symbol for this currency (eg. $, ¥, £)
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub symbol: String,
    /// The qualified currency symbol when the symbol is not specific enough, eg. 'US$', '¥', or the currency code
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub qualified_symbol: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PerformanceShowReportCombinedHoldingsPortfoliosSuccess {
    /// The unique id identifying the portfolio
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// Whether or not this is a consolidated view portfolio
    pub consolidated: bool,
    /// The name of the portfolio
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub name: String,
    /// The id of the holding in this portfolio
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub holding_id: i64,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PerformanceShowReportCashAccountsSuccess {
    /// Id of this cash account
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// A unique key for each cash account
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub key: i64,
    /// The name of the cash account
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub name: String,
    /// The source provider of the cash account. This is `null` for manual cash accounts, but may be `'xero'`, `'macquarie'`, etc
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub source: String,
    /// The value of the cash account
    #[serde_as(as = "DeserializeNumber")]
    pub value: Number,
    /// A currency object
    pub currency: PerformanceShowReportCashAccountsCurrencySuccess,
    /// The portfolio associated with this model
    pub portfolio: PerformanceShowReportCashAccountsPortfolioSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PerformanceShowReportCashAccountsCurrencySuccess {
    /// The 3-letter ISO 4217 currency code
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub code: String,
    /// The unique Sharesight identifier for this currency
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// The symbol for this currency (eg. $, ¥, £)
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub symbol: String,
    /// The qualified currency symbol when the symbol is not specific enough, eg. 'US$', '¥', or the currency code
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub qualified_symbol: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PerformanceShowReportCashAccountsPortfolioSuccess {
    /// The unique id identifying the portfolio
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// Whether or not this is a consolidated view portfolio
    pub consolidated: bool,
    /// The name of the portfolio
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub name: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PerformanceShowReportCustomGroupSuccess {
    /// The unique id of the custom group
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub id: i64,
    /// The name of the custom group
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub name: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PerformanceShowApiTransactionSuccess {
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

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PerformanceShowLinksSuccess {
    /// URL to a list of requested resources.
    #[serde(rename = "self")]
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub itself: String,
}

/// Disconnect a users API access. This operation invalidates the OAuth token for a user. We recommend you make this API call when a customer chooses to remove their connection via your system.
pub struct Revoke;

impl<'a> ApiEndpoint<'a> for Revoke {
    const URL_PATH: &'static str = "/oauth/revoke";
    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::Post;
    const VERSION: &'static str = "3.0.0";

    type UrlDisplay = &'static str;
    type Parameters = RevokeParameters;
    type Success = RevokeSuccess;

    fn url_path(_parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        "/oauth/revoke"
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct RevokeParameters {
    /// The client application ID
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub client_id: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct RevokeSuccess {
    /// The current API Transaction.
    pub api_transaction: RevokeApiTransactionSuccess,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct RevokeApiTransactionSuccess {
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
