use reqwest::header::HeaderMap;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::Arc;

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

#[derive(serde::Deserialize, Debug, Clone)]
pub struct UserError {
    pub field: Option<Vec<String>>,
    pub message: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct AccessScope {
    pub handle: String,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PageInfo {
    pub has_next_page: bool,
    pub has_previous_page: Option<bool>,
    pub start_cursor: Option<String>,
    pub end_cursor: Option<String>,
}

pub type BeforeRequestCallback = Arc<dyn Fn(&str, Option<&str>, &HeaderMap) + Send + Sync>;
pub type AfterRequestCallback = Arc<dyn Fn(&str, &str, &HeaderMap) + Send + Sync>;

#[derive(Clone)]
pub struct RequestCallbacks {
    pub before_request: Option<BeforeRequestCallback>,
    pub after_request: Option<AfterRequestCallback>,
}

impl RequestCallbacks {
    pub fn new(
        before_request: Option<BeforeRequestCallback>,
        after_request: Option<AfterRequestCallback>,
    ) -> Self {
        Self {
            before_request,
            after_request,
        }
    }

    pub fn none() -> Self {
        Self {
            before_request: None,
            after_request: None,
        }
    }

    pub fn call_before(&self, url: &str, body: Option<&str>, headers: &HeaderMap) {
        if let Some(callback) = &self.before_request {
            let _ = catch_unwind(AssertUnwindSafe(|| {
                callback(url, body, headers);
            }));
        }
    }

    pub fn call_after(&self, url: &str, response_body: &str, headers: &HeaderMap) {
        if let Some(callback) = &self.after_request {
            let _ = catch_unwind(AssertUnwindSafe(|| {
                callback(url, response_body, headers);
            }));
        }
    }
}
