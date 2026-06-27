pub mod remote;

use crate::common::ServiceContext;

use std::sync::Arc;

use crate::{
    admin::generated::types::theme::{
        CreatePreviewThemeInput, ListThemesResp, Theme, ThemeDuplicateResp,
    },
    common::types::{APIError, RequestCallbacks},
};

pub struct ThemeService {
    pub(crate) ctx: ServiceContext,
}

impl ThemeService {
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

    pub async fn list(&self) -> Result<ListThemesResp, APIError> {
        remote::list_themes(&self.ctx, 100, None).await
    }

    pub async fn get_live(&self) -> Result<Option<Theme>, APIError> {
        let resp = remote::list_themes(&self.ctx, 1, Some(&["MAIN"])).await?;
        Ok(resp.themes.edges.into_iter().next().map(|edge| edge.node))
    }

    pub async fn create_preview(
        &self,
        input: &CreatePreviewThemeInput,
    ) -> Result<ThemeDuplicateResp, APIError> {
        let source_id = match &input.source_theme_id {
            Some(id) => id.clone(),
            None => {
                self.get_live()
                    .await?
                    .ok_or_else(|| APIError::ServerError {
                        errors: "No live (MAIN) theme found to duplicate".to_string(),
                    })?
                    .id
            }
        };

        remote::duplicate_theme(&self.ctx, &source_id, input.name.as_deref()).await
    }
}
