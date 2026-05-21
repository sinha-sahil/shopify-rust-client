use super::common::Attribute;
use super::common::AttributeInput;
use super::common::Image;
use super::common::Money;
use super::common::PageInfo;
use super::common::SelectedOption;
use super::customer::CustomerSummary;
use super::customer::MailingAddress;
use super::metafields::Metafield;
use super::products::ProductSummary;
use serde::{Deserialize, Serialize};

/// A shopping cart representing merchandise a buyer intends to purchase
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cart {
    /// Globally unique identifier
    pub id: String,
    /// URL to the checkout page
    #[serde(rename = "checkoutUrl")]
    pub checkout_url: String,
    /// The date and time when the cart was created
    #[serde(rename = "createdAt")]
    pub created_at: time::OffsetDateTime,
    /// The date and time when the cart was updated
    #[serde(rename = "updatedAt")]
    pub updated_at: time::OffsetDateTime,
    /// Note attached to the cart
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    /// Total quantity of items in the cart
    #[serde(rename = "totalQuantity")]
    pub total_quantity: i32,
    #[serde(rename = "buyerIdentity")]
    pub buyer_identity: CartBuyerIdentity,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<Attribute>>,
    #[serde(rename = "discountCodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_codes: Option<Vec<CartDiscountCode>>,
    #[serde(rename = "discountAllocations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_allocations: Option<Vec<CartDiscountAllocation>>,
    /// Gift cards that have been applied to the cart
    #[serde(rename = "appliedGiftCards")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applied_gift_cards: Option<Vec<AppliedGiftCard>>,
    pub cost: CartCost,
    pub lines: BaseCartLineConnection,
    /// The delivery properties of the cart
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery: Option<CartDelivery>,
    /// Delivery groups available for the cart
    #[serde(rename = "deliveryGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_groups: Option<CartDeliveryGroupConnection>,
    /// A custom field associated with the cart
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metafield: Option<Metafield>,
    /// List of custom fields associated with the cart
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metafields: Option<Vec<Metafield>>,
    /// Warnings that occurred during cart operations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<CartWarning>>,
}

/// Buyer identity information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartBuyerIdentity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// Two-letter country code (ISO 3166-1 alpha-2)
    #[serde(rename = "countryCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<CustomerSummary>,
    /// Delivery address preferences
    #[serde(rename = "deliveryAddressPreferences")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_address_preferences: Option<Vec<DeliveryAddress>>,
    /// Wallet preferences for the buyer
    #[serde(rename = "walletPreferences")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet_preferences: Option<Vec<String>>,
}

/// A gift card that has been applied to the cart
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppliedGiftCard {
    /// Globally unique identifier
    pub id: String,
    /// Last four characters of the gift card code
    #[serde(rename = "lastCharacters")]
    pub last_characters: String,
    /// Amount used from this gift card
    #[serde(rename = "amountUsed")]
    pub amount_used: Money,
    /// Remaining balance on the gift card
    pub balance: Money,
    /// Amount used in presentment currency
    #[serde(rename = "presentmentAmountUsed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presentment_amount_used: Option<Money>,
}

/// The delivery properties of the cart
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartDelivery {
    /// Selectable addresses for the buyer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<CartSelectableAddress>>,
}

/// A selectable delivery address for the cart
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartSelectableAddress {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<MailingAddress>,
    /// Whether this address is selected
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected: Option<bool>,
}

/// Information about the options available for one or more line items to be delivered to a specific address
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartDeliveryGroup {
    /// Globally unique identifier
    pub id: String,
    #[serde(rename = "deliveryAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_address: Option<MailingAddress>,
    #[serde(rename = "deliveryOptions")]
    pub delivery_options: Vec<CartDeliveryOption>,
    #[serde(rename = "selectedDeliveryOption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_delivery_option: Option<CartDeliveryOption>,
    #[serde(rename = "cartLines")]
    pub cart_lines: BaseCartLineConnection,
    #[serde(rename = "groupType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_type: Option<CartDeliveryGroupType>,
}

/// Paginated list of cart delivery groups
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartDeliveryGroupConnection {
    pub edges: Vec<CartDeliveryGroupEdge>,
    #[serde(rename = "pageInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_info: Option<PageInfo>,
}

/// An edge in a cart delivery group connection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartDeliveryGroupEdge {
    pub node: CartDeliveryGroup,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

/// The type of cart delivery group
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CartDeliveryGroupType {
    #[serde(rename = "ONE_TIME_PURCHASE")]
    ONETIMEPURCHASE,
    #[serde(rename = "SUBSCRIPTION")]
    SUBSCRIPTION,
}

/// A delivery option for a cart delivery group
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartDeliveryOption {
    /// Unique identifier for the delivery option
    pub handle: String,
    /// Title of the delivery option
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "estimatedCost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_cost: Option<Money>,
    /// The code of the delivery option
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "deliveryMethodType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_method_type: Option<DeliveryMethodType>,
}

/// The method of delivery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeliveryMethodType {
    #[serde(rename = "SHIPPING")]
    SHIPPING,
    #[serde(rename = "PICK_UP")]
    PICKUP,
    #[serde(rename = "RETAIL")]
    RETAIL,
    #[serde(rename = "LOCAL")]
    LOCAL,
    #[serde(rename = "PICKUP_POINT")]
    PICKUPPOINT,
    #[serde(rename = "NONE")]
    NONE,
}

/// A delivery address preference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryAddress {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<MailingAddress>,
    /// Whether this is a one-time use address
    #[serde(rename = "oneTimeUse")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_time_use: Option<bool>,
}

/// A warning that occurred during a cart mutation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartWarning {
    pub code: CartWarningCode,
    /// The message text of the warning
    pub message: String,
    /// The target of the warning
    pub target: String,
}

/// Warning codes for cart operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CartWarningCode {
    #[serde(rename = "MERCHANDISE_NOT_ENOUGH_STOCK")]
    MERCHANDISENOTENOUGHSTOCK,
    #[serde(rename = "MERCHANDISE_OUT_OF_STOCK")]
    MERCHANDISEOUTOFSTOCK,
    #[serde(rename = "PAYMENTS_GIFT_CARD_UNUSABLE")]
    PAYMENTSGIFTCARDUNUSABLE,
    #[serde(rename = "CHECKOUT_THROTTLED")]
    CHECKOUTTHROTTLED,
    #[serde(rename = "DELIVERY_GROUP_PARTIALLY_AVAILABLE")]
    DELIVERYGROUPPARTIALLYAVAILABLE,
    #[serde(rename = "DELIVERY_GROUP_UNAVAILABLE")]
    DELIVERYGROUPUNAVAILABLE,
}

/// A discount code applied to the cart
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartDiscountCode {
    pub code: String,
    /// Whether the discount code is applicable
    pub applicable: bool,
}

/// A discount allocation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartDiscountAllocation {
    #[serde(rename = "discountedAmount")]
    pub discounted_amount: Money,
}

/// Cost breakdown for the cart
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartCost {
    /// Total amount after discounts and taxes
    #[serde(rename = "totalAmount")]
    pub total_amount: Money,
    /// Subtotal before taxes and cart-level discounts
    #[serde(rename = "subtotalAmount")]
    pub subtotal_amount: Money,
    /// Whether the subtotal amount is estimated
    #[serde(rename = "subtotalAmountEstimated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtotal_amount_estimated: Option<bool>,
    /// Whether the total amount is estimated
    #[serde(rename = "totalAmountEstimated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_amount_estimated: Option<bool>,
    #[serde(rename = "totalTaxAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_tax_amount: Option<Money>,
    /// Whether the total tax amount is estimated
    #[serde(rename = "totalTaxAmountEstimated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_tax_amount_estimated: Option<bool>,
    #[serde(rename = "totalDutyAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_duty_amount: Option<Money>,
    /// Whether the total duty amount is estimated
    #[serde(rename = "totalDutyAmountEstimated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_duty_amount_estimated: Option<bool>,
    /// Amount customer pays at checkout (excludes deferred payments)
    #[serde(rename = "checkoutChargeAmount")]
    pub checkout_charge_amount: Money,
}

/// Base cart line interface with common fields
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseCartLine {
    /// Globally unique identifier
    pub id: String,
    /// The quantity of the merchandise
    pub quantity: i32,
    pub merchandise: Merchandise,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<Attribute>>,
    pub cost: CartLineCost,
    #[serde(rename = "discountAllocations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_allocations: Option<Vec<CartDiscountAllocation>>,
    #[serde(rename = "sellingPlanAllocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selling_plan_allocation: Option<SellingPlanAllocation>,
}

/// A line item in the cart (implements BaseCartLine)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartLine {
    /// Globally unique identifier
    pub id: String,
    /// The quantity of the merchandise
    pub quantity: i32,
    pub merchandise: Merchandise,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<Attribute>>,
    pub cost: CartLineCost,
    #[serde(rename = "discountAllocations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_allocations: Option<Vec<CartDiscountAllocation>>,
    #[serde(rename = "sellingPlanAllocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selling_plan_allocation: Option<SellingPlanAllocation>,
    /// Instructions for the line item
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<CartLineInstructions>,
    /// The parent of the line item
    #[serde(rename = "parentRelationship")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_relationship: Option<CartLineParentRelationship>,
}

/// Instructions for a cart line
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartLineInstructions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<CartLineInstructionDiscounts>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// Discount instruction for cart line
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CartLineInstructionDiscounts {
    #[serde(rename = "APPLY")]
    APPLY,
    #[serde(rename = "IGNORE")]
    IGNORE,
}

/// Parent relationship for a cart line
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartLineParentRelationship {
    /// ID of the parent line
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<CartLineParentType>,
}

/// Type of parent line relationship
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CartLineParentType {
    #[serde(rename = "BUNDLE")]
    BUNDLE,
}

/// The merchandise in a cart line (ProductVariant)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Merchandise {
    pub id: String,
    pub title: String,
    pub price: Money,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<ProductSummary>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<Image>,
    #[serde(rename = "selectedOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_options: Option<Vec<SelectedOption>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    #[serde(rename = "availableForSale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_for_sale: Option<bool>,
    #[serde(rename = "requiresShipping")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requires_shipping: Option<bool>,
    #[serde(rename = "quantityAvailable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity_available: Option<i32>,
}

/// Cost breakdown for a cart line
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartLineCost {
    #[serde(rename = "totalAmount")]
    pub total_amount: Money,
    #[serde(rename = "amountPerQuantity")]
    pub amount_per_quantity: Money,
    #[serde(rename = "compareAtAmountPerQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compare_at_amount_per_quantity: Option<Money>,
    #[serde(rename = "subtotalAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtotal_amount: Option<Money>,
}

/// Paginated list of cart lines (BaseCartLine)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseCartLineConnection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edges: Option<Vec<BaseCartLineEdge>>,
    pub nodes: Vec<CartLine>,
    #[serde(rename = "pageInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_info: Option<PageInfo>,
}

/// An edge in a base cart line connection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseCartLineEdge {
    pub node: CartLine,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

/// Paginated list of cart lines
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartLineConnection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edges: Option<Vec<CartLineEdge>>,
    pub nodes: Vec<CartLine>,
    #[serde(rename = "pageInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_info: Option<PageInfo>,
}

/// An edge in a cart line connection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartLineEdge {
    pub node: CartLine,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

/// Association between a variant and a selling plan with pricing details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SellingPlanAllocation {
    #[serde(rename = "sellingPlan")]
    pub selling_plan: SellingPlan,
    /// Price adjustments for this allocation
    #[serde(rename = "priceAdjustments")]
    pub price_adjustments: Vec<SellingPlanAllocationPriceAdjustment>,
    /// The checkout charge amount due for the purchase
    #[serde(rename = "checkoutChargeAmount")]
    pub checkout_charge_amount: Money,
    /// The remaining balance charge amount
    #[serde(rename = "remainingBalanceChargeAmount")]
    pub remaining_balance_charge_amount: Money,
}

/// Price adjustment for a selling plan allocation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SellingPlanAllocationPriceAdjustment {
    /// The effective price
    pub price: Money,
    /// The compare at price
    #[serde(rename = "compareAtPrice")]
    pub compare_at_price: Money,
    /// The price per delivery
    #[serde(rename = "perDeliveryPrice")]
    pub per_delivery_price: Money,
    /// The unit price
    #[serde(rename = "unitPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<Money>,
}

/// A selling plan representing how products can be sold and purchased
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SellingPlan {
    /// Globally unique identifier
    pub id: String,
    /// Name of the selling plan
    pub name: String,
    /// Description of the selling plan
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Whether purchasing this plan results in multiple deliveries
    #[serde(rename = "recurringDeliveries")]
    pub recurring_deliveries: bool,
    /// The initial payment due for the purchase
    #[serde(rename = "checkoutCharge")]
    pub checkout_charge: SellingPlanCheckoutCharge,
    /// Price adjustments when a variant is purchased with this plan
    #[serde(rename = "priceAdjustments")]
    pub price_adjustments: Vec<SellingPlanPriceAdjustment>,
    /// Options available for this selling plan
    pub options: Vec<SellingPlanOption>,
    #[serde(rename = "billingPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_policy: Option<SellingPlanBillingPolicy>,
    #[serde(rename = "deliveryPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_policy: Option<SellingPlanDeliveryPolicy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metafield: Option<Metafield>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metafields: Option<Vec<Metafield>>,
}

/// An option provided by a selling plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SellingPlanOption {
    /// The name of the option (e.g., "Delivery every")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The value of the option (e.g., "Month")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// The initial payment due for a selling plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SellingPlanCheckoutCharge {
    pub r#type: SellingPlanCheckoutChargeType,
    pub value: SellingPlanCheckoutChargeValue,
}

/// The type of checkout charge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SellingPlanCheckoutChargeType {
    #[serde(rename = "PERCENTAGE")]
    PERCENTAGE,
    #[serde(rename = "PRICE")]
    PRICE,
}

/// The value of the checkout charge (percentage or money)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SellingPlanCheckoutChargeValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<Money>,
}

/// The billing policy for a selling plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SellingPlanBillingPolicy {
    #[serde(rename = "recurringPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring_policy: Option<SellingPlanRecurringBillingPolicy>,
}

/// Recurring billing policy details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SellingPlanRecurringBillingPolicy {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<SellingPlanInterval>,
    #[serde(rename = "intervalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<i32>,
}

/// The delivery policy for a selling plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SellingPlanDeliveryPolicy {
    #[serde(rename = "recurringPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring_policy: Option<SellingPlanRecurringDeliveryPolicy>,
}

/// Recurring delivery policy details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SellingPlanRecurringDeliveryPolicy {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<SellingPlanInterval>,
    #[serde(rename = "intervalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<i32>,
}

/// Time interval for selling plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SellingPlanInterval {
    #[serde(rename = "DAY")]
    DAY,
    #[serde(rename = "WEEK")]
    WEEK,
    #[serde(rename = "MONTH")]
    MONTH,
    #[serde(rename = "YEAR")]
    YEAR,
}

/// Price adjustment for a selling plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SellingPlanPriceAdjustment {
    pub price: Money,
    #[serde(rename = "compareAtPrice")]
    pub compare_at_price: Money,
    #[serde(rename = "perDeliveryPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_delivery_price: Option<Money>,
}

/// Error codes for cart operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CartUserErrorCode {
    #[serde(rename = "INVALID")]
    INVALID,
    #[serde(rename = "LESS_THAN")]
    LESSTHAN,
    #[serde(rename = "GREATER_THAN")]
    GREATERTHAN,
    #[serde(rename = "INVALID_MERCHANDISE_LINE")]
    INVALIDMERCHANDISELINE,
    #[serde(rename = "MISSING_DISCOUNT_CODE")]
    MISSINGDISCOUNTCODE,
    #[serde(rename = "MISSING_NOTE")]
    MISSINGNOTE,
    #[serde(rename = "INVALID_DELIVERY_GROUP")]
    INVALIDDELIVERYGROUP,
    #[serde(rename = "INVALID_DELIVERY_OPTION")]
    INVALIDDELIVERYOPTION,
    #[serde(rename = "INVALID_METAFIELDS")]
    INVALIDMETAFIELDS,
    #[serde(rename = "GIFT_CARD_UNUSABLE")]
    GIFTCARDUNUSABLE,
    #[serde(rename = "PRODUCT_NOT_AVAILABLE")]
    PRODUCTNOTAVAILABLE,
    #[serde(rename = "INVALID_INPUT")]
    INVALIDINPUT,
    #[serde(rename = "VALIDATION_ERROR")]
    VALIDATIONERROR,
}

/// An error that occurred during a cart operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartUserError {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<Vec<String>>,
    pub message: String,
    pub code: CartUserErrorCode,
}

/// Input for adding a line to the cart
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartLineInput {
    /// The ID of the merchandise (product variant)
    #[serde(rename = "merchandiseId")]
    pub merchandise_id: String,
    /// The quantity to add
    pub quantity: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<Attribute>>,
    /// The ID of the selling plan
    #[serde(rename = "sellingPlanId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selling_plan_id: Option<String>,
}

/// Input for updating a cart line
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartLineUpdateInput {
    /// The ID of the cart line
    pub id: String,
    /// The new quantity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<f64>,
    /// The new merchandise ID
    #[serde(rename = "merchandiseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchandise_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<Attribute>>,
    /// The ID of the selling plan
    #[serde(rename = "sellingPlanId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selling_plan_id: Option<String>,
}

/// Input for updating buyer identity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartBuyerIdentityInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// Two-letter country code
    #[serde(rename = "countryCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    /// Customer access token for authenticated customers
    #[serde(rename = "customerAccessToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_access_token: Option<String>,
}

/// Input for creating a cart
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartInput {
    /// Merchandise lines to add to the cart
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines: Option<Vec<CartLineInput>>,
    /// Additional information about the cart
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<AttributeInput>>,
    /// Note associated with the cart
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    /// Customer associated with the cart
    #[serde(rename = "buyerIdentity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_identity: Option<CartBuyerIdentityInput>,
    /// Discount codes to apply to the cart
    #[serde(rename = "discountCodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_codes: Option<Vec<String>>,
    /// Gift card codes to apply to the cart
    #[serde(rename = "giftCardCodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gift_card_codes: Option<Vec<String>>,
    /// Delivery-related fields for the cart
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery: Option<CartDeliveryInput>,
    /// Metafields to associate with the cart
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metafields: Option<Vec<CartInputMetafieldInput>>,
}

/// Input for cart delivery options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartDeliveryInput {
    #[serde(rename = "deliveryGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_groups: Option<Vec<CartDeliveryGroupInput>>,
}

/// Input for a cart delivery group
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartDeliveryGroupInput {
    /// Delivery group ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "selectedDeliveryOption")]
    pub selected_delivery_option: CartSelectedDeliveryOptionInput,
}

/// Input for selected delivery option
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartSelectedDeliveryOptionInput {
    /// Handle of the delivery option
    pub handle: String,
}

/// Input for a cart metafield
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartInputMetafieldInput {
    /// The key name of the metafield
    pub key: String,
    /// The value of the metafield
    pub value: String,
    /// The type of the metafield value
    pub r#type: String,
}

/// Result of a cart mutation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartMutationResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cart: Option<Cart>,
    #[serde(rename = "userErrors")]
    pub user_errors: Vec<CartUserError>,
}
