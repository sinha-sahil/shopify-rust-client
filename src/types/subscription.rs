// region: Response Types

#[derive(serde::Deserialize, Debug)]
pub struct CreateSubscriptionResp {
    #[serde(rename = "appSubscriptionCreate")]
    pub app_subscription_create: AppSubscriptionCreatePayload,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AppSubscriptionCreatePayload {
    pub app_subscription: Option<AppSubscription>,
    pub confirmation_url: Option<String>,
    pub user_errors: Vec<UserError>,
}

#[derive(serde::Deserialize, Debug)]
pub struct CancelSubscriptionResp {
    #[serde(rename = "appSubscriptionCancel")]
    pub app_subscription_cancel: AppSubscriptionCancelPayload,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AppSubscriptionCancelPayload {
    pub app_subscription: Option<AppSubscription>,
    pub user_errors: Vec<UserError>,
}

#[derive(serde::Deserialize, Debug)]
pub struct ExtendTrialResp {
    #[serde(rename = "appSubscriptionTrialExtend")]
    pub app_subscription_trial_extend: AppSubscriptionTrialExtendPayload,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AppSubscriptionTrialExtendPayload {
    pub app_subscription: Option<AppSubscription>,
    pub user_errors: Vec<UserError>,
}

#[derive(serde::Deserialize, Debug)]
pub struct UpdateCappedAmountResp {
    #[serde(rename = "appSubscriptionLineItemUpdate")]
    pub app_subscription_line_item_update: AppSubscriptionLineItemUpdatePayload,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AppSubscriptionLineItemUpdatePayload {
    pub app_subscription: Option<AppSubscription>,
    pub user_errors: Vec<UserError>,
}

#[derive(serde::Deserialize, Debug)]
pub struct CreateUsageRecordResp {
    #[serde(rename = "appUsageRecordCreate")]
    pub app_usage_record_create: AppUsageRecordCreatePayload,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AppUsageRecordCreatePayload {
    pub app_usage_record: Option<AppUsageRecord>,
    pub user_errors: Vec<UserError>,
}

#[derive(serde::Deserialize, Debug)]
pub struct ActiveSubscriptionsResp {
    #[serde(rename = "currentAppInstallation")]
    pub current_app_installation: AppInstallation,
}

// endregion

// region: Request Types

#[derive(serde::Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateRecurringSubscriptionRequest {
    pub name: String,
    pub return_url: String,
    pub price: f64,
    pub currency_code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<AppPricingInterval>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_days: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<AppSubscriptionDiscountInput>,
}

#[derive(serde::Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateUsageSubscriptionRequest {
    pub name: String,
    pub return_url: String,
    pub capped_amount: f64,
    pub currency_code: String,
    pub terms: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_days: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<bool>,
}

#[derive(serde::Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateCombinedSubscriptionRequest {
    pub name: String,
    pub return_url: String,
    pub recurring_price: f64,
    pub recurring_currency_code: String,
    pub capped_amount: f64,
    pub usage_currency_code: String,
    pub terms: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<AppPricingInterval>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_days: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<AppSubscriptionDiscountInput>,
}

#[derive(serde::Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateUsageRecordRequest {
    pub subscription_line_item_id: String,
    pub price: f64,
    pub currency_code: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
}

// endregion

// region: Domain Types

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AppSubscription {
    pub id: String,
    pub name: String,
    pub status: AppSubscriptionStatus,
    pub line_items: Vec<AppSubscriptionLineItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_period_end: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_days: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<bool>,
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AppSubscriptionLineItem {
    pub id: String,
    pub plan: AppPlanV2,
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AppPlanV2 {
    pub pricing_details: AppPricingDetails,
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum AppPricingDetails {
    Recurring(AppRecurringPricing),
    Usage(AppUsagePricing),
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AppRecurringPricing {
    pub price: MoneyV2,
    pub interval: AppPricingInterval,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<AppSubscriptionDiscount>,
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AppUsagePricing {
    pub capped_amount: MoneyV2,
    pub terms: String,
    pub balance_used: MoneyV2,
    pub interval: AppPricingInterval,
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AppSubscriptionDiscount {
    pub value: AppSubscriptionDiscountValue,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_limit_in_intervals: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining_duration_in_intervals: Option<i32>,
    pub price_after_discount: MoneyV2,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum AppSubscriptionDiscountValue {
    Percentage { percentage: f64 },
    Amount { amount: MoneyV2 },
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AppUsageRecord {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<MoneyV2>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AppInstallation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_subscriptions: Option<Vec<AppSubscription>>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct MoneyV2 {
    pub amount: String,
    #[serde(rename = "currencyCode")]
    pub currency_code: String,
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserError {
    pub field: Option<Vec<String>>,
    pub message: String,
}

// endregion

// region: Input Types

#[derive(serde::Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AppSubscriptionDiscountInput {
    pub value: AppSubscriptionDiscountValueInput,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_limit_in_intervals: Option<i32>,
}

#[derive(serde::Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum AppSubscriptionDiscountValueInput {
    Percentage { percentage: f64 },
    Amount { amount: MoneyInput },
}

#[derive(serde::Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MoneyInput {
    pub amount: f64,
    pub currency_code: String,
}

// endregion

// region: Enums

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AppPricingInterval {
    Annual,
    Every30Days,
}

#[derive(serde::Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AppSubscriptionStatus {
    Active,
    Cancelled,
    Declined,
    Expired,
    Frozen,
    Pending,
}

#[derive(serde::Serialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AppSubscriptionReplacementBehavior {
    Standard,
    ApplyImmediately,
    ApplyOnNextBillingCycle,
}

// endregion
