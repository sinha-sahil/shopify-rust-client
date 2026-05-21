use crate::common::ServiceContext;

use crate::common::types::APIError;
use serde_json::json;

use super::queries;
use crate::common::http::execute_storefront_graphql as execute_graphql;
use crate::storefront::generated::types::responses::{LocalizationResponse, ShopResponse};

pub async fn get(ctx: &ServiceContext) -> Result<ShopResponse, APIError> {
    execute_graphql(ctx, queries::get_shop(), json!({})).await
}

pub async fn get_localization(ctx: &ServiceContext) -> Result<LocalizationResponse, APIError> {
    execute_graphql(ctx, queries::get_localization(), json!({})).await
}
