pub mod remote;

use std::sync::Arc;

use crate::{
    common::types::APIError,
    types::subscription::{
        ActiveSubscriptionsResp, CancelSubscriptionResp, CreateCombinedSubscriptionRequest,
        CreateRecurringSubscriptionRequest, CreateSubscriptionResp,
        CreateUsageRecordRequest, CreateUsageRecordResp, CreateUsageSubscriptionRequest,
        ExtendTrialResp, MoneyInput, UpdateCappedAmountResp,
    },
};

pub struct Subscription {
    pub shop_url: Arc<String>,
    pub version: Arc<String>,
    pub access_token: Arc<String>,
}

impl Subscription {
    pub fn new(shop_url: Arc<String>, version: Arc<String>, access_token: Arc<String>) -> Self {
        Subscription {
            shop_url,
            version,
            access_token,
        }
    }

    pub async fn create_recurring(
        &self,
        request: &CreateRecurringSubscriptionRequest,
    ) -> Result<CreateSubscriptionResp, APIError> {
        remote::create_recurring_subscription(
            &self.shop_url,
            &self.version,
            &self.access_token,
            request,
        )
        .await
    }

    pub async fn create_usage(
        &self,
        request: &CreateUsageSubscriptionRequest,
    ) -> Result<CreateSubscriptionResp, APIError> {
        remote::create_usage_subscription(
            &self.shop_url,
            &self.version,
            &self.access_token,
            request,
        )
        .await
    }

    pub async fn create_combined(
        &self,
        request: &CreateCombinedSubscriptionRequest,
    ) -> Result<CreateSubscriptionResp, APIError> {
        remote::create_combined_subscription(
            &self.shop_url,
            &self.version,
            &self.access_token,
            request,
        )
        .await
    }

    pub async fn cancel(
        &self,
        subscription_id: &String,
        prorate: bool,
    ) -> Result<CancelSubscriptionResp, APIError> {
        remote::cancel_subscription(
            &self.shop_url,
            &self.version,
            &self.access_token,
            subscription_id,
            prorate,
        )
        .await
    }

    pub async fn extend_trial(
        &self,
        subscription_id: &String,
        days: i32,
    ) -> Result<ExtendTrialResp, APIError> {
        remote::extend_trial(
            &self.shop_url,
            &self.version,
            &self.access_token,
            subscription_id,
            days,
        )
        .await
    }

    pub async fn update_capped_amount(
        &self,
        line_item_id: &String,
        capped_amount: &MoneyInput,
    ) -> Result<UpdateCappedAmountResp, APIError> {
        remote::update_capped_amount(
            &self.shop_url,
            &self.version,
            &self.access_token,
            line_item_id,
            capped_amount,
        )
        .await
    }

    pub async fn create_usage_record(
        &self,
        request: &CreateUsageRecordRequest,
    ) -> Result<CreateUsageRecordResp, APIError> {
        remote::create_usage_record(
            &self.shop_url,
            &self.version,
            &self.access_token,
            request,
        )
        .await
    }

    pub async fn get_active_subscriptions(&self) -> Result<ActiveSubscriptionsResp, APIError> {
        remote::get_active_subscriptions(&self.shop_url, &self.version, &self.access_token).await
    }
}
