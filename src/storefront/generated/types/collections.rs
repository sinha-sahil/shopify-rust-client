use super::common::Image;
use super::common::PageInfo;
use super::common::ProductFilter;
use super::common::SEO;
use super::metafields::Metafield;
use super::products::ProductConnection;
use serde::{Deserialize, Serialize};

/// A collection of products to organize and make shops easier to browse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collection {
    /// Globally unique identifier
    pub id: String,
    /// Collection title (limit 255 characters)
    pub title: String,
    /// URL-friendly identifier (limit 255 characters)
    pub handle: String,
    /// Collection description (plain text)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Collection description (HTML)
    #[serde(rename = "descriptionHtml")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_html: Option<String>,
    /// When the collection was last updated
    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<time::OffsetDateTime>,
    /// Image associated with the collection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<Image>,
    /// SEO information for the collection
    pub seo: SEO,
    /// Products in the collection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub products: Option<ProductConnection>,
    /// A custom field associated with the collection
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

/// Paginated list of collections
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionConnection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edges: Option<Vec<CollectionEdge>>,
    pub nodes: Vec<Collection>,
    #[serde(rename = "pageInfo")]
    pub page_info: PageInfo,
    /// The total count of collections
    #[serde(rename = "totalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
}

/// An edge in a collection connection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionEdge {
    pub node: Collection,
    pub cursor: String,
}

/// Type of collection filter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FilterType {
    #[serde(rename = "LIST")]
    LIST,
    #[serde(rename = "PRICE_RANGE")]
    PRICERANGE,
    #[serde(rename = "BOOLEAN")]
    BOOLEAN,
}

/// A filter for products in a collection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Filter {
    pub id: String,
    pub label: String,
    pub r#type: FilterType,
    pub values: Vec<FilterValue>,
}

/// A value for a collection filter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterValue {
    pub id: String,
    pub label: String,
    /// Number of products matching this filter value
    pub count: f64,
    /// JSON-encoded filter input
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
}

/// Sort keys for collection queries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CollectionSortKeys {
    #[serde(rename = "TITLE")]
    TITLE,
    #[serde(rename = "UPDATED_AT")]
    UPDATEDAT,
    #[serde(rename = "ID")]
    ID,
    #[serde(rename = "RELEVANCE")]
    RELEVANCE,
}

/// Sort keys for products within a collection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProductCollectionSortKeys {
    #[serde(rename = "TITLE")]
    TITLE,
    #[serde(rename = "PRICE")]
    PRICE,
    #[serde(rename = "BEST_SELLING")]
    BESTSELLING,
    #[serde(rename = "CREATED")]
    CREATED,
    #[serde(rename = "ID")]
    ID,
    #[serde(rename = "MANUAL")]
    MANUAL,
    #[serde(rename = "COLLECTION_DEFAULT")]
    COLLECTIONDEFAULT,
    #[serde(rename = "RELEVANCE")]
    RELEVANCE,
}

/// Arguments for fetching collections
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCollectionsArgs {
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
    pub sort_key: Option<CollectionSortKeys>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
}

/// Arguments for fetching products in a collection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCollectionProductsArgs {
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
    pub sort_key: Option<ProductCollectionSortKeys>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<ProductFilter>>,
}
