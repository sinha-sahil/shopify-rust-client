use crate::common::ServiceContext;
use crate::{
    common::{http::execute_graphql, types::APIError},
    types::shop::{GetShopResp, GetShopStatusResp},
};

use serde_json::json;

pub async fn get_shop(ctx: &ServiceContext) -> Result<GetShopResp, APIError> {
    let query = r#"
        query {
            shop {
                id
                name
                email
                shopOwnerName
                contactEmail
                myshopifyDomain
                primaryDomain {
                    id
                    host
                }
                setupRequired
                plan {
                    publicDisplayName
                    partnerDevelopment
                    shopifyPlus
                }
                shopAddress {
                    address1
                    address2
                    city
                    province
                    country
                    zip
                    phone
                    company
                }
            }
        }
    "#;

    let variables = json!({});

    execute_graphql(ctx, query, variables).await
}

pub async fn get_shop_status(ctx: &ServiceContext) -> Result<GetShopStatusResp, APIError> {
    let query = r#"
        query {
            shop {
                name
                email
                myshopifyDomain
                plan {
                    partnerDevelopment
                    publicDisplayName
                    shopifyPlus
                }
                setupRequired
                primaryDomain {
                    host
                    url
                }
            }
            onlineStore {
                passwordProtection {
                    enabled
                }
            }
        }
    "#;

    let variables = json!({});

    execute_graphql(ctx, query, variables).await
}
