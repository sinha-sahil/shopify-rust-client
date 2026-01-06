pub mod types;
pub mod verify;

use serde_json::Value;
use types::{
    CustomersDataRequestPayload, CustomersRedactPayload, ShopRedactPayload, WebhookParseError,
    WebhookPayload,
};

pub use types::VerificationError;
pub use verify::{verify_hmac, verify_hmac_from_headers};

pub enum WebhookTopic {
    CustomersDataRequest,
    CustomersRedact,
    ShopRedact,
}

impl WebhookTopic {
    pub fn from_header(header: &str) -> Option<Self> {
        match header.to_lowercase().as_str() {
            "customers/data_request" => Some(WebhookTopic::CustomersDataRequest),
            "customers/redact" => Some(WebhookTopic::CustomersRedact),
            "shop/redact" => Some(WebhookTopic::ShopRedact),
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
    }
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
