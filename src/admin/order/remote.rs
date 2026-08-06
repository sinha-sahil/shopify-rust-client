use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use std::collections::HashMap;

use crate::{
    common::{
        http::{execute_graphql, http_client},
        types::APIError,
        utils::parse_response_from_text,
        ServiceContext,
    },
    types::order::{
        GetOrderResp, LineItemVariantDetail, OrderLineItemsVariantResponse, OrderQueryResp,
        PatchOrderRequest,
    },
};
use serde_json::json;

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

/// Fetch line-item variant details (selected options and media) for an order.
///
/// Queries order line items with variant information including selected options
/// and image media. LineItem has no `legacyResourceId` field — the numeric suffix
/// of its GID (`gid://shopify/LineItem/<numeric>`) is the join key that matches
/// the REST order payload's line-item id.
///
/// Returns a map keyed by the numeric line-item id extracted from the GID.
pub async fn get_order_line_items_variant(
    ctx: &ServiceContext,
    order_id: &str,
) -> Result<HashMap<String, LineItemVariantDetail>, APIError> {
    let query = r#"
        query OrderLineItemsVariantMedia($id: ID!) {
            order(id: $id) {
                id
                lineItems(first: 50) {
                    edges {
                        node {
                            id
                            variant {
                                id
                                selectedOptions {
                                    name
                                    value
                                }
                                media(first: 1) {
                                    edges {
                                        node {
                                            __typename
                                            ... on MediaImage {
                                                image {
                                                    url
                                                }
                                            }
                                        }
                                    }
                                }
                                product {
                                    featuredMedia {
                                        __typename
                                        ... on MediaImage {
                                            image {
                                                url
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    "#;

    let variables = json!({
        "id": order_id
    });

    let response: OrderLineItemsVariantResponse = execute_graphql(ctx, query, variables).await?;

    let mut result = HashMap::new();

    if let Some(order) = response.order {
        for edge in order.line_items.edges {
            let line_item_id = &edge.node.id;
            // Extract numeric suffix from GID (gid://shopify/LineItem/<numeric>)
            if let Some(numeric_id) = line_item_id.split('/').next_back() {
                if let Some(variant) = edge.node.variant {
                    // Extract image URL from variant media or product featured media
                    let image_url = if let Some(media) = variant.media {
                        if let Some(edge) = media.edges.first() {
                            match &edge.node {
                                crate::types::order::MediaNode::MediaImage { image } => {
                                    Some(image.url.clone())
                                }
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                    .or_else(|| {
                        variant
                            .product
                            .featured_media
                            .as_ref()
                            .map(|featured| match featured {
                                crate::types::order::FeaturedMedia::MediaImage { image } => {
                                    image.url.clone()
                                }
                            })
                    });

                    result.insert(
                        numeric_id.to_string(),
                        LineItemVariantDetail {
                            variant_id: variant.id,
                            selected_options: variant.selected_options,
                            image_url,
                        },
                    );
                }
            }
        }
    }

    Ok(result)
}
