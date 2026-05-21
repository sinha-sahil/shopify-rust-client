pub mod http;
pub mod query_filter;
pub mod types;
pub mod utils;

use std::sync::Arc;

use crate::common::types::RequestCallbacks;

#[derive(Clone)]
pub struct ServiceContext {
    pub shop_url: Arc<String>,
    pub version: Arc<String>,
    pub access_token: Arc<String>,
    pub callbacks: Arc<RequestCallbacks>,
}

impl ServiceContext {
    pub fn new(
        shop_url: Arc<String>,
        version: Arc<String>,
        access_token: Arc<String>,
        callbacks: Arc<RequestCallbacks>,
    ) -> Self {
        Self {
            shop_url,
            version,
            access_token,
            callbacks,
        }
    }
}
