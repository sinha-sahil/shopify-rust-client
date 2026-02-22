use std::sync::Arc;

pub mod common;
pub mod services;
pub mod types;
pub mod webhooks;

pub use common::types::{AfterRequestCallback, BeforeRequestCallback, RequestCallbacks};

pub struct ShopifyClient {
    pub order: services::order::Order,
    pub subscription: services::subscription::Subscription,
    pub app_installation: services::app_installation::AppInstallation,
    pub discount: services::discount::Discount,
    pub cart_transform: services::cart_transform::CartTransform,
    pub shopify_functions: services::shopify_functions::ShopifyFunctions,
    pub shop: services::shop::Shop,
    pub storefront_access_token: services::storefront_access_token::StorefrontAccessToken,
    pub bulk_operation: services::bulk_operation::BulkOperation,
}

impl ShopifyClient {
    pub fn new(shop_url: String, access_token: String, api_version: Option<String>) -> Self {
        Self::new_with_callbacks(shop_url, access_token, api_version, None, None)
    }

    pub fn new_with_callbacks(
        shop_url: String,
        access_token: String,
        api_version: Option<String>,
        before_request: Option<BeforeRequestCallback>,
        after_request: Option<AfterRequestCallback>,
    ) -> Self {
        let api_version = api_version.unwrap_or("2026-01".to_string());
        let shop_url_arc = Arc::new(shop_url);
        let api_version_arc = Arc::new(api_version);
        let access_token_arc = Arc::new(access_token);
        let callbacks_arc = Arc::new(RequestCallbacks::new(before_request, after_request));

        ShopifyClient {
            order: services::order::Order::new(
                Arc::clone(&shop_url_arc),
                Arc::clone(&api_version_arc),
                Arc::clone(&access_token_arc),
                Arc::clone(&callbacks_arc),
            ),
            subscription: services::subscription::Subscription::new(
                Arc::clone(&shop_url_arc),
                Arc::clone(&api_version_arc),
                Arc::clone(&access_token_arc),
                Arc::clone(&callbacks_arc),
            ),
            app_installation: services::app_installation::AppInstallation::new(
                Arc::clone(&shop_url_arc),
                Arc::clone(&api_version_arc),
                Arc::clone(&access_token_arc),
                Arc::clone(&callbacks_arc),
            ),
            discount: services::discount::Discount::new(
                Arc::clone(&shop_url_arc),
                Arc::clone(&api_version_arc),
                Arc::clone(&access_token_arc),
                Arc::clone(&callbacks_arc),
            ),
            cart_transform: services::cart_transform::CartTransform::new(
                Arc::clone(&shop_url_arc),
                Arc::clone(&api_version_arc),
                Arc::clone(&access_token_arc),
                Arc::clone(&callbacks_arc),
            ),
            shopify_functions: services::shopify_functions::ShopifyFunctions::new(
                Arc::clone(&shop_url_arc),
                Arc::clone(&api_version_arc),
                Arc::clone(&access_token_arc),
                Arc::clone(&callbacks_arc),
            ),
            shop: services::shop::Shop::new(
                Arc::clone(&shop_url_arc),
                Arc::clone(&api_version_arc),
                Arc::clone(&access_token_arc),
                Arc::clone(&callbacks_arc),
            ),
            storefront_access_token: services::storefront_access_token::StorefrontAccessToken::new(
                Arc::clone(&shop_url_arc),
                Arc::clone(&api_version_arc),
                Arc::clone(&access_token_arc),
                Arc::clone(&callbacks_arc),
            ),
            bulk_operation: services::bulk_operation::BulkOperation::new(
                Arc::clone(&shop_url_arc),
                Arc::clone(&api_version_arc),
                Arc::clone(&access_token_arc),
                Arc::clone(&callbacks_arc),
            ),
        }
    }
}
