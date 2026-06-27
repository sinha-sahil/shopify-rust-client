use crate::common::ServiceContext;
use crate::{
    admin::generated::types::theme::{ListThemesResp, ThemeDuplicateResp},
    common::{http::execute_graphql, types::APIError},
};

use serde_json::json;

const THEME_FIELDS: &str = r#"
    id
    name
    role
    processing
    createdAt
    updatedAt
"#;

pub async fn list_themes(
    ctx: &ServiceContext,
    first: i64,
    roles: Option<&[&str]>,
) -> Result<ListThemesResp, APIError> {
    let query = format!(
        r#"
        query ListThemes($first: Int!, $roles: [ThemeRole!]) {{
            themes(first: $first, roles: $roles) {{
                edges {{
                    node {{{fields}}}
                }}
                pageInfo {{
                    hasNextPage
                    endCursor
                }}
            }}
        }}
    "#,
        fields = THEME_FIELDS
    );

    let variables = json!({
        "first": first,
        "roles": roles,
    });

    execute_graphql(ctx, &query, variables).await
}

pub async fn duplicate_theme(
    ctx: &ServiceContext,
    id: &str,
    name: Option<&str>,
) -> Result<ThemeDuplicateResp, APIError> {
    let query = format!(
        r#"
        mutation ThemeDuplicate($id: ID!, $name: String) {{
            themeDuplicate(id: $id, name: $name) {{
                newTheme {{{fields}}}
                userErrors {{
                    field
                    message
                }}
            }}
        }}
    "#,
        fields = THEME_FIELDS
    );

    let variables = json!({
        "id": id,
        "name": name,
    });

    execute_graphql(ctx, &query, variables).await
}
