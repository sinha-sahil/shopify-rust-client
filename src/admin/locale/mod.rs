pub mod remote;

use crate::common::ServiceContext;

use std::sync::Arc;

use crate::{
    admin::generated::types::locale::ListShopLocalesResp,
    common::types::{APIError, RequestCallbacks},
};

pub struct LocaleService {
    pub(crate) ctx: ServiceContext,
}

impl LocaleService {
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

    pub async fn list(&self) -> Result<ListShopLocalesResp, APIError> {
        remote::list_shop_locales(&self.ctx).await
    }
}

#[cfg(test)]
mod tests {
    use crate::admin::generated::types::locale::ListShopLocalesResp;

    #[test]
    fn shop_locales_parse_with_the_primary_and_published_flags() {
        let resp: ListShopLocalesResp = serde_json::from_str(
            r#"{"shopLocales":[{"locale":"en","name":"English","primary":true,"published":true},{"locale":"hi","name":"Hindi","primary":false,"published":false}]}"#,
        )
        .expect("a shop locales response");

        assert!(resp.shop_locales[0].primary);
        assert!(!resp.shop_locales[1].published);
    }
}
