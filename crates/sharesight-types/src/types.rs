use crate::types_prelude::*;

pub struct CashAccountCreateParameters {
    pub cash_account: CashAccountCreateCashAccountParameters,
}

pub struct CashAccountCreateCashAccountParameters {
    pub name: String,
    pub currency: String,
}

pub struct CashAccountDeleteParameters {
    pub id: i64,
}

pub struct CashAccountShowParameters {
    pub id: i64,
    pub date: Option<NaiveDate>,
}

pub struct CashAccountTransactionCreateParameters {
    pub cash_account_id: i64,
    pub description: String,
    pub amount: f64,
    pub type_name: String,
    pub date_time: NaiveDateTime,
    pub foreign_identifier: Option<String>,
}

pub struct CashAccountTransactionDeleteParameters {
    pub id: i64,
}

pub struct CashAccountTransactionUpdateParameters {
    pub id: i64,
    pub description: String,
    pub amount: f64,
    pub type_name: String,
    pub date_time: NaiveDateTime,
    pub foreign_identifier: Option<String>,
}

pub struct CashAccountTransactionsListParameters {
    pub cash_account_id: i64,
    pub from: Option<NaiveDate>,
    pub to: Option<NaiveDate>,
    pub description: Option<String>,
    pub foreign_identifier: Option<String>,
}

pub struct CashAccountUpdateParameters {
    pub id: i64,
    pub name: String,
    pub currency: String,
}

pub struct CashAccountsListParameters {
    pub date: Option<NaiveDate>,
}

pub struct DocumentShowParameters {
    pub id: i64,
}

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

pub struct HoldingTradesParameters {
    pub id: Option<String>,
    pub unique_identifier: Option<String>,
}

pub struct HoldingTradesRejectedParameters {
    pub holding_id: String,
}

pub struct IdentityByTokenParameters {
    pub id_token: String,
    pub client_id: String,
}

pub struct IdentitySignupByTokenParameters {
    pub id_token: String,
    pub client_id: String,
    pub country_code: String,
}

pub struct MembershipCreateParameters {
    pub membership: MembershipCreateMembershipParameters,
    pub user: Option<MembershipCreateUserParameters>,
    pub invitation: MembershipCreateInvitationParameters,
}

pub struct MembershipCreateMembershipParameters {
    pub portfolio_id: i64,
    pub access_code: String,
    pub user_id: Option<i64>,
}

pub struct MembershipCreateUserParameters {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}

pub struct MembershipCreateInvitationParameters {
    pub text: Option<String>,
    pub no_email: Option<bool>,
}

pub struct MembershipDeleteParameters {
    pub id: i64,
}

pub struct MembershipUpdateParameters {
    pub id: i64,
    pub access_code: String,
}

pub struct ListHoldingPayoutsParameters {
    pub holding_id: i64,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub use_date: Option<String>,
}

pub struct ListPortfolioPayoutsParameters {
    pub portfolio_id: i64,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub use_date: Option<String>,
}

pub struct PayoutConfirmPayoutParameters {
    pub holding_id: i64,
    pub company_event_id: i64,
    pub paid_on: NaiveDate,
    pub state: String,
    pub drp_trade_attributes: Option<()>,
}

pub struct PayoutConfirmPayoutDrpTradeAttributesParameters {
    pub dividend_reinvested: Option<bool>,
    pub quantity: Option<f64>,
    pub price: Option<f64>,
    pub source_adjustment_id: Option<i64>,
}

pub struct PayoutCreateParameters {
    pub payout: PayoutCreatePayoutParameters,
}

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

pub struct PayoutCreatePayoutDrpTradeAttributesParameters {
    pub dividend_reinvested: Option<bool>,
    pub quantity: Option<f64>,
    pub price: Option<f64>,
    pub source_adjustment_id: Option<i64>,
}

pub struct PayoutDeleteParameters {
    pub id: i64,
}

pub struct PayoutRejectParameters {
    pub holding_id: i64,
    pub company_event_id: i64,
    pub state: String,
}

pub struct PayoutShowParameters {
    pub id: i64,
}

pub struct PayoutUpdateParameters {
    pub id: i64,
    pub payout: (),
}

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

pub struct PayoutUpdatePayoutDrpTradeAttributesParameters {
    pub dividend_reinvested: Option<bool>,
    pub quantity: Option<f64>,
    pub price: Option<f64>,
    pub source_adjustment_id: Option<i64>,
}

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

pub struct PortfolioDeleteParameters {
    pub id: i64,
}

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

pub struct CapitalGainsParameters {
    pub portfolio_id: i64,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
}

pub struct DiversityParameters {
    pub portfolio_id: i64,
    pub consolidated: Option<bool>,
    pub date: Option<NaiveDate>,
    pub grouping: Option<String>,
    pub custom_group_id: Option<i64>,
}

pub struct PerformanceParameters {
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub portfolio_id: i64,
    pub consolidated: Option<bool>,
    pub include_sales: Option<bool>,
    pub grouping: Option<String>,
    pub custom_group_id: Option<i64>,
}

pub struct UnrealisedCgtParameters {
    pub portfolio_id: i64,
    pub balance_date: NaiveDate,
}

pub struct ValuationParameters {
    pub balance_date: Option<NaiveDate>,
    pub portfolio_id: i64,
    pub consolidated: Option<bool>,
    pub include_sales: Option<bool>,
    pub grouping: Option<String>,
    pub custom_group_id: Option<i64>,
}

pub struct TradeConfirmParameters {
    pub holding_id: i64,
    pub company_event_id: i64,
    pub transaction_date: NaiveDate,
    pub state: String,
}

pub struct TradeRejectParameters {
    pub holding_id: i64,
    pub company_event_id: i64,
    pub transaction_date: NaiveDate,
    pub state: String,
}

pub struct TradesParameters {
    pub portfolio_id: String,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub unique_identifier: Option<String>,
}

pub struct TradesDestroyParameters {
    pub id: i64,
}

pub struct TradesShowParameters {
    pub id: String,
}
