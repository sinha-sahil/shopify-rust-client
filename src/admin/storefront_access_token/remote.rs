use crate::common::ServiceContext;
use crate::{
    common::{http::execute_graphql, types::APIError},
    types::storefront_access_token::{
        ListStorefrontAccessTokensResp, StorefrontAccessTokenCreateInput,
        StorefrontAccessTokenCreateResp, StorefrontAccessTokenDeleteInput,
        StorefrontAccessTokenDeleteResp,
    },
};

use serde_json::json;

pub async fn list_storefront_access_tokens(
    ctx: &ServiceContext,
) -> Result<ListStorefrontAccessTokensResp, APIError> {
    let query = r#"
        query {
            shop {
                storefrontAccessTokens(first: 100) {
                    edges {
                        node {
                            id
                            accessToken
                            accessScopes {
                                handle
                            }
                            createdAt
                            title
                            updatedAt
                        }
                    }
                    pageInfo {
                        hasNextPage
                        endCursor
                    }
                }
            }
        }
    "#;

    let variables = json!({});

    execute_graphql(ctx, query, variables).await
}

pub async fn create_storefront_access_token(
    ctx: &ServiceContext,
    input: &StorefrontAccessTokenCreateInput,
) -> Result<StorefrontAccessTokenCreateResp, APIError> {
    let query = r#"
        mutation storefrontAccessTokenCreate($input: StorefrontAccessTokenInput!) {
            storefrontAccessTokenCreate(input: $input) {
                storefrontAccessToken {
                    id
                    accessToken
                    accessScopes {
                        handle
                    }
                    createdAt
                    title
                }
                userErrors {
                    field
                    message
                }
            }
        }
    "#;

    let variables = json!({
        "input": {
            "title": input.title
        }
    });

    execute_graphql(ctx, query, variables).await
}

pub async fn delete_storefront_access_token(
    ctx: &ServiceContext,
    input: &StorefrontAccessTokenDeleteInput,
) -> Result<StorefrontAccessTokenDeleteResp, APIError> {
    let query = r#"
        mutation storefrontAccessTokenDelete($input: StorefrontAccessTokenDeleteInput!) {
            storefrontAccessTokenDelete(input: $input) {
                deletedStorefrontAccessTokenId
                userErrors {
                    field
                    message
                }
            }
        }
    "#;

    let variables = json!({
        "input": {
            "id": input.id
        }
    });

    execute_graphql(ctx, query, variables).await
}
