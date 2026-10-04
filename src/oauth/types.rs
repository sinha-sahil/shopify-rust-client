#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct AccessTokenResponse {
    pub access_token: String,
    #[serde(default)]
    pub scope: Option<String>,
    #[serde(default)]
    pub expires_in: Option<u64>,
    #[serde(default)]
    pub refresh_token: Option<String>,
    #[serde(default)]
    pub refresh_token_expires_in: Option<u64>,
}
