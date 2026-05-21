use serde::{Deserialize, Serialize};

/// Represents a monetary value with currency (MoneyV2)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Money {
    /// Decimal money amount (stored as string for precision)
    pub amount: String,
    /// Currency code (ISO 4217, e.g., USD, EUR)
    #[serde(rename = "currencyCode")]
    pub currency_code: String,
}

/// Input for monetary values
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoneyInput {
    /// Decimal money amount
    pub amount: String,
    /// Currency code (ISO 4217)
    #[serde(rename = "currencyCode")]
    pub currency_code: String,
}

/// Represents an image resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    /// Globally unique identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The image URL
    pub url: String,
    /// Alt text for accessibility
    #[serde(rename = "altText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_text: Option<String>,
    /// Image width in pixels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    /// Image height in pixels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
}

/// Pagination information for connections
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageInfo {
    /// Whether there are more items after
    #[serde(rename = "hasNextPage")]
    pub has_next_page: bool,
    /// Whether there are more items before
    #[serde(rename = "hasPreviousPage")]
    pub has_previous_page: bool,
    /// Cursor for the first item
    #[serde(rename = "startCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_cursor: Option<String>,
    /// Cursor for the last item
    #[serde(rename = "endCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_cursor: Option<String>,
}

/// SEO metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SEO {
    /// SEO title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// SEO description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Key-value attribute
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attribute {
    /// Attribute key
    pub key: String,
    /// Attribute value
    pub value: String,
}

/// Input for key-value attributes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttributeInput {
    /// Attribute key
    pub key: String,
    /// Attribute value
    pub value: String,
}

/// A count of items with precision
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Count {
    /// The count value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    /// Precision of the count
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precision: Option<CountPrecision>,
}

/// Precision of a count value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CountPrecision {
    #[serde(rename = "EXACT")]
    EXACT,
    #[serde(rename = "AT_LEAST")]
    ATLEAST,
}

/// Price range for a product (deprecated - use ProductPriceRange)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceRange {
    /// The lowest variant price
    #[serde(rename = "minVariantPrice")]
    pub min_variant_price: Money,
    /// The highest variant price
    #[serde(rename = "maxVariantPrice")]
    pub max_variant_price: Money,
}

/// A selected product option
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectedOption {
    /// Option name (e.g., Size, Color)
    pub name: String,
    /// Option value (e.g., Medium, Red)
    pub value: String,
}

/// Pagination arguments for list queries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationArgs {
    /// Number of items to fetch from the start
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<f64>,
    /// Cursor to fetch items after
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Number of items to fetch from the end
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<f64>,
    /// Cursor to fetch items before
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
}

/// Filter for products in collections or search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductFilter {
    #[serde(rename = "productVendor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_vendor: Option<String>,
    #[serde(rename = "productType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_type: Option<String>,
    #[serde(rename = "variantOption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant_option: Option<VariantOptionFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<PriceFilter>,
    #[serde(rename = "productMetafield")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_metafield: Option<MetafieldFilter>,
    #[serde(rename = "variantMetafield")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant_metafield: Option<MetafieldFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

/// Filter by variant option
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariantOptionFilter {
    pub name: String,
    pub value: String,
}

/// Filter by price range
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
}

/// Filter by metafield value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetafieldFilter {
    pub namespace: String,
    pub key: String,
    pub value: String,
}
