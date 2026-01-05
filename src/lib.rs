use std::sync::Arc;

pub mod common;
pub mod services;
pub mod types;
pub mod webhooks;

pub struct ShopifyClient {
    pub order: services::order::Order,
}

impl ShopifyClient {
    pub fn new(shop_url: String, access_token: String, api_version: Option<String>) -> Self {
        let api_version = api_version.unwrap_or("2024-07".to_string());
        let shop_url_arc = Arc::new(shop_url);
        let api_version_arc = Arc::new(api_version);
        let access_token_arc = Arc::new(access_token);

        ShopifyClient {
            order: services::order::Order::new(
                Arc::clone(&shop_url_arc),
                Arc::clone(&api_version_arc),
                Arc::clone(&access_token_arc),
            ),
        }
    }
}
