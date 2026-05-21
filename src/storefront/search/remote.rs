use crate::common::ServiceContext;

use crate::common::types::APIError;
use serde_json::json;

use super::queries;
use crate::common::http::execute_storefront_graphql as execute_graphql;
use crate::storefront::generated::types::responses::{PredictiveSearchResponse, SearchResponse};
use crate::storefront::generated::types::search::{PredictiveSearchArgs, SearchArgs};

pub async fn search(ctx: &ServiceContext, args: SearchArgs) -> Result<SearchResponse, APIError> {
    let variables = serde_json::to_value(&args).unwrap_or(json!({}));
    execute_graphql(ctx, queries::search(), variables).await
}

pub async fn predictive(
    ctx: &ServiceContext,
    args: PredictiveSearchArgs,
) -> Result<PredictiveSearchResponse, APIError> {
    let variables = serde_json::to_value(&args).unwrap_or(json!({}));
    execute_graphql(ctx, queries::predictive_search(), variables).await
}
