use serde::{Deserialize, Serialize};

/// Pagination metadata for a connection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageInfo {
    #[serde(rename = "hasNextPage")]
    pub has_next_page: bool,
    #[serde(rename = "hasPreviousPage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_previous_page: Option<bool>,
    #[serde(rename = "startCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_cursor: Option<String>,
    #[serde(rename = "endCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_cursor: Option<String>,
}

/// An OAuth access scope granted to the app
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessScope {
    pub handle: String,
}

/// An error returned from a GraphQL mutation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserError {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<Vec<String>>,
    pub message: String,
}
