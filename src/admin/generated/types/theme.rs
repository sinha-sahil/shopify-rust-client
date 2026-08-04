use super::common::PageInfo;
use super::common::UserError;
use serde::{Deserialize, Serialize};

/// A storefront theme
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub id: String,
    pub name: String,
    /// ThemeRole on the wire: MAIN (live), UNPUBLISHED (preview/draft), DEVELOPMENT, DEMO
    pub role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing: Option<bool>,
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeEdge {
    pub node: Theme,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeConnection {
    pub edges: Vec<ThemeEdge>,
    #[serde(rename = "pageInfo")]
    pub page_info: PageInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListThemesResp {
    pub themes: ThemeConnection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeDuplicatePayload {
    #[serde(rename = "newTheme")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_theme: Option<Theme>,
    #[serde(rename = "userErrors")]
    pub user_errors: Vec<UserError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeDuplicateResp {
    #[serde(rename = "themeDuplicate")]
    pub theme_duplicate: ThemeDuplicatePayload,
}

/// Input for creating a preview (unpublished) theme by duplicating an existing one
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreatePreviewThemeInput {
    /// Theme to duplicate; when omitted the live (MAIN) theme is used
    #[serde(rename = "sourceThemeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_theme_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
