pub mod queries;
pub mod remote;

use crate::common::ServiceContext;

use std::sync::Arc;

use crate::common::types::{APIError, RequestCallbacks};
use crate::storefront::generated::types::responses::{PredictiveSearchResponse, SearchResponse};
use crate::storefront::generated::types::search::{PredictiveSearchArgs, SearchArgs};

pub struct Search {
    pub(crate) ctx: ServiceContext,
}

impl Search {
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

    pub async fn search(&self, args: SearchArgs) -> Result<SearchResponse, APIError> {
        remote::search(&self.ctx, args).await
    }

    pub async fn predictive(
        &self,
        args: PredictiveSearchArgs,
    ) -> Result<PredictiveSearchResponse, APIError> {
        remote::predictive(&self.ctx, args).await
    }
}
