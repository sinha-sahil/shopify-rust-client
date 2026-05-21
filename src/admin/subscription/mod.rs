pub mod remote;

use crate::common::ServiceContext;

use std::sync::Arc;

use crate::{
    common::types::{APIError, RequestCallbacks},
    types::subscription::{
        ActiveSubscriptionsResp, CancelSubscriptionResp, CreateCombinedSubscriptionRequest,
        CreateRecurringSubscriptionRequest, CreateSubscriptionResp, CreateUsageRecordRequest,
        CreateUsageRecordResp, CreateUsageSubscriptionRequest, ExtendTrialResp, MoneyInput,
        UpdateCappedAmountResp,
    },
};

pub struct Subscription {
    pub(crate) ctx: ServiceContext,
}

impl Subscription {
    pub fn new(
        shop_url: Arc<String>,
        version: Arc<String>,
        access_token: Arc<String>,
        callbacks: Arc<RequestCallbacks>,
    ) -> Self {
        Self::with_ctx(ServiceContext::new(
            shop_url,
            version,
            access_token,
            callbacks,
        ))
    }

    /// Build the service from a shared `ServiceContext`. Cheaper than `new` at
    /// construction sites that already hold a context (one `Arc` clone per service).
    pub fn with_ctx(ctx: ServiceContext) -> Self {
        Self { ctx }
    }

    pub async fn create_recurring(
        &self,
        request: &CreateRecurringSubscriptionRequest,
    ) -> Result<CreateSubscriptionResp, APIError> {
        remote::create_recurring_subscription(&self.ctx, request).await
    }

    pub async fn create_usage(
        &self,
        request: &CreateUsageSubscriptionRequest,
    ) -> Result<CreateSubscriptionResp, APIError> {
        remote::create_usage_subscription(&self.ctx, request).await
    }

    pub async fn create_combined(
        &self,
        request: &CreateCombinedSubscriptionRequest,
    ) -> Result<CreateSubscriptionResp, APIError> {
        remote::create_combined_subscription(&self.ctx, request).await
    }

    pub async fn cancel(
        &self,
        subscription_id: &String,
        prorate: bool,
    ) -> Result<CancelSubscriptionResp, APIError> {
        remote::cancel_subscription(&self.ctx, subscription_id, prorate).await
    }

    pub async fn extend_trial(
        &self,
        subscription_id: &String,
        days: i32,
    ) -> Result<ExtendTrialResp, APIError> {
        remote::extend_trial(&self.ctx, subscription_id, days).await
    }

    pub async fn update_capped_amount(
        &self,
        line_item_id: &String,
        capped_amount: &MoneyInput,
    ) -> Result<UpdateCappedAmountResp, APIError> {
        remote::update_capped_amount(&self.ctx, line_item_id, capped_amount).await
    }

    pub async fn create_usage_record(
        &self,
        request: &CreateUsageRecordRequest,
    ) -> Result<CreateUsageRecordResp, APIError> {
        remote::create_usage_record(&self.ctx, request).await
    }

    pub async fn get_active_subscriptions(&self) -> Result<ActiveSubscriptionsResp, APIError> {
        remote::get_active_subscriptions(&self.ctx).await
    }
}
