#[derive(serde::Deserialize, Debug)]
pub struct OrderQueryResp {
    pub orders: Vec<Order>,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct GetOrderResp {
    pub order: Order,
}

#[derive(serde::Deserialize, Debug, Clone, serde::Serialize)]
pub struct Order {
    pub id: u128,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub line_items: Vec<LineItem>,
    pub fulfillment_status: Option<String>,
    pub fulfillments: Vec<OrderFulfillment>,
    pub payment_gateway_names: Vec<String>,
    pub subtotal_price_set: PriceSet,
    pub total_discounts_set: PriceSet,
    pub total_price_set: PriceSet,
    pub total_shipping_price_set: PriceSet,
    pub total_tax_set: PriceSet,
    pub order_status_url: Option<String>,
    pub financial_status: Option<String>,
    pub name: String,
    pub customer: Option<Customer>,
    pub created_at: String,
    pub updated_at: Option<String>,
    pub closed_at: Option<String>,
    pub cancelled_at: Option<String>,
    pub cancel_reason: Option<String>,
    pub currency: Option<String>,
    pub total_price: Option<String>,
    pub subtotal_price: Option<String>,
    pub total_tax: Option<String>,
    pub total_discounts: Option<String>,
    pub total_line_items_price: Option<String>,
    pub taxes_included: Option<bool>,
    pub total_weight: Option<i64>,
    pub tags: Option<String>,
    pub note: Option<String>,
    pub order_number: Option<i64>,
    pub number: Option<i64>,
    pub processed_at: Option<String>,
    pub source_name: Option<String>,
    pub confirmed: Option<bool>,
    pub billing_address: Option<Address>,
    pub shipping_address: Option<Address>,
}

#[derive(serde::Deserialize, Debug, Clone, serde::Serialize)]
pub struct LineItem {
    pub id: u128,
    pub quantity: i64,
    pub title: String,
    pub product_id: Option<u128>,
    pub variant_title: Option<String>,
    pub variant_id: Option<u128>,
    pub price: Option<String>,
    pub price_set: PriceSet,
    pub requires_shipping: bool,
    pub properties: Vec<Property>,
    pub sku: Option<String>,
    pub grams: Option<i64>,
    pub vendor: Option<String>,
    pub name: Option<String>,
    pub gift_card: Option<bool>,
    pub taxable: Option<bool>,
    pub fulfillment_status: Option<String>,
    pub fulfillable_quantity: Option<i64>,
    pub total_discount: Option<String>,
    pub total_discount_set: Option<PriceSet>,
    pub fulfillment_service: Option<String>,
}

#[derive(serde::Deserialize, Debug, Clone, serde::Serialize)]
pub struct OrderFulfillment {
    pub id: u128,
    pub status: String,
    pub tracking_number: Option<String>,
    pub tracking_numbers: Option<Vec<String>>,
    pub tracking_company: Option<String>,
    pub tracking_url: Option<String>,
    pub tracking_urls: Option<Vec<String>>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub line_items: Option<Vec<LineItem>>,
    pub order_id: Option<u128>,
    pub shipment_status: Option<String>,
    pub location_id: Option<u128>,
    pub name: Option<String>,
}

#[derive(serde::Deserialize, Debug, Clone, serde::Serialize)]
pub struct Address {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub address1: Option<String>,
    pub address2: Option<String>,
    pub city: Option<String>,
    pub province: Option<String>,
    pub province_code: Option<String>,
    pub country: Option<String>,
    pub country_code: Option<String>,
    pub zip: Option<String>,
    pub phone: Option<String>,
    pub company: Option<String>,
    pub name: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

#[derive(serde::Deserialize, Debug, Clone, serde::Serialize)]
pub struct Customer {
    pub id: u128,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub default_address: Option<Address>,
    pub verified_email: Option<bool>,
    pub tax_exempt: Option<bool>,
    pub tags: Option<String>,
    pub currency: Option<String>,
    pub note: Option<String>,
}

#[derive(serde::Deserialize, Debug, Clone, serde::Serialize)]
pub struct PriceSet {
    pub shop_money: Money,
    pub presentment_money: Money,
}

#[derive(serde::Deserialize, Debug, Clone, serde::Serialize)]
pub struct Money {
    pub amount: String,
    pub currency_code: String,
}

#[derive(serde::Deserialize, Debug, Clone, serde::Serialize)]
pub struct Property {
    pub name: String,
    pub value: String,
}

#[derive(serde::Deserialize, Debug, serde::Serialize)]
pub struct PatchOrderRequest {
    pub order: PatchOrder,
}

#[derive(serde::Deserialize, Debug, serde::Serialize)]
pub struct PatchOrder {
    pub tags: Vec<String>,
}

// region: Order Query Enums & Params

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderStatus {
    Open,
    Closed,
    Cancelled,
    Any,
}

impl std::fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderStatus::Open => write!(f, "open"),
            OrderStatus::Closed => write!(f, "closed"),
            OrderStatus::Cancelled => write!(f, "cancelled"),
            OrderStatus::Any => write!(f, "any"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderFinancialStatus {
    Authorized,
    Paid,
    Pending,
    PartiallyPaid,
    PartiallyRefunded,
    Refunded,
    Voided,
    Any,
}

impl std::fmt::Display for OrderFinancialStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderFinancialStatus::Authorized => write!(f, "authorized"),
            OrderFinancialStatus::Paid => write!(f, "paid"),
            OrderFinancialStatus::Pending => write!(f, "pending"),
            OrderFinancialStatus::PartiallyPaid => write!(f, "partially_paid"),
            OrderFinancialStatus::PartiallyRefunded => write!(f, "partially_refunded"),
            OrderFinancialStatus::Refunded => write!(f, "refunded"),
            OrderFinancialStatus::Voided => write!(f, "voided"),
            OrderFinancialStatus::Any => write!(f, "any"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderFulfillmentStatus {
    Shipped,
    Partial,
    Unshipped,
    Any,
    Unfulfilled,
}

impl std::fmt::Display for OrderFulfillmentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderFulfillmentStatus::Shipped => write!(f, "shipped"),
            OrderFulfillmentStatus::Partial => write!(f, "partial"),
            OrderFulfillmentStatus::Unshipped => write!(f, "unshipped"),
            OrderFulfillmentStatus::Any => write!(f, "any"),
            OrderFulfillmentStatus::Unfulfilled => write!(f, "unfulfilled"),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct OrderQueryParams {
    pub status: Option<OrderStatus>,
    pub financial_status: Option<OrderFinancialStatus>,
    pub fulfillment_status: Option<OrderFulfillmentStatus>,
    pub created_at: Option<crate::common::query_filter::DateFilter>,
    pub updated_at: Option<crate::common::query_filter::DateFilter>,
    pub processed_at: Option<crate::common::query_filter::DateFilter>,
    pub tag: Option<String>,
    pub source_name: Option<String>,
    pub risk_level: Option<String>,
    pub chargeback_status: Option<String>,
    pub test: Option<bool>,
}

impl OrderQueryParams {
    pub fn to_query_string(&self) -> Option<String> {
        let mut parts = Vec::new();
        if let Some(v) = &self.status {
            parts.push(format!("status:{}", v));
        }
        if let Some(v) = &self.financial_status {
            parts.push(format!("financial_status:{}", v));
        }
        if let Some(v) = &self.fulfillment_status {
            parts.push(format!("fulfillment_status:{}", v));
        }
        if let Some(v) = &self.created_at {
            parts.push(format!("created_at:{}", v));
        }
        if let Some(v) = &self.updated_at {
            parts.push(format!("updated_at:{}", v));
        }
        if let Some(v) = &self.processed_at {
            parts.push(format!("processed_at:{}", v));
        }
        if let Some(v) = &self.tag {
            parts.push(format!("tag:{}", v));
        }
        if let Some(v) = &self.source_name {
            parts.push(format!("source_name:{}", v));
        }
        if let Some(v) = &self.risk_level {
            parts.push(format!("risk_level:{}", v));
        }
        if let Some(v) = &self.chargeback_status {
            parts.push(format!("chargeback_status:{}", v));
        }
        if let Some(v) = &self.test {
            parts.push(format!("test:{}", v));
        }
        if parts.is_empty() {
            None
        } else {
            Some(parts.join(" "))
        }
    }
}

// endregion

// region: GraphQL Line Item Variant Details

/// Represents a selected option (e.g., Color, Size) on a product variant.
#[derive(serde::Deserialize, Debug, Clone)]
pub struct SelectedOption {
    pub name: String,
    pub value: String,
}

/// Represents media (image) information for a line item's variant.
/// LineItem has no `legacyResourceId` field — the numeric suffix of its GID
/// (`gid://shopify/LineItem/<numeric>`) is the join key that matches the REST
/// order payload's line-item id.
#[derive(serde::Deserialize, Debug, Clone)]
pub struct LineItemVariantDetail {
    pub variant_id: String,
    pub selected_options: Vec<SelectedOption>,
    pub image_url: Option<String>,
}

/// Response wrapper for GraphQL line item variant query (first: 50).
#[derive(serde::Deserialize, Debug)]
pub struct LineItemVariantEdge {
    pub node: LineItemVariantNode,
}

#[derive(serde::Deserialize, Debug)]
pub struct LineItemVariantNode {
    pub id: String,
    pub variant: Option<VariantData>,
}

#[derive(serde::Deserialize, Debug)]
pub struct VariantData {
    pub id: String,
    pub selected_options: Vec<SelectedOption>,
    pub media: Option<MediaConnection>,
    pub product: ProductMediaData,
}

#[derive(serde::Deserialize, Debug)]
pub struct MediaConnection {
    pub edges: Vec<MediaEdge>,
}

#[derive(serde::Deserialize, Debug)]
pub struct MediaEdge {
    pub node: MediaNode,
}

#[derive(serde::Deserialize, Debug)]
#[serde(tag = "__typename")]
pub enum MediaNode {
    MediaImage { image: ImageData },
}

#[derive(serde::Deserialize, Debug)]
pub struct ImageData {
    pub url: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct ProductMediaData {
    pub featured_media: Option<FeaturedMedia>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(tag = "__typename")]
pub enum FeaturedMedia {
    MediaImage { image: ImageData },
}

#[derive(serde::Deserialize, Debug)]
pub struct OrderLineItemsVariantResponse {
    pub order: Option<OrderWithLineItemsVariant>,
}

#[derive(serde::Deserialize, Debug)]
pub struct OrderWithLineItemsVariant {
    pub id: String,
    pub line_items: LineItemVariantConnection,
}

#[derive(serde::Deserialize, Debug)]
pub struct LineItemVariantConnection {
    pub edges: Vec<LineItemVariantEdge>,
}

// endregion
