use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use hmac::{Hmac, Mac};
use sha2::Sha256;

use super::types::VerificationError;

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
}
