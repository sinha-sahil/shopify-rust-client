pub mod remote;

use crate::common::ServiceContext;

use std::sync::Arc;

use crate::{
    common::types::{APIError, RequestCallbacks},
    types::product::ProductVariantResp,
};

pub struct Product {
    pub(crate) ctx: ServiceContext,
}

impl Product {
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

    pub fn with_ctx(ctx: ServiceContext) -> Self {
        Self { ctx }
    }

    pub async fn variant(&self, variant_id: &str) -> Result<ProductVariantResp, APIError> {
        remote::get_variant(&self.ctx, variant_id).await
    }
}
