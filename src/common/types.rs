#[derive(serde::Deserialize, Debug, serde::Serialize, Clone)]
pub enum APIError {
    ServerError { errors: String },
    FailedToParse,
    NetworkError,
}

#[derive(serde::Deserialize, Debug)]
pub struct ErrorResp {
    pub errors: String,
}
