use crate::common::ServiceContext;
use crate::{
    admin::generated::types::file::{
        FileCreateInput, FileCreateResp, GetFileResp, StagedUploadInput, StagedUploadsCreateResp,
    },
    common::{http::execute_graphql, types::APIError},
};

use serde_json::json;

const FILE_FIELDS: &str = r#"
    id
    fileStatus
    ... on MediaImage {
        image {
            url
        }
    }
"#;

pub async fn create_staged_uploads(
    ctx: &ServiceContext,
    input: &[StagedUploadInput],
) -> Result<StagedUploadsCreateResp, APIError> {
    let query = r#"
        mutation StagedUploadsCreate($input: [StagedUploadInput!]!) {
            stagedUploadsCreate(input: $input) {
                stagedTargets {
                    url
                    resourceUrl
                    parameters {
                        name
                        value
                    }
                }
                userErrors {
                    field
                    message
                }
            }
        }
    "#;

    execute_graphql(ctx, query, json!({ "input": input })).await
}

pub async fn create_files(
    ctx: &ServiceContext,
    files: &[FileCreateInput],
) -> Result<FileCreateResp, APIError> {
    let query = format!(
        r#"
        mutation FileCreate($files: [FileCreateInput!]!) {{
            fileCreate(files: $files) {{
                files {{{fields}}}
                userErrors {{
                    field
                    message
                }}
            }}
        }}
    "#,
        fields = FILE_FIELDS
    );

    execute_graphql(ctx, &query, json!({ "files": files })).await
}

pub async fn get_file(ctx: &ServiceContext, id: &str) -> Result<GetFileResp, APIError> {
    let query = format!(
        r#"
        query GetFile($id: ID!) {{
            node(id: $id) {{
                ... on File {{{fields}}}
            }}
        }}
    "#,
        fields = FILE_FIELDS
    );

    execute_graphql(ctx, &query, json!({ "id": id })).await
}
