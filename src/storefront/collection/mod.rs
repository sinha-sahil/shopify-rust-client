pub mod queries;
pub mod remote;

use crate::common::ServiceContext;

use std::sync::Arc;

use crate::common::types::{APIError, RequestCallbacks};
use crate::storefront::generated::types::collections::{
    GetCollectionProductsArgs, GetCollectionsArgs,
};
use crate::storefront::generated::types::responses::{CollectionResponse, CollectionsResponse};

/// Selector for `Collection.get_with_products`.
pub enum CollectionRef {
    Id(String),
    Handle(String),
}

pub struct Collection {
    pub(crate) ctx: ServiceContext,
}

impl Collection {
    pub fn new(
        shop_url: Arc<String>,
        version: Arc<String>,
        access_token: Arc<String>,
        callbacks: Arc<RequestCallbacks>,
    ) -> Self {
        Self::with_ctx(ServiceContext::new(
            shop_url,
            version,
            access_token,
            callbacks,
        ))
    }

    /// Build the service from a shared `ServiceContext`. Cheaper than `new` at
    /// construction sites that already hold a context (one `Arc` clone per service).
    pub fn with_ctx(ctx: ServiceContext) -> Self {
        Self { ctx }
    }

    pub async fn get_by_id(&self, id: &str) -> Result<CollectionResponse, APIError> {
        remote::get_by_id(&self.ctx, id).await
    }

    pub async fn get_by_handle(&self, handle: &str) -> Result<CollectionResponse, APIError> {
        remote::get_by_handle(&self.ctx, handle).await
    }

    pub async fn get_with_products(
        &self,
        selector: CollectionRef,
        args: GetCollectionProductsArgs,
    ) -> Result<CollectionResponse, APIError> {
        remote::get_with_products(&self.ctx, selector, args).await
    }

    pub async fn get_many(
        &self,
        args: GetCollectionsArgs,
    ) -> Result<CollectionsResponse, APIError> {
        remote::get_many(&self.ctx, args).await
    }
}
