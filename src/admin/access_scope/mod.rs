pub mod remote;

use crate::common::ServiceContext;

use std::sync::Arc;

use crate::{
    admin::generated::types::access_scope::AccessScopesResp,
    common::types::{APIError, RequestCallbacks},
};

pub struct AccessScope {
    pub(crate) ctx: ServiceContext,
}

impl AccessScope {
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

    pub fn with_ctx(ctx: ServiceContext) -> Self {
        Self { ctx }
    }

    pub async fn list(&self) -> Result<AccessScopesResp, APIError> {
        remote::list_access_scopes(&self.ctx).await
    }
}

#[cfg(test)]
mod tests {
    use crate::admin::generated::types::access_scope::AccessScopesResp;

    #[test]
    fn deserialize_access_scopes_response() {
        let json = r#"{"access_scopes":[{"handle":"write_orders"},{"handle":"read_products"}]}"#;
        let resp: AccessScopesResp = serde_json::from_str(json).unwrap();
        assert_eq!(resp.access_scopes.len(), 2);
        assert_eq!(resp.access_scopes[0].handle, "write_orders");
        assert_eq!(resp.access_scopes[1].handle, "read_products");
    }

    #[test]
    fn deserialize_empty_access_scopes_response() {
        let json = r#"{"access_scopes":[]}"#;
        let resp: AccessScopesResp = serde_json::from_str(json).unwrap();
        assert!(resp.access_scopes.is_empty());
    }
}
