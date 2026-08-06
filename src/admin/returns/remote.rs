use crate::common::ServiceContext;
use crate::{
    common::{http::execute_graphql, types::APIError},
    types::returns::{
        OrderRefundList, OrderReturnsAndRefunds, OrderReturnsAndRefundsResponse,
        OrderReturnsAndRefundsResponseRaw, RefundMethodAllocation, ReturnApproveRequestInput,
        ReturnApproveRequestPayload, ReturnCancelPayload, ReturnClosePayload, ReturnCreatePayload,
        ReturnDeclineRequestInput, ReturnDeclineRequestPayload, ReturnInput,
        ReturnLineItemRemoveFromReturnInput, ReturnLineItemRemoveFromReturnPayload,
        ReturnProcessInput, ReturnProcessPayload, ReturnReopenPayload, ReturnRequestInput,
        ReturnRequestPayload, ReturnableFulfillmentResponse, SuggestedFinancialOutcomeResponse,
        SuggestedOutcomeExchangeLineItemInput, SuggestedOutcomeReturnLineItemInput,
    },
};

use serde_json::json;

pub async fn return_request(
    ctx: &ServiceContext,
    input: &ReturnRequestInput,
) -> Result<ReturnRequestPayload, APIError> {
    let query = r#"
        mutation ReturnRequest($input: ReturnRequestInput!) {
            returnRequest(input: $input) {
                return {
                    id
                    name
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
        "input": input
    });

    execute_graphql(ctx, query, variables).await
}

pub async fn return_approve_request(
    ctx: &ServiceContext,
    input: &ReturnApproveRequestInput,
) -> Result<ReturnApproveRequestPayload, APIError> {
    let query = r#"
        mutation ReturnApproveRequest($input: ReturnApproveRequestInput!) {
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

    let variables = json!({
        "input": input
    });

    execute_graphql(ctx, query, variables).await
}

pub async fn return_decline_request(
    ctx: &ServiceContext,
    input: &ReturnDeclineRequestInput,
) -> Result<ReturnDeclineRequestPayload, APIError> {
    let query = r#"
        mutation ReturnDeclineRequest($input: ReturnDeclineRequestInput!) {
            returnDeclineRequest(input: $input) {
                return {
                    id
                    status
                    decline {
                        reason
                        note
                    }
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
        "input": input
    });

    execute_graphql(ctx, query, variables).await
}

pub async fn return_create(
    ctx: &ServiceContext,
    input: &ReturnInput,
) -> Result<ReturnCreatePayload, APIError> {
    let query = r#"
        mutation ReturnCreate($returnInput: ReturnInput!) {
            returnCreate(returnInput: $returnInput) {
                return {
                    id
                    name
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
        "returnInput": input
    });

    execute_graphql(ctx, query, variables).await
}

pub async fn return_process(
    ctx: &ServiceContext,
    input: &ReturnProcessInput,
) -> Result<ReturnProcessPayload, APIError> {
    let query = r#"
        mutation ReturnProcess($input: ReturnProcessInput!) {
            returnProcess(input: $input) {
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
        "input": input
    });

    execute_graphql(ctx, query, variables).await
}

pub async fn suggested_financial_outcome(
    ctx: &ServiceContext,
    return_id: &str,
    return_line_items: Vec<SuggestedOutcomeReturnLineItemInput>,
    exchange_line_items: Vec<SuggestedOutcomeExchangeLineItemInput>,
    refund_method_allocation: Option<RefundMethodAllocation>,
) -> Result<SuggestedFinancialOutcomeResponse, APIError> {
    let query = r#"
        query ReturnSuggestedFinancialOutcome(
            $id: ID!
            $refundMethodAllocation: RefundMethodAllocation
            $returnLineItems: [SuggestedOutcomeReturnLineItemInput!]!
            $exchangeLineItems: [SuggestedOutcomeExchangeLineItemInput!]!
        ) {
            return(id: $id) {
                id
                suggestedFinancialOutcome(
                    refundMethodAllocation: $refundMethodAllocation
                    returnLineItems: $returnLineItems
                    exchangeLineItems: $exchangeLineItems
                ) {
                    maximumRefundable {
                        shopMoney {
                            amount
                            currencyCode
                        }
                    }
                    financialTransfer {
                        __typename
                        ... on RefundReturnOutcome {
                            amount {
                                shopMoney {
                                    amount
                                    currencyCode
                                }
                            }
                        }
                        ... on InvoiceReturnOutcome {
                            amount {
                                shopMoney {
                                    amount
                                    currencyCode
                                }
                            }
                        }
                    }
                }
            }
        }
    "#;

    let variables = json!({
        "id": return_id,
        "refundMethodAllocation": refund_method_allocation,
        "returnLineItems": return_line_items,
        "exchangeLineItems": exchange_line_items
    });

    execute_graphql(ctx, query, variables).await
}

pub async fn returnable_fulfillment(
    ctx: &ServiceContext,
    fulfillment_order_id: &str,
    first: i64,
    after: Option<&str>,
) -> Result<ReturnableFulfillmentResponse, APIError> {
    let query = r#"
        query ReturnableFulfillment($id: ID!, $first: Int!, $after: String) {
            returnableFulfillment(id: $id) {
                id
                returnableFulfillmentLineItems(first: $first, after: $after) {
                    nodes {
                        quantity
                        fulfillmentLineItem {
                            id
                            lineItem {
                                id
                                title
                            }
                        }
                    }
                    pageInfo {
                        hasNextPage
                    }
                }
            }
        }
    "#;

    let variables = json!({
        "id": fulfillment_order_id,
        "first": first,
        "after": after
    });

    execute_graphql(ctx, query, variables).await
}

pub async fn order_returns_and_refunds(
    ctx: &ServiceContext,
    order_id: &str,
    returns_first: i64,
    returns_after: Option<&str>,
    refunds_first: i64,
) -> Result<OrderReturnsAndRefundsResponse, APIError> {
    let query = r#"
        query OrderReturnsAndRefunds($id: ID!, $returnsFirst: Int!, $returnsAfter: String, $refundsFirst: Int!) {
            order(id: $id) {
                id
                returns(first: $returnsFirst, after: $returnsAfter) {
                    nodes {
                        id
                        name
                        status
                        returnLineItems(first: 250) {
                            nodes {
                                __typename
                                ... on ReturnLineItem {
                                    quantity
                                    refundableQuantity
                                    refundedQuantity
                                    processableQuantity
                                    processedQuantity
                                    unprocessedQuantity
                                    fulfillmentLineItem {
                                        id
                                        lineItem { id }
                                    }
                                }
                                ... on UnverifiedReturnLineItem {
                                    quantity
                                    refundableQuantity
                                    refundedQuantity
                                    unprocessedQuantity
                                }
                            }
                            pageInfo {
                                hasNextPage
                            }
                        }
                    }
                    pageInfo {
                        hasNextPage
                    }
                }
                refunds(first: $refundsFirst) {
                    id
                    refundLineItems(first: 250) {
                        nodes {
                            id
                            quantity
                            restockType
                            lineItem { id }
                        }
                        pageInfo {
                            hasNextPage
                        }
                    }
                }
            }
        }
    "#;

    let variables = json!({
        "id": order_id,
        "returnsFirst": returns_first,
        "returnsAfter": returns_after,
        "refundsFirst": refunds_first
    });

    let raw_response: OrderReturnsAndRefundsResponseRaw =
        execute_graphql(ctx, query, variables).await?;

    let order = raw_response.order.map(|o| {
        let refunds = o.refunds.clone();
        OrderReturnsAndRefunds {
            id: o.id,
            returns: o.returns,
            refunds: OrderRefundList {
                items: refunds.clone(),
                truncated: refunds.len() == refunds_first as usize,
            },
        }
    });

    Ok(OrderReturnsAndRefundsResponse { order })
}

pub async fn return_close(ctx: &ServiceContext, id: &str) -> Result<ReturnClosePayload, APIError> {
    let query = r#"
        mutation ReturnClose($id: ID!) {
            returnClose(id: $id) {
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
        "id": id
    });

    execute_graphql(ctx, query, variables).await
}

pub async fn return_reopen(
    ctx: &ServiceContext,
    id: &str,
) -> Result<ReturnReopenPayload, APIError> {
    let query = r#"
        mutation ReturnReopen($id: ID!) {
            returnReopen(id: $id) {
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
        "id": id
    });

    execute_graphql(ctx, query, variables).await
}

pub async fn return_cancel(
    ctx: &ServiceContext,
    id: &str,
    notify_customer: Option<bool>,
) -> Result<ReturnCancelPayload, APIError> {
    let query = r#"
        mutation ReturnCancel($id: ID!, $notifyCustomer: Boolean) {
            returnCancel(id: $id, notifyCustomer: $notifyCustomer) {
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
        "id": id,
        "notifyCustomer": notify_customer
    });

    execute_graphql(ctx, query, variables).await
}

pub async fn return_line_item_remove_from_return(
    ctx: &ServiceContext,
    return_id: &str,
    return_line_items: Vec<ReturnLineItemRemoveFromReturnInput>,
) -> Result<ReturnLineItemRemoveFromReturnPayload, APIError> {
    let query = r#"
        mutation ReturnLineItemRemoveFromReturn($returnId: ID!, $returnLineItems: [ReturnLineItemRemoveFromReturnInput!]!) {
            returnLineItemRemoveFromReturn(returnId: $returnId, returnLineItems: $returnLineItems) {
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
        "returnId": return_id,
        "returnLineItems": return_line_items
    });

    execute_graphql(ctx, query, variables).await
}
