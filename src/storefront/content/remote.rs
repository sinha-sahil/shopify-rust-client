use crate::common::ServiceContext;

use crate::common::types::APIError;
use serde_json::{json, Value};

use super::queries;
use crate::common::http::execute_storefront_graphql as execute_graphql;
use crate::storefront::generated::types::content::{
    GetArticlesArgs, GetBlogArticlesArgs, GetBlogsArgs, GetPagesArgs,
};
use crate::storefront::generated::types::responses::{
    ArticleResponse, ArticlesResponse, BlogResponse, BlogsResponse, MenuResponse, PageResponse,
    PagesResponse,
};

use super::BlogRef;

pub async fn get_page_by_id(ctx: &ServiceContext, id: &str) -> Result<PageResponse, APIError> {
    execute_graphql(ctx, queries::get_page_by_id(), json!({ "id": id })).await
}

pub async fn get_page_by_handle(
    ctx: &ServiceContext,
    handle: &str,
) -> Result<PageResponse, APIError> {
    execute_graphql(
        ctx,
        queries::get_page_by_handle(),
        json!({ "handle": handle }),
    )
    .await
}

pub async fn get_pages(
    ctx: &ServiceContext,
    args: GetPagesArgs,
) -> Result<PagesResponse, APIError> {
    let variables = serde_json::to_value(&args).unwrap_or(json!({}));
    execute_graphql(ctx, queries::get_pages(), variables).await
}

pub async fn get_blog_by_id(ctx: &ServiceContext, id: &str) -> Result<BlogResponse, APIError> {
    execute_graphql(ctx, queries::get_blog_by_id(), json!({ "id": id })).await
}

pub async fn get_blog_by_handle(
    ctx: &ServiceContext,
    handle: &str,
) -> Result<BlogResponse, APIError> {
    execute_graphql(
        ctx,
        queries::get_blog_by_handle(),
        json!({ "handle": handle }),
    )
    .await
}

pub async fn get_blogs(
    ctx: &ServiceContext,
    args: GetBlogsArgs,
) -> Result<BlogsResponse, APIError> {
    let variables = serde_json::to_value(&args).unwrap_or(json!({}));
    execute_graphql(ctx, queries::get_blogs(), variables).await
}

pub async fn get_blog_with_articles(
    ctx: &ServiceContext,
    selector: BlogRef,
    args: GetBlogArticlesArgs,
) -> Result<BlogResponse, APIError> {
    let mut variables = serde_json::to_value(&args).unwrap_or(json!({}));
    if let Value::Object(ref mut map) = variables {
        match selector {
            BlogRef::Id(id) => {
                map.insert("id".to_string(), Value::String(id));
            }
            BlogRef::Handle(handle) => {
                map.insert("handle".to_string(), Value::String(handle));
            }
        }
    }
    execute_graphql(ctx, queries::get_blog_with_articles(), variables).await
}

pub async fn get_article_by_id(
    ctx: &ServiceContext,
    id: &str,
) -> Result<ArticleResponse, APIError> {
    execute_graphql(ctx, queries::get_article_by_id(), json!({ "id": id })).await
}

pub async fn get_articles(
    ctx: &ServiceContext,
    args: GetArticlesArgs,
) -> Result<ArticlesResponse, APIError> {
    let variables = serde_json::to_value(&args).unwrap_or(json!({}));
    execute_graphql(ctx, queries::get_articles(), variables).await
}

pub async fn get_menu(ctx: &ServiceContext, handle: &str) -> Result<MenuResponse, APIError> {
    execute_graphql(ctx, queries::get_menu(), json!({ "handle": handle })).await
}
