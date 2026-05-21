pub mod remote;

use crate::common::ServiceContext;

use std::sync::Arc;

use crate::{
    common::types::{APIError, RequestCallbacks},
    types::order::{GetOrderResp, OrderQueryResp, PatchOrderRequest},
};

pub struct Order {
    pub(crate) ctx: ServiceContext,
}

impl Order {
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

    pub async fn get_with_id(&self, order_id: &String) -> Result<GetOrderResp, APIError> {
        remote::get_order_with_id(&self.ctx, order_id).await
    }

    pub async fn get_with_name(&self, order_name: &String) -> Result<OrderQueryResp, APIError> {
        remote::get_order_with_name(&self.ctx, order_name).await
    }

    pub async fn patch(
        &self,
        order_id: &String,
        patch_request: &PatchOrderRequest,
    ) -> Result<GetOrderResp, APIError> {
        remote::patch_order(&self.ctx, order_id, patch_request).await
    }
}
