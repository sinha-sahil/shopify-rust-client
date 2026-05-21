use crate::common::ServiceContext;
use crate::{
    common::{http::execute_graphql, types::APIError},
    types::shopify_functions::ShopifyFunctionsResp,
};

use serde_json::json;

pub async fn list_shopify_functions(
    ctx: &ServiceContext,
) -> Result<ShopifyFunctionsResp, APIError> {
    let query = r#"
        query shopifyFunctions {
            shopifyFunctions(first: 25) {
                nodes {
                    id
                    title
                    description
                    apiType
                    apiVersion
                    handle
                    app {
                        id
                        title
                    }
                }
            }
        }
    "#;

    let variables = json!({});

    execute_graphql(ctx, query, variables).await
}
