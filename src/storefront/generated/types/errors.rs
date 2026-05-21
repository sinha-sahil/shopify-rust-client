use serde::{Deserialize, Serialize};

/// Error codes from Storefront API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StorefrontErrorCode {
    #[serde(rename = "THROTTLED")]
    THROTTLED,
    #[serde(rename = "ACCESS_DENIED")]
    ACCESSDENIED,
    #[serde(rename = "SHOP_INACTIVE")]
    SHOPINACTIVE,
    #[serde(rename = "INTERNAL_SERVER_ERROR")]
    INTERNALSERVERERROR,
    #[serde(rename = "UNPROCESSABLE")]
    UNPROCESSABLE,
    #[serde(rename = "INVALID_ARGUMENT")]
    INVALIDARGUMENT,
    #[serde(rename = "NOT_FOUND")]
    NOTFOUND,
    #[serde(rename = "TOO_COMPLEX")]
    TOOCOMPLEX,
    #[serde(rename = "TIMEOUT")]
    TIMEOUT,
}

/// An error from the Storefront API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorefrontError {
    /// Error message
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locations: Option<Vec<ErrorLocation>>,
    /// Path to the field that caused the error
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<ErrorExtensions>,
}

/// Location of an error in the query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorLocation {
    pub line: f64,
    pub column: f64,
}

/// Additional error information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorExtensions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<StorefrontErrorCode>,
    /// Request ID for debugging
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// A user-facing error from a mutation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserError {
    /// Path to the field that caused the error
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<Vec<String>>,
    /// Error message
    pub message: String,
    /// Error code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

/// Response data container
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphQLResponseData {
    #[serde(flatten)]
    pub additional_properties: std::collections::HashMap<String, serde_json::Value>,
}

/// Raw GraphQL response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphQLResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<GraphQLResponseData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<StorefrontError>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<ResponseExtensions>,
}

/// Extensions in a GraphQL response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseExtensions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost: Option<QueryCost>,
}

/// Query cost information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryCost {
    #[serde(rename = "requestedQueryCost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_query_cost: Option<f64>,
    #[serde(rename = "actualQueryCost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_query_cost: Option<f64>,
    #[serde(rename = "throttleStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throttle_status: Option<ThrottleStatus>,
}

/// Throttle status for rate limiting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThrottleStatus {
    #[serde(rename = "maximumAvailable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_available: Option<f64>,
    #[serde(rename = "currentlyAvailable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currently_available: Option<f64>,
    #[serde(rename = "restoreRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_rate: Option<f64>,
}
