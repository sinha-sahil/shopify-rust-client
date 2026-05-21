pub mod cart;
pub mod collection;
pub mod content;
pub mod customer;
pub mod fragments;
pub mod generated;
pub mod metaobject;
pub mod product;
pub mod search;
pub mod shop;

pub use crate::common::types::{
    APIError, AfterRequestCallback, BeforeRequestCallback, RequestCallbacks,
};
pub use crate::common::ServiceContext;

use std::sync::Arc;

pub struct ShopifyStorefront {
    pub product: product::Product,
    pub cart: cart::Cart,
    pub collection: collection::Collection,
    pub customer: customer::Customer,
    pub shop: shop::Shop,
    pub search: search::Search,
    pub content: content::Content,
    pub metaobject: metaobject::Metaobject,
}

impl ShopifyStorefront {
    pub const DEFAULT_API_VERSION: &'static str = "2026-01";

    pub fn new(
        shop_url: String,
        storefront_access_token: String,
        api_version: Option<String>,
    ) -> Self {
        Self::new_with_callbacks(shop_url, storefront_access_token, api_version, None, None)
    }

    pub fn new_with_callbacks(
        shop_url: String,
        storefront_access_token: String,
        api_version: Option<String>,
        before_request: Option<BeforeRequestCallback>,
        after_request: Option<AfterRequestCallback>,
    ) -> Self {
        let api_version = api_version.unwrap_or_else(|| Self::DEFAULT_API_VERSION.to_string());
        let ctx = ServiceContext::new(
            Arc::new(shop_url),
            Arc::new(api_version),
            Arc::new(storefront_access_token),
            Arc::new(RequestCallbacks::new(before_request, after_request)),
        );

        ShopifyStorefront {
            product: product::Product::with_ctx(ctx.clone()),
            cart: cart::Cart::with_ctx(ctx.clone()),
            collection: collection::Collection::with_ctx(ctx.clone()),
            customer: customer::Customer::with_ctx(ctx.clone()),
            shop: shop::Shop::with_ctx(ctx.clone()),
            search: search::Search::with_ctx(ctx.clone()),
            content: content::Content::with_ctx(ctx.clone()),
            metaobject: metaobject::Metaobject::with_ctx(ctx),
        }
    }
}
