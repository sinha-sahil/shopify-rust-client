pub mod remote;

use crate::common::ServiceContext;

use std::sync::Arc;

use crate::{
    common::types::{APIError, RequestCallbacks},
    types::returns::{
        OrderTransactionInput, ReturnLineItemInput, ReturnProcessLineInput, ReturnProcessResp,
        ReturnRequestResp,
    },
};

pub struct Returns {
    pub(crate) ctx: ServiceContext,
}

impl Returns {
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

    pub async fn request(
        &self,
        order_id: &str,
        line_items: &[ReturnLineItemInput],
    ) -> Result<ReturnRequestResp, APIError> {
        remote::request_return(&self.ctx, order_id, line_items).await
    }

    pub async fn process(
        &self,
        return_id: &str,
        return_line_items: &[ReturnProcessLineInput],
        order_transactions: &[OrderTransactionInput],
        idempotency_key: &str,
    ) -> Result<ReturnProcessResp, APIError> {
        remote::process_return(
            &self.ctx,
            return_id,
            return_line_items,
            order_transactions,
            idempotency_key,
        )
        .await
    }
}
