pub mod remote;

use crate::common::ServiceContext;

use std::sync::Arc;

use crate::{
    common::types::{APIError, RequestCallbacks},
    types::app_installation::{
        DeleteMetafieldsResp, GetCurrentAppInstallationResp, GetMetafieldResp, ListMetafieldsResp,
        MetafieldIdentifierInput, MetafieldInput, SetMetafieldsResp,
    },
};

pub struct AppInstallation {
    pub(crate) ctx: ServiceContext,
}

impl AppInstallation {
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

    pub async fn get_current(&self) -> Result<GetCurrentAppInstallationResp, APIError> {
        remote::get_current_app_installation(&self.ctx).await
    }

    pub async fn set_metafields(
        &self,
        metafields: Vec<MetafieldInput>,
    ) -> Result<SetMetafieldsResp, APIError> {
        remote::set_metafields(&self.ctx, metafields).await
    }

    pub async fn get_metafield(
        &self,
        app_installation_id: &str,
        namespace: &str,
        key: &str,
    ) -> Result<GetMetafieldResp, APIError> {
        remote::get_metafield(&self.ctx, app_installation_id, namespace, key).await
    }

    pub async fn list_metafields(
        &self,
        app_installation_id: &str,
        first: Option<i32>,
    ) -> Result<ListMetafieldsResp, APIError> {
        remote::list_metafields(&self.ctx, app_installation_id, first).await
    }

    pub async fn delete_metafields(
        &self,
        metafields: &[MetafieldIdentifierInput],
    ) -> Result<DeleteMetafieldsResp, APIError> {
        remote::delete_metafields(&self.ctx, metafields).await
    }
}
