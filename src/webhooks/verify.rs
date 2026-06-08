use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use hmac::{Hmac, Mac};
use sha2::Sha256;

use super::types::{OAuthRedirectParams, VerificationError};

type HmacSha256 = Hmac<Sha256>;

pub fn verify_hmac(
    hmac_header: &str,
    raw_body: &[u8],
    client_secret: &str,
) -> Result<(), VerificationError> {
    let received_hmac = BASE64
        .decode(hmac_header)
        .map_err(|_| VerificationError::InvalidBase64)?;

    let mut mac = HmacSha256::new_from_slice(client_secret.as_bytes())
        .expect("HMAC can take key of any size");
    mac.update(raw_body);

    mac.verify_slice(&received_hmac)
        .map_err(|_| VerificationError::InvalidHmac)
}

pub fn verify_hmac_from_headers(
    headers: &std::collections::HashMap<String, String>,
    raw_body: &[u8],
    client_secret: &str,
) -> Result<(), VerificationError> {
    let hmac_header = headers
        .get("x-shopify-hmac-sha256")
        .or_else(|| headers.get("X-Shopify-Hmac-SHA256"))
        .ok_or(VerificationError::MissingHmacHeader)?;

    verify_hmac(hmac_header, raw_body, client_secret)
}

pub fn verify_oauth_redirect_hmac(
    params: &OAuthRedirectParams,
    client_secret: &str,
) -> Result<(), VerificationError> {
    let mut pairs: Vec<(&str, &str)> = vec![
        ("embedded", &params.embedded),
        ("host", &params.host),
        ("locale", &params.locale),
        ("session", &params.session),
        ("shop", &params.shop),
        ("timestamp", &params.timestamp),
    ];
    if let Some(v) = &params.id_token {
        pairs.push(("id_token", v));
    }
    if let Some(v) = &params.app_load_id {
        pairs.push(("app_load_id", v));
    }
    pairs.sort_by_key(|(k, _)| *k);

    let message = pairs
        .iter()
        .map(|(k, v)| format!("{k}={v}"))
        .collect::<Vec<_>>()
        .join("&");

    let digest = hex::decode(&params.hmac).map_err(|_| VerificationError::InvalidHmac)?;
    verify_hmac(&BASE64.encode(digest), message.as_bytes(), client_secret)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_hmac_valid() {
        let client_secret = "test_secret";
        let body = b"{\"test\":\"data\"}";

        let mut mac = HmacSha256::new_from_slice(client_secret.as_bytes()).unwrap();
        mac.update(body);
        let result = mac.finalize();
        let expected_hmac = BASE64.encode(result.into_bytes());

        let result = verify_hmac(&expected_hmac, body, client_secret);
        assert!(result.is_ok());
    }

    #[test]
    fn test_verify_hmac_invalid() {
        let client_secret = "test_secret";
        let body = b"{\"test\":\"data\"}";
        let wrong_hmac = BASE64.encode(b"wrong_hmac_value_here_123456");

        let result = verify_hmac(&wrong_hmac, body, client_secret);
        assert!(matches!(result, Err(VerificationError::InvalidHmac)));
    }

    #[test]
    fn test_verify_hmac_invalid_base64() {
        let client_secret = "test_secret";
        let body = b"{\"test\":\"data\"}";
        let invalid_base64 = "not_valid_base64!@#$%";

        let result = verify_hmac(invalid_base64, body, client_secret);
        assert!(matches!(result, Err(VerificationError::InvalidBase64)));
    }

    #[test]
    fn test_verify_hmac_from_headers_valid() {
        let client_secret = "test_secret";
        let body = b"{\"test\":\"data\"}";

        let mut mac = HmacSha256::new_from_slice(client_secret.as_bytes()).unwrap();
        mac.update(body);
        let result = mac.finalize();
        let expected_hmac = BASE64.encode(result.into_bytes());

        let mut headers = std::collections::HashMap::new();
        headers.insert("x-shopify-hmac-sha256".to_string(), expected_hmac);

        let result = verify_hmac_from_headers(&headers, body, client_secret);
        assert!(result.is_ok());
    }

    #[test]
    fn test_verify_hmac_from_headers_case_insensitive() {
        let client_secret = "test_secret";
        let body = b"{\"test\":\"data\"}";

        let mut mac = HmacSha256::new_from_slice(client_secret.as_bytes()).unwrap();
        mac.update(body);
        let result = mac.finalize();
        let expected_hmac = BASE64.encode(result.into_bytes());

        let mut headers = std::collections::HashMap::new();
        headers.insert("X-Shopify-Hmac-SHA256".to_string(), expected_hmac);

        let result = verify_hmac_from_headers(&headers, body, client_secret);
        assert!(result.is_ok());
    }

    #[test]
    fn test_verify_hmac_from_headers_missing() {
        let client_secret = "test_secret";
        let body = b"{\"test\":\"data\"}";
        let headers = std::collections::HashMap::new();

        let result = verify_hmac_from_headers(&headers, body, client_secret);
        assert!(matches!(result, Err(VerificationError::MissingHmacHeader)));
    }

    #[test]
    fn test_shopify_example() {
        let client_secret = "hush";
        let body = b"{\"example\":\"webhook_data\"}";

        let mut mac = HmacSha256::new_from_slice(client_secret.as_bytes()).unwrap();
        mac.update(body);
        let result = mac.finalize();
        let hmac = BASE64.encode(result.into_bytes());

        let verification = verify_hmac(&hmac, body, client_secret);
        assert!(verification.is_ok());

        let wrong_verification = verify_hmac(&hmac, b"different_body", client_secret);
        assert!(matches!(
            wrong_verification,
            Err(VerificationError::InvalidHmac)
        ));
    }

    fn sample_params(hmac: String) -> OAuthRedirectParams {
        OAuthRedirectParams {
            hmac,
            embedded: "1".to_string(),
            host: "ZXhhbXBsZS5teXNob3BpZnkuY29t".to_string(),
            locale: "en".to_string(),
            session: "abc123".to_string(),
            shop: "example.myshopify.com".to_string(),
            timestamp: "1700000000".to_string(),
            id_token: None,
            app_load_id: None,
        }
    }

    #[test]
    fn test_verify_oauth_redirect_hmac_valid() {
        let client_secret = "test_secret";
        let message = "embedded=1&host=ZXhhbXBsZS5teXNob3BpZnkuY29t&locale=en&session=abc123&shop=example.myshopify.com&timestamp=1700000000";

        let mut mac = HmacSha256::new_from_slice(client_secret.as_bytes()).unwrap();
        mac.update(message.as_bytes());
        let hmac_hex = hex::encode(mac.finalize().into_bytes());

        let params = sample_params(hmac_hex);
        assert!(verify_oauth_redirect_hmac(&params, client_secret).is_ok());
    }

    #[test]
    fn test_verify_oauth_redirect_hmac_invalid() {
        let client_secret = "test_secret";
        let params = sample_params(hex::encode([0u8; 32]));
        assert!(matches!(
            verify_oauth_redirect_hmac(&params, client_secret),
            Err(VerificationError::InvalidHmac)
        ));
    }

    #[test]
    fn test_verify_oauth_redirect_hmac_non_hex() {
        let client_secret = "test_secret";
        let params = sample_params("not-hex!!".to_string());
        assert!(matches!(
            verify_oauth_redirect_hmac(&params, client_secret),
            Err(VerificationError::InvalidHmac)
        ));
    }
}
