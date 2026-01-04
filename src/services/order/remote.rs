use crate::{
    common::{types::APIError, utils::parse_response},
    services::order::types::{GetOrderResp, OrderQueryResp, PatchOrderRequest},
};

pub async fn patch_order(
    shop_url: &String,
    version: &String,
    access_token: &String,
    order_id: &String,
    patch_request: &PatchOrderRequest,
) -> Result<GetOrderResp, APIError> {
    let endpoint = format!(
        "{}/admin/api/{}/orders/{}.json",
        shop_url, version, order_id
    );

    let client = reqwest::Client::new();
    let response = client
        .put(&endpoint)
        .header("X-Shopify-Access-Token", access_token)
        .json(&patch_request)
        .send()
        .await;

    match response {
        Ok(resp) => parse_response::<GetOrderResp>(resp).await,
        Err(_) => Err(APIError::NetworkError),
    }
}

pub async fn get_order_with_name(
    shop_url: &String,
    version: &String,
    order_name: &String,
    access_token: &String,
) -> Result<OrderQueryResp, APIError> {
    let endpoint = format!(
        "{}/admin/api/{}/orders.json?query=name:%23{}&status=any",
        shop_url, version, order_name
    );
    let client = reqwest::Client::new();
    let response = client
        .get(&endpoint)
        .header("X-Shopify-Access-Token", access_token)
        .send()
        .await;

    match response {
        Ok(resp) => parse_response::<OrderQueryResp>(resp).await,
        Err(_) => Err(APIError::NetworkError),
    }
}

pub async fn get_order_with_id(
    shop_url: &String,
    version: &String,
    access_token: &String,
    order_id: &String,
) -> Result<GetOrderResp, APIError> {
    let endpoint = format!(
        "{}/admin/api/{}/orders/{}.json",
        shop_url, version, order_id
    );

    let client = reqwest::Client::new();
    let response = client
        .get(&endpoint)
        .header("X-Shopify-Access-Token", access_token)
        .send()
        .await;

    match response {
        Ok(resp) => parse_response::<GetOrderResp>(resp).await,
        Err(_) => Err(APIError::NetworkError),
    }
}
