pub mod remote;

use crate::common::ServiceContext;

use std::sync::Arc;

use crate::{
    common::types::{APIError, RequestCallbacks},
    types::returns::{
        OrderReturnsAndRefundsResponse, RefundMethodAllocation, ReturnApproveRequestInput,
        ReturnApproveRequestPayload, ReturnCancelPayload, ReturnClosePayload, ReturnCreatePayload,
        ReturnDeclineRequestInput, ReturnDeclineRequestPayload, ReturnInput,
        ReturnLineItemRemoveFromReturnInput, ReturnLineItemRemoveFromReturnPayload,
        ReturnProcessInput, ReturnProcessPayload, ReturnReopenPayload, ReturnRequestInput,
        ReturnRequestPayload, ReturnableFulfillmentResponse, SuggestedFinancialOutcomeResponse,
        SuggestedOutcomeExchangeLineItemInput, SuggestedOutcomeReturnLineItemInput,
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

    /// Build the service from a shared `ServiceContext`. Cheaper than `new` at
    /// construction sites that already hold a context (one `Arc` clone per service).
    pub fn with_ctx(ctx: ServiceContext) -> Self {
        Self { ctx }
    }

    pub async fn return_request(
        &self,
        input: &ReturnRequestInput,
    ) -> Result<ReturnRequestPayload, APIError> {
        remote::return_request(&self.ctx, input).await
    }

    pub async fn return_approve_request(
        &self,
        input: &ReturnApproveRequestInput,
    ) -> Result<ReturnApproveRequestPayload, APIError> {
        remote::return_approve_request(&self.ctx, input).await
    }

    pub async fn return_decline_request(
        &self,
        input: &ReturnDeclineRequestInput,
    ) -> Result<ReturnDeclineRequestPayload, APIError> {
        remote::return_decline_request(&self.ctx, input).await
    }

    pub async fn return_create(
        &self,
        input: &ReturnInput,
    ) -> Result<ReturnCreatePayload, APIError> {
        remote::return_create(&self.ctx, input).await
    }

    pub async fn return_process(
        &self,
        input: &ReturnProcessInput,
    ) -> Result<ReturnProcessPayload, APIError> {
        remote::return_process(&self.ctx, input).await
    }

    pub async fn suggested_financial_outcome(
        &self,
        return_id: &str,
        return_line_items: Vec<SuggestedOutcomeReturnLineItemInput>,
        exchange_line_items: Vec<SuggestedOutcomeExchangeLineItemInput>,
        refund_method_allocation: Option<RefundMethodAllocation>,
    ) -> Result<SuggestedFinancialOutcomeResponse, APIError> {
        remote::suggested_financial_outcome(
            &self.ctx,
            return_id,
            return_line_items,
            exchange_line_items,
            refund_method_allocation,
        )
        .await
    }

    pub async fn returnable_fulfillment(
        &self,
        fulfillment_order_id: &str,
        first: i64,
        after: Option<&str>,
    ) -> Result<ReturnableFulfillmentResponse, APIError> {
        remote::returnable_fulfillment(&self.ctx, fulfillment_order_id, first, after).await
    }

    pub async fn order_returns_and_refunds(
        &self,
        order_id: &str,
        returns_first: i64,
        returns_after: Option<&str>,
        refunds_first: i64,
    ) -> Result<OrderReturnsAndRefundsResponse, APIError> {
        remote::order_returns_and_refunds(
            &self.ctx,
            order_id,
            returns_first,
            returns_after,
            refunds_first,
        )
        .await
    }

    pub async fn return_close(&self, id: &str) -> Result<ReturnClosePayload, APIError> {
        remote::return_close(&self.ctx, id).await
    }

    pub async fn return_reopen(&self, id: &str) -> Result<ReturnReopenPayload, APIError> {
        remote::return_reopen(&self.ctx, id).await
    }

    pub async fn return_cancel(&self, id: &str) -> Result<ReturnCancelPayload, APIError> {
        remote::return_cancel(&self.ctx, id, None).await
    }

    pub async fn return_line_item_remove_from_return(
        &self,
        return_id: &str,
        return_line_items: Vec<ReturnLineItemRemoveFromReturnInput>,
    ) -> Result<ReturnLineItemRemoveFromReturnPayload, APIError> {
        remote::return_line_item_remove_from_return(&self.ctx, return_id, return_line_items).await
    }
}
