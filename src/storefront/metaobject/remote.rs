use crate::common::ServiceContext;

use crate::common::types::APIError;
use serde_json::json;

use super::queries;
use crate::common::http::execute_storefront_graphql as execute_graphql;
use crate::storefront::generated::types::metafields::{GetMetaobjectsArgs, MetaobjectHandleInput};
use crate::storefront::generated::types::responses::{MetaobjectResponse, MetaobjectsResponse};

pub async fn get_by_id(ctx: &ServiceContext, id: &str) -> Result<MetaobjectResponse, APIError> {
    execute_graphql(ctx, queries::get_metaobject_by_id(), json!({ "id": id })).await
}

pub async fn get_by_handle(
    ctx: &ServiceContext,
    handle: MetaobjectHandleInput,
) -> Result<MetaobjectResponse, APIError> {
    execute_graphql(
        ctx,
        queries::get_metaobject_by_handle(),
        json!({ "handle": handle }),
    )
    .await
}

pub async fn get_many(
    ctx: &ServiceContext,
    args: GetMetaobjectsArgs,
) -> Result<MetaobjectsResponse, APIError> {
    let variables = serde_json::to_value(&args).unwrap_or(json!({}));
    execute_graphql(ctx, queries::get_metaobjects(), variables).await
}
