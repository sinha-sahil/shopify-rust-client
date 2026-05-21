use crate::common::ServiceContext;

use crate::common::types::APIError;
use serde_json::{json, Value};

use super::queries;
use crate::common::http::execute_storefront_graphql as execute_graphql;
use crate::storefront::generated::types::collections::{
    GetCollectionProductsArgs, GetCollectionsArgs,
};
use crate::storefront::generated::types::responses::{CollectionResponse, CollectionsResponse};

use super::CollectionRef;

pub async fn get_by_id(ctx: &ServiceContext, id: &str) -> Result<CollectionResponse, APIError> {
    execute_graphql(ctx, queries::get_collection_by_id(), json!({ "id": id })).await
}

pub async fn get_by_handle(
    ctx: &ServiceContext,
    handle: &str,
) -> Result<CollectionResponse, APIError> {
    execute_graphql(
        ctx,
        queries::get_collection_by_handle(),
        json!({ "handle": handle }),
    )
    .await
}

pub async fn get_with_products(
    ctx: &ServiceContext,
    selector: CollectionRef,
    args: GetCollectionProductsArgs,
) -> Result<CollectionResponse, APIError> {
    let mut variables = serde_json::to_value(&args).unwrap_or(json!({}));
    if let Value::Object(ref mut map) = variables {
        match selector {
            CollectionRef::Id(id) => {
                map.insert("id".to_string(), Value::String(id));
            }
            CollectionRef::Handle(handle) => {
                map.insert("handle".to_string(), Value::String(handle));
            }
        }
    }
    execute_graphql(ctx, queries::get_collection_with_products(), variables).await
}

pub async fn get_many(
    ctx: &ServiceContext,
    args: GetCollectionsArgs,
) -> Result<CollectionsResponse, APIError> {
    let variables = serde_json::to_value(&args).unwrap_or(json!({}));
    execute_graphql(ctx, queries::get_collections(), variables).await
}
