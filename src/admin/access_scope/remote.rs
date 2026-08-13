use reqwest::header::HeaderMap;

use crate::{
    admin::generated::types::access_scope::AccessScopesResp,
    common::{http::http_client, types::APIError, utils::parse_response_from_text, ServiceContext},
};

pub async fn list_access_scopes(ctx: &ServiceContext) -> Result<AccessScopesResp, APIError> {
    let endpoint = format!(
        "{}/admin/oauth/access_scopes.json",
        ctx.shop_url.trim_end_matches('/')
    );

    let callback_headers = HeaderMap::new();

    ctx.callbacks
        .call_before(&endpoint, None, &callback_headers);

    let response = http_client()
        .get(&endpoint)
        .header("X-Shopify-Access-Token", &*ctx.access_token)
        .header("Content-Type", "application/json")
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

            parse_response_from_text::<AccessScopesResp>(&response_text)
        }
        Err(e) => {
            let error_msg = format!("<network error: {}>", e);
            ctx.callbacks
                .call_after(&endpoint, &error_msg, &HeaderMap::new());
            Err(APIError::NetworkError)
        }
    }
}
