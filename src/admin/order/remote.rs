use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};

use crate::{
    common::{
        http::{execute_graphql, http_client},
        types::APIError,
        utils::parse_response_from_text,
        ServiceContext,
    },
    types::order::{
        GetOrderResp, OrderDetailByNameResp, OrderDetailResp, OrderDiscountsAndTransactionsResp,
        OrderQueryResp, PatchOrderRequest,
    },
};

pub async fn patch_order(
    ctx: &ServiceContext,
    order_id: &String,
    patch_request: &PatchOrderRequest,
) -> Result<GetOrderResp, APIError> {
    let endpoint = format!(
        "{}/admin/api/{}/orders/{}.json",
        ctx.shop_url.trim_end_matches('/'),
        ctx.version,
        order_id
    );

    let body_str = serde_json::to_string(&patch_request).unwrap_or_default();

    let mut callback_headers = HeaderMap::new();
    callback_headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    ctx.callbacks
        .call_before(&endpoint, Some(&body_str), &callback_headers);

    let response = http_client()
        .put(&endpoint)
        .header("X-Shopify-Access-Token", &*ctx.access_token)
        .header("Content-Type", "application/json")
        .json(&patch_request)
        .send()
        .await;

    match response {
        Ok(resp) => {
            let response_headers = resp.headers().clone();
            let response_text = match resp.text().await {
                Ok(text) => text,
                Err(e) => {
                    let error_msg = format!("<failed to read response body: {}>", e);
                    ctx.callbacks
                        .call_after(&endpoint, &error_msg, &response_headers);
                    return Err(APIError::FailedToParse);
                }
            };

            ctx.callbacks
                .call_after(&endpoint, &response_text, &response_headers);

            parse_response_from_text::<GetOrderResp>(&response_text)
        }
        Err(e) => {
            let error_msg = format!("<network error: {}>", e);
            ctx.callbacks
                .call_after(&endpoint, &error_msg, &HeaderMap::new());
            Err(APIError::NetworkError)
        }
    }
}

pub async fn get_order_with_name(
    ctx: &ServiceContext,
    order_name: &String,
) -> Result<OrderQueryResp, APIError> {
    let endpoint = format!(
        "{}/admin/api/{}/orders.json?query=name:%23{}&status=any",
        ctx.shop_url.trim_end_matches('/'),
        ctx.version,
        order_name
    );

    let callback_headers = HeaderMap::new();

    ctx.callbacks
        .call_before(&endpoint, None, &callback_headers);

    let response = http_client()
        .get(&endpoint)
        .header("X-Shopify-Access-Token", &*ctx.access_token)
        .send()
        .await;

    match response {
        Ok(resp) => {
            let response_headers = resp.headers().clone();
            let response_text = match resp.text().await {
                Ok(text) => text,
                Err(e) => {
                    let error_msg = format!("<failed to read response body: {}>", e);
                    ctx.callbacks
                        .call_after(&endpoint, &error_msg, &response_headers);
                    return Err(APIError::FailedToParse);
                }
            };

            ctx.callbacks
                .call_after(&endpoint, &response_text, &response_headers);

            parse_response_from_text::<OrderQueryResp>(&response_text)
        }
        Err(e) => {
            let error_msg = format!("<network error: {}>", e);
            ctx.callbacks
                .call_after(&endpoint, &error_msg, &HeaderMap::new());
            Err(APIError::NetworkError)
        }
    }
}

pub async fn get_order_with_id(
    ctx: &ServiceContext,
    order_id: &String,
) -> Result<GetOrderResp, APIError> {
    let endpoint = format!(
        "{}/admin/api/{}/orders/{}.json",
        ctx.shop_url.trim_end_matches('/'),
        ctx.version,
        order_id
    );

    let callback_headers = HeaderMap::new();

    ctx.callbacks
        .call_before(&endpoint, None, &callback_headers);

    let response = http_client()
        .get(&endpoint)
        .header("X-Shopify-Access-Token", &*ctx.access_token)
        .send()
        .await;

    match response {
        Ok(resp) => {
            let response_headers = resp.headers().clone();
            let response_text = match resp.text().await {
                Ok(text) => text,
                Err(e) => {
                    let error_msg = format!("<failed to read response body: {}>", e);
                    ctx.callbacks
                        .call_after(&endpoint, &error_msg, &response_headers);
                    return Err(APIError::FailedToParse);
                }
            };

            ctx.callbacks
                .call_after(&endpoint, &response_text, &response_headers);

            parse_response_from_text::<GetOrderResp>(&response_text)
        }
        Err(e) => {
            let error_msg = format!("<network error: {}>", e);
            ctx.callbacks
                .call_after(&endpoint, &error_msg, &HeaderMap::new());
            Err(APIError::NetworkError)
        }
    }
}

pub async fn get_order_discounts_and_transactions(
    ctx: &ServiceContext,
    order_gid: &str,
    lines: u32,
) -> Result<OrderDiscountsAndTransactionsResp, APIError> {
    let query = r#"
        query orderDiscountsAndTransactions($id: ID!, $lines: Int!) {
            order(id: $id) {
                discountCodes
                transactions {
                    gateway
                    kind
                    status
                    amountSet { shopMoney { amount currencyCode } }
                }
                lineItems(first: $lines) {
                    nodes {
                        id
                        discountAllocations {
                            allocatedAmountSet { shopMoney { amount currencyCode } }
                        }
                    }
                    pageInfo { hasNextPage }
                }
                cartDiscountAmountSet { shopMoney { amount currencyCode } }
            }
        }
    "#;

    let variables = serde_json::json!({ "id": order_gid, "lines": lines });

    execute_graphql(ctx, query, variables).await
}

const ORDER_DETAIL_FIELDS: &str = r#"
    id
    name
    email
    phone
    createdAt
    currencyCode
    displayFinancialStatus
    taxesIncluded
    discountCodes
    cartDiscountAmountSet { shopMoney { amount currencyCode } }
    customer {
        id
        firstName
        lastName
        defaultEmailAddress { emailAddress }
        defaultPhoneNumber { phoneNumber }
    }
    shippingAddress {
        name firstName lastName phone address1 address2
        city province country countryCodeV2 zip
    }
    transactions {
        gateway
        kind
        status
        amountSet { shopMoney { amount currencyCode } }
    }
    fulfillments(first: $fulfillments) {
        status
        createdAt
        updatedAt
        trackingInfo { company number url }
        fulfillmentLineItems(first: $lines) {
            nodes { quantity lineItem { id } }
        }
    }
    lineItems(first: $lines) {
        nodes {
            id
            title
            variantTitle
            quantity
            unfulfilledQuantity
            sku
            product { id }
            variant { id }
            originalUnitPriceSet { shopMoney { amount currencyCode } }
            totalDiscountSet { shopMoney { amount currencyCode } }
            discountAllocations { allocatedAmountSet { shopMoney { amount currencyCode } } }
        }
        pageInfo { hasNextPage }
    }
"#;

pub async fn get_order_detail(
    ctx: &ServiceContext,
    order_gid: &str,
    lines: u32,
    fulfillments: u32,
) -> Result<OrderDetailResp, APIError> {
    let query = format!(
        "query orderDetail($id: ID!, $lines: Int!, $fulfillments: Int!) {{ order(id: $id) {{ {ORDER_DETAIL_FIELDS} }} }}"
    );

    let variables = serde_json::json!({
        "id": order_gid,
        "lines": lines,
        "fulfillments": fulfillments
    });

    execute_graphql(ctx, &query, variables).await
}

pub async fn find_order_detail_by_name(
    ctx: &ServiceContext,
    name: &str,
    lines: u32,
    fulfillments: u32,
) -> Result<OrderDetailByNameResp, APIError> {
    let query = format!(
        "query orderDetailByName($search: String!, $lines: Int!, $fulfillments: Int!) {{ orders(first: 1, query: $search) {{ nodes {{ {ORDER_DETAIL_FIELDS} }} }} }}"
    );

    let variables = serde_json::json!({
        "search": format!("name:{name}"),
        "lines": lines,
        "fulfillments": fulfillments
    });

    execute_graphql(ctx, &query, variables).await
}
