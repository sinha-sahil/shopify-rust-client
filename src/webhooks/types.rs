#[derive(serde::Deserialize, Debug, Clone, serde::Serialize)]
pub struct WebhookCustomer {
    pub id: u64,
    pub email: Option<String>,
    pub phone: Option<String>,
}

#[derive(serde::Deserialize, Debug, Clone, serde::Serialize)]
pub struct DataRequest {
    pub id: u64,
}

#[derive(serde::Deserialize, Debug, Clone, serde::Serialize)]
pub struct CustomersDataRequestPayload {
    pub shop_id: u64,
    pub shop_domain: String,
    pub orders_requested: Vec<u64>,
    pub customer: WebhookCustomer,
    pub data_request: DataRequest,
}

#[derive(serde::Deserialize, Debug, Clone, serde::Serialize)]
pub struct CustomersRedactPayload {
    pub shop_id: u64,
    pub shop_domain: String,
    pub customer: WebhookCustomer,
    pub orders_to_redact: Vec<u64>,
}

#[derive(serde::Deserialize, Debug, Clone, serde::Serialize)]
pub struct ShopRedactPayload {
    pub shop_id: u64,
    pub shop_domain: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub enum WebhookPayload {
    CustomersDataRequest(CustomersDataRequestPayload),
    CustomersRedact(CustomersRedactPayload),
    ShopRedact(ShopRedactPayload),
}

#[derive(Debug, Clone)]
pub enum WebhookParseError {
    ParseError(String),
    UnknownWebhookType,
}

impl std::fmt::Display for WebhookParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WebhookParseError::ParseError(msg) => write!(f, "Failed to parse webhook: {}", msg),
            WebhookParseError::UnknownWebhookType => write!(f, "Unknown webhook type"),
        }
    }
}

impl std::error::Error for WebhookParseError {}

#[derive(Debug, Clone, PartialEq)]
pub enum VerificationError {
    MissingHmacHeader,
    InvalidBase64,
    InvalidHmac,
}

impl std::fmt::Display for VerificationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VerificationError::MissingHmacHeader => {
                write!(f, "Missing X-Shopify-Hmac-SHA256 header")
            }
            VerificationError::InvalidBase64 => {
                write!(f, "Invalid base64 encoding in HMAC header")
            }
            VerificationError::InvalidHmac => write!(f, "HMAC verification failed"),
        }
    }
}

impl std::error::Error for VerificationError {}
