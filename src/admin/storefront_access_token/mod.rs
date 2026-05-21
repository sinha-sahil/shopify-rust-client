pub mod remote;

use crate::common::ServiceContext;

use std::sync::Arc;

use crate::{
    common::types::{APIError, RequestCallbacks},
    types::storefront_access_token::{
        ListStorefrontAccessTokensResp, StorefrontAccessTokenCreateInput,
        StorefrontAccessTokenCreateResp, StorefrontAccessTokenDeleteInput,
        StorefrontAccessTokenDeleteResp,
    },
};

pub struct StorefrontAccessToken {
    pub(crate) ctx: ServiceContext,
}

impl StorefrontAccessToken {
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

    pub async fn list(&self) -> Result<ListStorefrontAccessTokensResp, APIError> {
        remote::list_storefront_access_tokens(&self.ctx).await
    }

    pub async fn create(
        &self,
        input: &StorefrontAccessTokenCreateInput,
    ) -> Result<StorefrontAccessTokenCreateResp, APIError> {
        remote::create_storefront_access_token(&self.ctx, input).await
    }

    pub async fn delete(
        &self,
        input: &StorefrontAccessTokenDeleteInput,
    ) -> Result<StorefrontAccessTokenDeleteResp, APIError> {
        remote::delete_storefront_access_token(&self.ctx, input).await
    }
}
