use crate::common::types::PageInfo;

// region: Response Types

#[derive(serde::Deserialize, Debug)]
pub struct DiscountAutomaticAppCreateResp {
    #[serde(rename = "discountAutomaticAppCreate")]
    pub discount_automatic_app_create: DiscountAutomaticAppCreatePayload,
}

#[derive(serde::Deserialize, Debug)]
pub struct DiscountAutomaticAppUpdateResp {
    #[serde(rename = "discountAutomaticAppUpdate")]
    pub discount_automatic_app_update: DiscountAutomaticAppUpdatePayload,
}

#[derive(serde::Deserialize, Debug)]
pub struct DiscountNodesResp {
    #[serde(rename = "discountNodes")]
    pub discount_nodes: DiscountNodesConnection,
}

#[derive(serde::Deserialize, Debug)]
pub struct DiscountNodesConnection {
    pub nodes: Vec<DiscountNode>,
    #[serde(rename = "pageInfo")]
    pub page_info: PageInfo,
}

#[derive(serde::Deserialize, Debug)]
pub struct DiscountNode {
    pub id: String,
    pub metafields: Option<MetafieldConnection>,
    pub discount: DiscountType,
}

#[derive(serde::Deserialize, Debug)]
#[serde(tag = "__typename")]
pub enum DiscountType {
    DiscountAutomaticApp(DiscountAutomaticAppDetails),
    DiscountCodeApp(DiscountCodeAppDetails),
    DiscountAutomaticBasic(DiscountBasicDetails),
    DiscountCodeBasic(DiscountBasicDetails),
    DiscountAutomaticBxgy(DiscountBxgyDetails),
    DiscountCodeBxgy(DiscountBxgyDetails),
    DiscountCodeFreeShipping(DiscountFreeShippingDetails),
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DiscountAutomaticAppDetails {
    pub title: String,
    pub status: String,
    pub discount_id: Option<String>,
    pub starts_at: Option<String>,
    pub ends_at: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub combines_with: Option<DiscountCombinesWith>,
    pub discount_classes: Option<Vec<DiscountClass>>,
    pub async_usage_count: Option<i32>,
    pub applies_on_one_time_purchase: Option<bool>,
    pub applies_on_subscription: Option<bool>,
    pub recurring_cycle_limit: Option<i32>,
    pub error_history: Option<FunctionsErrorHistory>,
    pub app_discount_type: AppDiscountType,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DiscountCodeAppDetails {
    pub title: String,
    pub status: String,
    pub combines_with: Option<DiscountCombinesWith>,
    pub applies_once_per_customer: Option<bool>,
    pub async_usage_count: Option<i32>,
    pub codes: Option<DiscountCodesConnection>,
    pub app_discount_type: AppDiscountType,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DiscountBasicDetails {
    pub title: String,
    pub status: String,
    pub summary: Option<String>,
    pub combines_with: Option<DiscountCombinesWith>,
    pub async_usage_count: Option<i32>,
    pub recurring_cycle_limit: Option<i32>,
    pub applies_once_per_customer: Option<bool>,
    pub usage_limit: Option<i32>,
    pub codes: Option<DiscountCodesConnection>,
    pub context: Option<DiscountContext>,
    pub minimum_requirement: Option<DiscountMinimumRequirement>,
    pub customer_gets: Option<DiscountCustomerGets>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DiscountBxgyDetails {
    pub title: String,
    pub status: String,
    pub summary: Option<String>,
    pub combines_with: Option<DiscountCombinesWith>,
    pub async_usage_count: Option<i32>,
    pub uses_per_order_limit: Option<i32>,
    pub applies_once_per_customer: Option<bool>,
    pub usage_limit: Option<i32>,
    pub codes: Option<DiscountCodesConnection>,
    pub context: Option<DiscountContext>,
    pub customer_buys: Option<DiscountCustomerBuys>,
    pub customer_gets: Option<DiscountCustomerGets>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DiscountFreeShippingDetails {
    pub title: String,
    pub status: String,
    pub summary: Option<String>,
    pub combines_with: Option<DiscountCombinesWith>,
    pub async_usage_count: Option<i32>,
    pub recurring_cycle_limit: Option<i32>,
    pub applies_once_per_customer: Option<bool>,
    pub usage_limit: Option<i32>,
    pub codes: Option<DiscountCodesConnection>,
    pub maximum_shipping_price: Option<Money>,
    pub context: Option<DiscountContext>,
    pub minimum_requirement: Option<DiscountMinimumRequirement>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DiscountAutomaticAppCreatePayload {
    pub automatic_app_discount: Option<DiscountAutomaticApp>,
    pub user_errors: Vec<DiscountUserError>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DiscountAutomaticAppUpdatePayload {
    pub automatic_app_discount: Option<DiscountAutomaticApp>,
    pub user_errors: Vec<DiscountUserError>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DiscountAutomaticApp {
    pub discount_id: String,
    pub title: String,
    pub starts_at: String,
    pub ends_at: Option<String>,
    pub status: String,
    pub app_discount_type: AppDiscountType,
    pub combines_with: DiscountCombinesWith,
    pub applies_on_one_time_purchase: bool,
    pub applies_on_subscription: bool,
    pub recurring_cycle_limit: i32,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AppDiscountType {
    pub app_key: String,
    pub function_id: String,
    pub title: Option<String>,
    pub description: Option<String>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DiscountCombinesWith {
    pub order_discounts: bool,
    pub product_discounts: bool,
    pub shipping_discounts: bool,
}

#[derive(serde::Deserialize, Debug)]
pub struct DiscountCodesConnection {
    pub nodes: Vec<DiscountCode>,
}

#[derive(serde::Deserialize, Debug)]
pub struct DiscountCode {
    pub code: String,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Money {
    pub amount: String,
    pub currency_code: String,
}

#[derive(serde::Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct DiscountContext {
    #[serde(rename = "__typename", default)]
    pub buyer_selection_typename: Option<String>,
    #[serde(default)]
    pub customers: Option<Vec<DiscountCustomer>>,
    #[serde(default)]
    pub segments: Option<Vec<DiscountCustomerSegment>>,
}

#[derive(serde::Deserialize, Debug)]
pub struct DiscountCustomer {
    pub id: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct DiscountCustomerSegment {
    pub id: String,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DiscountMinimumRequirement {
    #[serde(default)]
    pub greater_than_or_equal_to_quantity: Option<String>,
    #[serde(default)]
    pub greater_than_or_equal_to_subtotal: Option<Money>,
}

#[derive(serde::Deserialize, Debug)]
pub struct DiscountCustomerGets {
    pub value: DiscountValue,
    pub items: Option<DiscountItems>,
}

#[derive(serde::Deserialize, Debug)]
pub struct DiscountCustomerBuys {
    pub value: DiscountCustomerBuysValue,
    pub items: Option<DiscountItems>,
}

#[derive(Debug)]
pub enum DiscountValue {
    DiscountAmount(DiscountAmountValue),
    DiscountPercentage(DiscountPercentageValue),
    DiscountOnQuantity(DiscountOnQuantityValue),
}

impl<'de> serde::Deserialize<'de> for DiscountValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        if value.get("percentage").is_some() {
            serde_json::from_value(value)
                .map(DiscountValue::DiscountPercentage)
                .map_err(serde::de::Error::custom)
        } else if value.get("amount").is_some() {
            serde_json::from_value(value)
                .map(DiscountValue::DiscountAmount)
                .map_err(serde::de::Error::custom)
        } else if value.get("quantity").is_some() {
            serde_json::from_value(value)
                .map(DiscountValue::DiscountOnQuantity)
                .map_err(serde::de::Error::custom)
        } else {
            Err(serde::de::Error::custom(
                "DiscountValue: expected one of `amount`, `percentage`, or `quantity`",
            ))
        }
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct DiscountAmountValue {
    pub amount: Money,
}

#[derive(serde::Deserialize, Debug)]
pub struct DiscountPercentageValue {
    pub percentage: f64,
}

#[derive(serde::Deserialize, Debug)]
pub struct DiscountOnQuantityValue {
    pub quantity: DiscountOnQuantityQuantity,
    pub effect: Option<DiscountEffect>,
}

#[derive(Debug)]
pub enum DiscountEffect {
    DiscountPercentage(DiscountPercentageValue),
    DiscountAmount(DiscountAmountValue),
}

impl<'de> serde::Deserialize<'de> for DiscountEffect {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        if value.get("percentage").is_some() {
            serde_json::from_value(value)
                .map(DiscountEffect::DiscountPercentage)
                .map_err(serde::de::Error::custom)
        } else if value.get("amount").is_some() {
            serde_json::from_value(value)
                .map(DiscountEffect::DiscountAmount)
                .map_err(serde::de::Error::custom)
        } else {
            Err(serde::de::Error::custom(
                "DiscountEffect: expected `percentage` or `amount`",
            ))
        }
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct DiscountOnQuantityQuantity {
    pub quantity: String,
}

#[derive(serde::Deserialize, Debug)]
#[serde(untagged)]
pub enum DiscountCustomerBuysValue {
    DiscountQuantity(DiscountCustomerBuysQuantityValue),
    DiscountPurchaseAmount(DiscountPurchaseAmountValue),
}

#[derive(serde::Deserialize, Debug)]
pub struct DiscountCustomerBuysQuantityValue {
    pub quantity: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct DiscountPurchaseAmountValue {
    pub amount: String,
}

#[derive(serde::Deserialize, Debug, Default)]
pub struct DiscountItems {
    #[serde(default)]
    pub products: Option<DiscountItemsConnection>,
    #[serde(default)]
    pub collections: Option<DiscountItemsConnection>,
}

#[derive(serde::Deserialize, Debug)]
pub struct DiscountItemsConnection {
    pub nodes: Vec<DiscountItem>,
}

#[derive(serde::Deserialize, Debug)]
pub struct DiscountItem {
    pub id: String,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FunctionsErrorHistory {
    pub errors_first_occurred_at: String,
    pub first_occurred_at: String,
    pub has_been_shared_since_last_error: bool,
    pub has_shared_recent_errors: bool,
}

#[derive(serde::Deserialize, Debug)]
pub struct DiscountUserError {
    pub field: Option<Vec<String>>,
    pub message: String,
    pub code: Option<String>,
}

#[derive(serde::Deserialize, Debug)]
pub struct MetafieldConnection {
    pub edges: Vec<MetafieldEdge>,
    #[serde(rename = "pageInfo")]
    pub page_info: PageInfo,
}

#[derive(serde::Deserialize, Debug)]
pub struct MetafieldEdge {
    pub node: Metafield,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Metafield {
    pub id: String,
    pub namespace: String,
    pub key: String,
    pub value: String,
    #[serde(rename = "type")]
    pub metafield_type: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct GetDiscountNodeResp {
    #[serde(rename = "discountNode")]
    pub discount_node: Option<DiscountNodeWithMetafields>,
}

#[derive(serde::Deserialize, Debug)]
pub struct DiscountNodeWithMetafields {
    pub id: String,
    pub metafields: MetafieldConnection,
    pub discount: DiscountType,
}

#[derive(serde::Deserialize, Debug)]
pub struct GetDiscountMetafieldResp {
    #[serde(rename = "discountNode")]
    pub discount_node: Option<DiscountNodeWithSingleMetafield>,
}

#[derive(serde::Deserialize, Debug)]
pub struct DiscountNodeWithSingleMetafield {
    pub id: String,
    pub metafield: Option<Metafield>,
}

// endregion

// region: Request Types

#[derive(serde::Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DiscountAutomaticAppInput {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_handle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starts_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ends_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub combines_with: Option<DiscountCombinesWithInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_classes: Option<Vec<DiscountClass>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<DiscountContextInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metafields: Option<Vec<MetafieldInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applies_on_subscription: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applies_on_one_time_purchase: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring_cycle_limit: Option<i32>,
}

#[derive(serde::Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DiscountAutomaticAppUpdateInput {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_handle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starts_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ends_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub combines_with: Option<DiscountCombinesWithInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_classes: Option<Vec<DiscountClass>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<DiscountContextInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metafields: Option<Vec<MetafieldInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applies_on_subscription: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applies_on_one_time_purchase: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring_cycle_limit: Option<i32>,
}

#[derive(serde::Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DiscountCombinesWithInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_discounts: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_discounts: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_discounts: Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DiscountClass {
    Product,
    Order,
    Shipping,
}

#[derive(serde::Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DiscountContextInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all: Option<DiscountBuyerSelection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customers: Option<DiscountCustomersInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_segments: Option<DiscountCustomerSegmentsInput>,
}

#[derive(serde::Serialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DiscountBuyerSelection {
    All,
}

#[derive(serde::Serialize, Debug, Clone)]
pub struct DiscountCustomersInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove: Option<Vec<String>>,
}

#[derive(serde::Serialize, Debug, Clone)]
pub struct DiscountCustomerSegmentsInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove: Option<Vec<String>>,
}

#[derive(serde::Serialize, Debug, Clone)]
pub struct MetafieldInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub metafield_type: Option<String>,
}

// endregion

// region: Request Builders

impl DiscountAutomaticAppInput {
    pub fn new(title: String) -> Self {
        DiscountAutomaticAppInput {
            title,
            function_handle: None,
            starts_at: None,
            ends_at: None,
            combines_with: None,
            discount_classes: None,
            context: None,
            metafields: None,
            applies_on_subscription: None,
            applies_on_one_time_purchase: None,
            recurring_cycle_limit: None,
        }
    }

    pub fn with_function_handle(mut self, function_handle: String) -> Self {
        self.function_handle = Some(function_handle);
        self
    }

    pub fn with_starts_at(mut self, starts_at: String) -> Self {
        self.starts_at = Some(starts_at);
        self
    }

    pub fn with_ends_at(mut self, ends_at: String) -> Self {
        self.ends_at = Some(ends_at);
        self
    }

    pub fn with_combines_with(mut self, combines_with: DiscountCombinesWithInput) -> Self {
        self.combines_with = Some(combines_with);
        self
    }

    pub fn with_discount_classes(mut self, discount_classes: Vec<DiscountClass>) -> Self {
        self.discount_classes = Some(discount_classes);
        self
    }

    pub fn with_context(mut self, context: DiscountContextInput) -> Self {
        self.context = Some(context);
        self
    }

    pub fn with_metafields(mut self, metafields: Vec<MetafieldInput>) -> Self {
        self.metafields = Some(metafields);
        self
    }

    pub fn with_applies_on_subscription(mut self, applies: bool) -> Self {
        self.applies_on_subscription = Some(applies);
        self
    }

    pub fn with_applies_on_one_time_purchase(mut self, applies: bool) -> Self {
        self.applies_on_one_time_purchase = Some(applies);
        self
    }

    pub fn with_recurring_cycle_limit(mut self, limit: i32) -> Self {
        self.recurring_cycle_limit = Some(limit);
        self
    }
}

impl DiscountCombinesWithInput {
    pub fn new() -> Self {
        DiscountCombinesWithInput {
            product_discounts: None,
            order_discounts: None,
            shipping_discounts: None,
        }
    }

    pub fn with_product_discounts(mut self, value: bool) -> Self {
        self.product_discounts = Some(value);
        self
    }

    pub fn with_order_discounts(mut self, value: bool) -> Self {
        self.order_discounts = Some(value);
        self
    }

    pub fn with_shipping_discounts(mut self, value: bool) -> Self {
        self.shipping_discounts = Some(value);
        self
    }
}

impl Default for DiscountCombinesWithInput {
    fn default() -> Self {
        Self::new()
    }
}

impl DiscountContextInput {
    pub fn all() -> Self {
        DiscountContextInput {
            all: Some(DiscountBuyerSelection::All),
            customers: None,
            customer_segments: None,
        }
    }

    pub fn customers(add: Vec<String>) -> Self {
        DiscountContextInput {
            all: None,
            customers: Some(DiscountCustomersInput {
                add: Some(add),
                remove: None,
            }),
            customer_segments: None,
        }
    }

    pub fn customer_segments(add: Vec<String>) -> Self {
        DiscountContextInput {
            all: None,
            customers: None,
            customer_segments: Some(DiscountCustomerSegmentsInput {
                add: Some(add),
                remove: None,
            }),
        }
    }
}

impl MetafieldInput {
    pub fn new(namespace: String, key: String, value: String, metafield_type: String) -> Self {
        MetafieldInput {
            id: None,
            namespace: Some(namespace),
            key: Some(key),
            value: Some(value),
            metafield_type: Some(metafield_type),
        }
    }

    pub fn update(id: String, value: String) -> Self {
        MetafieldInput {
            id: Some(id),
            namespace: None,
            key: None,
            value: Some(value),
            metafield_type: None,
        }
    }
}

impl DiscountAutomaticAppUpdateInput {
    pub fn new(id: String) -> Self {
        DiscountAutomaticAppUpdateInput {
            id,
            title: None,
            function_handle: None,
            starts_at: None,
            ends_at: None,
            combines_with: None,
            discount_classes: None,
            context: None,
            metafields: None,
            applies_on_subscription: None,
            applies_on_one_time_purchase: None,
            recurring_cycle_limit: None,
        }
    }

    pub fn with_title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    pub fn with_function_handle(mut self, function_handle: String) -> Self {
        self.function_handle = Some(function_handle);
        self
    }

    pub fn with_starts_at(mut self, starts_at: String) -> Self {
        self.starts_at = Some(starts_at);
        self
    }

    pub fn with_ends_at(mut self, ends_at: String) -> Self {
        self.ends_at = Some(ends_at);
        self
    }

    pub fn with_combines_with(mut self, combines_with: DiscountCombinesWithInput) -> Self {
        self.combines_with = Some(combines_with);
        self
    }

    pub fn with_discount_classes(mut self, discount_classes: Vec<DiscountClass>) -> Self {
        self.discount_classes = Some(discount_classes);
        self
    }

    pub fn with_context(mut self, context: DiscountContextInput) -> Self {
        self.context = Some(context);
        self
    }

    pub fn with_metafields(mut self, metafields: Vec<MetafieldInput>) -> Self {
        self.metafields = Some(metafields);
        self
    }

    pub fn with_applies_on_subscription(mut self, applies: bool) -> Self {
        self.applies_on_subscription = Some(applies);
        self
    }

    pub fn with_applies_on_one_time_purchase(mut self, applies: bool) -> Self {
        self.applies_on_one_time_purchase = Some(applies);
        self
    }

    pub fn with_recurring_cycle_limit(mut self, limit: i32) -> Self {
        self.recurring_cycle_limit = Some(limit);
        self
    }
}

// endregion
