pub mod remote;

use crate::common::ServiceContext;

use std::sync::Arc;

use crate::{
    common::types::{APIError, RequestCallbacks},
    types::discount::{
        DiscountAutomaticAppCreateResp, DiscountAutomaticAppInput, DiscountAutomaticAppUpdateInput,
        DiscountAutomaticAppUpdateResp, DiscountNodesResp, GetDiscountMetafieldResp,
        GetDiscountNodeResp,
    },
};

pub struct Discount {
    pub(crate) ctx: ServiceContext,
}

impl Discount {
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

    pub async fn create_automatic_app_discount(
        &self,
        input: &DiscountAutomaticAppInput,
    ) -> Result<DiscountAutomaticAppCreateResp, APIError> {
        remote::create_automatic_app_discount(&self.ctx, input).await
    }

    pub async fn update_automatic_app_discount(
        &self,
        input: &DiscountAutomaticAppUpdateInput,
    ) -> Result<DiscountAutomaticAppUpdateResp, APIError> {
        remote::update_automatic_app_discount(&self.ctx, input).await
    }

    pub async fn list_discounts(
        &self,
        first: Option<i32>,
        after: Option<String>,
        query: Option<String>,
    ) -> Result<DiscountNodesResp, APIError> {
        remote::list_discounts(&self.ctx, first, after, query).await
    }

    pub async fn get_discount_by_id(
        &self,
        id: &str,
        first: Option<i32>,
        after: Option<String>,
    ) -> Result<GetDiscountNodeResp, APIError> {
        remote::get_discount_by_id(&self.ctx, id, first, after).await
    }

    pub async fn get_discount_metafield(
        &self,
        id: &str,
        namespace: &str,
        key: &str,
    ) -> Result<GetDiscountMetafieldResp, APIError> {
        remote::get_discount_metafield(&self.ctx, id, namespace, key).await
    }
}
