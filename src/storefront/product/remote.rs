use crate::common::ServiceContext;

use crate::common::types::APIError;
use serde_json::json;

use super::queries;
use crate::common::http::execute_storefront_graphql as execute_graphql;
use crate::storefront::generated::types::products::{
    GetProductRecommendationsArgs, GetProductsArgs,
};
use crate::storefront::generated::types::responses::{
    ProductRecommendationsResponse, ProductResponse, ProductsResponse,
};

pub async fn get_by_id(ctx: &ServiceContext, id: &str) -> Result<ProductResponse, APIError> {
    execute_graphql(ctx, queries::get_product_by_id(), json!({ "id": id })).await
}

pub async fn get_by_handle(
    ctx: &ServiceContext,
    handle: &str,
) -> Result<ProductResponse, APIError> {
    execute_graphql(
        ctx,
        queries::get_product_by_handle(),
        json!({ "handle": handle }),
    )
    .await
}

pub async fn get_many(
    ctx: &ServiceContext,
    args: GetProductsArgs,
) -> Result<ProductsResponse, APIError> {
    let variables = serde_json::to_value(&args).unwrap_or(json!({}));
    execute_graphql(ctx, queries::get_products(), variables).await
}

pub async fn get_recommendations(
    ctx: &ServiceContext,
    args: GetProductRecommendationsArgs,
) -> Result<ProductRecommendationsResponse, APIError> {
    let variables = serde_json::to_value(&args).unwrap_or(json!({}));
    execute_graphql(ctx, queries::get_product_recommendations(), variables).await
}
