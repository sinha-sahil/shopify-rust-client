pub mod types;
pub mod verify;

use serde_json::Value;
use types::{
    CustomersDataRequestPayload, CustomersRedactPayload, ReturnWebhookPayload, ShopRedactPayload,
    WebhookParseError, WebhookPayload,
};

pub use types::{OAuthRedirectParams, VerificationError};
pub use verify::{verify_hmac, verify_hmac_from_headers, verify_oauth_redirect_hmac};

pub enum WebhookTopic {
    CustomersDataRequest,
    CustomersRedact,
    ShopRedact,
    ReturnsApprove,
    ReturnsDecline,
    ReturnsCancel,
    ReturnsClose,
}

impl WebhookTopic {
    pub fn from_header(header: &str) -> Option<Self> {
        match header.to_lowercase().as_str() {
            "customers/data_request" => Some(WebhookTopic::CustomersDataRequest),
            "customers/redact" => Some(WebhookTopic::CustomersRedact),
            "shop/redact" => Some(WebhookTopic::ShopRedact),
            "returns/approve" => Some(WebhookTopic::ReturnsApprove),
            "returns/decline" => Some(WebhookTopic::ReturnsDecline),
            "returns/cancel" => Some(WebhookTopic::ReturnsCancel),
            "returns/close" => Some(WebhookTopic::ReturnsClose),
            _ => None,
        }
    }
}

pub fn parse_webhook(
    topic: WebhookTopic,
    payload: &str,
) -> Result<WebhookPayload, WebhookParseError> {
    match topic {
        WebhookTopic::CustomersDataRequest => {
            serde_json::from_str::<CustomersDataRequestPayload>(payload)
                .map(WebhookPayload::CustomersDataRequest)
                .map_err(|e| WebhookParseError::ParseError(e.to_string()))
        }
        WebhookTopic::CustomersRedact => serde_json::from_str::<CustomersRedactPayload>(payload)
            .map(WebhookPayload::CustomersRedact)
            .map_err(|e| WebhookParseError::ParseError(e.to_string())),
        WebhookTopic::ShopRedact => serde_json::from_str::<ShopRedactPayload>(payload)
            .map(WebhookPayload::ShopRedact)
            .map_err(|e| WebhookParseError::ParseError(e.to_string())),
        WebhookTopic::ReturnsApprove => parse_return(payload).map(WebhookPayload::ReturnsApprove),
        WebhookTopic::ReturnsDecline => parse_return(payload).map(WebhookPayload::ReturnsDecline),
        WebhookTopic::ReturnsCancel => parse_return(payload).map(WebhookPayload::ReturnsCancel),
        WebhookTopic::ReturnsClose => parse_return(payload).map(WebhookPayload::ReturnsClose),
    }
}

pub fn parse_webhook_from_value(
    topic: WebhookTopic,
    payload: Value,
) -> Result<WebhookPayload, WebhookParseError> {
    match topic {
        WebhookTopic::CustomersDataRequest => {
            serde_json::from_value::<CustomersDataRequestPayload>(payload)
                .map(WebhookPayload::CustomersDataRequest)
                .map_err(|e| WebhookParseError::ParseError(e.to_string()))
        }
        WebhookTopic::CustomersRedact => serde_json::from_value::<CustomersRedactPayload>(payload)
            .map(WebhookPayload::CustomersRedact)
            .map_err(|e| WebhookParseError::ParseError(e.to_string())),
        WebhookTopic::ShopRedact => serde_json::from_value::<ShopRedactPayload>(payload)
            .map(WebhookPayload::ShopRedact)
            .map_err(|e| WebhookParseError::ParseError(e.to_string())),
        WebhookTopic::ReturnsApprove => {
            parse_return_value(payload).map(WebhookPayload::ReturnsApprove)
        }
        WebhookTopic::ReturnsDecline => {
            parse_return_value(payload).map(WebhookPayload::ReturnsDecline)
        }
        WebhookTopic::ReturnsCancel => {
            parse_return_value(payload).map(WebhookPayload::ReturnsCancel)
        }
        WebhookTopic::ReturnsClose => parse_return_value(payload).map(WebhookPayload::ReturnsClose),
    }
}

fn parse_return(payload: &str) -> Result<ReturnWebhookPayload, WebhookParseError> {
    serde_json::from_str(payload).map_err(|e| WebhookParseError::ParseError(e.to_string()))
}

fn parse_return_value(payload: Value) -> Result<ReturnWebhookPayload, WebhookParseError> {
    serde_json::from_value(payload).map_err(|e| WebhookParseError::ParseError(e.to_string()))
}

pub fn parse_webhook_with_header(
    topic_header: &str,
    payload: &str,
) -> Result<WebhookPayload, WebhookParseError> {
    let topic =
        WebhookTopic::from_header(topic_header).ok_or(WebhookParseError::UnknownWebhookType)?;
    parse_webhook(topic, payload)
}

pub fn parse_webhook_with_header_from_value(
    topic_header: &str,
    payload: Value,
) -> Result<WebhookPayload, WebhookParseError> {
    let topic =
        WebhookTopic::from_header(topic_header).ok_or(WebhookParseError::UnknownWebhookType)?;
    parse_webhook_from_value(topic, payload)
}

pub fn try_parse_webhook(payload: &str) -> Result<WebhookPayload, WebhookParseError> {
    if let Ok(data_request) = serde_json::from_str::<CustomersDataRequestPayload>(payload) {
        return Ok(WebhookPayload::CustomersDataRequest(data_request));
    }

    if let Ok(customers_redact) = serde_json::from_str::<CustomersRedactPayload>(payload) {
        return Ok(WebhookPayload::CustomersRedact(customers_redact));
    }

    if let Ok(shop_redact) = serde_json::from_str::<ShopRedactPayload>(payload) {
        return Ok(WebhookPayload::ShopRedact(shop_redact));
    }

    Err(WebhookParseError::UnknownWebhookType)
}

pub fn try_parse_webhook_from_value(payload: Value) -> Result<WebhookPayload, WebhookParseError> {
    if let Ok(data_request) = serde_json::from_value::<CustomersDataRequestPayload>(payload.clone())
    {
        return Ok(WebhookPayload::CustomersDataRequest(data_request));
    }

    if let Ok(customers_redact) = serde_json::from_value::<CustomersRedactPayload>(payload.clone())
    {
        return Ok(WebhookPayload::CustomersRedact(customers_redact));
    }

    if let Ok(shop_redact) = serde_json::from_value::<ShopRedactPayload>(payload) {
        return Ok(WebhookPayload::ShopRedact(shop_redact));
    }

    Err(WebhookParseError::UnknownWebhookType)
}

#[cfg(test)]
mod tests {
    use super::*;

    const DECLINED: &str = r#"{"id":37674025238,"admin_graphql_api_id":"gid://shopify/Return/37674025238","status":"declined","decline":{"reason":"other","note":"Seal broken"}}"#;

    #[test]
    fn a_return_topic_parses_into_its_own_variant() {
        let Ok(WebhookPayload::ReturnsDecline(parsed)) =
            parse_webhook_with_header("returns/decline", DECLINED)
        else {
            panic!("returns/decline parses as ReturnsDecline");
        };
        assert_eq!(
            parsed.admin_graphql_api_id,
            "gid://shopify/Return/37674025238"
        );
        assert_eq!(
            parsed.decline.and_then(|decline| decline.note).as_deref(),
            Some("Seal broken")
        );
    }

    #[test]
    fn a_return_payload_without_decline_or_status_still_parses() {
        let minimal = r#"{"id":1,"admin_graphql_api_id":"gid://shopify/Return/1"}"#;
        assert!(matches!(
            parse_webhook_with_header("RETURNS/APPROVE", minimal),
            Ok(WebhookPayload::ReturnsApprove(_))
        ));
        assert!(matches!(
            parse_webhook_with_header("returns/close", minimal),
            Ok(WebhookPayload::ReturnsClose(_))
        ));
    }

    #[test]
    fn an_unhandled_topic_is_unknown() {
        assert!(matches!(
            parse_webhook_with_header("returns/request", DECLINED),
            Err(WebhookParseError::UnknownWebhookType)
        ));
    }
}
