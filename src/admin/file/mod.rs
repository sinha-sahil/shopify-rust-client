pub mod remote;

use crate::common::ServiceContext;

use std::sync::Arc;

use crate::{
    admin::generated::types::file::{
        FileCreateInput, FileCreateResp, GetFileResp, StagedUploadInput, StagedUploadsCreateResp,
    },
    common::types::{APIError, RequestCallbacks},
};

pub struct FileService {
    pub(crate) ctx: ServiceContext,
}

impl FileService {
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

    pub async fn create_staged_uploads(
        &self,
        input: &[StagedUploadInput],
    ) -> Result<StagedUploadsCreateResp, APIError> {
        remote::create_staged_uploads(&self.ctx, input).await
    }

    pub async fn create(&self, files: &[FileCreateInput]) -> Result<FileCreateResp, APIError> {
        remote::create_files(&self.ctx, files).await
    }

    pub async fn get(&self, id: &str) -> Result<GetFileResp, APIError> {
        remote::get_file(&self.ctx, id).await
    }
}

#[cfg(test)]
mod tests {
    use crate::admin::generated::types::file::{
        FileCreateResp, GetFileResp, StagedUploadInput, StagedUploadsCreateResp,
    };

    #[test]
    fn a_staged_target_parses_with_its_form_fields() {
        let resp: StagedUploadsCreateResp = serde_json::from_str(
            r#"{"stagedUploadsCreate":{"stagedTargets":[{"url":"https://shopify-staged-uploads.storage.googleapis.com/","resourceUrl":"https://shopify-staged-uploads.storage.googleapis.com/tmp/badge.png","parameters":[{"name":"key","value":"tmp/badge.png"}]}],"userErrors":[]}}"#,
        )
        .expect("a staged upload response");

        let target = &resp.staged_uploads_create.staged_targets.expect("targets")[0];
        assert_eq!(target.parameters[0].name, "key");
    }

    #[test]
    fn a_file_is_ready_once_it_has_an_image_url() {
        let created: FileCreateResp = serde_json::from_str(
            r#"{"fileCreate":{"files":[{"id":"gid://shopify/MediaImage/1","fileStatus":"UPLOADED"}],"userErrors":[]}}"#,
        )
        .expect("a file create response");
        let read: GetFileResp = serde_json::from_str(
            r#"{"node":{"id":"gid://shopify/MediaImage/1","fileStatus":"READY","image":{"url":"https://cdn.shopify.com/s/files/badge.png"}}}"#,
        )
        .expect("a file response");

        assert_eq!(
            created.file_create.files.unwrap()[0].file_status,
            "UPLOADED"
        );
        let file = read.node.expect("a file");
        assert_eq!(
            file.image.and_then(|image| image.url).as_deref(),
            Some("https://cdn.shopify.com/s/files/badge.png")
        );
    }

    #[test]
    fn a_staged_input_serializes_as_shopify_spells_it() {
        let input = StagedUploadInput {
            filename: "badge.png".to_string(),
            mime_type: "image/png".to_string(),
            resource: "IMAGE".to_string(),
            http_method: "POST".to_string(),
            file_size: None,
        };

        let json = serde_json::to_value(&input).expect("serializes");
        assert_eq!(json["mimeType"], "image/png");
        assert_eq!(json["httpMethod"], "POST");
    }
}
