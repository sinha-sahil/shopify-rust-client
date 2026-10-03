pub mod remote;

use crate::common::ServiceContext;

use std::sync::Arc;

use crate::{
    common::types::{APIError, RequestCallbacks},
    types::returns::{
        ExchangeLineItemInput, ProcessableReturnResp, RefundMethodAllocation, RemoveFromReturnResp,
        ReturnApproveRequestResp, ReturnCancelResp, ReturnCreateLineItemInput, ReturnCreateResp,
        ReturnDeclineReason, ReturnDeclineRequestResp, ReturnFinancialOutcomeResp,
        ReturnLineItemInput, ReturnLineQuantityInput, ReturnProcessInput, ReturnProcessResp,
        ReturnRefundsResp, ReturnRequestResp, ReturnStatusResp, ReturnableFulfillmentResp,
        ReturnableFulfillmentsResp,
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

    pub async fn create(
        &self,
        order_id: &str,
        return_line_items: &[ReturnCreateLineItemInput],
        exchange_line_items: &[ExchangeLineItemInput],
    ) -> Result<ReturnCreateResp, APIError> {
        remote::create_return(&self.ctx, order_id, return_line_items, exchange_line_items).await
    }

    pub async fn approve(&self, return_id: &str) -> Result<ReturnApproveRequestResp, APIError> {
        remote::approve_return(&self.ctx, return_id).await
    }

    pub async fn decline(
        &self,
        return_id: &str,
        reason: ReturnDeclineReason,
        note: Option<&str>,
    ) -> Result<ReturnDeclineRequestResp, APIError> {
        remote::decline_return(&self.ctx, return_id, reason, note).await
    }

    pub async fn cancel(&self, return_id: &str) -> Result<ReturnCancelResp, APIError> {
        remote::cancel_return(&self.ctx, return_id).await
    }

    pub async fn remove_lines(
        &self,
        return_id: &str,
        return_line_items: &[ReturnLineQuantityInput],
        exchange_line_items: &[ReturnLineQuantityInput],
    ) -> Result<RemoveFromReturnResp, APIError> {
        remote::remove_from_return(&self.ctx, return_id, return_line_items, exchange_line_items)
            .await
    }

    pub async fn status(&self, return_id: &str) -> Result<ReturnStatusResp, APIError> {
        remote::get_return_status(&self.ctx, return_id).await
    }

    pub async fn returnable_fulfillments(
        &self,
        order_id: &str,
        first: u32,
        lines: u32,
        after: Option<&str>,
    ) -> Result<ReturnableFulfillmentsResp, APIError> {
        remote::get_returnable_fulfillments(&self.ctx, order_id, first, lines, after).await
    }

    pub async fn returnable_fulfillment(
        &self,
        fulfillment_id: &str,
        lines: u32,
        after: Option<&str>,
    ) -> Result<ReturnableFulfillmentResp, APIError> {
        remote::get_returnable_fulfillment(&self.ctx, fulfillment_id, lines, after).await
    }

    pub async fn processable(
        &self,
        return_id: &str,
        lines: u32,
    ) -> Result<ProcessableReturnResp, APIError> {
        remote::get_processable_return(&self.ctx, return_id, lines).await
    }

    pub async fn suggested_financial_outcome(
        &self,
        return_id: &str,
        return_line_items: &[ReturnLineQuantityInput],
        exchange_line_items: &[ReturnLineQuantityInput],
        allocation: RefundMethodAllocation,
    ) -> Result<ReturnFinancialOutcomeResp, APIError> {
        remote::get_suggested_financial_outcome(
            &self.ctx,
            return_id,
            return_line_items,
            exchange_line_items,
            allocation,
        )
        .await
    }

    pub async fn refunds(
        &self,
        return_id: &str,
        refunds: u32,
        transactions: u32,
    ) -> Result<ReturnRefundsResp, APIError> {
        remote::get_return_refunds(&self.ctx, return_id, refunds, transactions).await
    }

    pub async fn process(
        &self,
        input: &ReturnProcessInput,
        idempotency_key: &str,
    ) -> Result<ReturnProcessResp, APIError> {
        remote::process_return(&self.ctx, input, idempotency_key).await
    }
}
