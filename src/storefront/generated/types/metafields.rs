use super::common::Image;
use super::common::PageInfo;
use super::products::Model3dSource;
use super::products::VideoSource;
use serde::{Deserialize, Serialize};

/// Type of resource a metafield references (discriminator for union)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetafieldReferenceType {
    #[serde(rename = "Collection")]
    Collection,
    #[serde(rename = "GenericFile")]
    GenericFile,
    #[serde(rename = "MediaImage")]
    MediaImage,
    #[serde(rename = "Metaobject")]
    Metaobject,
    #[serde(rename = "Model3d")]
    Model3d,
    #[serde(rename = "Page")]
    Page,
    #[serde(rename = "Product")]
    Product,
    #[serde(rename = "ProductVariant")]
    ProductVariant,
    #[serde(rename = "Video")]
    Video,
}

/// GraphQL type name for metafield parent resources
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetafieldParentResourceTypename {
    #[serde(rename = "Article")]
    Article,
    #[serde(rename = "Blog")]
    Blog,
    #[serde(rename = "Cart")]
    Cart,
    #[serde(rename = "Collection")]
    Collection,
    #[serde(rename = "Customer")]
    Customer,
    #[serde(rename = "Location")]
    Location,
    #[serde(rename = "Market")]
    Market,
    #[serde(rename = "Order")]
    Order,
    #[serde(rename = "Page")]
    Page,
    #[serde(rename = "Product")]
    Product,
    #[serde(rename = "ProductVariant")]
    ProductVariant,
    #[serde(rename = "Shop")]
    Shop,
}

/// GraphQL type name for metafield references
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetafieldReferenceTypename {
    #[serde(rename = "Collection")]
    Collection,
    #[serde(rename = "GenericFile")]
    GenericFile,
    #[serde(rename = "MediaImage")]
    MediaImage,
    #[serde(rename = "Metaobject")]
    Metaobject,
    #[serde(rename = "Model3d")]
    Model3d,
    #[serde(rename = "Page")]
    Page,
    #[serde(rename = "Product")]
    Product,
    #[serde(rename = "ProductVariant")]
    ProductVariant,
    #[serde(rename = "Video")]
    Video,
}

/// A metafield representing custom metadata attached to a resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metafield {
    /// Globally unique identifier
    pub id: String,
    /// Container for a group of metafields
    pub namespace: String,
    /// Unique identifier within the namespace
    pub key: String,
    /// Value stored as a string (may be JSON for complex types)
    pub value: String,
    /// Metafield type (e.g., single_line_text_field, json, number_integer)
    pub r#type: String,
    /// Description of the metafield
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// When the metafield was created
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<time::OffsetDateTime>,
    /// When the metafield was last updated
    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<time::OffsetDateTime>,
    /// The resource the metafield is attached to
    #[serde(rename = "parentResource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_resource: Option<MetafieldParentResource>,
    /// Referenced object if type is a resource reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<MetafieldReference>,
    /// List of references if type is a resource reference list
    #[serde(skip_serializing_if = "Option::is_none")]
    pub references: Option<MetafieldReferenceConnection>,
}

/// Union type for resources that can have metafields. Check __typename.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetafieldParentResource {
    /// The GraphQL type name
    #[serde(rename = "__typename")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _typename: Option<MetafieldParentResourceTypename>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<String>,
}

/// A reference to another resource from a metafield. Check __typename for actual type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetafieldReference {
    /// The GraphQL type name
    #[serde(rename = "__typename")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _typename: Option<MetafieldReferenceTypename>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    #[serde(rename = "mimeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(rename = "originalFileSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_file_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<Image>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<VideoSource>>,
    #[serde(rename = "previewImage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_image: Option<Image>,
    #[serde(rename = "model3dSources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model3d_sources: Option<Vec<Model3dSource>>,
}

/// A generic file reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenericFile {
    pub id: String,
    /// URL of the file
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt: Option<String>,
    #[serde(rename = "mimeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    /// File size in bytes
    #[serde(rename = "originalFileSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_file_size: Option<i32>,
    #[serde(rename = "previewImage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_image: Option<Image>,
}

/// Paginated list of metafield references
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetafieldReferenceConnection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edges: Option<Vec<MetafieldReferenceEdge>>,
    pub nodes: Vec<MetafieldReference>,
    #[serde(rename = "pageInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_info: Option<PageInfo>,
}

/// An edge in a metafield reference connection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetafieldReferenceEdge {
    pub node: MetafieldReference,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

/// A metaobject instance based on a MetaobjectDefinition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metaobject {
    /// Globally unique identifier
    pub id: String,
    /// Unique handle (useful as custom ID)
    pub handle: String,
    /// The type of the metaobject
    pub r#type: String,
    /// When the metaobject was last updated
    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<time::OffsetDateTime>,
    /// All object fields with defined values
    pub fields: Vec<MetaobjectField>,
    /// Access a single field by key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<MetaobjectField>,
    /// URL on the online store (for renderable metaobjects)
    #[serde(rename = "onlineStoreUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub online_store_url: Option<String>,
    /// SEO information (for renderable metaobjects)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seo: Option<MetaobjectSEO>,
}

/// SEO information for a metaobject
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaobjectSEO {
    /// The SEO title field
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<MetaobjectField>,
    /// The meta description field
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<MetaobjectField>,
}

/// A field in a metaobject
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaobjectField {
    /// The field key
    pub key: String,
    /// The field value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// Field type (e.g., single_line_text_field, file_reference)
    pub r#type: String,
    /// Referenced object if field is a reference type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<MetafieldReference>,
    /// List of references if field is a reference list type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub references: Option<MetafieldReferenceConnection>,
}

/// Paginated list of metaobjects
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaobjectConnection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edges: Option<Vec<MetaobjectEdge>>,
    pub nodes: Vec<Metaobject>,
    #[serde(rename = "pageInfo")]
    pub page_info: PageInfo,
}

/// An edge in a metaobject connection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaobjectEdge {
    pub node: Metaobject,
    pub cursor: String,
}

/// Input for fetching a metaobject by handle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaobjectHandleInput {
    /// The handle of the metaobject
    pub handle: String,
    /// The type of the metaobject
    pub r#type: String,
}

/// Input to identify a metafield by namespace and key
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HasMetafieldsIdentifier {
    /// The namespace (omit for app-reserved namespace)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// The metafield key
    pub key: String,
}

/// Arguments for fetching metaobjects
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMetaobjectsArgs {
    /// The metaobject type to fetch
    pub r#type: String,
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
    /// Sort key for the query
    #[serde(rename = "sortKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_key: Option<String>,
}
