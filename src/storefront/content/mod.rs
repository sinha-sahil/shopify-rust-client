pub mod queries;
pub mod remote;

use std::sync::Arc;

use crate::common::types::{APIError, RequestCallbacks};
use crate::common::ServiceContext;
use crate::storefront::generated::types::content::{
    GetArticlesArgs, GetBlogArticlesArgs, GetBlogsArgs, GetPagesArgs,
};
use crate::storefront::generated::types::responses::{
    ArticleResponse, ArticlesResponse, BlogResponse, BlogsResponse, MenuResponse, PageResponse,
    PagesResponse,
};

/// Selector for `ContentBlogs.get_with_articles`.
pub enum BlogRef {
    Id(String),
    Handle(String),
}

pub struct Content {
    pub pages: ContentPages,
    pub blogs: ContentBlogs,
    pub articles: ContentArticles,
    pub menus: ContentMenus,
}

impl Content {
    pub fn new(
        shop_url: Arc<String>,
        version: Arc<String>,
        access_token: Arc<String>,
        callbacks: Arc<RequestCallbacks>,
    ) -> Self {
        Self::with_ctx(ServiceContext::new(
            shop_url,
            version,
            access_token,
            callbacks,
        ))
    }

    /// Build the service from a shared `ServiceContext`.
    pub fn with_ctx(ctx: ServiceContext) -> Self {
        Self {
            pages: ContentPages { ctx: ctx.clone() },
            blogs: ContentBlogs { ctx: ctx.clone() },
            articles: ContentArticles { ctx: ctx.clone() },
            // Last sub-service consumes the ctx.
            menus: ContentMenus { ctx },
        }
    }
}

pub struct ContentPages {
    pub(crate) ctx: ServiceContext,
}

impl ContentPages {
    pub async fn get_by_id(&self, id: &str) -> Result<PageResponse, APIError> {
        remote::get_page_by_id(&self.ctx, id).await
    }

    pub async fn get_by_handle(&self, handle: &str) -> Result<PageResponse, APIError> {
        remote::get_page_by_handle(&self.ctx, handle).await
    }

    pub async fn get_many(&self, args: GetPagesArgs) -> Result<PagesResponse, APIError> {
        remote::get_pages(&self.ctx, args).await
    }
}

pub struct ContentBlogs {
    pub(crate) ctx: ServiceContext,
}

impl ContentBlogs {
    pub async fn get_by_id(&self, id: &str) -> Result<BlogResponse, APIError> {
        remote::get_blog_by_id(&self.ctx, id).await
    }

    pub async fn get_by_handle(&self, handle: &str) -> Result<BlogResponse, APIError> {
        remote::get_blog_by_handle(&self.ctx, handle).await
    }

    pub async fn get_many(&self, args: GetBlogsArgs) -> Result<BlogsResponse, APIError> {
        remote::get_blogs(&self.ctx, args).await
    }

    pub async fn get_with_articles(
        &self,
        selector: BlogRef,
        args: GetBlogArticlesArgs,
    ) -> Result<BlogResponse, APIError> {
        remote::get_blog_with_articles(&self.ctx, selector, args).await
    }
}

pub struct ContentArticles {
    pub(crate) ctx: ServiceContext,
}

impl ContentArticles {
    pub async fn get_by_id(&self, id: &str) -> Result<ArticleResponse, APIError> {
        remote::get_article_by_id(&self.ctx, id).await
    }

    pub async fn get_many(&self, args: GetArticlesArgs) -> Result<ArticlesResponse, APIError> {
        remote::get_articles(&self.ctx, args).await
    }
}

pub struct ContentMenus {
    pub(crate) ctx: ServiceContext,
}

impl ContentMenus {
    pub async fn get_by_handle(&self, handle: &str) -> Result<MenuResponse, APIError> {
        remote::get_menu(&self.ctx, handle).await
    }
}
