use crate::common::ServiceContext;
use crate::{
    admin::generated::types::locale::ListShopLocalesResp,
    common::{http::execute_graphql, types::APIError},
};

use serde_json::json;

pub async fn list_shop_locales(ctx: &ServiceContext) -> Result<ListShopLocalesResp, APIError> {
    let query = r#"
        query ListShopLocales {
            shopLocales {
                locale
                name
                primary
                published
            }
        }
    "#;

    execute_graphql(ctx, query, json!({})).await
}
