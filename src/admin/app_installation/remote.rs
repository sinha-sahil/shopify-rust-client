use crate::common::ServiceContext;
use crate::{
    common::{http::execute_graphql, types::APIError},
    types::app_installation::{
        DeleteMetafieldsResp, GetCurrentAppInstallationResp, GetMetafieldResp, ListMetafieldsResp,
        MetafieldIdentifierInput, MetafieldInput, SetMetafieldsResp,
    },
};

use serde_json::json;

pub async fn get_current_app_installation(
    ctx: &ServiceContext,
) -> Result<GetCurrentAppInstallationResp, APIError> {
    let query = r#"
        query {
            currentAppInstallation {
                id
                accessScopes {
                    handle
                }
                activeSubscriptions {
                    id
                    name
                    status
                }
                launchUrl
            }
        }
    "#;

    let variables = json!({});

    execute_graphql(ctx, query, variables).await
}

pub async fn set_metafields(
    ctx: &ServiceContext,
    metafields: Vec<MetafieldInput>,
) -> Result<SetMetafieldsResp, APIError> {
    let query = r#"
        mutation MetafieldsSet($metafields: [MetafieldsSetInput!]!) {
            metafieldsSet(metafields: $metafields) {
                metafields {
                    id
                    namespace
                    key
                    value
                    type
                    createdAt
                    updatedAt
                }
                userErrors {
                    field
                    message
                }
            }
        }
    "#;

    let variables = json!({
        "metafields": metafields
    });

    execute_graphql(ctx, query, variables).await
}

pub async fn get_metafield(
    ctx: &ServiceContext,
    app_installation_id: &str,
    namespace: &str,
    key: &str,
) -> Result<GetMetafieldResp, APIError> {
    let query = r#"
        query GetMetafield($id: ID!, $namespace: String!, $key: String!) {
            appInstallation(id: $id) {
                metafield(namespace: $namespace, key: $key) {
                    id
                    namespace
                    key
                    value
                    type
                    createdAt
                    updatedAt
                }
            }
        }
    "#;

    let variables = json!({
        "id": app_installation_id,
        "namespace": namespace,
        "key": key
    });

    execute_graphql(ctx, query, variables).await
}

pub async fn list_metafields(
    ctx: &ServiceContext,
    app_installation_id: &str,
    first: Option<i32>,
) -> Result<ListMetafieldsResp, APIError> {
    let query = r#"
        query ListMetafields($id: ID!, $first: Int) {
            appInstallation(id: $id) {
                metafields(first: $first) {
                    edges {
                        node {
                            id
                            namespace
                            key
                            value
                            type
                            createdAt
                            updatedAt
                        }
                    }
                }
            }
        }
    "#;

    let variables = json!({
        "id": app_installation_id,
        "first": first.unwrap_or(10)
    });

    execute_graphql(ctx, query, variables).await
}

pub async fn delete_metafields(
    ctx: &ServiceContext,
    metafields: &[MetafieldIdentifierInput],
) -> Result<DeleteMetafieldsResp, APIError> {
    let query = r#"
        mutation metafieldsDelete($metafields: [MetafieldIdentifierInput!]!) {
            metafieldsDelete(metafields: $metafields) {
                deletedMetafields {
                    ownerId
                    namespace
                    key
                }
                userErrors {
                    field
                    message
                }
            }
        }
    "#;

    let variables = json!({
        "metafields": metafields
    });

    execute_graphql(ctx, query, variables).await
}
