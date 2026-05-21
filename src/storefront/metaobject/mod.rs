pub mod queries;
pub mod remote;

use crate::common::ServiceContext;

use std::sync::Arc;

use crate::common::types::{APIError, RequestCallbacks};
use crate::storefront::generated::types::metafields::{GetMetaobjectsArgs, MetaobjectHandleInput};
use crate::storefront::generated::types::responses::{MetaobjectResponse, MetaobjectsResponse};

pub struct Metaobject {
    pub(crate) ctx: ServiceContext,
}

impl Metaobject {
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

    pub async fn get_by_id(&self, id: &str) -> Result<MetaobjectResponse, APIError> {
        remote::get_by_id(&self.ctx, id).await
    }

    pub async fn get_by_handle(
        &self,
        handle: MetaobjectHandleInput,
    ) -> Result<MetaobjectResponse, APIError> {
        remote::get_by_handle(&self.ctx, handle).await
    }

    pub async fn get_many(
        &self,
        args: GetMetaobjectsArgs,
    ) -> Result<MetaobjectsResponse, APIError> {
        remote::get_many(&self.ctx, args).await
    }
}
