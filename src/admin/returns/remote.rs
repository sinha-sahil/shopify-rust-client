use crate::common::ServiceContext;
use crate::{
    common::{http::execute_graphql, types::APIError},
    types::returns::{
        OrderTransactionInput, ReturnLineItemInput, ReturnProcessLineInput, ReturnProcessResp,
        ReturnRequestResp,
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

pub async fn process_return(
    ctx: &ServiceContext,
    return_id: &str,
    return_line_items: &[ReturnProcessLineInput],
    order_transactions: &[OrderTransactionInput],
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
                }
            }
        }
    "#;

    let variables = json!({
        "input": {
            "returnId": return_id,
            "returnLineItems": return_line_items,
            "financialTransfer": {
                "issueRefund": { "orderTransactions": order_transactions }
            }
        },
        "idempotencyKey": idempotency_key
    });

    execute_graphql(ctx, query, variables).await
}
