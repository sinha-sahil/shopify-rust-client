pub mod remote;

use crate::common::ServiceContext;

use std::sync::Arc;

use crate::{
    common::types::{APIError, RequestCallbacks},
    types::cart_transform::{
        CartTransformCreateInput, CartTransformCreateResp, MetafieldsSetInput, MetafieldsSetResp,
    },
};

pub struct CartTransform {
    pub(crate) ctx: ServiceContext,
}

impl CartTransform {
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

    pub async fn create(
        &self,
        input: &CartTransformCreateInput,
    ) -> Result<CartTransformCreateResp, APIError> {
        remote::create_cart_transform(&self.ctx, input).await
    }

    pub async fn set_metafields(
        &self,
        metafields: &[MetafieldsSetInput],
    ) -> Result<MetafieldsSetResp, APIError> {
        remote::set_metafields(&self.ctx, metafields).await
    }
}
