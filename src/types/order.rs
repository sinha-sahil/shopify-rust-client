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
