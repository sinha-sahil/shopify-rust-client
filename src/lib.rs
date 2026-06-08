use std::sync::Arc;

pub mod admin;
pub mod common;
pub mod oauth;
#[cfg(feature = "storefront")]
pub mod storefront;
pub mod types;
pub mod webhooks;

pub use admin as services;
pub use common::types::{AfterRequestCallback, BeforeRequestCallback, RequestCallbacks};
pub use common::ServiceContext;

pub struct ShopifyClient {
    pub order: admin::order::Order,
    pub subscription: admin::subscription::Subscription,
    pub app_installation: admin::app_installation::AppInstallation,
    pub discount: admin::discount::Discount,
    pub cart_transform: admin::cart_transform::CartTransform,
    pub shopify_functions: admin::shopify_functions::ShopifyFunctions,
    pub shop: admin::shop::Shop,
    pub storefront_access_token: admin::storefront_access_token::StorefrontAccessToken,
    pub bulk_operation: admin::bulk_operation::BulkOperation,
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
        let api_version = api_version.unwrap_or_else(|| "2026-01".to_string());
        let ctx = ServiceContext::new(
            Arc::new(shop_url),
            Arc::new(api_version),
            Arc::new(access_token),
            Arc::new(RequestCallbacks::new(before_request, after_request)),
        );

        ShopifyClient {
            order: admin::order::Order::with_ctx(ctx.clone()),
            subscription: admin::subscription::Subscription::with_ctx(ctx.clone()),
            app_installation: admin::app_installation::AppInstallation::with_ctx(ctx.clone()),
            discount: admin::discount::Discount::with_ctx(ctx.clone()),
            cart_transform: admin::cart_transform::CartTransform::with_ctx(ctx.clone()),
            shopify_functions: admin::shopify_functions::ShopifyFunctions::with_ctx(ctx.clone()),
            shop: admin::shop::Shop::with_ctx(ctx.clone()),
            storefront_access_token:
                admin::storefront_access_token::StorefrontAccessToken::with_ctx(ctx.clone()),
            bulk_operation: admin::bulk_operation::BulkOperation::with_ctx(ctx),
        }
    }
}
