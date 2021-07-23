use crate::types_prelude::*;

pub struct CashAccountCreate;

impl<'a> ApiEndpoint<'a> for CashAccountCreate {
    const URL_PATH: &'static str = "/portfolios/:portfolio_id/cash_accounts.json";

    type UrlDisplay = CashAccountCreateUrlDisplay<'a>;
    type Parameters = CashAccountCreateParameters;

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

#[derive(Debug, Clone, Serialize)]
pub struct CashAccountCreateParameters {
    pub portfolio_id: i64,
    pub cash_account: CashAccountCreateCashAccountParameters,
}

#[derive(Debug, Clone, Serialize)]
pub struct CashAccountCreateCashAccountParameters {
    pub name: String,
    pub currency: String,
}

pub struct CashAccountDelete;

impl<'a> ApiEndpoint<'a> for CashAccountDelete {
    const URL_PATH: &'static str = "/cash_accounts/:id.json";

    type UrlDisplay = CashAccountDeleteUrlDisplay<'a>;
    type Parameters = CashAccountDeleteParameters;

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

#[derive(Debug, Clone, Serialize)]
pub struct CashAccountDeleteParameters {
    pub id: i64,
}

pub struct CashAccountShow;

impl<'a> ApiEndpoint<'a> for CashAccountShow {
    const URL_PATH: &'static str = "/cash_accounts/:id.json";

    type UrlDisplay = CashAccountShowUrlDisplay<'a>;
    type Parameters = CashAccountShowParameters;

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

#[derive(Debug, Clone, Serialize)]
pub struct CashAccountShowParameters {
    pub id: i64,
    pub date: Option<NaiveDate>,
}

pub struct CashAccountTransactionCreate;

impl<'a> ApiEndpoint<'a> for CashAccountTransactionCreate {
    const URL_PATH: &'static str = "/cash_accounts/:cash_account_id/cash_account_transactions.json";

    type UrlDisplay = CashAccountTransactionCreateUrlDisplay<'a>;
    type Parameters = CashAccountTransactionCreateParameters;

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

#[derive(Debug, Clone, Serialize)]
pub struct CashAccountTransactionCreateParameters {
    pub cash_account_id: i64,
    pub description: String,
    pub amount: f64,
    pub type_name: String,
    pub date_time: NaiveDateTime,
    pub foreign_identifier: Option<String>,
}

pub struct CashAccountTransactionDelete;

impl<'a> ApiEndpoint<'a> for CashAccountTransactionDelete {
    const URL_PATH: &'static str = "/cash_account_transactions/:id.json";

    type UrlDisplay = CashAccountTransactionDeleteUrlDisplay<'a>;
    type Parameters = CashAccountTransactionDeleteParameters;

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

#[derive(Debug, Clone, Serialize)]
pub struct CashAccountTransactionDeleteParameters {
    pub id: i64,
}

pub struct CashAccountTransactionUpdate;

impl<'a> ApiEndpoint<'a> for CashAccountTransactionUpdate {
    const URL_PATH: &'static str = "/cash_account_transactions/:id.json";

    type UrlDisplay = CashAccountTransactionUpdateUrlDisplay<'a>;
    type Parameters = CashAccountTransactionUpdateParameters;

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

#[derive(Debug, Clone, Serialize)]
pub struct CashAccountTransactionUpdateParameters {
    pub id: i64,
    pub description: String,
    pub amount: f64,
    pub type_name: String,
    pub date_time: NaiveDateTime,
    pub foreign_identifier: Option<String>,
}

pub struct CashAccountTransactionsList;

impl<'a> ApiEndpoint<'a> for CashAccountTransactionsList {
    const URL_PATH: &'static str = "/cash_accounts/:cash_account_id/cash_account_transactions.json";

    type UrlDisplay = CashAccountTransactionsListUrlDisplay<'a>;
    type Parameters = CashAccountTransactionsListParameters;

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

#[derive(Debug, Clone, Serialize)]
pub struct CashAccountTransactionsListParameters {
    pub cash_account_id: i64,
    pub from: Option<NaiveDate>,
    pub to: Option<NaiveDate>,
    pub description: Option<String>,
    pub foreign_identifier: Option<String>,
}

pub struct CashAccountUpdate;

impl<'a> ApiEndpoint<'a> for CashAccountUpdate {
    const URL_PATH: &'static str = "/cash_accounts/:id.json";

    type UrlDisplay = CashAccountUpdateUrlDisplay<'a>;
    type Parameters = CashAccountUpdateParameters;

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

#[derive(Debug, Clone, Serialize)]
pub struct CashAccountUpdateParameters {
    pub id: i64,
    pub name: String,
    pub currency: String,
}

pub struct CashAccountsList;

impl<'a> ApiEndpoint<'a> for CashAccountsList {
    const URL_PATH: &'static str = "/cash_accounts.json";

    type UrlDisplay = &'static str;
    type Parameters = CashAccountsListParameters;

    fn url_path(_parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        "/cash_accounts.json"
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct CashAccountsListParameters {
    pub date: Option<NaiveDate>,
}

pub struct DocumentShow;

impl<'a> ApiEndpoint<'a> for DocumentShow {
    const URL_PATH: &'static str = "/documents/:id.json";

    type UrlDisplay = DocumentShowUrlDisplay<'a>;
    type Parameters = DocumentShowParameters;

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

#[derive(Debug, Clone, Serialize)]
pub struct DocumentShowParameters {
    pub id: i64,
}

pub struct HoldingMergesCreate;

impl<'a> ApiEndpoint<'a> for HoldingMergesCreate {
    const URL_PATH: &'static str = "/portfolios/:portfolio_id/holding_merges.json";

    type UrlDisplay = HoldingMergesCreateUrlDisplay<'a>;
    type Parameters = HoldingMergesCreateParameters;

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

#[derive(Debug, Clone, Serialize)]
pub struct HoldingMergesCreateParameters {
    pub portfolio_id: i64,
    pub holding_id: i64,
    pub merge_date: NaiveDate,
    pub quantity: f64,
    pub symbol: String,
    pub market: String,
    pub cancelled_price: Option<f64>,
    pub comments: Option<String>,
    pub unique_identifier: Option<String>,
    pub attachment: Option<String>,
    pub attachment_filename: Option<String>,
}

pub struct HoldingMergesUpdate;

impl<'a> ApiEndpoint<'a> for HoldingMergesUpdate {
    const URL_PATH: &'static str = "/portfolios/:portfolio_id/holding_merges/:id.json";

    type UrlDisplay = HoldingMergesUpdateUrlDisplay<'a>;
    type Parameters = HoldingMergesUpdateParameters;

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

#[derive(Debug, Clone, Serialize)]
pub struct HoldingMergesUpdateParameters {
    pub portfolio_id: i64,
    pub id: i64,
    pub merge_date: Option<NaiveDate>,
    pub quantity: Option<f64>,
    pub symbol: Option<String>,
    pub market: Option<String>,
    pub cancelled_price: Option<f64>,
    pub comments: Option<String>,
    pub unique_identifier: Option<String>,
    pub attachment: Option<String>,
    pub attachment_filename: Option<String>,
}

pub struct HoldingTrades;

impl<'a> ApiEndpoint<'a> for HoldingTrades {
    const URL_PATH: &'static str = "/holdings/:holding_id/trades.json";

    type UrlDisplay = HoldingTradesUrlDisplay<'a>;
    type Parameters = HoldingTradesParameters;

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

#[derive(Debug, Clone, Serialize)]
pub struct HoldingTradesParameters {
    pub holding_id: String,
    pub unique_identifier: Option<String>,
}

pub struct HoldingTradesRejected;

impl<'a> ApiEndpoint<'a> for HoldingTradesRejected {
    const URL_PATH: &'static str = "/holdings/:holding_id/rejected_trades.json";

    type UrlDisplay = HoldingTradesRejectedUrlDisplay<'a>;
    type Parameters = HoldingTradesRejectedParameters;

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

#[derive(Debug, Clone, Serialize)]
pub struct HoldingTradesRejectedParameters {
    pub holding_id: String,
}

pub struct IdentityByToken;

impl<'a> ApiEndpoint<'a> for IdentityByToken {
    const URL_PATH: &'static str = ".1-mobile/identity/by_token.json";

    type UrlDisplay = &'static str;
    type Parameters = IdentityByTokenParameters;

    fn url_path(_parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        "/.1-mobile/identity/by_token.json"
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct IdentityByTokenParameters {
    pub id_token: String,
    pub client_id: String,
}

pub struct IdentitySignupByToken;

impl<'a> ApiEndpoint<'a> for IdentitySignupByToken {
    const URL_PATH: &'static str = ".1-mobile/identity/signup_by_token.json";

    type UrlDisplay = &'static str;
    type Parameters = IdentitySignupByTokenParameters;

    fn url_path(_parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        "/.1-mobile/identity/signup_by_token.json"
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct IdentitySignupByTokenParameters {
    pub id_token: String,
    pub client_id: String,
    pub country_code: String,
}

pub struct MembershipCreate;

impl<'a> ApiEndpoint<'a> for MembershipCreate {
    const URL_PATH: &'static str = "/memberships.json";

    type UrlDisplay = &'static str;
    type Parameters = MembershipCreateParameters;

    fn url_path(_parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        "/memberships.json"
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct MembershipCreateParameters {
    pub membership: MembershipCreateMembershipParameters,
    pub user: Option<MembershipCreateUserParameters>,
    pub invitation: MembershipCreateInvitationParameters,
}

#[derive(Debug, Clone, Serialize)]
pub struct MembershipCreateMembershipParameters {
    pub portfolio_id: i64,
    pub access_code: String,
    pub user_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct MembershipCreateUserParameters {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct MembershipCreateInvitationParameters {
    pub text: Option<String>,
    pub no_email: Option<bool>,
}

pub struct MembershipDelete;

impl<'a> ApiEndpoint<'a> for MembershipDelete {
    const URL_PATH: &'static str = "/memberships/:id.json";

    type UrlDisplay = MembershipDeleteUrlDisplay<'a>;
    type Parameters = MembershipDeleteParameters;

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

#[derive(Debug, Clone, Serialize)]
pub struct MembershipDeleteParameters {
    pub id: i64,
}

pub struct MembershipUpdate;

impl<'a> ApiEndpoint<'a> for MembershipUpdate {
    const URL_PATH: &'static str = "/memberships/:id.json";

    type UrlDisplay = MembershipUpdateUrlDisplay<'a>;
    type Parameters = MembershipUpdateParameters;

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

#[derive(Debug, Clone, Serialize)]
pub struct MembershipUpdateParameters {
    pub id: i64,
    pub access_code: String,
}

pub struct ListHoldingPayouts;

impl<'a> ApiEndpoint<'a> for ListHoldingPayouts {
    const URL_PATH: &'static str = "/holdings/:holding_id/payouts.json";

    type UrlDisplay = ListHoldingPayoutsUrlDisplay<'a>;
    type Parameters = ListHoldingPayoutsParameters;

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

#[derive(Debug, Clone, Serialize)]
pub struct ListHoldingPayoutsParameters {
    pub holding_id: i64,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub use_date: Option<String>,
}

pub struct ListPortfolioPayouts;

impl<'a> ApiEndpoint<'a> for ListPortfolioPayouts {
    const URL_PATH: &'static str = "/portfolios/:portfolio_id/payouts.json";

    type UrlDisplay = ListPortfolioPayoutsUrlDisplay<'a>;
    type Parameters = ListPortfolioPayoutsParameters;

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

#[derive(Debug, Clone, Serialize)]
pub struct ListPortfolioPayoutsParameters {
    pub portfolio_id: i64,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub use_date: Option<String>,
}

pub struct PayoutCreate;

impl<'a> ApiEndpoint<'a> for PayoutCreate {
    const URL_PATH: &'static str = "/payouts";

    type UrlDisplay = &'static str;
    type Parameters = PayoutCreateParameters;

    fn url_path(_parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        "/payouts"
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PayoutCreateParameters {
    pub payout: PayoutCreatePayoutParameters,
}

#[derive(Debug, Clone, Serialize)]
pub struct PayoutCreatePayoutParameters {
    pub portfolio_id: i64,
    pub holding_id: i64,
    pub symbol: String,
    pub market: String,
    pub paid_on: NaiveDate,
    pub amount: f64,
    pub currency_code: String,
    pub goes_ex_on: Option<NaiveDate>,
    pub resident_withholding_tax: Option<f64>,
    pub non_resident_withholding_tax: Option<f64>,
    pub tax_credit: Option<f64>,
    pub exchange_rate: Option<f64>,
    pub adjustment_id: Option<i64>,
    pub comments: Option<String>,
    pub non_taxable: Option<bool>,
    pub source_payment_date: Option<String>,
    pub send_to_xero: Option<bool>,
    pub banked_amount: Option<f64>,
    pub drp_trade_attributes: Option<()>,
    pub franked_amount: f64,
    pub unfranked_amount: f64,
    pub trust: bool,
    pub extra_interest_payment_amount: Option<f64>,
    pub capital_gains: Option<f64>,
    pub discounted_capital_gains: Option<f64>,
    pub foreign_source_income: Option<f64>,
    pub lic_capital_gain: Option<f64>,
    pub non_assessable: Option<bool>,
    pub deferred_income: f64,
    pub cgt_concession_amount: f64,
    pub amit_decrease_amount: f64,
    pub amit_increase_amount: f64,
    pub file_name: String,
    pub file_attachment: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct PayoutCreatePayoutDrpTradeAttributesParameters {
    pub dividend_reinvested: Option<bool>,
    pub quantity: Option<f64>,
    pub price: Option<f64>,
    pub source_adjustment_id: Option<i64>,
}

pub struct PayoutDelete;

impl<'a> ApiEndpoint<'a> for PayoutDelete {
    const URL_PATH: &'static str = "/payouts/:id.json";

    type UrlDisplay = PayoutDeleteUrlDisplay<'a>;
    type Parameters = PayoutDeleteParameters;

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

#[derive(Debug, Clone, Serialize)]
pub struct PayoutDeleteParameters {
    pub id: i64,
}

pub struct PayoutReject;

impl<'a> ApiEndpoint<'a> for PayoutReject {
    const URL_PATH: &'static str = "/payouts.json";

    type UrlDisplay = &'static str;
    type Parameters = PayoutRejectParameters;

    fn url_path(_parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        "/payouts.json"
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PayoutRejectParameters {
    pub holding_id: i64,
    pub company_event_id: i64,
    pub state: String,
}

pub struct PayoutShow;

impl<'a> ApiEndpoint<'a> for PayoutShow {
    const URL_PATH: &'static str = "/payouts/:id.json";

    type UrlDisplay = PayoutShowUrlDisplay<'a>;
    type Parameters = PayoutShowParameters;

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

#[derive(Debug, Clone, Serialize)]
pub struct PayoutShowParameters {
    pub id: i64,
}

pub struct PayoutUpdate;

impl<'a> ApiEndpoint<'a> for PayoutUpdate {
    const URL_PATH: &'static str = "/payouts/:id.json";

    type UrlDisplay = PayoutUpdateUrlDisplay<'a>;
    type Parameters = PayoutUpdateParameters;

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

#[derive(Debug, Clone, Serialize)]
pub struct PayoutUpdateParameters {
    pub id: i64,
    pub payout: (),
}

#[derive(Debug, Clone, Serialize)]
pub struct PayoutUpdatePayoutParameters {
    pub paid_on: Option<NaiveDate>,
    pub goes_ex_on: Option<NaiveDate>,
    pub resident_withholding_tax: Option<f64>,
    pub non_resident_withholding_tax: Option<f64>,
    pub tax_credit: Option<f64>,
    pub exchange_rate: Option<f64>,
    pub amount: Option<f64>,
    pub adjustment_id: Option<i64>,
    pub comments: Option<String>,
    pub non_taxable: Option<bool>,
    pub currency_code: Option<String>,
    pub source_payment_date: Option<NaiveDate>,
    pub send_to_xero: Option<bool>,
    pub banked_amount: Option<f64>,
    pub source_adjustment_id: Option<i64>,
    pub drp_trade_attributes: Option<()>,
    pub franked_amount: Option<f64>,
    pub unfranked_amount: Option<f64>,
    pub trust: Option<bool>,
    pub extra_interest_payment_amount: Option<f64>,
    pub capital_gains: Option<f64>,
    pub discounted_capital_gains: Option<f64>,
    pub foreign_source_income: Option<f64>,
    pub lic_capital_gain: Option<f64>,
    pub non_assessable: Option<bool>,
    pub deferred_income: f64,
    pub cgt_concession_amount: f64,
    pub amit_decrease_amount: f64,
    pub amit_increase_amount: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct PayoutUpdatePayoutDrpTradeAttributesParameters {
    pub dividend_reinvested: Option<bool>,
    pub quantity: Option<f64>,
    pub price: Option<f64>,
    pub source_adjustment_id: Option<i64>,
}

pub struct PortfolioCreate;

impl<'a> ApiEndpoint<'a> for PortfolioCreate {
    const URL_PATH: &'static str = "/portfolios.json";

    type UrlDisplay = &'static str;
    type Parameters = PortfolioCreateParameters;

    fn url_path(_parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        "/portfolios.json"
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PortfolioCreateParameters {
    pub name: String,
    pub broker_email_api_enabled: bool,
    pub financial_year_end_month_id: i64,
    pub financial_year_end: String,
    pub default_sale_allocation_method: String,
    pub interest_method: String,
    pub trader: bool,
    pub tax_entity_type: String,
    pub disable_automatic_transactions: Option<bool>,
    pub cg_discount_id: i64,
    pub rwtr_rate: f64,
    pub country_code: Option<String>,
    pub apply_cash_account_adjustments: Option<bool>,
    pub buy_trade_settlement_delay: Option<i64>,
    pub sell_trade_settlement_delay: Option<i64>,
    pub account_for_delayed_cash_transactions: Option<bool>,
}

pub struct PortfolioDelete;

impl<'a> ApiEndpoint<'a> for PortfolioDelete {
    const URL_PATH: &'static str = "/portfolios/:id.json";

    type UrlDisplay = PortfolioDeleteUrlDisplay<'a>;
    type Parameters = PortfolioDeleteParameters;

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

#[derive(Debug, Clone, Serialize)]
pub struct PortfolioDeleteParameters {
    pub id: i64,
}

pub struct PortfolioUpdate;

impl<'a> ApiEndpoint<'a> for PortfolioUpdate {
    const URL_PATH: &'static str = "/portfolios/:id.json";

    type UrlDisplay = PortfolioUpdateUrlDisplay<'a>;
    type Parameters = PortfolioUpdateParameters;

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

#[derive(Debug, Clone, Serialize)]
pub struct PortfolioUpdateParameters {
    pub id: i64,
    pub name: String,
    pub broker_email_api_enabled: bool,
    pub financial_year_end_month_id: i64,
    pub financial_year_end: String,
    pub default_sale_allocation_method: String,
    pub interest_method: String,
    pub trader: bool,
    pub tax_entity_type: String,
    pub cg_discount_id: i64,
    pub rwtr_rate: f64,
    pub apply_cash_account_adjustments: Option<bool>,
    pub buy_trade_settlement_delay: Option<i64>,
    pub sell_trade_settlement_delay: Option<i64>,
    pub account_for_delayed_cash_transactions: Option<bool>,
    pub trade_sync_cash_account_id: i64,
}

pub struct CapitalGains;

impl<'a> ApiEndpoint<'a> for CapitalGains {
    const URL_PATH: &'static str = "/portfolios/:portfolio_id/capital_gains.json";

    type UrlDisplay = CapitalGainsUrlDisplay<'a>;
    type Parameters = CapitalGainsParameters;

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

#[derive(Debug, Clone, Serialize)]
pub struct CapitalGainsParameters {
    pub portfolio_id: i64,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
}

pub struct Diversity;

impl<'a> ApiEndpoint<'a> for Diversity {
    const URL_PATH: &'static str = "/portfolios/:portfolio_id/diversity.json";

    type UrlDisplay = DiversityUrlDisplay<'a>;
    type Parameters = DiversityParameters;

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

#[derive(Debug, Clone, Serialize)]
pub struct DiversityParameters {
    pub portfolio_id: i64,
    pub consolidated: Option<bool>,
    pub date: Option<NaiveDate>,
    pub grouping: Option<String>,
    pub custom_group_id: Option<i64>,
}

pub struct Performance;

impl<'a> ApiEndpoint<'a> for Performance {
    const URL_PATH: &'static str = "/portfolios/:portfolio_id/performance.json";

    type UrlDisplay = PerformanceUrlDisplay<'a>;
    type Parameters = PerformanceParameters;

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

#[derive(Debug, Clone, Serialize)]
pub struct PerformanceParameters {
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub portfolio_id: i64,
    pub consolidated: Option<bool>,
    pub include_sales: Option<bool>,
    pub grouping: Option<String>,
    pub custom_group_id: Option<i64>,
}

pub struct UnrealisedCgt;

impl<'a> ApiEndpoint<'a> for UnrealisedCgt {
    const URL_PATH: &'static str = "/portfolios/:portfolio_id/unrealised_cgt.json";

    type UrlDisplay = UnrealisedCgtUrlDisplay<'a>;
    type Parameters = UnrealisedCgtParameters;

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

#[derive(Debug, Clone, Serialize)]
pub struct UnrealisedCgtParameters {
    pub portfolio_id: i64,
    pub balance_date: NaiveDate,
}

pub struct Valuation;

impl<'a> ApiEndpoint<'a> for Valuation {
    const URL_PATH: &'static str = "/portfolios/:portfolio_id/valuation.json";

    type UrlDisplay = ValuationUrlDisplay<'a>;
    type Parameters = ValuationParameters;

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

#[derive(Debug, Clone, Serialize)]
pub struct ValuationParameters {
    pub balance_date: Option<NaiveDate>,
    pub portfolio_id: i64,
    pub consolidated: Option<bool>,
    pub include_sales: Option<bool>,
    pub grouping: Option<String>,
    pub custom_group_id: Option<i64>,
}

pub struct TradeConfirm;

impl<'a> ApiEndpoint<'a> for TradeConfirm {
    const URL_PATH: &'static str = "/trades.json";

    type UrlDisplay = &'static str;
    type Parameters = TradeConfirmParameters;

    fn url_path(_parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        "/trades.json"
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct TradeConfirmParameters {
    pub holding_id: i64,
    pub company_event_id: i64,
    pub transaction_date: NaiveDate,
    pub state: String,
}

pub struct TradeReject;

impl<'a> ApiEndpoint<'a> for TradeReject {
    const URL_PATH: &'static str = "/trades.json";

    type UrlDisplay = &'static str;
    type Parameters = TradeRejectParameters;

    fn url_path(_parameters: &'a Self::Parameters) -> Self::UrlDisplay {
        "/trades.json"
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct TradeRejectParameters {
    pub holding_id: i64,
    pub company_event_id: i64,
    pub transaction_date: NaiveDate,
    pub state: String,
}

pub struct Trades;

impl<'a> ApiEndpoint<'a> for Trades {
    const URL_PATH: &'static str = "/portfolios/:portfolio_id/trades.json";

    type UrlDisplay = TradesUrlDisplay<'a>;
    type Parameters = TradesParameters;

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

#[derive(Debug, Clone, Serialize)]
pub struct TradesParameters {
    pub portfolio_id: String,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub unique_identifier: Option<String>,
}

pub struct TradesDestroy;

impl<'a> ApiEndpoint<'a> for TradesDestroy {
    const URL_PATH: &'static str = "/trades/:id.json";

    type UrlDisplay = TradesDestroyUrlDisplay<'a>;
    type Parameters = TradesDestroyParameters;

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

#[derive(Debug, Clone, Serialize)]
pub struct TradesDestroyParameters {
    pub id: i64,
}

pub struct TradesShow;

impl<'a> ApiEndpoint<'a> for TradesShow {
    const URL_PATH: &'static str = "/trades/:id.json";

    type UrlDisplay = TradesShowUrlDisplay<'a>;
    type Parameters = TradesShowParameters;

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

#[derive(Debug, Clone, Serialize)]
pub struct TradesShowParameters {
    pub id: String,
}
