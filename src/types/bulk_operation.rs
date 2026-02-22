use crate::common::types::UserError;

// region: Core Types

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BulkOperation {
    pub id: String,
    pub status: BulkOperationStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<BulkOperationErrorCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_count: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_object_count: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partial_data_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_type: Option<BulkOperationType>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BulkOperationStatus {
    Created,
    Running,
    Completed,
    Canceling,
    Canceled,
    Failed,
    Expired,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BulkOperationType {
    Query,
    Mutation,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BulkOperationErrorCode {
    AccessDenied,
    InternalServerError,
    Timeout,
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BulkOperationUserError {
    pub code: Option<BulkOperationUserErrorCode>,
    pub field: Option<Vec<String>>,
    pub message: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BulkOperationUserErrorCode {
    InvalidQuery,
    OperationInProgress,
    InvalidFileContentType,
    InvalidStagedUploadFile,
    NoSuchFile,
    InternalServerError,
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BulkMutationUserError {
    pub code: Option<BulkMutationErrorCode>,
    pub field: Option<Vec<String>>,
    pub message: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BulkMutationErrorCode {
    InternalServerError,
    InvalidMutation,
    InvalidStagedUploadFile,
    NoSuchFile,
    OperationInProgress,
}

// endregion

// region: Response Types

#[derive(serde::Deserialize, Debug)]
pub struct RunQueryResp {
    #[serde(rename = "bulkOperationRunQuery")]
    pub bulk_operation_run_query: RunQueryPayload,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RunQueryPayload {
    pub bulk_operation: Option<BulkOperation>,
    pub user_errors: Vec<BulkOperationUserError>,
}

#[derive(serde::Deserialize, Debug)]
pub struct RunMutationResp {
    #[serde(rename = "bulkOperationRunMutation")]
    pub bulk_operation_run_mutation: RunMutationPayload,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RunMutationPayload {
    pub bulk_operation: Option<BulkOperation>,
    pub user_errors: Vec<BulkMutationUserError>,
}

#[derive(serde::Deserialize, Debug)]
pub struct CancelResp {
    #[serde(rename = "bulkOperationCancel")]
    pub bulk_operation_cancel: CancelPayload,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CancelPayload {
    pub bulk_operation: Option<BulkOperation>,
    pub user_errors: Vec<UserError>,
}

#[derive(serde::Deserialize, Debug)]
pub struct GetBulkOperationResp {
    #[serde(rename = "bulkOperation")]
    pub bulk_operation: Option<BulkOperation>,
}

#[derive(serde::Deserialize, Debug)]
pub struct ListBulkOperationsResp {
    #[serde(rename = "bulkOperations")]
    pub bulk_operations: BulkOperationConnection,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BulkOperationConnection {
    pub nodes: Vec<BulkOperation>,
    pub page_info: crate::common::types::PageInfo,
}

// endregion

// region: Staged Upload Types

#[derive(serde::Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StagedUploadInput {
    pub resource: StagedUploadResource,
    pub filename: String,
    pub mime_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_method: Option<StagedUploadHttpMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StagedUploadResource {
    BulkMutationVariables,
    Image,
    Video,
    Model3d,
    File,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StagedUploadHttpMethod {
    Post,
    Put,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct StagedUploadsCreateResp {
    #[serde(rename = "stagedUploadsCreate")]
    pub staged_uploads_create: StagedUploadsCreatePayload,
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StagedUploadsCreatePayload {
    pub staged_targets: Option<Vec<StagedMediaUploadTarget>>,
    pub user_errors: Vec<UserError>,
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StagedMediaUploadTarget {
    pub url: String,
    pub resource_url: Option<String>,
    pub parameters: Vec<StagedUploadParameter>,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct StagedUploadParameter {
    pub name: String,
    pub value: String,
}

// endregion

// region: List Query Types

#[derive(serde::Serialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BulkOperationsSortKeys {
    CreatedAt,
    Id,
    Status,
}

#[derive(Debug, Clone, Default)]
pub struct ListBulkOperationsParams {
    pub first: Option<i32>,
    pub after: Option<String>,
    pub last: Option<i32>,
    pub before: Option<String>,
    pub reverse: Option<bool>,
    pub sort_key: Option<BulkOperationsSortKeys>,
    pub status: Option<BulkOperationStatus>,
    pub operation_type: Option<BulkOperationType>,
}

// endregion

// region: Shared Export Types

#[derive(serde::Deserialize, Debug, Clone)]
pub struct BulkExportMoney {
    pub amount: String,
    #[serde(rename = "currencyCode")]
    pub currency_code: String,
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BulkExportMoneyBag {
    pub shop_money: BulkExportMoney,
    pub presentment_money: BulkExportMoney,
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BulkExportMailingAddress {
    pub address1: Option<String>,
    pub address2: Option<String>,
    pub city: Option<String>,
    pub province: Option<String>,
    pub province_code: Option<String>,
    pub country: Option<String>,
    pub country_code_v2: Option<String>,
    pub zip: Option<String>,
    pub phone: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub company: Option<String>,
    pub name: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct BulkExportImage {
    pub url: Option<String>,
    #[serde(rename = "altText")]
    pub alt_text: Option<String>,
    pub width: Option<i32>,
    pub height: Option<i32>,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct BulkExportSeo {
    pub title: Option<String>,
    pub description: Option<String>,
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BulkExportWeight {
    pub value: f64,
    pub unit: String,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct BulkExportSelectedOption {
    pub name: String,
    pub value: String,
}

// endregion

// region: Product Export Types

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BulkExportProduct {
    pub id: String,
    pub title: String,
    pub handle: String,
    pub description_html: Option<String>,
    pub status: Option<String>,
    pub vendor: Option<String>,
    pub product_type: Option<String>,
    pub tags: Option<Vec<String>>,
    pub is_gift_card: Option<bool>,
    pub has_only_default_variant: Option<bool>,
    pub template_suffix: Option<String>,
    pub online_store_url: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub published_at: Option<String>,
    pub seo: Option<BulkExportSeo>,
    pub price_range_v2: Option<BulkExportPriceRange>,
    pub options: Option<Vec<BulkExportProductOption>>,
    pub featured_media: Option<BulkExportMediaPreview>,
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BulkExportPriceRange {
    pub min_variant_price: BulkExportMoney,
    pub max_variant_price: BulkExportMoney,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct BulkExportProductOption {
    pub id: String,
    pub name: String,
    pub values: Vec<String>,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct BulkExportMediaPreview {
    pub preview: Option<BulkExportMediaPreviewImage>,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct BulkExportMediaPreviewImage {
    pub image: Option<BulkExportImage>,
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BulkExportProductVariant {
    pub id: String,
    #[serde(rename = "__parentId")]
    pub parent_id: String,
    pub title: Option<String>,
    pub sku: Option<String>,
    pub price: Option<String>,
    pub compare_at_price: Option<String>,
    pub barcode: Option<String>,
    pub position: Option<i32>,
    pub inventory_quantity: Option<i32>,
    pub taxable: Option<bool>,
    pub tax_code: Option<String>,
    pub available_for_sale: Option<bool>,
    pub weight: Option<f64>,
    pub weight_unit: Option<String>,
    pub selected_options: Option<Vec<BulkExportSelectedOption>>,
    pub inventory_item: Option<BulkExportInventoryItemRef>,
    pub image: Option<BulkExportImage>,
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BulkExportInventoryItemRef {
    pub id: String,
    pub tracked: Option<bool>,
    pub requires_shipping: Option<bool>,
    pub unit_cost: Option<BulkExportMoney>,
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BulkExportProductMedia {
    pub id: String,
    #[serde(rename = "__parentId")]
    pub parent_id: String,
    pub media_content_type: Option<String>,
    pub status: Option<String>,
    pub preview: Option<BulkExportMediaPreviewImage>,
    pub alt: Option<String>,
    pub mime_type: Option<String>,
    pub image: Option<BulkExportImage>,
    pub sources: Option<Vec<BulkExportVideoSource>>,
    pub embed_url: Option<String>,
    pub host: Option<String>,
    pub filename: Option<String>,
    pub original_source: Option<BulkExportModel3dSource>,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct BulkExportVideoSource {
    pub url: String,
    pub format: Option<String>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    #[serde(rename = "mimeType")]
    pub mime_type: Option<String>,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct BulkExportModel3dSource {
    pub url: String,
    pub format: Option<String>,
    #[serde(rename = "filesize")]
    pub file_size: Option<i64>,
}

#[derive(Debug, Clone)]
pub enum ProductExportLine {
    Product(BulkExportProduct),
    Variant(BulkExportProductVariant),
    Media(BulkExportProductMedia),
}

impl ProductExportLine {
    pub fn parse_line(line: &str) -> Result<Self, serde_json::Error> {
        let value: serde_json::Value = serde_json::from_str(line)?;
        if let Some(id) = value.get("id").and_then(|v| v.as_str()) {
            if value.get("__parentId").is_some() {
                if value.get("mediaContentType").is_some() {
                    let media: BulkExportProductMedia = serde_json::from_value(value)?;
                    return Ok(ProductExportLine::Media(media));
                }
                let variant: BulkExportProductVariant = serde_json::from_value(value)?;
                return Ok(ProductExportLine::Variant(variant));
            }
            if id.contains("Product") {
                let product: BulkExportProduct = serde_json::from_value(value)?;
                return Ok(ProductExportLine::Product(product));
            }
        }
        let product: BulkExportProduct = serde_json::from_value(value)?;
        Ok(ProductExportLine::Product(product))
    }
}

// endregion

// region: Order Export Types

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BulkExportOrder {
    pub id: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub note: Option<String>,
    pub tags: Option<Vec<String>>,
    pub display_financial_status: Option<String>,
    pub display_fulfillment_status: Option<String>,
    pub cancelled_at: Option<String>,
    pub cancel_reason: Option<String>,
    pub closed_at: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub processed_at: Option<String>,
    pub test: Option<bool>,
    pub confirmed: Option<bool>,
    pub taxes_included: Option<bool>,
    pub currency_code: Option<String>,
    pub presentment_currency_code: Option<String>,
    pub subtotal_price_set: Option<BulkExportMoneyBag>,
    pub total_price_set: Option<BulkExportMoneyBag>,
    pub total_discount_set: Option<BulkExportMoneyBag>,
    pub total_tax_set: Option<BulkExportMoneyBag>,
    pub total_shipping_price_set: Option<BulkExportMoneyBag>,
    pub total_refunded_set: Option<BulkExportMoneyBag>,
    pub total_weight: Option<String>,
    pub customer: Option<BulkExportOrderCustomer>,
    pub shipping_address: Option<BulkExportMailingAddress>,
    pub billing_address: Option<BulkExportMailingAddress>,
    pub source_name: Option<String>,
    pub fulfillable: Option<bool>,
    pub requires_shipping: Option<bool>,
    pub risk_level: Option<String>,
    pub discount_codes: Option<Vec<String>>,
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BulkExportOrderCustomer {
    pub id: String,
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BulkExportLineItem {
    pub id: String,
    #[serde(rename = "__parentId")]
    pub parent_id: String,
    pub title: Option<String>,
    pub name: Option<String>,
    pub sku: Option<String>,
    pub quantity: Option<i32>,
    pub variant_title: Option<String>,
    pub vendor: Option<String>,
    pub product: Option<BulkExportLineItemProduct>,
    pub variant: Option<BulkExportLineItemVariant>,
    pub original_unit_price_set: Option<BulkExportMoneyBag>,
    pub discounted_unit_price_set: Option<BulkExportMoneyBag>,
    pub discounted_total_set: Option<BulkExportMoneyBag>,
    pub total_discount_set: Option<BulkExportMoneyBag>,
    pub tax_lines: Option<Vec<BulkExportTaxLine>>,
    pub requires_shipping: Option<bool>,
    pub taxable: Option<bool>,
    pub fulfillable_quantity: Option<i32>,
    pub fulfillment_status: Option<String>,
    pub custom_attributes: Option<Vec<BulkExportAttribute>>,
    pub duties: Option<Vec<BulkExportDuty>>,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct BulkExportLineItemProduct {
    pub id: String,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct BulkExportLineItemVariant {
    pub id: String,
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BulkExportTaxLine {
    pub title: Option<String>,
    pub rate: Option<f64>,
    pub rate_percentage: Option<f64>,
    pub price_set: Option<BulkExportMoneyBag>,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct BulkExportAttribute {
    pub key: String,
    pub value: Option<String>,
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BulkExportDuty {
    pub id: String,
    pub harmonized_system_code: Option<String>,
    pub price: Option<BulkExportMoneyBag>,
    pub tax_lines: Option<Vec<BulkExportTaxLine>>,
}

#[derive(Debug, Clone)]
pub enum OrderExportLine {
    Order(Box<BulkExportOrder>),
    LineItem(Box<BulkExportLineItem>),
}

impl OrderExportLine {
    pub fn parse_line(line: &str) -> Result<Self, serde_json::Error> {
        let value: serde_json::Value = serde_json::from_str(line)?;
        if value.get("__parentId").is_some() {
            let item: BulkExportLineItem = serde_json::from_value(value)?;
            return Ok(OrderExportLine::LineItem(Box::new(item)));
        }
        let order: BulkExportOrder = serde_json::from_value(value)?;
        Ok(OrderExportLine::Order(Box::new(order)))
    }
}

// endregion

// region: Collection Export Types

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BulkExportCollection {
    pub id: String,
    pub title: String,
    pub handle: Option<String>,
    pub description_html: Option<String>,
    pub sort_order: Option<String>,
    pub template_suffix: Option<String>,
    pub products_count: Option<i32>,
    pub updated_at: Option<String>,
    pub published_at: Option<String>,
    pub seo: Option<BulkExportSeo>,
    pub image: Option<BulkExportImage>,
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BulkExportCollectionProduct {
    pub id: String,
    #[serde(rename = "__parentId")]
    pub parent_id: String,
    pub title: Option<String>,
    pub handle: Option<String>,
    pub status: Option<String>,
    pub vendor: Option<String>,
    pub product_type: Option<String>,
}

#[derive(Debug, Clone)]
pub enum CollectionExportLine {
    Collection(BulkExportCollection),
    Product(BulkExportCollectionProduct),
}

impl CollectionExportLine {
    pub fn parse_line(line: &str) -> Result<Self, serde_json::Error> {
        let value: serde_json::Value = serde_json::from_str(line)?;
        if value.get("__parentId").is_some() {
            let product: BulkExportCollectionProduct = serde_json::from_value(value)?;
            return Ok(CollectionExportLine::Product(product));
        }
        let collection: BulkExportCollection = serde_json::from_value(value)?;
        Ok(CollectionExportLine::Collection(collection))
    }
}

// endregion

// region: Customer Export Types

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BulkExportCustomer {
    pub id: String,
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub display_name: Option<String>,
    pub phone: Option<String>,
    pub note: Option<String>,
    pub tags: Option<Vec<String>>,
    pub state: Option<String>,
    pub tax_exempt: Option<bool>,
    pub verified_email: Option<bool>,
    pub accepts_marketing: Option<bool>,
    pub locale: Option<String>,
    pub orders_count: Option<String>,
    pub total_spent: Option<String>,
    pub total_spent_v2: Option<BulkExportMoney>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub default_address: Option<BulkExportMailingAddress>,
    pub image: Option<BulkExportImage>,
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BulkExportCustomerAddress {
    pub id: String,
    #[serde(rename = "__parentId")]
    pub parent_id: String,
    pub address1: Option<String>,
    pub address2: Option<String>,
    pub city: Option<String>,
    pub province: Option<String>,
    pub province_code: Option<String>,
    pub country: Option<String>,
    pub country_code_v2: Option<String>,
    pub zip: Option<String>,
    pub phone: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub company: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Clone)]
pub enum CustomerExportLine {
    Customer(Box<BulkExportCustomer>),
    Address(Box<BulkExportCustomerAddress>),
}

impl CustomerExportLine {
    pub fn parse_line(line: &str) -> Result<Self, serde_json::Error> {
        let value: serde_json::Value = serde_json::from_str(line)?;
        if value.get("__parentId").is_some() {
            let address: BulkExportCustomerAddress = serde_json::from_value(value)?;
            return Ok(CustomerExportLine::Address(Box::new(address)));
        }
        let customer: BulkExportCustomer = serde_json::from_value(value)?;
        Ok(CustomerExportLine::Customer(Box::new(customer)))
    }
}

// endregion

// region: Inventory Item Export Types

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BulkExportInventoryItem {
    pub id: String,
    pub sku: Option<String>,
    pub tracked: Option<bool>,
    pub requires_shipping: Option<bool>,
    pub country_code_of_origin: Option<String>,
    pub province_code_of_origin: Option<String>,
    pub harmonized_system_code: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub unit_cost: Option<BulkExportMoney>,
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BulkExportInventoryLevel {
    pub id: String,
    #[serde(rename = "__parentId")]
    pub parent_id: String,
    pub available: Option<i32>,
    pub location: Option<BulkExportInventoryLocation>,
    pub updated_at: Option<String>,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct BulkExportInventoryLocation {
    pub id: String,
    pub name: Option<String>,
}

#[derive(Debug, Clone)]
pub enum InventoryItemExportLine {
    InventoryItem(BulkExportInventoryItem),
    InventoryLevel(BulkExportInventoryLevel),
}

impl InventoryItemExportLine {
    pub fn parse_line(line: &str) -> Result<Self, serde_json::Error> {
        let value: serde_json::Value = serde_json::from_str(line)?;
        if value.get("__parentId").is_some() {
            let level: BulkExportInventoryLevel = serde_json::from_value(value)?;
            return Ok(InventoryItemExportLine::InventoryLevel(level));
        }
        let item: BulkExportInventoryItem = serde_json::from_value(value)?;
        Ok(InventoryItemExportLine::InventoryItem(item))
    }
}

// endregion

// region: Draft Order Export Types

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BulkExportDraftOrder {
    pub id: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub note2: Option<String>,
    pub tags: Option<Vec<String>>,
    pub status: Option<String>,
    pub currency_code: Option<String>,
    pub tax_exempt: Option<bool>,
    pub taxes_included: Option<bool>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub completed_at: Option<String>,
    pub invoice_sent_at: Option<String>,
    pub subtotal_price_set: Option<BulkExportMoneyBag>,
    pub total_price_set: Option<BulkExportMoneyBag>,
    pub total_tax_set: Option<BulkExportMoneyBag>,
    pub total_discount_set: Option<BulkExportMoneyBag>,
    pub total_shipping_price_set: Option<BulkExportMoneyBag>,
    pub customer: Option<BulkExportOrderCustomer>,
    pub shipping_address: Option<BulkExportMailingAddress>,
    pub billing_address: Option<BulkExportMailingAddress>,
    pub order: Option<BulkExportDraftOrderRef>,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct BulkExportDraftOrderRef {
    pub id: String,
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BulkExportDraftOrderLineItem {
    pub id: String,
    #[serde(rename = "__parentId")]
    pub parent_id: String,
    pub title: Option<String>,
    pub name: Option<String>,
    pub sku: Option<String>,
    pub quantity: Option<i32>,
    pub variant_title: Option<String>,
    pub vendor: Option<String>,
    pub product: Option<BulkExportLineItemProduct>,
    pub variant: Option<BulkExportLineItemVariant>,
    pub original_unit_price_set: Option<BulkExportMoneyBag>,
    pub discounted_unit_price_set: Option<BulkExportMoneyBag>,
    pub discounted_total_set: Option<BulkExportMoneyBag>,
    pub total_discount_set: Option<BulkExportMoneyBag>,
    pub requires_shipping: Option<bool>,
    pub taxable: Option<bool>,
    pub custom_attributes: Option<Vec<BulkExportAttribute>>,
}

#[derive(Debug, Clone)]
pub enum DraftOrderExportLine {
    DraftOrder(Box<BulkExportDraftOrder>),
    LineItem(Box<BulkExportDraftOrderLineItem>),
}

impl DraftOrderExportLine {
    pub fn parse_line(line: &str) -> Result<Self, serde_json::Error> {
        let value: serde_json::Value = serde_json::from_str(line)?;
        if value.get("__parentId").is_some() {
            let item: BulkExportDraftOrderLineItem = serde_json::from_value(value)?;
            return Ok(DraftOrderExportLine::LineItem(Box::new(item)));
        }
        let draft: BulkExportDraftOrder = serde_json::from_value(value)?;
        Ok(DraftOrderExportLine::DraftOrder(Box::new(draft)))
    }
}

// endregion
