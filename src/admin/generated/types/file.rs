use super::common::UserError;
use serde::{Deserialize, Serialize};

/// One file to upload to Shopify staging
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StagedUploadInput {
    pub filename: String,
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    /// StagedUploadTargetGenerateUploadResource on the wire, such as IMAGE or FILE
    pub resource: String,
    /// POST or PUT
    #[serde(rename = "httpMethod")]
    pub http_method: String,
    /// Bytes, as a string; required for VIDEO and MODEL_3D
    #[serde(rename = "fileSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StagedUploadParameter {
    pub name: String,
    pub value: String,
}

/// Where to send the file, and the form fields to send with it
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StagedTarget {
    pub url: String,
    /// Passed to fileCreate as originalSource once the file is uploaded
    #[serde(rename = "resourceUrl")]
    pub resource_url: String,
    pub parameters: Vec<StagedUploadParameter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StagedUploadsCreatePayload {
    #[serde(rename = "stagedTargets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub staged_targets: Option<Vec<StagedTarget>>,
    #[serde(rename = "userErrors")]
    pub user_errors: Vec<UserError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StagedUploadsCreateResp {
    #[serde(rename = "stagedUploadsCreate")]
    pub staged_uploads_create: StagedUploadsCreatePayload,
}

/// A file to add to Shopify Files from a staged upload or a URL
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileCreateInput {
    #[serde(rename = "originalSource")]
    pub original_source: String,
    /// FileContentType on the wire, such as IMAGE or FILE
    #[serde(rename = "contentType")]
    pub content_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileImage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// A file in Shopify Files; image is set once a MediaImage is READY
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatedFile {
    pub id: String,
    /// UPLOADED, PROCESSING, READY or FAILED
    #[serde(rename = "fileStatus")]
    pub file_status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<FileImage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileCreatePayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<CreatedFile>>,
    #[serde(rename = "userErrors")]
    pub user_errors: Vec<UserError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileCreateResp {
    #[serde(rename = "fileCreate")]
    pub file_create: FileCreatePayload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFileResp {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node: Option<CreatedFile>,
}
