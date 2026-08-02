pub mod queries;
pub mod remote;

use crate::common::ServiceContext;

use std::sync::Arc;

use crate::common::types::{APIError, RequestCallbacks};
use crate::storefront::generated::types::products::{
    GetProductRecommendationsArgs, GetProductVariantsArgs, GetProductsArgs,
};
use crate::storefront::generated::types::responses::{
    ProductRecommendationsResponse, ProductResponse, ProductVariantsResponse, ProductsResponse,
};

pub struct Product {
    pub(crate) ctx: ServiceContext,
}

impl Product {
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

    pub async fn get_by_id(&self, id: &str) -> Result<ProductResponse, APIError> {
        remote::get_by_id(&self.ctx, id).await
    }

    pub async fn get_by_handle(&self, handle: &str) -> Result<ProductResponse, APIError> {
        remote::get_by_handle(&self.ctx, handle).await
    }

    pub async fn get_many(&self, args: GetProductsArgs) -> Result<ProductsResponse, APIError> {
        remote::get_many(&self.ctx, args).await
    }

    pub async fn get_recommendations(
        &self,
        args: GetProductRecommendationsArgs,
    ) -> Result<ProductRecommendationsResponse, APIError> {
        remote::get_recommendations(&self.ctx, args).await
    }

    pub async fn get_variants_by_ids(
        &self,
        args: GetProductVariantsArgs,
    ) -> Result<ProductVariantsResponse, APIError> {
        remote::get_variants_by_ids(&self.ctx, args).await
    }
}
