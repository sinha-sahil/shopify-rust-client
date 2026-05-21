use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};

use crate::{
    common::{http::http_client, types::APIError, utils::parse_response_from_text, ServiceContext},
    types::order::{GetOrderResp, OrderQueryResp, PatchOrderRequest},
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
