use super::common::Image;
use super::common::PageInfo;
use super::common::SEO;
use super::metafields::Metafield;
use serde::{Deserialize, Serialize};

/// A page holding static HTML content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page {
    /// Globally unique identifier
    pub id: String,
    /// Page title
    pub title: String,
    /// URL-friendly identifier
    pub handle: String,
    /// Page content (HTML)
    pub body: String,
    /// Summary of the page content
    #[serde(rename = "bodySummary")]
    pub body_summary: String,
    /// When the page was created
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<time::OffsetDateTime>,
    /// When the page was last updated
    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<time::OffsetDateTime>,
    /// SEO information for the page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seo: Option<SEO>,
    /// A custom field associated with the page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metafield: Option<Metafield>,
    /// List of custom fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metafields: Option<Vec<Metafield>>,
    /// URL on the online store (null if not published)
    #[serde(rename = "onlineStoreUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub online_store_url: Option<String>,
    /// URL parameters for analytics tracking
    #[serde(rename = "trackingParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_parameters: Option<String>,
}

/// Paginated list of pages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageConnection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edges: Option<Vec<PageEdge>>,
    pub nodes: Vec<Page>,
    #[serde(rename = "pageInfo")]
    pub page_info: PageInfo,
}

/// An edge in a page connection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageEdge {
    pub node: Page,
    pub cursor: String,
}

/// A blog containing articles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Blog {
    /// Globally unique identifier
    pub id: String,
    /// Blog title
    pub title: String,
    /// URL-friendly identifier
    pub handle: String,
    /// SEO information for the blog
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seo: Option<SEO>,
    /// Articles in the blog
    #[serde(skip_serializing_if = "Option::is_none")]
    pub articles: Option<ArticleConnection>,
    /// Authors who have written articles in this blog
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authors: Option<Vec<ArticleAuthor>>,
    /// A custom field associated with the blog
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metafield: Option<Metafield>,
    /// List of custom fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metafields: Option<Vec<Metafield>>,
    /// URL on the online store
    #[serde(rename = "onlineStoreUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub online_store_url: Option<String>,
}

/// Paginated list of blogs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogConnection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edges: Option<Vec<BlogEdge>>,
    pub nodes: Vec<Blog>,
    #[serde(rename = "pageInfo")]
    pub page_info: PageInfo,
}

/// An edge in a blog connection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogEdge {
    pub node: Blog,
    pub cursor: String,
}

/// A blog article
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Article {
    /// Globally unique identifier
    pub id: String,
    /// Article title
    pub title: String,
    /// URL-friendly identifier
    pub handle: String,
    /// Article content (plain text)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// Article content (HTML)
    #[serde(rename = "contentHtml")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_html: Option<String>,
    /// Article excerpt (plain text)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excerpt: Option<String>,
    /// Article excerpt (HTML)
    #[serde(rename = "excerptHtml")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excerpt_html: Option<String>,
    /// When the article was published
    #[serde(rename = "publishedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_at: Option<time::OffsetDateTime>,
    /// The author of the article
    #[serde(rename = "authorV2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_v2: Option<ArticleAuthor>,
    /// Featured image for the article
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<Image>,
    /// SEO information for the article
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seo: Option<SEO>,
    /// Tags associated with the article
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// The blog this article belongs to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blog: Option<BlogSummary>,
    /// A custom field associated with the article
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metafield: Option<Metafield>,
    /// List of custom fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metafields: Option<Vec<Metafield>>,
    /// URL on the online store
    #[serde(rename = "onlineStoreUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub online_store_url: Option<String>,
    /// URL parameters for analytics tracking
    #[serde(rename = "trackingParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_parameters: Option<String>,
}

/// An article author
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleAuthor {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "firstName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(rename = "lastName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
}

/// Minimal blog info for nested references
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogSummary {
    pub id: String,
    pub title: String,
    pub handle: String,
}

/// Paginated list of articles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleConnection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edges: Option<Vec<ArticleEdge>>,
    pub nodes: Vec<Article>,
    #[serde(rename = "pageInfo")]
    pub page_info: PageInfo,
}

/// An edge in an article connection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleEdge {
    pub node: Article,
    pub cursor: String,
}

/// A navigation menu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Menu {
    pub id: String,
    pub handle: String,
    pub title: String,
    pub items: Vec<MenuItem>,
}

/// Type of menu item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MenuItemType {
    #[serde(rename = "HTTP")]
    HTTP,
    #[serde(rename = "COLLECTION")]
    COLLECTION,
    #[serde(rename = "PRODUCT")]
    PRODUCT,
    #[serde(rename = "PAGE")]
    PAGE,
    #[serde(rename = "BLOG")]
    BLOG,
    #[serde(rename = "ARTICLE")]
    ARTICLE,
    #[serde(rename = "SHOP_POLICY")]
    SHOPPOLICY,
    #[serde(rename = "SEARCH")]
    SEARCH,
    #[serde(rename = "CATALOG")]
    CATALOG,
    #[serde(rename = "FRONTPAGE")]
    FRONTPAGE,
}

/// Type of resource a menu item links to
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MenuItemResourceType {
    #[serde(rename = "Collection")]
    Collection,
    #[serde(rename = "Product")]
    Product,
    #[serde(rename = "Page")]
    Page,
    #[serde(rename = "Blog")]
    Blog,
    #[serde(rename = "Article")]
    Article,
    #[serde(rename = "ShopPolicy")]
    ShopPolicy,
}

/// A menu item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuItem {
    pub id: String,
    pub title: String,
    pub r#type: MenuItemType,
    pub url: String,
    /// Nested menu items
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<MenuItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<MenuItemResource>,
}

/// The resource a menu item links to
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuItemResource {
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<MenuItemResourceType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<String>,
}

/// Sort keys for page queries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PageSortKeys {
    #[serde(rename = "TITLE")]
    TITLE,
    #[serde(rename = "UPDATED_AT")]
    UPDATEDAT,
    #[serde(rename = "ID")]
    ID,
    #[serde(rename = "RELEVANCE")]
    RELEVANCE,
}

/// Sort keys for blog queries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BlogSortKeys {
    #[serde(rename = "TITLE")]
    TITLE,
    #[serde(rename = "HANDLE")]
    HANDLE,
    #[serde(rename = "ID")]
    ID,
    #[serde(rename = "RELEVANCE")]
    RELEVANCE,
}

/// Sort keys for article queries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArticleSortKeys {
    #[serde(rename = "TITLE")]
    TITLE,
    #[serde(rename = "BLOG_TITLE")]
    BLOGTITLE,
    #[serde(rename = "AUTHOR")]
    AUTHOR,
    #[serde(rename = "UPDATED_AT")]
    UPDATEDAT,
    #[serde(rename = "PUBLISHED_AT")]
    PUBLISHEDAT,
    #[serde(rename = "ID")]
    ID,
    #[serde(rename = "RELEVANCE")]
    RELEVANCE,
}

/// Arguments for fetching pages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPagesArgs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse: Option<bool>,
    #[serde(rename = "sortKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_key: Option<PageSortKeys>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
}

/// Arguments for fetching blogs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBlogsArgs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse: Option<bool>,
    #[serde(rename = "sortKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_key: Option<BlogSortKeys>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
}

/// Arguments for fetching articles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetArticlesArgs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse: Option<bool>,
    #[serde(rename = "sortKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_key: Option<ArticleSortKeys>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
}

/// Arguments for fetching articles within a blog
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBlogArticlesArgs {
    #[serde(rename = "articlesFirst")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub articles_first: Option<f64>,
    #[serde(rename = "articlesAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub articles_after: Option<String>,
    #[serde(rename = "articlesReverse")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub articles_reverse: Option<bool>,
    #[serde(rename = "articlesSortKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub articles_sort_key: Option<ArticleSortKeys>,
}
