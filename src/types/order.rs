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

#[derive(serde::Deserialize, Debug, Clone)]
pub struct OrderDiscountsAndTransactionsResp {
    pub order: Option<OrderDiscountsAndTransactions>,
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderDiscountsAndTransactions {
    #[serde(default)]
    pub discount_codes: Vec<String>,
    #[serde(default)]
    pub transactions: Vec<OrderTransaction>,
    pub line_items: DiscountedLineItems,
    pub cart_discount_amount_set: Option<MoneyBag>,
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderTransaction {
    #[serde(default)]
    pub gateway: Option<String>,
    #[serde(default)]
    pub kind: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    pub amount_set: MoneyBag,
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DiscountedLineItems {
    pub nodes: Vec<DiscountedLineItem>,
    pub page_info: PageInfo,
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PageInfo {
    pub has_next_page: bool,
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DiscountedLineItem {
    pub id: String,
    #[serde(default)]
    pub discount_allocations: Vec<DiscountAllocation>,
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DiscountAllocation {
    pub allocated_amount_set: MoneyBag,
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MoneyBag {
    pub shop_money: MoneyV2,
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MoneyV2 {
    pub amount: String,
    pub currency_code: String,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct OrderDetailResp {
    pub order: Option<OrderDetail>,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct OrderDetailByNameResp {
    pub orders: OrderDetailNodes,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct OrderDetailNodes {
    pub nodes: Vec<OrderDetail>,
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderDetail {
    pub id: String,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub created_at: String,
    pub currency_code: String,
    pub display_financial_status: Option<String>,
    pub taxes_included: bool,
    #[serde(default)]
    pub discount_codes: Vec<String>,
    pub cart_discount_amount_set: Option<MoneyBag>,
    pub customer: Option<OrderDetailCustomer>,
    pub shipping_address: Option<OrderDetailAddress>,
    #[serde(default)]
    pub transactions: Vec<OrderTransaction>,
    #[serde(default)]
    pub fulfillments: Vec<OrderDetailFulfillment>,
    pub line_items: OrderDetailLineItems,
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderDetailCustomer {
    pub id: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub default_email_address: Option<CustomerEmailAddress>,
    pub default_phone_number: Option<CustomerPhoneNumber>,
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CustomerEmailAddress {
    pub email_address: Option<String>,
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CustomerPhoneNumber {
    pub phone_number: Option<String>,
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderDetailAddress {
    pub name: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
    pub address1: Option<String>,
    pub address2: Option<String>,
    pub city: Option<String>,
    pub province: Option<String>,
    pub country: Option<String>,
    pub country_code_v2: Option<String>,
    pub zip: Option<String>,
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderDetailFulfillment {
    pub status: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    #[serde(default)]
    pub tracking_info: Vec<FulfillmentTrackingInfo>,
    pub fulfillment_line_items: FulfillmentLineItemNodes,
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FulfillmentTrackingInfo {
    pub company: Option<String>,
    pub number: Option<String>,
    pub url: Option<String>,
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FulfillmentLineItemNodes {
    pub nodes: Vec<FulfillmentLineItemNode>,
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FulfillmentLineItemNode {
    pub quantity: Option<i64>,
    pub line_item: Option<LineItemRef>,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct LineItemRef {
    pub id: String,
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderDetailLineItems {
    pub nodes: Vec<OrderDetailLineItem>,
    pub page_info: PageInfo,
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderDetailLineItem {
    pub id: String,
    pub title: String,
    pub variant_title: Option<String>,
    pub quantity: i64,
    pub unfulfilled_quantity: Option<i64>,
    pub sku: Option<String>,
    pub product: Option<LineItemRef>,
    pub variant: Option<LineItemRef>,
    pub original_unit_price_set: Option<MoneyBag>,
    pub total_discount_set: Option<MoneyBag>,
    #[serde(default)]
    pub discount_allocations: Vec<DiscountAllocation>,
}

#[cfg(test)]
mod graphql_shape_tests {
    use super::*;

    #[test]
    fn money_bag_parses_the_graphql_camel_case_shape() {
        let bag: MoneyBag =
            serde_json::from_str(r#"{"shopMoney":{"amount":"19.99","currencyCode":"INR"}}"#)
                .expect("MoneyBag must parse the shape Shopify actually sends");
        assert_eq!(bag.shop_money.amount, "19.99");
        assert_eq!(bag.shop_money.currency_code, "INR");
    }

    #[test]
    fn order_detail_parses_a_full_graphql_payload() {
        let resp: OrderDetailResp = serde_json::from_str(
            r##"{"order":{
                "id":"gid://shopify/Order/1001",
                "name":"#1001",
                "email":"a@b.com",
                "phone":null,
                "createdAt":"2026-01-01T00:00:00Z",
                "currencyCode":"INR",
                "displayFinancialStatus":"PAID",
                "taxesIncluded":true,
                "discountCodes":["VIP20"],
                "cartDiscountAmountSet":{"shopMoney":{"amount":"5.00","currencyCode":"INR"}},
                "customer":{
                    "id":"gid://shopify/Customer/7",
                    "firstName":"Ada","lastName":"L",
                    "defaultEmailAddress":{"emailAddress":"a@b.com"},
                    "defaultPhoneNumber":{"phoneNumber":"+911234567890"}
                },
                "shippingAddress":{
                    "name":"Ada L","firstName":"Ada","lastName":"L","phone":"+91",
                    "address1":"1 St","address2":null,"city":"BLR","province":"KA",
                    "country":"India","countryCodeV2":"IN","zip":"560001"
                },
                "transactions":[{"gateway":"razorpay","kind":"SALE","status":"SUCCESS",
                    "amountSet":{"shopMoney":{"amount":"100.00","currencyCode":"INR"}}}],
                "fulfillments":[{
                    "status":"SUCCESS","createdAt":"2026-01-02T00:00:00Z","updatedAt":"2026-01-03T00:00:00Z",
                    "trackingInfo":[{"company":"BD","number":"XYZ","url":"https://t/XYZ"}],
                    "fulfillmentLineItems":{"nodes":[{"quantity":1,"lineItem":{"id":"gid://shopify/LineItem/5"}}]}
                }],
                "lineItems":{
                    "nodes":[{
                        "id":"gid://shopify/LineItem/5","title":"Tee","variantTitle":"M",
                        "quantity":2,"unfulfilledQuantity":0,"sku":"TEE-M",
                        "product":{"id":"gid://shopify/Product/9"},
                        "variant":{"id":"gid://shopify/ProductVariant/11"},
                        "originalUnitPriceSet":{"shopMoney":{"amount":"50.00","currencyCode":"INR"}},
                        "totalDiscountSet":{"shopMoney":{"amount":"2.00","currencyCode":"INR"}},
                        "discountAllocations":[{"allocatedAmountSet":{"shopMoney":{"amount":"2.00","currencyCode":"INR"}}}]
                    }],
                    "pageInfo":{"hasNextPage":false}
                }
            }}"##,
        )
        .expect("OrderDetail must parse a real GraphQL payload");

        let order = resp.order.expect("order present");
        assert_eq!(order.id, "gid://shopify/Order/1001");
        assert_eq!(order.discount_codes, vec!["VIP20".to_string()]);
        assert!(order.taxes_included);
        assert_eq!(
            order
                .customer
                .and_then(|c| c.default_email_address)
                .and_then(|e| e.email_address)
                .as_deref(),
            Some("a@b.com")
        );
        assert_eq!(order.transactions[0].amount_set.shop_money.amount, "100.00");
        assert_eq!(
            order.fulfillments[0].tracking_info[0].number.as_deref(),
            Some("XYZ")
        );
        assert_eq!(
            order.fulfillments[0].fulfillment_line_items.nodes[0]
                .line_item
                .as_ref()
                .map(|l| l.id.as_str()),
            Some("gid://shopify/LineItem/5")
        );
        assert_eq!(order.line_items.nodes[0].sku.as_deref(), Some("TEE-M"));
        assert!(!order.line_items.page_info.has_next_page);
    }

    #[test]
    fn order_discounts_parses_the_graphql_shape() {
        let resp: OrderDiscountsAndTransactionsResp = serde_json::from_str(
            r#"{"order":{"discountCodes":[],"transactions":[],
                "cartDiscountAmountSet":null,
                "lineItems":{"nodes":[{"id":"gid://shopify/LineItem/5",
                    "discountAllocations":[{"allocatedAmountSet":{"shopMoney":{"amount":"1.00","currencyCode":"INR"}}}]}],
                    "pageInfo":{"hasNextPage":false}}}}"#,
        )
        .expect("discounts response must parse the shape Shopify sends");
        let order = resp.order.expect("order present");
        assert_eq!(
            order.line_items.nodes[0].discount_allocations[0]
                .allocated_amount_set
                .shop_money
                .amount,
            "1.00"
        );
    }
}
