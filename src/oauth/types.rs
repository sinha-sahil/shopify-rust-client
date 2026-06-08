#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct AccessTokenResponse {
    pub access_token: String,
    #[serde(default)]
    pub scope: Option<String>,
}
