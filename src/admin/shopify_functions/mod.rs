pub mod remote;

use crate::common::ServiceContext;

use crate::{
    common::types::{APIError, RequestCallbacks},
    types::shopify_functions::ShopifyFunctionsResp,
};
use std::sync::Arc;

pub struct ShopifyFunctions {
    pub(crate) ctx: ServiceContext,
}

impl ShopifyFunctions {
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

    pub async fn list(&self) -> Result<ShopifyFunctionsResp, APIError> {
        remote::list_shopify_functions(&self.ctx).await
    }
}
