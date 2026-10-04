use crate::common::ServiceContext;
use crate::{
    common::{http::execute_graphql, types::APIError},
    types::returns::{
        ExchangeLineItemInput, ProcessableReturnResp, RefundMethodAllocation, RemoveFromReturnResp,
        ReturnApproveRequestResp, ReturnCancelResp, ReturnCreateLineItemInput, ReturnCreateResp,
        ReturnDeclineReason, ReturnDeclineRequestResp, ReturnFinancialOutcomeResp,
        ReturnLineItemInput, ReturnLineQuantityInput, ReturnProcessInput, ReturnProcessResp,
        ReturnReasonDefinitionsResp, ReturnRefundsResp, ReturnRequestResp, ReturnStatusResp,
        ReturnableFulfillmentResp, ReturnableFulfillmentsResp,
    },
};

use serde_json::json;

pub async fn request_return(
    ctx: &ServiceContext,
    order_id: &str,
    line_items: &[ReturnLineItemInput],
) -> Result<ReturnRequestResp, APIError> {
    let query = r#"
        mutation returnRequest($input: ReturnRequestInput!) {
            returnRequest(input: $input) {
                return {
                    id
                    status
                }
                userErrors {
                    field
                    message
                    code
                }
            }
        }
    "#;

    let variables = json!({
        "input": {
            "orderId": order_id,
            "returnLineItems": line_items
        }
    });

    execute_graphql(ctx, query, variables).await
}

pub async fn create_return(
    ctx: &ServiceContext,
    order_id: &str,
    return_line_items: &[ReturnCreateLineItemInput],
    exchange_line_items: &[ExchangeLineItemInput],
) -> Result<ReturnCreateResp, APIError> {
    let query = r#"
        mutation returnCreate($returnInput: ReturnInput!) {
            returnCreate(returnInput: $returnInput) {
                return {
                    id
                    status
                }
                userErrors {
                    field
                    message
                    code
                }
            }
        }
    "#;

    let variables = json!({
        "returnInput": {
            "orderId": order_id,
            "returnLineItems": return_line_items,
            "exchangeLineItems": exchange_line_items
        }
    });

    execute_graphql(ctx, query, variables).await
}

pub async fn approve_return(
    ctx: &ServiceContext,
    return_id: &str,
) -> Result<ReturnApproveRequestResp, APIError> {
    let query = r#"
        mutation returnApproveRequest($input: ReturnApproveRequestInput!) {
            returnApproveRequest(input: $input) {
                return {
                    id
                    status
                }
                userErrors {
                    field
                    message
                    code
                }
            }
        }
    "#;

    let variables = json!({ "input": { "id": return_id } });

    execute_graphql(ctx, query, variables).await
}

pub async fn decline_return(
    ctx: &ServiceContext,
    return_id: &str,
    reason: ReturnDeclineReason,
    note: Option<&str>,
) -> Result<ReturnDeclineRequestResp, APIError> {
    let query = r#"
        mutation returnDeclineRequest($input: ReturnDeclineRequestInput!) {
            returnDeclineRequest(input: $input) {
                return {
                    id
                    status
                }
                userErrors {
                    field
                    message
                    code
                }
            }
        }
    "#;

    let variables = json!({
        "input": {
            "id": return_id,
            "declineReason": reason,
            "declineNote": note
        }
    });

    execute_graphql(ctx, query, variables).await
}

pub async fn cancel_return(
    ctx: &ServiceContext,
    return_id: &str,
) -> Result<ReturnCancelResp, APIError> {
    let query = r#"
        mutation returnCancel($id: ID!) {
            returnCancel(id: $id) {
                return {
                    id
                    status
                }
                userErrors {
                    field
                    message
                    code
                }
            }
        }
    "#;

    execute_graphql(ctx, query, json!({ "id": return_id })).await
}

pub async fn remove_from_return(
    ctx: &ServiceContext,
    return_id: &str,
    return_line_items: &[ReturnLineQuantityInput],
    exchange_line_items: &[ReturnLineQuantityInput],
) -> Result<RemoveFromReturnResp, APIError> {
    let query = r#"
        mutation removeFromReturn(
            $returnId: ID!
            $returnLineItems: [ReturnLineItemRemoveFromReturnInput!]
            $exchangeLineItems: [ExchangeLineItemRemoveFromReturnInput!]
        ) {
            removeFromReturn(
                returnId: $returnId
                returnLineItems: $returnLineItems
                exchangeLineItems: $exchangeLineItems
            ) {
                return {
                    id
                    status
                }
                userErrors {
                    field
                    message
                    code
                }
            }
        }
    "#;

    let returns: Vec<_> = return_line_items
        .iter()
        .map(|line| json!({ "returnLineItemId": line.id, "quantity": line.quantity }))
        .collect();
    let exchanges: Vec<_> = exchange_line_items
        .iter()
        .map(|line| json!({ "exchangeLineItemId": line.id, "quantity": line.quantity }))
        .collect();
    let variables = json!({
        "returnId": return_id,
        "returnLineItems": returns,
        "exchangeLineItems": exchanges
    });

    execute_graphql(ctx, query, variables).await
}

pub async fn get_return_status(
    ctx: &ServiceContext,
    return_id: &str,
) -> Result<ReturnStatusResp, APIError> {
    let query = r#"
        query returnStatus($id: ID!) {
            return(id: $id) {
                id
                status
            }
        }
    "#;

    execute_graphql(ctx, query, json!({ "id": return_id })).await
}

pub async fn get_return_reason_definitions(
    ctx: &ServiceContext,
    first: u32,
    after: Option<&str>,
) -> Result<ReturnReasonDefinitionsResp, APIError> {
    let query = r#"
        query returnReasonDefinitions($first: Int!, $after: String) {
            returnReasonDefinitions(first: $first, after: $after) {
                nodes { id handle name }
                pageInfo { hasNextPage endCursor }
            }
        }
    "#;

    execute_graphql(ctx, query, json!({ "first": first, "after": after })).await
}

pub async fn get_returnable_fulfillments(
    ctx: &ServiceContext,
    order_id: &str,
    first: u32,
    lines: u32,
    after: Option<&str>,
) -> Result<ReturnableFulfillmentsResp, APIError> {
    let query = r#"
        query returnableFulfillments($orderId: ID!, $first: Int!, $lines: Int!, $after: String) {
            returnableFulfillments(orderId: $orderId, first: $first, after: $after) {
                nodes {
                    id
                    returnableFulfillmentLineItems(first: $lines) {
                        nodes {
                            quantity
                            fulfillmentLineItem { id lineItem { id } }
                        }
                        pageInfo { hasNextPage endCursor }
                    }
                }
                pageInfo { hasNextPage endCursor }
            }
        }
    "#;

    let variables = json!({
        "orderId": order_id,
        "first": first,
        "lines": lines,
        "after": after
    });

    execute_graphql(ctx, query, variables).await
}

pub async fn get_returnable_fulfillment(
    ctx: &ServiceContext,
    fulfillment_id: &str,
    lines: u32,
    after: Option<&str>,
) -> Result<ReturnableFulfillmentResp, APIError> {
    let query = r#"
        query returnableFulfillment($id: ID!, $lines: Int!, $after: String) {
            returnableFulfillment(id: $id) {
                id
                returnableFulfillmentLineItems(first: $lines, after: $after) {
                    nodes {
                        quantity
                        fulfillmentLineItem { id lineItem { id } }
                    }
                    pageInfo { hasNextPage endCursor }
                }
            }
        }
    "#;

    let variables = json!({ "id": fulfillment_id, "lines": lines, "after": after });

    execute_graphql(ctx, query, variables).await
}

pub async fn get_processable_return(
    ctx: &ServiceContext,
    return_id: &str,
    lines: u32,
) -> Result<ProcessableReturnResp, APIError> {
    let query = r#"
        query processableReturn($id: ID!, $lines: Int!) {
            return(id: $id) {
                id
                status
                returnLineItems(first: $lines) {
                    nodes {
                        id
                        processableQuantity
                        ... on ReturnLineItem {
                            fulfillmentLineItem { id lineItem { id } }
                        }
                    }
                    pageInfo { hasNextPage endCursor }
                }
                exchangeLineItems(first: $lines) {
                    nodes { id processableQuantity variantId }
                    pageInfo { hasNextPage endCursor }
                }
            }
        }
    "#;

    execute_graphql(ctx, query, json!({ "id": return_id, "lines": lines })).await
}

pub async fn get_suggested_financial_outcome(
    ctx: &ServiceContext,
    return_id: &str,
    return_line_items: &[ReturnLineQuantityInput],
    exchange_line_items: &[ReturnLineQuantityInput],
    allocation: RefundMethodAllocation,
) -> Result<ReturnFinancialOutcomeResp, APIError> {
    let query = r#"
        query suggestedFinancialOutcome(
            $id: ID!
            $returnLineItems: [SuggestedOutcomeReturnLineItemInput!]!
            $exchangeLineItems: [SuggestedOutcomeExchangeLineItemInput!]!
            $refundMethodAllocation: RefundMethodAllocation!
        ) {
            return(id: $id) {
                suggestedFinancialOutcome(
                    returnLineItems: $returnLineItems
                    exchangeLineItems: $exchangeLineItems
                    refundMethodAllocation: $refundMethodAllocation
                ) {
                    financialTransfer {
                        __typename
                        ... on RefundReturnOutcome {
                            suggestedTransactions {
                                amountSet {
                                    shopMoney { amount currencyCode }
                                    presentmentMoney { amount currencyCode }
                                }
                                parentTransaction { id }
                            }
                        }
                        ... on InvoiceReturnOutcome {
                            amount {
                                shopMoney { amount currencyCode }
                                presentmentMoney { amount currencyCode }
                            }
                        }
                    }
                }
            }
        }
    "#;

    let variables = json!({
        "id": return_id,
        "returnLineItems": return_line_items,
        "exchangeLineItems": exchange_line_items,
        "refundMethodAllocation": allocation
    });

    execute_graphql(ctx, query, variables).await
}

pub async fn get_return_refunds(
    ctx: &ServiceContext,
    return_id: &str,
    refunds: u32,
    transactions: u32,
) -> Result<ReturnRefundsResp, APIError> {
    let query = r#"
        query returnRefunds($id: ID!, $refunds: Int!, $transactions: Int!) {
            return(id: $id) {
                refunds(first: $refunds) {
                    nodes {
                        transactions(first: $transactions) {
                            nodes {
                                status
                                amountSet { shopMoney { amount currencyCode } }
                            }
                            pageInfo { hasNextPage endCursor }
                        }
                    }
                    pageInfo { hasNextPage endCursor }
                }
            }
        }
    "#;

    let variables = json!({
        "id": return_id,
        "refunds": refunds,
        "transactions": transactions
    });

    execute_graphql(ctx, query, variables).await
}

pub async fn process_return(
    ctx: &ServiceContext,
    input: &ReturnProcessInput,
    idempotency_key: &str,
) -> Result<ReturnProcessResp, APIError> {
    let query = r#"
        mutation returnProcess($input: ReturnProcessInput!, $idempotencyKey: String!) {
            returnProcess(input: $input) @idempotent(key: $idempotencyKey) {
                return {
                    id
                    status
                }
                userErrors {
                    field
                    message
                    code
                }
            }
        }
    "#;

    let variables = json!({
        "input": input,
        "idempotencyKey": idempotency_key
    });

    execute_graphql(ctx, query, variables).await
}
