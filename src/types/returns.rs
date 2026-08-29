use serde::{Deserialize, Serialize};

// region: Response Types

/// Shared across all five `return*` mutations. `returnRequest` / `returnCreate`
/// select only `{ field, message }` while `returnApproveRequest` /
/// `returnDeclineRequest` / `returnProcess` also select `code`, so `code` is
/// `#[serde(default)]` — an unselected field simply never appears in the JSON.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReturnUserError {
    pub field: Option<Vec<String>>,
    pub message: String,
    #[serde(default)]
    pub code: Option<String>,
}

/// Minimal to our selection sets — NOT the full Shopify `Return` object.
/// `name` and `return_line_items` are `None` for operations that never select
/// them.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Return {
    pub id: String,
    pub name: Option<String>,
    pub status: Option<ReturnStatus>,
    pub order: Option<ReturnOrderRef>,
    pub return_line_items: Option<ReturnLineItemList>,
    pub decline: Option<ReturnDecline>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ReturnOrderRef {
    pub id: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReturnDecline {
    pub reason: String,
    pub note: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReturnRequestPayloadInner {
    #[serde(rename = "return")]
    pub r#return: Option<Return>,
    pub user_errors: Vec<ReturnUserError>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ReturnRequestPayload {
    #[serde(rename = "returnRequest")]
    pub return_request: ReturnRequestPayloadInner,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReturnApproveRequestPayloadInner {
    #[serde(rename = "return")]
    pub r#return: Option<Return>,
    pub user_errors: Vec<ReturnUserError>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ReturnApproveRequestPayload {
    #[serde(rename = "returnApproveRequest")]
    pub return_approve_request: ReturnApproveRequestPayloadInner,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReturnDeclineRequestPayloadInner {
    #[serde(rename = "return")]
    pub r#return: Option<Return>,
    pub user_errors: Vec<ReturnUserError>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ReturnDeclineRequestPayload {
    #[serde(rename = "returnDeclineRequest")]
    pub return_decline_request: ReturnDeclineRequestPayloadInner,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReturnCreatePayloadInner {
    #[serde(rename = "return")]
    pub r#return: Option<Return>,
    pub user_errors: Vec<ReturnUserError>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ReturnCreatePayload {
    #[serde(rename = "returnCreate")]
    pub return_create: ReturnCreatePayloadInner,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReturnProcessPayloadInner {
    #[serde(rename = "return")]
    pub r#return: Option<Return>,
    pub user_errors: Vec<ReturnUserError>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ReturnProcessPayload {
    #[serde(rename = "returnProcess")]
    pub return_process: ReturnProcessPayloadInner,
}

// `suggestedFinancialOutcome` query response

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ShopMoney {
    pub amount: String,
    pub currency_code: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MoneyBag {
    pub shop_money: ShopMoney,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RefundReturnOutcome {
    pub amount: MoneyBag,
}

#[derive(Deserialize, Debug, Clone)]
pub struct InvoiceReturnOutcome {
    pub amount: MoneyBag,
}

/// Union `RefundReturnOutcome | InvoiceReturnOutcome`. The query selects
/// `__typename`, so serde's internally-tagged representation maps directly onto
/// the GraphQL type names.
#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "__typename")]
pub enum ReturnOutcomeFinancialTransfer {
    RefundReturnOutcome(RefundReturnOutcome),
    InvoiceReturnOutcome(InvoiceReturnOutcome),
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SuggestedReturnFinancialOutcome {
    pub maximum_refundable: Option<MoneyBag>,
    pub financial_transfer: Option<ReturnOutcomeFinancialTransfer>,
}

/// Separate from `Return` — this query's `return(id:)` selection set is disjoint
/// from any mutation's `return { ... }` selection.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReturnWithSuggestedOutcome {
    pub id: String,
    pub suggested_financial_outcome: Option<SuggestedReturnFinancialOutcome>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SuggestedFinancialOutcomeResponse {
    #[serde(rename = "return")]
    pub r#return: Option<ReturnWithSuggestedOutcome>,
}

// `returnable_fulfillment` query response
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReturnableFulfillmentResponse {
    pub returnable_fulfillment: Option<ReturnableFulfillment>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReturnableFulfillment {
    pub id: String,
    pub returnable_fulfillment_line_items: ReturnableFulfillmentLineItemList,
}

#[derive(Debug, Clone)]
pub struct ReturnableFulfillmentLineItemList {
    pub items: Vec<ReturnableFulfillmentLineItem>,
    pub truncated: bool,
}

impl<'de> Deserialize<'de> for ReturnableFulfillmentLineItemList {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Raw {
            nodes: Vec<ReturnableFulfillmentLineItem>,
            page_info: RawPageInfo,
        }
        let raw = Raw::deserialize(deserializer)?;
        Ok(ReturnableFulfillmentLineItemList {
            items: raw.nodes,
            truncated: raw.page_info.has_next_page,
        })
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReturnableFulfillmentLineItem {
    /// The quantity available to be returned.
    pub quantity: i64,
    pub fulfillment_line_item: ReturnFulfillmentLineItemRef,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReturnFulfillmentLineItemRef {
    pub id: String,
    pub line_item: OrderLineItemRef,
}

#[derive(Deserialize, Debug, Clone)]
pub struct OrderLineItemRef {
    pub id: String,
    #[serde(default)]
    pub title: Option<String>,
}

// `order_returns_and_refunds` query response

#[derive(Debug, Clone)]
pub struct OrderReturnsAndRefundsResponse {
    pub order: Option<OrderReturnsAndRefunds>,
}

#[derive(Debug, Clone)]
pub struct OrderReturnsAndRefunds {
    pub id: String,
    pub returns: ReturnList,
    pub refunds: OrderRefundList,
}

// Private wire type for deserialization
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct OrderReturnsAndRefundsRaw {
    pub id: String,
    pub returns: ReturnList,
    #[serde(default)]
    pub refunds: Vec<OrderRefund>,
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct OrderReturnsAndRefundsResponseRaw {
    pub order: Option<OrderReturnsAndRefundsRaw>,
}

#[derive(Debug, Clone)]
pub struct ReturnList {
    pub items: Vec<OrderReturnNode>,
    pub truncated: bool,
}

impl<'de> Deserialize<'de> for ReturnList {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Raw {
            nodes: Vec<OrderReturnNode>,
            page_info: RawPageInfo,
        }
        let raw = Raw::deserialize(deserializer)?;
        Ok(ReturnList {
            items: raw.nodes,
            truncated: raw.page_info.has_next_page,
        })
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderReturnNode {
    pub id: String,
    pub name: String,
    pub status: ReturnStatus,
    pub return_line_items: ReturnLineItemList,
}

#[derive(Debug, Clone)]
pub struct ReturnLineItemList {
    pub items: Vec<ReturnLineItemType>,
    pub truncated: bool,
}

impl<'de> Deserialize<'de> for ReturnLineItemList {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Raw {
            nodes: Vec<ReturnLineItemType>,
            #[serde(default)]
            page_info: RawPageInfo,
        }
        let raw = Raw::deserialize(deserializer)?;
        Ok(ReturnLineItemList {
            items: raw.nodes,
            truncated: raw.page_info.has_next_page,
        })
    }
}

/// Union over the `ReturnLineItemType` interface's two concrete members
/// (`ReturnLineItem`, `UnverifiedReturnLineItem`). The query selects
/// `__typename`, so serde's internally-tagged representation maps directly
/// onto the GraphQL type names.
#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "__typename")]
pub enum ReturnLineItemType {
    ReturnLineItem(ReturnLineItem),
    UnverifiedReturnLineItem(UnverifiedReturnLineItem),
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReturnLineItem {
    pub quantity: i64,
    pub refundable_quantity: i64,
    pub refunded_quantity: i64,
    pub processable_quantity: i64,
    pub processed_quantity: i64,
    pub unprocessed_quantity: i64,
    pub fulfillment_line_item: Option<ReturnFulfillmentLineItemRef>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UnverifiedReturnLineItem {
    pub quantity: i64,
    pub refundable_quantity: i64,
    pub refunded_quantity: i64,
    pub unprocessed_quantity: i64,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderRefund {
    pub id: String,
    pub refund_line_items: RefundLineItemList,
}

#[derive(Debug, Clone)]
pub struct OrderRefundList {
    pub items: Vec<OrderRefund>,
    pub truncated: bool,
}

#[derive(Debug, Clone)]
pub struct RefundLineItemList {
    pub items: Vec<RefundLineItem>,
    pub truncated: bool,
}

impl<'de> Deserialize<'de> for RefundLineItemList {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Raw {
            nodes: Vec<RefundLineItem>,
            page_info: RawPageInfo,
        }
        let raw = Raw::deserialize(deserializer)?;
        Ok(RefundLineItemList {
            items: raw.nodes,
            truncated: raw.page_info.has_next_page,
        })
    }
}

/// Private helper for deserializing pageInfo from GraphQL connections.
#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct RawPageInfo {
    #[serde(default)]
    has_next_page: bool,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RefundLineItem {
    pub id: String,
    pub quantity: i64,
    pub restock_type: String,
    pub line_item: OrderLineItemRef,
}

// Shared return reference for mutation payloads
#[derive(Deserialize, Debug, Clone)]
pub struct ReturnRef {
    pub id: String,
    pub status: ReturnStatus,
}

// `return_close` mutation payload
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReturnClosePayloadInner {
    #[serde(rename = "return")]
    pub r#return: Option<ReturnRef>,
    pub user_errors: Vec<ReturnUserError>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ReturnClosePayload {
    #[serde(rename = "returnClose")]
    pub return_close: ReturnClosePayloadInner,
}

// `return_reopen` mutation payload
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReturnReopenPayloadInner {
    #[serde(rename = "return")]
    pub r#return: Option<ReturnRef>,
    pub user_errors: Vec<ReturnUserError>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ReturnReopenPayload {
    #[serde(rename = "returnReopen")]
    pub return_reopen: ReturnReopenPayloadInner,
}

// `return_cancel` mutation payload
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReturnCancelPayloadInner {
    #[serde(rename = "return")]
    pub r#return: Option<ReturnRef>,
    pub user_errors: Vec<ReturnUserError>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ReturnCancelPayload {
    #[serde(rename = "returnCancel")]
    pub return_cancel: ReturnCancelPayloadInner,
}

// `return_line_item_remove_from_return` mutation payload
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReturnLineItemRemoveFromReturnPayloadInner {
    #[serde(rename = "return")]
    pub r#return: Option<ReturnRef>,
    pub user_errors: Vec<ReturnUserError>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ReturnLineItemRemoveFromReturnPayload {
    #[serde(rename = "returnLineItemRemoveFromReturn")]
    pub return_line_item_remove_from_return: ReturnLineItemRemoveFromReturnPayloadInner,
}

// endregion

// region: Enums

/// Reason a return was declined. Required (non-null) on
/// `ReturnDeclineRequestInput`.
#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReturnDeclineReason {
    FinalSale,
    Other,
    ReturnPeriodEnded,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RefundDutyRefundType {
    Full,
    Proportional,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RefundMethodAllocation {
    OriginalPaymentMethods,
    StoreCredit,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReverseFulfillmentOrderDispositionType {
    Missing,
    NotRestocked,
    ProcessingRequired,
    Restocked,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReturnStatus {
    Canceled,
    Closed,
    Declined,
    Open,
    Requested,
}

// endregion

// region: Request Types

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MoneyInput {
    pub amount: f64,
    pub currency_code: String,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RestockingFeeInput {
    pub percentage: f64,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReturnShippingFeeInput {
    pub amount: MoneyInput,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReturnRequestLineItemInput {
    pub fulfillment_line_item_id: String,
    pub quantity: i64,
    /// Max 300 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_reason_definition_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restocking_fee: Option<RestockingFeeInput>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReturnRequestInput {
    pub order_id: String,
    pub return_line_items: Vec<ReturnRequestLineItemInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_shipping_fee: Option<ReturnShippingFeeInput>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReturnApproveRequestInput {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_customer: Option<bool>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReturnDeclineRequestInput {
    pub id: String,
    pub decline_reason: ReturnDeclineReason,
    /// Max 500 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decline_note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_customer: Option<bool>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReturnLineItemInput {
    pub fulfillment_line_item_id: String,
    pub quantity: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restocking_fee: Option<RestockingFeeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_reason_definition_id: Option<String>,
    /// Max 255 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_reason_note: Option<String>,
}

/// `amount` and `percentage` are mutually exclusive.
#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeLineItemAppliedDiscountValueInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<MoneyInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage: Option<f64>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeLineItemAppliedDiscountInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub value: ExchangeLineItemAppliedDiscountValueInput,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeLineItemInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant_id: Option<String>,
    pub quantity: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applied_discount: Option<ExchangeLineItemAppliedDiscountInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gift_card_codes: Option<Vec<String>>,
}

/// The `returnCreate` mutation's argument is named `returnInput`, not `input`.
/// The deprecated `notifyCustomer` / `unprocessed` fields are not exposed.
#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReturnInput {
    pub order_id: String,
    pub return_line_items: Vec<ReturnLineItemInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_line_items: Option<Vec<ExchangeLineItemInput>>,
    /// ISO8601 `DateTime`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_shipping_fee: Option<ReturnShippingFeeInput>,
}

/// `location_id` is required when the disposition type is `RESTOCKED`.
#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReverseFulfillmentOrderDisposeInput {
    pub reverse_fulfillment_order_line_item_id: String,
    pub disposition_type: ReverseFulfillmentOrderDispositionType,
    pub quantity: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_id: Option<String>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReturnProcessReturnLineItemInput {
    pub id: String,
    pub quantity: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dispositions: Option<Vec<ReverseFulfillmentOrderDisposeInput>>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReturnProcessExchangeLineItemInput {
    pub id: String,
    pub quantity: i64,
}

/// `parent_id` must reference a `CAPTURE` or `SALE` transaction.
#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReturnRefundOrderTransactionInput {
    pub parent_id: String,
    pub transaction_amount: MoneyInput,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StoreCreditRefundInput {
    pub amount: MoneyInput,
    /// ISO8601 `DateTime`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RefundMethodInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_credit_refund: Option<StoreCreditRefundInput>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReturnProcessRefundInput {
    pub order_transactions: Vec<ReturnRefundOrderTransactionInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_methods: Option<Vec<RefundMethodInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_over_refunding: Option<bool>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReturnProcessFinancialTransferInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_refund: Option<ReturnProcessRefundInput>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RefundDutyInput {
    pub duty_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_type: Option<RefundDutyRefundType>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RefundShippingInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_refund: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_refund_amount: Option<MoneyInput>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReturnProcessInput {
    pub return_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_line_items: Option<Vec<ReturnProcessReturnLineItemInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_line_items: Option<Vec<ReturnProcessExchangeLineItemInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_transfer: Option<ReturnProcessFinancialTransferInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_duties: Option<Vec<RefundDutyInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_shipping: Option<RefundShippingInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_customer: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tip_line_id: Option<String>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SuggestedOutcomeReturnLineItemInput {
    pub id: String,
    pub quantity: i64,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SuggestedOutcomeExchangeLineItemInput {
    pub id: String,
    pub quantity: i64,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReturnLineItemRemoveFromReturnInput {
    pub return_line_item_id: String,
    pub quantity: i64,
}

// endregion

// region: Request Builders

impl MoneyInput {
    pub fn new(amount: f64, currency_code: String) -> Self {
        MoneyInput {
            amount,
            currency_code,
        }
    }
}

impl RestockingFeeInput {
    pub fn new(percentage: f64) -> Self {
        RestockingFeeInput { percentage }
    }
}

impl ReturnShippingFeeInput {
    pub fn new(amount: MoneyInput) -> Self {
        ReturnShippingFeeInput { amount }
    }
}

impl ReturnRequestLineItemInput {
    pub fn new(fulfillment_line_item_id: String, quantity: i64) -> Self {
        ReturnRequestLineItemInput {
            fulfillment_line_item_id,
            quantity,
            customer_note: None,
            return_reason_definition_id: None,
            restocking_fee: None,
        }
    }

    pub fn with_customer_note(mut self, customer_note: String) -> Self {
        self.customer_note = Some(customer_note);
        self
    }

    pub fn with_return_reason_definition_id(mut self, return_reason_definition_id: String) -> Self {
        self.return_reason_definition_id = Some(return_reason_definition_id);
        self
    }

    pub fn with_restocking_fee(mut self, restocking_fee: RestockingFeeInput) -> Self {
        self.restocking_fee = Some(restocking_fee);
        self
    }
}

impl ReturnRequestInput {
    pub fn new(order_id: String, return_line_items: Vec<ReturnRequestLineItemInput>) -> Self {
        ReturnRequestInput {
            order_id,
            return_line_items,
            return_shipping_fee: None,
        }
    }

    pub fn with_return_shipping_fee(mut self, return_shipping_fee: ReturnShippingFeeInput) -> Self {
        self.return_shipping_fee = Some(return_shipping_fee);
        self
    }
}

impl ReturnApproveRequestInput {
    pub fn new(id: String) -> Self {
        ReturnApproveRequestInput {
            id,
            notify_customer: None,
        }
    }

    pub fn with_notify_customer(mut self, notify_customer: bool) -> Self {
        self.notify_customer = Some(notify_customer);
        self
    }
}

impl ReturnDeclineRequestInput {
    pub fn new(id: String, decline_reason: ReturnDeclineReason) -> Self {
        ReturnDeclineRequestInput {
            id,
            decline_reason,
            decline_note: None,
            notify_customer: None,
        }
    }

    pub fn with_decline_note(mut self, decline_note: String) -> Self {
        self.decline_note = Some(decline_note);
        self
    }

    pub fn with_notify_customer(mut self, notify_customer: bool) -> Self {
        self.notify_customer = Some(notify_customer);
        self
    }
}

impl ReturnLineItemInput {
    pub fn new(fulfillment_line_item_id: String, quantity: i64) -> Self {
        ReturnLineItemInput {
            fulfillment_line_item_id,
            quantity,
            restocking_fee: None,
            return_reason_definition_id: None,
            return_reason_note: None,
        }
    }

    pub fn with_restocking_fee(mut self, restocking_fee: RestockingFeeInput) -> Self {
        self.restocking_fee = Some(restocking_fee);
        self
    }

    pub fn with_return_reason_definition_id(mut self, return_reason_definition_id: String) -> Self {
        self.return_reason_definition_id = Some(return_reason_definition_id);
        self
    }

    pub fn with_return_reason_note(mut self, return_reason_note: String) -> Self {
        self.return_reason_note = Some(return_reason_note);
        self
    }
}

impl ExchangeLineItemAppliedDiscountValueInput {
    pub fn amount(amount: MoneyInput) -> Self {
        ExchangeLineItemAppliedDiscountValueInput {
            amount: Some(amount),
            percentage: None,
        }
    }

    pub fn percentage(percentage: f64) -> Self {
        ExchangeLineItemAppliedDiscountValueInput {
            amount: None,
            percentage: Some(percentage),
        }
    }
}

impl ExchangeLineItemAppliedDiscountInput {
    pub fn new(value: ExchangeLineItemAppliedDiscountValueInput) -> Self {
        ExchangeLineItemAppliedDiscountInput {
            description: None,
            value,
        }
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }
}

impl ExchangeLineItemInput {
    pub fn new(quantity: i64) -> Self {
        ExchangeLineItemInput {
            variant_id: None,
            quantity,
            applied_discount: None,
            gift_card_codes: None,
        }
    }

    pub fn with_variant_id(mut self, variant_id: String) -> Self {
        self.variant_id = Some(variant_id);
        self
    }

    pub fn with_applied_discount(
        mut self,
        applied_discount: ExchangeLineItemAppliedDiscountInput,
    ) -> Self {
        self.applied_discount = Some(applied_discount);
        self
    }

    pub fn with_gift_card_codes(mut self, gift_card_codes: Vec<String>) -> Self {
        self.gift_card_codes = Some(gift_card_codes);
        self
    }
}

impl ReturnInput {
    pub fn new(order_id: String, return_line_items: Vec<ReturnLineItemInput>) -> Self {
        ReturnInput {
            order_id,
            return_line_items,
            exchange_line_items: None,
            requested_at: None,
            return_shipping_fee: None,
        }
    }

    pub fn with_exchange_line_items(
        mut self,
        exchange_line_items: Vec<ExchangeLineItemInput>,
    ) -> Self {
        self.exchange_line_items = Some(exchange_line_items);
        self
    }

    pub fn with_requested_at(mut self, requested_at: String) -> Self {
        self.requested_at = Some(requested_at);
        self
    }

    pub fn with_return_shipping_fee(mut self, return_shipping_fee: ReturnShippingFeeInput) -> Self {
        self.return_shipping_fee = Some(return_shipping_fee);
        self
    }
}

impl ReverseFulfillmentOrderDisposeInput {
    pub fn new(
        reverse_fulfillment_order_line_item_id: String,
        disposition_type: ReverseFulfillmentOrderDispositionType,
        quantity: i64,
    ) -> Self {
        ReverseFulfillmentOrderDisposeInput {
            reverse_fulfillment_order_line_item_id,
            disposition_type,
            quantity,
            location_id: None,
        }
    }

    pub fn with_location_id(mut self, location_id: String) -> Self {
        self.location_id = Some(location_id);
        self
    }
}

impl ReturnProcessReturnLineItemInput {
    pub fn new(id: String, quantity: i64) -> Self {
        ReturnProcessReturnLineItemInput {
            id,
            quantity,
            dispositions: None,
        }
    }

    pub fn with_dispositions(
        mut self,
        dispositions: Vec<ReverseFulfillmentOrderDisposeInput>,
    ) -> Self {
        self.dispositions = Some(dispositions);
        self
    }
}

impl ReturnProcessExchangeLineItemInput {
    pub fn new(id: String, quantity: i64) -> Self {
        ReturnProcessExchangeLineItemInput { id, quantity }
    }
}

impl ReturnRefundOrderTransactionInput {
    pub fn new(parent_id: String, transaction_amount: MoneyInput) -> Self {
        ReturnRefundOrderTransactionInput {
            parent_id,
            transaction_amount,
        }
    }
}

impl StoreCreditRefundInput {
    pub fn new(amount: MoneyInput) -> Self {
        StoreCreditRefundInput {
            amount,
            expires_at: None,
        }
    }

    pub fn with_expires_at(mut self, expires_at: String) -> Self {
        self.expires_at = Some(expires_at);
        self
    }
}

impl RefundMethodInput {
    pub fn new() -> Self {
        RefundMethodInput {
            store_credit_refund: None,
        }
    }

    pub fn with_store_credit_refund(mut self, store_credit_refund: StoreCreditRefundInput) -> Self {
        self.store_credit_refund = Some(store_credit_refund);
        self
    }
}

impl Default for RefundMethodInput {
    fn default() -> Self {
        Self::new()
    }
}

impl ReturnProcessRefundInput {
    pub fn new(order_transactions: Vec<ReturnRefundOrderTransactionInput>) -> Self {
        ReturnProcessRefundInput {
            order_transactions,
            refund_methods: None,
            allow_over_refunding: None,
        }
    }

    pub fn with_refund_methods(mut self, refund_methods: Vec<RefundMethodInput>) -> Self {
        self.refund_methods = Some(refund_methods);
        self
    }

    pub fn with_allow_over_refunding(mut self, allow_over_refunding: bool) -> Self {
        self.allow_over_refunding = Some(allow_over_refunding);
        self
    }
}

impl ReturnProcessFinancialTransferInput {
    pub fn new() -> Self {
        ReturnProcessFinancialTransferInput { issue_refund: None }
    }

    pub fn with_issue_refund(mut self, issue_refund: ReturnProcessRefundInput) -> Self {
        self.issue_refund = Some(issue_refund);
        self
    }
}

impl Default for ReturnProcessFinancialTransferInput {
    fn default() -> Self {
        Self::new()
    }
}

impl RefundDutyInput {
    pub fn new(duty_id: String) -> Self {
        RefundDutyInput {
            duty_id,
            refund_type: None,
        }
    }

    pub fn with_refund_type(mut self, refund_type: RefundDutyRefundType) -> Self {
        self.refund_type = Some(refund_type);
        self
    }
}

impl RefundShippingInput {
    pub fn new() -> Self {
        RefundShippingInput {
            full_refund: None,
            shipping_refund_amount: None,
        }
    }

    pub fn with_full_refund(mut self, full_refund: bool) -> Self {
        self.full_refund = Some(full_refund);
        self
    }

    pub fn with_shipping_refund_amount(mut self, shipping_refund_amount: MoneyInput) -> Self {
        self.shipping_refund_amount = Some(shipping_refund_amount);
        self
    }
}

impl Default for RefundShippingInput {
    fn default() -> Self {
        Self::new()
    }
}

impl ReturnProcessInput {
    pub fn new(return_id: String) -> Self {
        ReturnProcessInput {
            return_id,
            return_line_items: None,
            exchange_line_items: None,
            financial_transfer: None,
            refund_duties: None,
            refund_shipping: None,
            note: None,
            notify_customer: None,
            tip_line_id: None,
        }
    }

    pub fn with_return_line_items(
        mut self,
        return_line_items: Vec<ReturnProcessReturnLineItemInput>,
    ) -> Self {
        self.return_line_items = Some(return_line_items);
        self
    }

    pub fn with_exchange_line_items(
        mut self,
        exchange_line_items: Vec<ReturnProcessExchangeLineItemInput>,
    ) -> Self {
        self.exchange_line_items = Some(exchange_line_items);
        self
    }

    pub fn with_financial_transfer(
        mut self,
        financial_transfer: ReturnProcessFinancialTransferInput,
    ) -> Self {
        self.financial_transfer = Some(financial_transfer);
        self
    }

    pub fn with_refund_duties(mut self, refund_duties: Vec<RefundDutyInput>) -> Self {
        self.refund_duties = Some(refund_duties);
        self
    }

    pub fn with_refund_shipping(mut self, refund_shipping: RefundShippingInput) -> Self {
        self.refund_shipping = Some(refund_shipping);
        self
    }

    pub fn with_note(mut self, note: String) -> Self {
        self.note = Some(note);
        self
    }

    pub fn with_notify_customer(mut self, notify_customer: bool) -> Self {
        self.notify_customer = Some(notify_customer);
        self
    }

    pub fn with_tip_line_id(mut self, tip_line_id: String) -> Self {
        self.tip_line_id = Some(tip_line_id);
        self
    }
}

impl SuggestedOutcomeReturnLineItemInput {
    pub fn new(id: String, quantity: i64) -> Self {
        SuggestedOutcomeReturnLineItemInput { id, quantity }
    }
}

impl SuggestedOutcomeExchangeLineItemInput {
    pub fn new(id: String, quantity: i64) -> Self {
        SuggestedOutcomeExchangeLineItemInput { id, quantity }
    }
}

impl ReturnLineItemRemoveFromReturnInput {
    pub fn new(return_line_item_id: String, quantity: i64) -> Self {
        ReturnLineItemRemoveFromReturnInput {
            return_line_item_id,
            quantity,
        }
    }
}

// endregion
