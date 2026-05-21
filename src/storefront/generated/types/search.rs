use super::collections::Collection;
use super::collections::Filter;
use super::common::Image;
use super::common::PageInfo;
use super::common::ProductFilter;
use super::content::Article;
use super::content::ArticleAuthor;
use super::content::BlogSummary;
use super::content::Page;
use super::products::Product;
use super::products::ProductPriceRange;
use serde::{Deserialize, Serialize};

/// Type of search result item (discriminator for union)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SearchResultItemType {
    #[serde(rename = "PRODUCT")]
    PRODUCT,
    #[serde(rename = "PAGE")]
    PAGE,
    #[serde(rename = "ARTICLE")]
    ARTICLE,
    #[serde(rename = "COLLECTION")]
    COLLECTION,
}

/// GraphQL type name for search result items
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SearchResultItemTypename {
    #[serde(rename = "Product")]
    Product,
    #[serde(rename = "Page")]
    Page,
    #[serde(rename = "Article")]
    Article,
    #[serde(rename = "Collection")]
    Collection,
}

/// Search results from a full-text search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResultItemConnection {
    /// Total number of matching results
    #[serde(rename = "totalCount")]
    pub total_count: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edges: Option<Vec<SearchResultItemEdge>>,
    pub nodes: Vec<SearchResultItem>,
    #[serde(rename = "pageInfo")]
    pub page_info: PageInfo,
    /// Available product filters for refining search
    #[serde(rename = "productFilters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_filters: Option<Vec<Filter>>,
}

/// An edge in search results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResultItemEdge {
    pub node: SearchResultItem,
    pub cursor: String,
}

/// A search result item - union of Product, Page, Article, Collection. Check __typename field.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResultItem {
    /// The GraphQL type name (Product, Page, Article, Collection)
    #[serde(rename = "__typename")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _typename: Option<SearchResultItemTypename>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<String>,
    /// URL parameters for analytics tracking
    #[serde(rename = "trackingParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_parameters: Option<String>,
    #[serde(rename = "availableForSale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_for_sale: Option<bool>,
    #[serde(rename = "priceRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_range: Option<ProductPriceRange>,
    #[serde(rename = "compareAtPriceRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compare_at_price_range: Option<ProductPriceRange>,
    #[serde(rename = "featuredImage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub featured_image: Option<Image>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,
    #[serde(rename = "productType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(rename = "bodySummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(rename = "contentHtml")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excerpt: Option<String>,
    #[serde(rename = "excerptHtml")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excerpt_html: Option<String>,
    #[serde(rename = "publishedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_at: Option<time::OffsetDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<ArticleAuthor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blog: Option<BlogSummary>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "descriptionHtml")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<Image>,
}

/// Results from predictive search query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictiveSearchResult {
    /// Products matching the search
    pub products: Vec<Product>,
    /// Collections matching the search
    pub collections: Vec<Collection>,
    /// Pages matching the search
    pub pages: Vec<Page>,
    /// Articles matching the search
    pub articles: Vec<Article>,
    /// Search query suggestions
    pub queries: Vec<SearchQuerySuggestion>,
}

/// A search query suggestion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchQuerySuggestion {
    /// The suggested query text
    pub text: String,
    /// Query text with HTML highlighting
    #[serde(rename = "styledText")]
    pub styled_text: String,
    /// URL parameters for analytics tracking
    #[serde(rename = "trackingParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_parameters: Option<String>,
}

/// Sort keys for search queries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SearchSortKeys {
    #[serde(rename = "RELEVANCE")]
    RELEVANCE,
    #[serde(rename = "PRICE")]
    PRICE,
}

/// Types of resources to search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SearchType {
    #[serde(rename = "PRODUCT")]
    PRODUCT,
    #[serde(rename = "PAGE")]
    PAGE,
    #[serde(rename = "ARTICLE")]
    ARTICLE,
}

/// Types of resources for predictive search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PredictiveSearchType {
    #[serde(rename = "PRODUCT")]
    PRODUCT,
    #[serde(rename = "COLLECTION")]
    COLLECTION,
    #[serde(rename = "PAGE")]
    PAGE,
    #[serde(rename = "ARTICLE")]
    ARTICLE,
    #[serde(rename = "QUERY")]
    QUERY,
}

/// How to treat the last word in the search query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SearchPrefixQueryType {
    #[serde(rename = "LAST")]
    LAST,
    #[serde(rename = "NONE")]
    NONE,
}

/// How to handle unavailable products in search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SearchUnavailableProductsType {
    #[serde(rename = "SHOW")]
    SHOW,
    #[serde(rename = "HIDE")]
    HIDE,
    #[serde(rename = "LAST")]
    LAST,
}

/// Scope for predictive search result limit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PredictiveSearchLimitScope {
    #[serde(rename = "ALL")]
    ALL,
    #[serde(rename = "EACH")]
    EACH,
}

/// Specifies fields to search within
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SearchableField {
    #[serde(rename = "AUTHOR")]
    AUTHOR,
    #[serde(rename = "BODY")]
    BODY,
    #[serde(rename = "PRODUCT_TYPE")]
    PRODUCTTYPE,
    #[serde(rename = "TAG")]
    TAG,
    #[serde(rename = "TITLE")]
    TITLE,
    #[serde(rename = "VARIANTS_BARCODE")]
    VARIANTSBARCODE,
    #[serde(rename = "VARIANTS_SKU")]
    VARIANTSSKU,
    #[serde(rename = "VARIANTS_TITLE")]
    VARIANTSTITLE,
    #[serde(rename = "VENDOR")]
    VENDOR,
}

/// Arguments for search queries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchArgs {
    /// Search query string
    pub query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse: Option<bool>,
    #[serde(rename = "sortKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_key: Option<SearchSortKeys>,
    /// Types of resources to search
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<SearchType>>,
    #[serde(rename = "productFilters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_filters: Option<Vec<ProductFilter>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<SearchPrefixQueryType>,
    #[serde(rename = "unavailableProducts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unavailable_products: Option<SearchUnavailableProductsType>,
}

/// Arguments for predictive search queries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictiveSearchArgs {
    /// Search query string
    pub query: String,
    /// Maximum number of results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "limitScope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_scope: Option<PredictiveSearchLimitScope>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<PredictiveSearchType>>,
    /// Fields to search within
    #[serde(rename = "searchableFields")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub searchable_fields: Option<Vec<SearchableField>>,
    #[serde(rename = "unavailableProducts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unavailable_products: Option<SearchUnavailableProductsType>,
}
