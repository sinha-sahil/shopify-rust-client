use crate::common::ServiceContext;
use crate::{
    common::{http::execute_graphql, types::APIError},
    types::cart_transform::{
        CartTransformCreateInput, CartTransformCreateResp, MetafieldsSetInput, MetafieldsSetResp,
    },
};

use serde_json::json;

pub async fn create_cart_transform(
    ctx: &ServiceContext,
    input: &CartTransformCreateInput,
) -> Result<CartTransformCreateResp, APIError> {
    let query = r#"
        mutation cartTransformCreate($blockOnFailure: Boolean, $functionHandle: String, $metafields: [MetafieldInput!]) {
            cartTransformCreate(blockOnFailure: $blockOnFailure, functionHandle: $functionHandle, metafields: $metafields) {
                cartTransform {
                    id
                    functionId
                    blockOnFailure
                }
                userErrors {
                    field
                    message
                    code
                }
            }
        }
    "#;

    let variables = json!({
        "blockOnFailure": input.block_on_failure,
        "functionHandle": input.function_handle,
        "metafields": input.metafields
    });

    execute_graphql(ctx, query, variables).await
}

pub async fn set_metafields(
    ctx: &ServiceContext,
    metafields: &[MetafieldsSetInput],
) -> Result<MetafieldsSetResp, APIError> {
    let query = r#"
        mutation metafieldsSet($metafields: [MetafieldsSetInput!]!) {
            metafieldsSet(metafields: $metafields) {
                metafields {
                    key
                    namespace
                    value
                    createdAt
                    updatedAt
                    compareDigest
                }
                userErrors {
                    field
                    message
                    code
                }
            }
        }
    "#;

    let variables = json!({
        "metafields": metafields
    });

    execute_graphql(ctx, query, variables).await
}
