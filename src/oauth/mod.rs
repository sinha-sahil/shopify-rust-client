pub mod types;

pub use types::AccessTokenResponse;

use crate::common::http::http_client;
use crate::common::types::APIError;

const TOKEN_EXCHANGE_GRANT_TYPE: &str = "urn:ietf:params:oauth:grant-type:token-exchange";
const ID_TOKEN_TYPE: &str = "urn:ietf:params:oauth:token-type:id_token";
const OFFLINE_ACCESS_TOKEN_TYPE: &str = "urn:shopify:params:oauth:token-type:offline-access-token";

pub async fn exchange_session_token(
    shop_url: &str,
    id_token: &str,
    client_id: &str,
    client_secret: &str,
) -> Result<AccessTokenResponse, APIError> {
    let body = serde_json::json!({
        "client_id": client_id,
        "client_secret": client_secret,
        "grant_type": TOKEN_EXCHANGE_GRANT_TYPE,
        "subject_token": id_token,
        "subject_token_type": ID_TOKEN_TYPE,
        "requested_token_type": OFFLINE_ACCESS_TOKEN_TYPE,
    });

    request_access_token(shop_url, &body).await
}

pub async fn exchange_code(
    shop_url: &str,
    code: &str,
    client_id: &str,
    client_secret: &str,
) -> Result<AccessTokenResponse, APIError> {
    let body = serde_json::json!({
        "client_id": client_id,
        "client_secret": client_secret,
        "code": code,
    });

    request_access_token(shop_url, &body).await
}

async fn request_access_token(
    shop_url: &str,
    body: &serde_json::Value,
) -> Result<AccessTokenResponse, APIError> {
    let endpoint = format!(
        "{}/admin/oauth/access_token",
        shop_url.trim_end_matches('/')
    );

    let response = http_client()
        .post(&endpoint)
        .header("Content-Type", "application/json")
        .json(body)
        .send()
        .await
        .map_err(|_| APIError::NetworkError)?;

    let status = response.status();
    let response_text = response.text().await.map_err(|_| APIError::FailedToParse)?;

    if !status.is_success() {
        return Err(APIError::ServerError {
            errors: format!("{}: {}", status, response_text),
        });
    }

    serde_json::from_str::<AccessTokenResponse>(&response_text).map_err(|_| APIError::FailedToParse)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_access_token_response_with_scope() {
        let json = r#"{"access_token":"shpat_abc123","scope":"read_products,write_products"}"#;
        let resp: AccessTokenResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.access_token, "shpat_abc123");
        assert_eq!(resp.scope.as_deref(), Some("read_products,write_products"));
    }

    #[test]
    fn deserialize_access_token_response_without_scope() {
        let json = r#"{"access_token":"shpat_abc123"}"#;
        let resp: AccessTokenResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.access_token, "shpat_abc123");
        assert_eq!(resp.scope, None);
    }
}
