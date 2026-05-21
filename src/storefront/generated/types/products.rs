use super::cart::SellingPlan;
use super::cart::SellingPlanAllocation;
use super::collections::Filter;
use super::common::Count;
use super::common::Image;
use super::common::Money;
use super::common::PageInfo;
use super::common::SelectedOption;
use super::common::SEO;
use super::customer::MailingAddress;
use super::metafields::Metafield;
use serde::{Deserialize, Serialize};

/// A product in the store with variants, options, and media
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    /// Globally unique identifier
    pub id: String,
    /// Product title
    pub title: String,
    /// URL-friendly identifier
    pub handle: String,
    /// Product description (plain text)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Product description (HTML)
    #[serde(rename = "descriptionHtml")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_html: Option<String>,
    /// Product vendor
    pub vendor: String,
    /// Product type
    #[serde(rename = "productType")]
    pub product_type: String,
    /// When the product was created
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<time::OffsetDateTime>,
    /// When the product was last updated
    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<time::OffsetDateTime>,
    /// When the product was published
    #[serde(rename = "publishedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_at: Option<time::OffsetDateTime>,
    /// Whether at least one variant is available for sale
    #[serde(rename = "availableForSale")]
    pub available_for_sale: bool,
    /// Total inventory across all variants
    #[serde(rename = "totalInventory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_inventory: Option<i32>,
    /// The featured image (equivalent to images first: 1)
    #[serde(rename = "featuredImage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub featured_image: Option<Image>,
    /// Min and max prices across variants
    #[serde(rename = "priceRange")]
    pub price_range: ProductPriceRange,
    /// Compare at price range for sale pricing
    #[serde(rename = "compareAtPriceRange")]
    pub compare_at_price_range: ProductPriceRange,
    pub seo: SEO,
    /// Searchable keywords associated with the product
    pub tags: Vec<String>,
    /// Product options like Size, Color
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<ProductOption>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<ImageConnection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variants: Option<ProductVariantConnection>,
    /// A custom field associated with the product
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metafield: Option<Metafield>,
    /// List of custom fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metafields: Option<Vec<Metafield>>,
    /// Whether the product is a gift card
    #[serde(rename = "isGiftCard")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_gift_card: Option<bool>,
    /// URL on the online store (null if not published)
    #[serde(rename = "onlineStoreUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub online_store_url: Option<String>,
    /// Whether the product can only be purchased with a selling plan
    #[serde(rename = "requiresSellingPlan")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requires_selling_plan: Option<bool>,
    /// Product category from Shopify Standard Product Taxonomy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<TaxonomyCategory>,
    /// Media associated with the product (images, videos, 3D models)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<MediaConnection>,
    /// Selling plan groups for subscriptions
    #[serde(rename = "sellingPlanGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selling_plan_groups: Option<SellingPlanGroupConnection>,
    /// Variants differing by one option from selected options
    #[serde(rename = "adjacentVariants")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjacent_variants: Option<Vec<ProductVariant>>,
    /// Find active variant based on selected options or first available
    #[serde(rename = "selectedOrFirstAvailableVariant")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_or_first_available_variant: Option<ProductVariant>,
    /// Find variant by selected options
    #[serde(rename = "variantBySelectedOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant_by_selected_options: Option<ProductVariant>,
    /// Number of variants
    #[serde(rename = "variantsCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variants_count: Option<Count>,
    /// Encoded string of all option value combinations with variants
    #[serde(rename = "encodedVariantExistence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoded_variant_existence: Option<String>,
    /// Encoded string of available variant combinations
    #[serde(rename = "encodedVariantAvailability")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoded_variant_availability: Option<String>,
    /// URL parameters for analytics tracking
    #[serde(rename = "trackingParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_parameters: Option<String>,
}

/// The price range of the product
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductPriceRange {
    /// The lowest variant price
    #[serde(rename = "minVariantPrice")]
    pub min_variant_price: Money,
    /// The highest variant price
    #[serde(rename = "maxVariantPrice")]
    pub max_variant_price: Money,
}

/// Product category from Shopify Standard Product Taxonomy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxonomyCategory {
    /// Category ID
    pub id: String,
    /// Category name
    pub name: String,
    /// Full category path
    #[serde(rename = "fullName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    /// Parent categories
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ancestors: Option<Vec<TaxonomyCategory>>,
}

/// A product option (e.g., Size, Color)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductOption {
    pub id: String,
    /// Option name (limit 255 characters)
    pub name: String,
    /// Option values with swatch support
    #[serde(rename = "optionValues")]
    pub option_values: Vec<ProductOptionValue>,
}

/// A product option value (e.g., "Red", "Blue")
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductOptionValue {
    /// Globally unique identifier
    pub id: String,
    /// The name of the option value
    pub name: String,
    /// Visual swatch for the option value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swatch: Option<ProductOptionValueSwatch>,
    /// The first selectable variant with this option value
    #[serde(rename = "firstSelectableVariant")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_selectable_variant: Option<ProductVariant>,
}

/// Visual swatch for a product option value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductOptionValueSwatch {
    /// Color hex code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    /// Swatch image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<Image>,
}

/// A product variant representing a specific version of a product
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductVariant {
    /// Globally unique identifier
    pub id: String,
    /// Variant title
    pub title: String,
    /// Stock keeping unit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    /// Barcode (ISBN, UPC, or GTIN)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub barcode: Option<String>,
    /// Whether the variant is available for sale
    #[serde(rename = "availableForSale")]
    pub available_for_sale: bool,
    /// Total sellable quantity for online sales channels
    #[serde(rename = "quantityAvailable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity_available: Option<i32>,
    /// Whether out of stock but available for backorder
    #[serde(rename = "currentlyNotInStock")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currently_not_in_stock: Option<bool>,
    /// Whether shipping is required
    #[serde(rename = "requiresShipping")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requires_shipping: Option<bool>,
    /// Whether variant requires components (bundle parent)
    #[serde(rename = "requiresComponents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requires_components: Option<bool>,
    /// Whether tax is charged when sold
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxable: Option<bool>,
    /// Weight in the specified unit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<f64>,
    #[serde(rename = "weightUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight_unit: Option<WeightUnit>,
    /// The variant price
    pub price: Money,
    /// Compare at price (for sale pricing)
    #[serde(rename = "compareAtPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compare_at_price: Option<Money>,
    /// Image associated with the variant
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<Image>,
    /// List of selected options for this variant
    #[serde(rename = "selectedOptions")]
    pub selected_options: Vec<SelectedOption>,
    /// Summary of the product this variant belongs to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<ProductSummary>,
    /// Unit price based on measurement
    #[serde(rename = "unitPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<Money>,
    #[serde(rename = "unitPriceMeasurement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_price_measurement: Option<UnitPriceMeasurement>,
    /// A custom field associated with the variant
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metafield: Option<Metafield>,
    /// List of custom fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metafields: Option<Vec<Metafield>>,
    /// Selling plan allocations for subscriptions
    #[serde(rename = "sellingPlanAllocations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selling_plan_allocations: Option<SellingPlanAllocationConnection>,
    /// In-store pickup availability by location
    #[serde(rename = "storeAvailability")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_availability: Option<StoreAvailabilityConnection>,
    /// Quantity rules for the variant
    #[serde(rename = "quantityRule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity_rule: Option<QuantityRule>,
    /// B2B quantity price breaks
    #[serde(rename = "quantityPriceBreaks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity_price_breaks: Option<QuantityPriceBreakConnection>,
    /// Bundle components (for fixed bundles)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<ProductVariantComponentConnection>,
    /// Bundles that include this variant
    #[serde(rename = "groupedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grouped_by: Option<ProductVariantConnection>,
    /// Shop Pay Installments pricing
    #[serde(rename = "shopPayInstallmentsPricing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shop_pay_installments_pricing: Option<ShopPayInstallmentsProductVariantPricing>,
}

/// Unit of measurement for weight
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WeightUnit {
    #[serde(rename = "GRAMS")]
    GRAMS,
    #[serde(rename = "KILOGRAMS")]
    KILOGRAMS,
    #[serde(rename = "OUNCES")]
    OUNCES,
    #[serde(rename = "POUNDS")]
    POUNDS,
}

/// Minimal product info for nested references
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductSummary {
    pub id: String,
    pub title: String,
    pub handle: String,
}

/// Unit price measurement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnitPriceMeasurement {
    #[serde(rename = "measuredType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub measured_type: Option<UnitPriceMeasurementMeasuredType>,
    #[serde(rename = "quantityUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity_unit: Option<UnitPriceMeasurementMeasuredUnit>,
    #[serde(rename = "quantityValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity_value: Option<f64>,
    #[serde(rename = "referenceUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_unit: Option<UnitPriceMeasurementMeasuredUnit>,
    #[serde(rename = "referenceValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_value: Option<i32>,
}

/// Type of unit price measurement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UnitPriceMeasurementMeasuredType {
    #[serde(rename = "VOLUME")]
    VOLUME,
    #[serde(rename = "WEIGHT")]
    WEIGHT,
    #[serde(rename = "LENGTH")]
    LENGTH,
    #[serde(rename = "AREA")]
    AREA,
}

/// Unit of measurement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UnitPriceMeasurementMeasuredUnit {
    #[serde(rename = "ML")]
    ML,
    #[serde(rename = "CL")]
    CL,
    #[serde(rename = "L")]
    L,
    #[serde(rename = "M3")]
    M3,
    #[serde(rename = "MG")]
    MG,
    #[serde(rename = "G")]
    G,
    #[serde(rename = "KG")]
    KG,
    #[serde(rename = "MM")]
    MM,
    #[serde(rename = "CM")]
    CM,
    #[serde(rename = "M")]
    M,
    #[serde(rename = "M2")]
    M2,
}

/// Quantity rules for a product variant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantityRule {
    /// Minimum quantity that must be purchased
    pub minimum: i32,
    /// Maximum quantity that can be purchased (null for unlimited)
    pub maximum: i32,
    /// Quantity must be a multiple of this value
    pub increment: i32,
}

/// A quantity-based price break for B2B
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantityPriceBreak {
    /// Minimum quantity to qualify for this price
    #[serde(rename = "minimumQuantity")]
    pub minimum_quantity: i32,
    /// Price at this quantity level
    pub price: Money,
}

/// Paginated list of quantity price breaks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantityPriceBreakConnection {
    pub edges: Vec<QuantityPriceBreakEdge>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<QuantityPriceBreak>>,
    #[serde(rename = "pageInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_info: Option<PageInfo>,
}

/// An edge in a quantity price break connection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantityPriceBreakEdge {
    pub node: QuantityPriceBreak,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

/// In-store pickup availability for a variant at a location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreAvailability {
    /// Whether the variant is available for pickup
    pub available: bool,
    /// Estimated pickup time (e.g., "Usually ready in 24 hours")
    #[serde(rename = "pickUpTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pick_up_time: Option<String>,
    /// Quantity available at this location
    #[serde(rename = "quantityAvailable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity_available: Option<i32>,
    pub location: Location,
}

/// Paginated list of store availability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreAvailabilityConnection {
    pub edges: Vec<StoreAvailabilityEdge>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<StoreAvailability>>,
    #[serde(rename = "pageInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_info: Option<PageInfo>,
}

/// An edge in a store availability connection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreAvailabilityEdge {
    pub node: StoreAvailability,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

/// A store location for pickup
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    /// Globally unique identifier
    pub id: String,
    /// Location name
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<MailingAddress>,
}

/// A component of a bundle variant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductVariantComponent {
    #[serde(rename = "productVariant")]
    pub product_variant: ProductVariant,
    pub quantity: i32,
}

/// Paginated list of bundle components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductVariantComponentConnection {
    pub edges: Vec<ProductVariantComponentEdge>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<ProductVariantComponent>>,
    #[serde(rename = "pageInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_info: Option<PageInfo>,
}

/// An edge in a component connection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductVariantComponentEdge {
    pub node: ProductVariantComponent,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

/// Shop Pay Installments pricing for a variant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShopPayInstallmentsProductVariantPricing {
    /// Whether eligible for Shop Pay Installments
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eligible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<Money>,
}

/// Paginated list of product variants
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductVariantConnection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edges: Option<Vec<ProductVariantEdge>>,
    pub nodes: Vec<ProductVariant>,
    #[serde(rename = "pageInfo")]
    pub page_info: PageInfo,
}

/// An edge in a product variant connection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductVariantEdge {
    pub node: ProductVariant,
    pub cursor: String,
}

/// Paginated list of products
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductConnection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edges: Option<Vec<ProductEdge>>,
    pub nodes: Vec<Product>,
    /// Available filters for products
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    #[serde(rename = "pageInfo")]
    pub page_info: PageInfo,
}

/// An edge in a product connection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductEdge {
    pub node: Product,
    pub cursor: String,
}

/// Paginated list of images
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageConnection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edges: Option<Vec<ImageEdge>>,
    pub nodes: Vec<Image>,
    #[serde(rename = "pageInfo")]
    pub page_info: PageInfo,
}

/// An edge in an image connection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageEdge {
    pub node: Image,
    pub cursor: String,
}

/// Media associated with a product (base type)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Media {
    /// Globally unique identifier
    pub id: String,
    #[serde(rename = "mediaContentType")]
    pub media_content_type: MediaContentType,
    /// Alt text for accessibility
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt: Option<String>,
    /// Preview image for the media
    #[serde(rename = "previewImage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_image: Option<Image>,
}

/// The type of media content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MediaContentType {
    #[serde(rename = "IMAGE")]
    IMAGE,
    #[serde(rename = "VIDEO")]
    VIDEO,
    #[serde(rename = "EXTERNAL_VIDEO")]
    EXTERNALVIDEO,
    #[serde(rename = "MODEL_3D")]
    MODEL3D,
}

/// An image media type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaImage {
    pub id: String,
    #[serde(rename = "mediaContentType")]
    pub media_content_type: MediaContentType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt: Option<String>,
    #[serde(rename = "previewImage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_image: Option<Image>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<Image>,
}

/// A video media type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Video {
    pub id: String,
    #[serde(rename = "mediaContentType")]
    pub media_content_type: MediaContentType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt: Option<String>,
    #[serde(rename = "previewImage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_image: Option<Image>,
    pub sources: Vec<VideoSource>,
}

/// A video source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoSource {
    pub url: String,
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
}

/// An external video (e.g., YouTube, Vimeo)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalVideo {
    pub id: String,
    #[serde(rename = "mediaContentType")]
    pub media_content_type: MediaContentType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt: Option<String>,
    #[serde(rename = "previewImage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_image: Option<Image>,
    /// URL to embed the video
    #[serde(rename = "embedUrl")]
    pub embed_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<MediaHost>,
    /// Original URL of the video
    #[serde(rename = "originUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_url: Option<String>,
}

/// Host of external video
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MediaHost {
    #[serde(rename = "YOUTUBE")]
    YOUTUBE,
    #[serde(rename = "VIMEO")]
    VIMEO,
}

/// A 3D model media type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Model3d {
    pub id: String,
    #[serde(rename = "mediaContentType")]
    pub media_content_type: MediaContentType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt: Option<String>,
    #[serde(rename = "previewImage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_image: Option<Image>,
    pub sources: Vec<Model3dSource>,
}

/// A 3D model source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Model3dSource {
    pub url: String,
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filesize: Option<i32>,
}

/// Paginated list of media
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaConnection {
    pub edges: Vec<MediaEdge>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<Media>>,
    #[serde(rename = "pageInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_info: Option<PageInfo>,
}

/// An edge in a media connection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaEdge {
    pub node: Media,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

/// A group of selling plans (e.g., Subscribe and save)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SellingPlanGroup {
    /// The name of the selling plan group
    pub name: String,
    /// Name of the app that created the group
    #[serde(rename = "appName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_name: Option<String>,
    /// Options available in the storefront dropdown
    pub options: Vec<SellingPlanGroupOption>,
    #[serde(rename = "sellingPlans")]
    pub selling_plans: SellingPlanConnection,
}

/// An option in a selling plan group
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SellingPlanGroupOption {
    /// The name of the option (e.g., "Delivery every")
    pub name: String,
    /// Available values (e.g., "1 week", "2 weeks")
    pub values: Vec<String>,
}

/// Paginated list of selling plan groups
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SellingPlanGroupConnection {
    pub edges: Vec<SellingPlanGroupEdge>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<SellingPlanGroup>>,
    #[serde(rename = "pageInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_info: Option<PageInfo>,
}

/// An edge in a selling plan group connection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SellingPlanGroupEdge {
    pub node: SellingPlanGroup,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

/// Paginated list of selling plans
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SellingPlanConnection {
    pub edges: Vec<SellingPlanEdge>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<SellingPlan>>,
    #[serde(rename = "pageInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_info: Option<PageInfo>,
}

/// An edge in a selling plan connection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SellingPlanEdge {
    pub node: SellingPlan,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

/// Paginated list of selling plan allocations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SellingPlanAllocationConnection {
    pub edges: Vec<SellingPlanAllocationEdge>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<SellingPlanAllocation>>,
    #[serde(rename = "pageInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_info: Option<PageInfo>,
}

/// An edge in a selling plan allocation connection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SellingPlanAllocationEdge {
    pub node: SellingPlanAllocation,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

/// Sort keys for product queries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProductSortKeys {
    #[serde(rename = "TITLE")]
    TITLE,
    #[serde(rename = "PRODUCT_TYPE")]
    PRODUCTTYPE,
    #[serde(rename = "VENDOR")]
    VENDOR,
    #[serde(rename = "UPDATED_AT")]
    UPDATEDAT,
    #[serde(rename = "CREATED_AT")]
    CREATEDAT,
    #[serde(rename = "BEST_SELLING")]
    BESTSELLING,
    #[serde(rename = "PRICE")]
    PRICE,
    #[serde(rename = "ID")]
    ID,
    #[serde(rename = "RELEVANCE")]
    RELEVANCE,
}

/// Intent for product recommendations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProductRecommendationIntent {
    #[serde(rename = "RELATED")]
    RELATED,
    #[serde(rename = "COMPLEMENTARY")]
    COMPLEMENTARY,
}

/// Arguments for fetching products
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetProductsArgs {
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
    pub sort_key: Option<ProductSortKeys>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
}

/// Arguments for fetching product recommendations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetProductRecommendationsArgs {
    #[serde(rename = "productId")]
    pub product_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent: Option<ProductRecommendationIntent>,
}
