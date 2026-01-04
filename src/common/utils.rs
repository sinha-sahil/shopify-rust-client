use reqwest::Response;

use crate::common::types::{APIError, ErrorResp};

pub async fn parse_response<T: serde::de::DeserializeOwned>(resp: Response) -> Result<T, APIError> {
    let response_text = resp.text().await;
    match response_text {
        Ok(response_text) => {
            let response_json = serde_json::from_str::<T>(&response_text);
            match response_json {
                Ok(response_json) => Ok(response_json),
                Err(_) => {
                    let error_response_json = serde_json::from_str::<ErrorResp>(&response_text);

                    match error_response_json {
                        Ok(error_response_json) => Err(APIError::ServerError {
                            errors: error_response_json.errors,
                        }),
                        Err(_) => Err(APIError::FailedToParse),
                    }
                }
            }
        }
        Err(_) => Err(APIError::FailedToParse),
    }
}
