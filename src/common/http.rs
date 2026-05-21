use crate::common::types::{APIError, RequestCallbacks};
use crate::common::ServiceContext;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use std::sync::OnceLock;

#[derive(serde::Serialize)]
struct GraphQLRequest<'a> {
    query: &'a str,
    variables: serde_json::Value,
}

#[derive(serde::Deserialize)]
struct GraphQLResponse<T> {
    data: Option<T>,
    errors: Option<Vec<GraphQLError>>,
}

#[derive(serde::Deserialize, Debug)]
struct GraphQLError {
    message: String,
    #[serde(default)]
    extensions: Option<serde_json::Value>,
}

impl GraphQLError {
    fn format_with_code(&self) -> String {
        let code = self
            .extensions
            .as_ref()
            .and_then(|ext| ext.get("code"))
            .and_then(|v| v.as_str());
        match code {
            Some(c) => format!("[{}] {}", c, self.message),
            None => self.message.clone(),
        }
    }
}

/// Shared HTTP client. Constructed once per process so connection pooling and
/// keep-alive work — `reqwest::Client::new()` per-call would defeat both.
/// Exposed `pub(crate)` so REST handlers (e.g. admin `order`) can reuse the
/// same pool instead of allocating their own client.
pub(crate) fn http_client() -> &'static reqwest::Client {
    static CLIENT: OnceLock<reqwest::Client> = OnceLock::new();
    CLIENT.get_or_init(reqwest::Client::new)
}

/// Generic GraphQL executor.
pub async fn execute_graphql_raw<T: serde::de::DeserializeOwned>(
    endpoint: &str,
    auth_header_name: &'static str,
    auth_token: &str,
    callbacks: &RequestCallbacks,
    query: &str,
    variables: serde_json::Value,
) -> Result<T, APIError> {
    let request_body = GraphQLRequest { query, variables };

    let body_str = serde_json::to_string(&request_body).unwrap_or_default();

    let mut callback_headers = HeaderMap::new();
    callback_headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    callbacks.call_before(endpoint, Some(&body_str), &callback_headers);

    let response = http_client()
        .post(endpoint)
        .header(auth_header_name, auth_token)
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await;

    match response {
        Ok(resp) => {
            let response_headers = resp.headers().clone();
            let response_text = match resp.text().await {
                Ok(text) => text,
                Err(e) => {
                    let error_msg = format!("<failed to read response body: {}>", e);
                    callbacks.call_after(endpoint, &error_msg, &response_headers);
                    return Err(APIError::FailedToParse);
                }
            };

            callbacks.call_after(endpoint, &response_text, &response_headers);

            let graphql_response = serde_json::from_str::<GraphQLResponse<T>>(&response_text)
                .map_err(|_| APIError::FailedToParse)?;

            if let Some(errors) = graphql_response.errors {
                let error_messages: Vec<String> =
                    errors.iter().map(GraphQLError::format_with_code).collect();
                return Err(APIError::ServerError {
                    errors: error_messages.join("; "),
                });
            }

            graphql_response.data.ok_or(APIError::FailedToParse)
        }
        Err(e) => {
            let error_msg = format!("<network error: {}>", e);
            callbacks.call_after(endpoint, &error_msg, &HeaderMap::new());
            Err(APIError::NetworkError)
        }
    }
}

/// Admin Shopify GraphQL endpoint executor.
pub async fn execute_graphql<T: serde::de::DeserializeOwned>(
    ctx: &ServiceContext,
    query: &str,
    variables: serde_json::Value,
) -> Result<T, APIError> {
    let endpoint = format!(
        "{}/admin/api/{}/graphql.json",
        ctx.shop_url.trim_end_matches('/'),
        ctx.version
    );
    execute_graphql_raw(
        &endpoint,
        "X-Shopify-Access-Token",
        &ctx.access_token,
        &ctx.callbacks,
        query,
        variables,
    )
    .await
}

/// Storefront Shopify GraphQL endpoint executor.
#[cfg(feature = "storefront")]
pub async fn execute_storefront_graphql<T: serde::de::DeserializeOwned>(
    ctx: &ServiceContext,
    query: &str,
    variables: serde_json::Value,
) -> Result<T, APIError> {
    let endpoint = format!(
        "{}/api/{}/graphql.json",
        ctx.shop_url.trim_end_matches('/'),
        ctx.version
    );
    execute_graphql_raw(
        &endpoint,
        "X-Shopify-Storefront-Access-Token",
        &ctx.access_token,
        &ctx.callbacks,
        query,
        variables,
    )
    .await
}
