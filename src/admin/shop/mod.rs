pub mod remote;

use crate::common::ServiceContext;

use std::sync::Arc;

use crate::{
    common::types::{APIError, RequestCallbacks},
    types::shop::{GetShopResp, GetShopStatusResp},
};

pub struct Shop {
    pub(crate) ctx: ServiceContext,
}

impl Shop {
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

    pub async fn get(&self) -> Result<GetShopResp, APIError> {
        remote::get_shop(&self.ctx).await
    }

    pub async fn get_status(&self) -> Result<GetShopStatusResp, APIError> {
        remote::get_shop_status(&self.ctx).await
    }
}
