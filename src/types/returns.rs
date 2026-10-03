use std::fmt;

use crate::common::types::{Connection, Node};
use crate::types::order::{MoneyBag, MoneyV2, OrderTransaction};

#[derive(serde::Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReturnLineItemInput {
    pub fulfillment_line_item_id: String,
    pub quantity: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_reason_definition_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_note: Option<String>,
}

#[derive(serde::Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReturnCreateLineItemInput {
    pub fulfillment_line_item_id: String,
    pub quantity: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_reason_note: Option<String>,
}

#[derive(serde::Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeLineItemInput {
    pub variant_id: String,
    pub quantity: i32,
}

#[derive(serde::Serialize, Debug, Clone, PartialEq, Eq)]
pub struct ReturnLineQuantityInput {
    pub id: String,
    pub quantity: i32,
}

#[derive(serde::Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReturnProcessInput {
    pub return_id: String,
    pub return_line_items: Vec<ReturnLineQuantityInput>,
    pub exchange_line_items: Vec<ReturnLineQuantityInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_transfer: Option<ReturnProcessFinancialTransferInput>,
    pub notify_customer: bool,
}

#[derive(serde::Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReturnProcessFinancialTransferInput {
    pub issue_refund: ReturnProcessRefundInput,
}

#[derive(serde::Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReturnProcessRefundInput {
    pub order_transactions: Vec<ReturnRefundOrderTransactionInput>,
}

#[derive(serde::Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReturnRefundOrderTransactionInput {
    pub parent_id: String,
    pub transaction_amount: MoneyInput,
}

#[derive(serde::Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MoneyInput {
    pub amount: String,
    pub currency_code: String,
}

impl From<&MoneyV2> for MoneyInput {
    fn from(money: &MoneyV2) -> Self {
        MoneyInput {
            amount: money.amount.clone(),
            currency_code: money.currency_code.clone(),
        }
    }
}

#[derive(serde::Serialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReturnDeclineReason {
    ReturnPeriodEnded,
    FinalSale,
    Other,
}

#[derive(serde::Serialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RefundMethodAllocation {
    OriginalPaymentMethods,
    StoreCredit,
}

#[derive(serde::Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReturnStatus {
    Canceled,
    Closed,
    Declined,
    Open,
    Requested,
    #[serde(other)]
    Unknown,
}

impl fmt::Display for ReturnStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReturnStatus::Canceled => write!(f, "CANCELED"),
            ReturnStatus::Closed => write!(f, "CLOSED"),
            ReturnStatus::Declined => write!(f, "DECLINED"),
            ReturnStatus::Open => write!(f, "OPEN"),
            ReturnStatus::Requested => write!(f, "REQUESTED"),
            ReturnStatus::Unknown => write!(f, "UNKNOWN"),
        }
    }
}

#[derive(serde::Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReturnErrorCode {
    AlreadyExists,
    Blank,
    CreationFailed,
    EqualTo,
    FeatureNotEnabled,
    GreaterThan,
    GreaterThanOrEqualTo,
    Inclusion,
    IncompatibleWithStandardPolicy,
    InternalError,
    Invalid,
    InvalidState,
    LessThan,
    LessThanOrEqualTo,
    MissingPermission,
    NotANumber,
    NotEditable,
    NotFound,
    NotificationFailed,
    Present,
    Taken,
    TooBig,
    TooLong,
    TooManyArguments,
    TooShort,
    WrongLength,
    IdempotencyConcurrentRequest,
    IdempotencyKeyParameterMismatch,
    #[serde(other)]
    Unknown,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct ReturnRef {
    pub id: String,
    pub status: ReturnStatus,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct ReturnUserError {
    #[serde(default)]
    pub field: Option<Vec<String>>,
    pub message: String,
    #[serde(default)]
    pub code: Option<ReturnErrorCode>,
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReturnPayload {
    #[serde(rename = "return")]
    pub return_ref: Option<ReturnRef>,
    #[serde(default)]
    pub user_errors: Vec<ReturnUserError>,
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReturnRequestResp {
    pub return_request: ReturnPayload,
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReturnCreateResp {
    pub return_create: ReturnPayload,
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReturnApproveRequestResp {
    pub return_approve_request: ReturnPayload,
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReturnDeclineRequestResp {
    pub return_decline_request: ReturnPayload,
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReturnCancelResp {
    pub return_cancel: ReturnPayload,
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RemoveFromReturnResp {
    pub remove_from_return: ReturnPayload,
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReturnProcessResp {
    pub return_process: ReturnPayload,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct ReturnStatusResp {
    #[serde(rename = "return")]
    pub return_ref: Option<ReturnRef>,
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReturnableFulfillmentsResp {
    pub returnable_fulfillments: Connection<ReturnableFulfillment>,
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReturnableFulfillmentResp {
    pub returnable_fulfillment: Option<ReturnableFulfillment>,
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReturnableFulfillment {
    pub id: String,
    pub returnable_fulfillment_line_items: Connection<ReturnableFulfillmentLineItem>,
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReturnableFulfillmentLineItem {
    pub quantity: i32,
    pub fulfillment_line_item: FulfillmentLineItemRef,
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FulfillmentLineItemRef {
    pub id: String,
    pub line_item: Node,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct ProcessableReturnResp {
    #[serde(rename = "return")]
    pub return_ref: Option<ProcessableReturn>,
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProcessableReturn {
    pub id: String,
    pub status: ReturnStatus,
    pub return_line_items: Connection<ProcessableReturnLine>,
    pub exchange_line_items: Connection<ProcessableExchangeLine>,
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProcessableReturnLine {
    pub id: String,
    pub processable_quantity: i32,
    pub fulfillment_line_item: Option<FulfillmentLineItemRef>,
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProcessableExchangeLine {
    pub id: String,
    pub processable_quantity: i32,
    pub variant_id: Option<String>,
}

pub trait ProcessableLine {
    fn id(&self) -> &str;
    fn processable_quantity(&self) -> i32;

    fn processable(&self) -> Option<ReturnLineQuantityInput> {
        let quantity = self.processable_quantity();
        (quantity > 0).then(|| ReturnLineQuantityInput {
            id: self.id().to_string(),
            quantity,
        })
    }
}

impl ProcessableLine for ProcessableReturnLine {
    fn id(&self) -> &str {
        &self.id
    }

    fn processable_quantity(&self) -> i32 {
        self.processable_quantity
    }
}

impl ProcessableLine for ProcessableExchangeLine {
    fn id(&self) -> &str {
        &self.id
    }

    fn processable_quantity(&self) -> i32 {
        self.processable_quantity
    }
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct ReturnFinancialOutcomeResp {
    #[serde(rename = "return")]
    pub return_ref: Option<ReturnFinancialOutcome>,
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReturnFinancialOutcome {
    pub suggested_financial_outcome: Option<SuggestedReturnFinancialOutcome>,
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SuggestedReturnFinancialOutcome {
    pub financial_transfer: Option<ReturnOutcomeFinancialTransfer>,
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(tag = "__typename")]
pub enum ReturnOutcomeFinancialTransfer {
    #[serde(rename_all = "camelCase")]
    RefundReturnOutcome {
        suggested_transactions: Vec<SuggestedOrderTransaction>,
    },
    InvoiceReturnOutcome {
        amount: MoneyBag,
    },
    #[serde(other)]
    Unknown,
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SuggestedOrderTransaction {
    pub amount_set: MoneyBag,
    pub parent_transaction: Option<Node>,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct ReturnRefundsResp {
    #[serde(rename = "return")]
    pub return_ref: Option<ReturnRefunds>,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct ReturnRefunds {
    pub refunds: Connection<Refund>,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct Refund {
    pub transactions: Connection<OrderTransaction>,
}

#[cfg(test)]
mod graphql_shape_tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn a_request_line_names_its_reason_definition_by_id() {
        let line = ReturnLineItemInput {
            fulfillment_line_item_id: "gid://shopify/FulfillmentLineItem/1".to_string(),
            quantity: 2,
            return_reason_definition_id: Some("gid://shopify/ReturnReasonDefinition/3".to_string()),
            customer_note: None,
        };
        assert_eq!(
            serde_json::to_value(&line).expect("serializes"),
            json!({
                "fulfillmentLineItemId": "gid://shopify/FulfillmentLineItem/1",
                "quantity": 2,
                "returnReasonDefinitionId": "gid://shopify/ReturnReasonDefinition/3"
            })
        );
    }

    #[test]
    fn process_input_serializes_the_shape_shopify_validates() {
        let line = |id: &str| ReturnLineQuantityInput {
            id: id.to_string(),
            quantity: 1,
        };
        let refund = ReturnProcessInput {
            return_id: "gid://shopify/Return/1".to_string(),
            return_line_items: vec![line("gid://shopify/ReturnLineItem/1")],
            exchange_line_items: vec![line("gid://shopify/ExchangeLineItem/1")],
            financial_transfer: Some(ReturnProcessFinancialTransferInput {
                issue_refund: ReturnProcessRefundInput {
                    order_transactions: vec![ReturnRefundOrderTransactionInput {
                        parent_id: "gid://shopify/OrderTransaction/1".to_string(),
                        transaction_amount: MoneyInput {
                            amount: "1.00".to_string(),
                            currency_code: "INR".to_string(),
                        },
                    }],
                },
            }),
            notify_customer: false,
        };
        assert_eq!(
            serde_json::to_value(&refund).expect("serializes"),
            json!({
                "returnId": "gid://shopify/Return/1",
                "returnLineItems": [{ "id": "gid://shopify/ReturnLineItem/1", "quantity": 1 }],
                "exchangeLineItems": [{ "id": "gid://shopify/ExchangeLineItem/1", "quantity": 1 }],
                "financialTransfer": { "issueRefund": { "orderTransactions": [{
                    "parentId": "gid://shopify/OrderTransaction/1",
                    "transactionAmount": { "amount": "1.00", "currencyCode": "INR" }
                }] } },
                "notifyCustomer": false
            })
        );
        let unpaid = ReturnProcessInput {
            financial_transfer: None,
            ..refund
        };
        assert!(serde_json::to_value(&unpaid)
            .expect("serializes")
            .get("financialTransfer")
            .is_none());
    }

    #[test]
    fn enums_serialize_as_shopify_spells_them() {
        assert_eq!(json!(ReturnDeclineReason::Other), json!("OTHER"));
        assert_eq!(
            json!(RefundMethodAllocation::OriginalPaymentMethods),
            json!("ORIGINAL_PAYMENT_METHODS")
        );
    }

    #[test]
    fn a_payload_carries_its_return_or_typed_user_errors() {
        let moved: ReturnProcessResp = serde_json::from_value(json!({
            "returnProcess": { "return": { "id": "gid://shopify/Return/1", "status": "CLOSED" }, "userErrors": [] }
        }))
        .expect("parses");
        let held = moved.return_process.return_ref.expect("return");
        assert_eq!(held.status, ReturnStatus::Closed);
        assert_eq!(held.status.to_string(), "CLOSED");

        let refused: ReturnCancelResp = serde_json::from_value(json!({
            "returnCancel": { "return": null, "userErrors": [
                { "field": ["id"], "message": "busy", "code": "IDEMPOTENCY_CONCURRENT_REQUEST" },
                { "field": null, "message": "state", "code": "INVALID_STATE" },
                { "field": null, "message": "new", "code": "SOMETHING_SHOPIFY_ADDED" },
                { "field": null, "message": "bare" }
            ] }
        }))
        .expect("parses");
        let codes: Vec<Option<ReturnErrorCode>> = refused
            .return_cancel
            .user_errors
            .iter()
            .map(|error| error.code)
            .collect();
        assert_eq!(
            codes,
            vec![
                Some(ReturnErrorCode::IdempotencyConcurrentRequest),
                Some(ReturnErrorCode::InvalidState),
                Some(ReturnErrorCode::Unknown),
                None
            ]
        );
        assert!(refused.return_cancel.return_ref.is_none());
    }

    #[test]
    fn an_unknown_status_parses_rather_than_failing_the_response() {
        let resp: ReturnStatusResp = serde_json::from_value(
            json!({ "return": { "id": "gid://shopify/Return/1", "status": "ARCHIVED" } }),
        )
        .expect("parses");
        assert_eq!(
            resp.return_ref.map(|found| found.status),
            Some(ReturnStatus::Unknown)
        );
    }

    #[test]
    fn returnable_fulfillments_parse_with_their_cursors() {
        let resp: ReturnableFulfillmentsResp = serde_json::from_value(json!({
            "returnableFulfillments": {
                "nodes": [{
                    "id": "gid://shopify/ReturnableFulfillment/1",
                    "returnableFulfillmentLineItems": {
                        "nodes": [{ "quantity": 2, "fulfillmentLineItem": {
                            "id": "gid://shopify/FulfillmentLineItem/7",
                            "lineItem": { "id": "gid://shopify/LineItem/5" }
                        } }],
                        "pageInfo": { "hasNextPage": true, "endCursor": "line-cursor" }
                    }
                }],
                "pageInfo": { "hasNextPage": false, "endCursor": null }
            }
        }))
        .expect("parses");
        let page = resp.returnable_fulfillments;
        assert!(!page.page_info.has_next_page);
        let lines = &page.nodes[0].returnable_fulfillment_line_items;
        assert_eq!(lines.page_info.end_cursor.as_deref(), Some("line-cursor"));
        assert_eq!(
            lines.nodes[0].fulfillment_line_item.line_item.id,
            "gid://shopify/LineItem/5"
        );
    }

    #[test]
    fn processable_lines_offer_only_what_is_left_to_process() {
        let resp: ProcessableReturnResp = serde_json::from_value(json!({
            "return": {
                "id": "gid://shopify/Return/1",
                "status": "OPEN",
                "returnLineItems": {
                    "nodes": [
                        { "id": "gid://shopify/ReturnLineItem/1", "processableQuantity": 2,
                          "fulfillmentLineItem": { "id": "gid://shopify/FulfillmentLineItem/7",
                                                   "lineItem": { "id": "gid://shopify/LineItem/5" } } },
                        { "id": "gid://shopify/UnverifiedReturnLineItem/2", "processableQuantity": 0 }
                    ],
                    "pageInfo": { "hasNextPage": false, "endCursor": null }
                },
                "exchangeLineItems": {
                    "nodes": [{ "id": "gid://shopify/ExchangeLineItem/3", "processableQuantity": 1,
                                "variantId": "gid://shopify/ProductVariant/9" }],
                    "pageInfo": { "hasNextPage": false, "endCursor": null }
                }
            }
        }))
        .expect("parses");
        let found = resp.return_ref.expect("return");
        assert_eq!(found.status, ReturnStatus::Open);
        assert!(found.return_line_items.nodes[1]
            .fulfillment_line_item
            .is_none());
        let returns: Vec<ReturnLineQuantityInput> = found
            .return_line_items
            .nodes
            .iter()
            .filter_map(ProcessableLine::processable)
            .collect();
        let exchanges: Vec<ReturnLineQuantityInput> = found
            .exchange_line_items
            .nodes
            .iter()
            .filter_map(ProcessableLine::processable)
            .collect();
        assert_eq!(
            returns,
            vec![ReturnLineQuantityInput {
                id: "gid://shopify/ReturnLineItem/1".to_string(),
                quantity: 2
            }]
        );
        assert_eq!(exchanges[0].id, "gid://shopify/ExchangeLineItem/3");
    }

    #[test]
    fn a_financial_transfer_is_a_refund_an_invoice_or_neither() {
        let outcome = |transfer: serde_json::Value| -> Option<ReturnOutcomeFinancialTransfer> {
            let resp: ReturnFinancialOutcomeResp = serde_json::from_value(json!({
                "return": { "suggestedFinancialOutcome": { "financialTransfer": transfer } }
            }))
            .expect("parses");
            resp.return_ref
                .and_then(|found| found.suggested_financial_outcome)
                .and_then(|suggested| suggested.financial_transfer)
        };
        let Some(ReturnOutcomeFinancialTransfer::RefundReturnOutcome {
            suggested_transactions,
        }) = outcome(json!({
            "__typename": "RefundReturnOutcome",
            "suggestedTransactions": [{
                "amountSet": {
                    "shopMoney": { "amount": "10.00", "currencyCode": "INR" },
                    "presentmentMoney": { "amount": "0.12", "currencyCode": "USD" }
                },
                "parentTransaction": { "id": "gid://shopify/OrderTransaction/4" }
            }]
        }))
        else {
            panic!("expected a refund");
        };
        let suggested = &suggested_transactions[0];
        assert_eq!(
            suggested
                .amount_set
                .presentment_money
                .as_ref()
                .map(MoneyInput::from),
            Some(MoneyInput {
                amount: "0.12".to_string(),
                currency_code: "USD".to_string()
            })
        );
        assert_eq!(
            suggested
                .parent_transaction
                .as_ref()
                .map(|parent| parent.id.as_str()),
            Some("gid://shopify/OrderTransaction/4")
        );

        let Some(ReturnOutcomeFinancialTransfer::InvoiceReturnOutcome { amount }) =
            outcome(json!({
                "__typename": "InvoiceReturnOutcome",
                "amount": { "shopMoney": { "amount": "200.00", "currencyCode": "INR" } }
            }))
        else {
            panic!("expected an invoice");
        };
        assert_eq!(amount.shop_money.amount, "200.00");

        assert!(matches!(
            outcome(json!({ "__typename": "OutcomeShopifyAddsLater" })),
            Some(ReturnOutcomeFinancialTransfer::Unknown)
        ));
        assert!(outcome(json!(null)).is_none());
    }

    #[test]
    fn refunds_parse_their_transactions() {
        let resp: ReturnRefundsResp = serde_json::from_value(json!({
            "return": { "refunds": {
                "nodes": [{ "transactions": {
                    "nodes": [{ "status": "PENDING",
                                "amountSet": { "shopMoney": { "amount": "498.50", "currencyCode": "INR" } } }],
                    "pageInfo": { "hasNextPage": false, "endCursor": null }
                } }],
                "pageInfo": { "hasNextPage": false, "endCursor": null }
            } }
        }))
        .expect("parses");
        let refunds = resp.return_ref.expect("return").refunds;
        let transaction = &refunds.nodes[0].transactions.nodes[0];
        assert_eq!(transaction.status.as_deref(), Some("PENDING"));
        assert_eq!(transaction.amount_set.shop_money.amount, "498.50");
    }
}
