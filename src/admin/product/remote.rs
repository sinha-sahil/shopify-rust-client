use crate::common::ServiceContext;
use crate::{
    common::{http::execute_graphql, types::APIError},
    types::product::ProductVariantResp,
};

use serde_json::json;

pub async fn get_variant(
    ctx: &ServiceContext,
    variant_id: &str,
) -> Result<ProductVariantResp, APIError> {
    let query = r#"
        query productVariant($id: ID!) {
            productVariant(id: $id) {
                id
                title
                media(first: 1) {
                    nodes { preview { image { url } } }
                    pageInfo { hasNextPage }
                }
                product {
                    id
                    title
                    featuredMedia { preview { image { url } } }
                }
            }
        }
    "#;

    execute_graphql(ctx, query, json!({ "id": variant_id })).await
}
