use super::cart::CartDiscountAllocation;
use super::common::Attribute;
use super::common::Money;
use super::common::PageInfo;
use super::metafields::Metafield;
use super::products::ProductVariant;
use serde::{Deserialize, Serialize};

/// Reason for order cancellation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderCancelReason {
    #[serde(rename = "CUSTOMER")]
    CUSTOMER,
    #[serde(rename = "FRAUD")]
    FRAUD,
    #[serde(rename = "INVENTORY")]
    INVENTORY,
    #[serde(rename = "DECLINED")]
    DECLINED,
    #[serde(rename = "OTHER")]
    OTHER,
}

/// Fulfillment status of an order
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderFulfillmentStatus {
    #[serde(rename = "UNFULFILLED")]
    UNFULFILLED,
    #[serde(rename = "PARTIALLY_FULFILLED")]
    PARTIALLYFULFILLED,
    #[serde(rename = "FULFILLED")]
    FULFILLED,
    #[serde(rename = "RESTOCKED")]
    RESTOCKED,
    #[serde(rename = "PENDING_FULFILLMENT")]
    PENDINGFULFILLMENT,
    #[serde(rename = "OPEN")]
    OPEN,
    #[serde(rename = "IN_PROGRESS")]
    INPROGRESS,
    #[serde(rename = "ON_HOLD")]
    ONHOLD,
    #[serde(rename = "SCHEDULED")]
    SCHEDULED,
}

/// Financial status of an order
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderFinancialStatus {
    #[serde(rename = "PENDING")]
    PENDING,
    #[serde(rename = "AUTHORIZED")]
    AUTHORIZED,
    #[serde(rename = "PARTIALLY_PAID")]
    PARTIALLYPAID,
    #[serde(rename = "PARTIALLY_REFUNDED")]
    PARTIALLYREFUNDED,
    #[serde(rename = "VOIDED")]
    VOIDED,
    #[serde(rename = "PAID")]
    PAID,
    #[serde(rename = "REFUNDED")]
    REFUNDED,
}

/// How a discount is allocated
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiscountAllocationMethod {
    #[serde(rename = "ACROSS")]
    ACROSS,
    #[serde(rename = "EACH")]
    EACH,
    #[serde(rename = "ONE")]
    ONE,
}

/// What a discount targets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiscountTargetSelection {
    #[serde(rename = "ALL")]
    ALL,
    #[serde(rename = "ENTITLED")]
    ENTITLED,
    #[serde(rename = "EXPLICIT")]
    EXPLICIT,
}

/// Type of discount target
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiscountTargetType {
    #[serde(rename = "LINE_ITEM")]
    LINEITEM,
    #[serde(rename = "SHIPPING_LINE")]
    SHIPPINGLINE,
}

/// Error codes for customer operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CustomerUserErrorCode {
    #[serde(rename = "BLANK")]
    BLANK,
    #[serde(rename = "INVALID")]
    INVALID,
    #[serde(rename = "TAKEN")]
    TAKEN,
    #[serde(rename = "TOO_LONG")]
    TOOLONG,
    #[serde(rename = "TOO_SHORT")]
    TOOSHORT,
    #[serde(rename = "UNIDENTIFIED_CUSTOMER")]
    UNIDENTIFIEDCUSTOMER,
    #[serde(rename = "CUSTOMER_DISABLED")]
    CUSTOMERDISABLED,
    #[serde(rename = "PASSWORD_STARTS_OR_ENDS_WITH_WHITESPACE")]
    PASSWORDSTARTSORENDSWITHWHITESPACE,
    #[serde(rename = "CONTAINS_HTML_TAGS")]
    CONTAINSHTMLTAGS,
    #[serde(rename = "CONTAINS_URL")]
    CONTAINSURL,
    #[serde(rename = "TOKEN_INVALID")]
    TOKENINVALID,
    #[serde(rename = "ALREADY_ENABLED")]
    ALREADYENABLED,
    #[serde(rename = "NOT_FOUND")]
    NOTFOUND,
    #[serde(rename = "BAD_DOMAIN")]
    BADDOMAIN,
    #[serde(rename = "INVALID_MULTIPASS_REQUEST")]
    INVALIDMULTIPASSREQUEST,
}

/// A customer account with contact information and order history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Customer {
    /// Globally unique identifier
    pub id: String,
    /// Customer email address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Customer first name
    #[serde(rename = "firstName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Customer last name
    #[serde(rename = "lastName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Customer name, email or phone number
    #[serde(rename = "displayName")]
    pub display_name: String,
    /// Customer phone number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// Whether customer consents to marketing emails
    #[serde(rename = "acceptsMarketing")]
    pub accepts_marketing: bool,
    /// When the customer was created
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<time::OffsetDateTime>,
    /// When the customer was last updated
    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<time::OffsetDateTime>,
    /// Total number of orders in lifetime
    #[serde(rename = "numberOfOrders")]
    pub number_of_orders: i32,
    /// Default address
    #[serde(rename = "defaultAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_address: Option<MailingAddress>,
    /// Customer addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addresses: Option<MailingAddressConnection>,
    /// Customer orders
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orders: Option<OrderConnection>,
    /// Tags added to the customer
    pub tags: Vec<String>,
    /// URL of the customer avatar image
    #[serde(rename = "avatarUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    /// A custom field associated with the customer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metafield: Option<Metafield>,
    /// List of custom fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metafields: Option<Vec<Metafield>>,
}

/// Minimal customer info for nested references
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerSummary {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "firstName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(rename = "lastName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
}

/// Access token for customer authentication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerAccessToken {
    #[serde(rename = "accessToken")]
    pub access_token: String,
    #[serde(rename = "expiresAt")]
    pub expires_at: time::Date,
}

/// A mailing address
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailingAddress {
    pub id: String,
    #[serde(rename = "firstName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(rename = "lastName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Full name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub province: Option<String>,
    #[serde(rename = "provinceCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub province_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(rename = "countryCodeV2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code_v2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// Formatted address lines
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatted: Option<Vec<String>>,
    /// City, province, country formatted
    #[serde(rename = "formattedArea")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatted_area: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,
}

/// Paginated list of addresses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailingAddressConnection {
    pub edges: Vec<MailingAddressEdge>,
    #[serde(rename = "pageInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_info: Option<PageInfo>,
}

/// An edge in an address connection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailingAddressEdge {
    pub node: MailingAddress,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

/// A customer order completed through checkout
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    /// Globally unique identifier
    pub id: String,
    /// Order name (e.g., #1001)
    pub name: String,
    /// Unique numeric identifier
    #[serde(rename = "orderNumber")]
    pub order_number: i32,
    /// When the order was processed
    #[serde(rename = "processedAt")]
    pub processed_at: time::OffsetDateTime,
    /// When the order was canceled
    #[serde(rename = "canceledAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<time::OffsetDateTime>,
    #[serde(rename = "cancelReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_reason: Option<OrderCancelReason>,
    #[serde(rename = "fulfillmentStatus")]
    pub fulfillment_status: OrderFulfillmentStatus,
    #[serde(rename = "financialStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_status: Option<OrderFinancialStatus>,
    /// Unique URL for the order status page
    #[serde(rename = "statusUrl")]
    pub status_url: String,
    /// URL the customer can use to access the order
    #[serde(rename = "customerUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_url: Option<String>,
    /// Customer email address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Customer phone for SMS notifications
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// Locale code of the order
    #[serde(rename = "customerLocale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_locale: Option<String>,
    /// Currency code for payment
    #[serde(rename = "currencyCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    /// Whether the order has had edits applied
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edited: Option<bool>,
    /// Current total minus removed items
    #[serde(rename = "currentTotalPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_total_price: Option<Money>,
    /// Current subtotal minus removed items
    #[serde(rename = "currentSubtotalPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_subtotal_price: Option<Money>,
    /// Current total tax minus returned items
    #[serde(rename = "currentTotalTax")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_total_tax: Option<Money>,
    /// Current shipping total minus refunds
    #[serde(rename = "currentTotalShippingPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_total_shipping_price: Option<Money>,
    /// Current duties total including refunds
    #[serde(rename = "currentTotalDuties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_total_duties: Option<Money>,
    /// Total price before any edits
    #[serde(rename = "originalTotalPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_total_price: Option<Money>,
    /// Duties charged at checkout
    #[serde(rename = "originalTotalDuties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_total_duties: Option<Money>,
    /// Sum of all prices including discounts and taxes
    #[serde(rename = "totalPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_price: Option<Money>,
    /// Price before shipping and taxes
    #[serde(rename = "subtotalPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtotal_price: Option<Money>,
    /// Total cost of shipping
    #[serde(rename = "totalShippingPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_shipping_price: Option<Money>,
    /// Total cost of taxes
    #[serde(rename = "totalTax")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_tax: Option<Money>,
    /// Total amount refunded
    #[serde(rename = "totalRefunded")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_refunded: Option<Money>,
    /// Custom attributes on the order
    #[serde(rename = "customAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_attributes: Option<Vec<Attribute>>,
    #[serde(rename = "shippingAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<MailingAddress>,
    #[serde(rename = "billingAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<MailingAddress>,
    #[serde(rename = "lineItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<OrderLineItemConnection>,
    /// Discounts applied to shipping
    #[serde(rename = "shippingDiscountAllocations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_discount_allocations: Option<Vec<DiscountAllocation>>,
    #[serde(rename = "discountApplications")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_applications: Option<DiscountApplicationConnection>,
    #[serde(rename = "successfulFulfillments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful_fulfillments: Option<Vec<Fulfillment>>,
    /// A custom field associated with the order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metafield: Option<Metafield>,
    /// List of custom fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metafields: Option<Vec<Metafield>>,
}

/// A discount allocation on a line item or shipping
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscountAllocation {
    /// Amount of the discount
    #[serde(rename = "allocatedAmount")]
    pub allocated_amount: Money,
    #[serde(rename = "discountApplication")]
    pub discount_application: DiscountApplication,
}

/// A line item in an order
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderLineItem {
    pub title: String,
    pub quantity: f64,
    #[serde(rename = "originalTotalPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_total_price: Option<Money>,
    #[serde(rename = "discountedTotalPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounted_total_price: Option<Money>,
    #[serde(rename = "currentQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_quantity: Option<f64>,
    #[serde(rename = "customAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_attributes: Option<Vec<Attribute>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant: Option<ProductVariant>,
    #[serde(rename = "discountAllocations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_allocations: Option<Vec<CartDiscountAllocation>>,
}

/// An edge in an order line item connection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderLineItemEdge {
    pub node: OrderLineItem,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

/// Paginated list of order line items
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderLineItemConnection {
    pub edges: Vec<OrderLineItemEdge>,
    #[serde(rename = "pageInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_info: Option<PageInfo>,
}

/// Paginated list of orders
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderConnection {
    pub edges: Vec<OrderEdge>,
    #[serde(rename = "pageInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_info: Option<PageInfo>,
}

/// An edge in an order connection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderEdge {
    pub node: Order,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

/// Order fulfillment details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fulfillment {
    #[serde(rename = "trackingCompany")]
    pub tracking_company: String,
    #[serde(rename = "trackingInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_info: Option<Vec<FulfillmentTrackingInfo>>,
}

/// Tracking information for a fulfillment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FulfillmentTrackingInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// A discount application
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscountApplication {
    #[serde(rename = "allocationMethod")]
    pub allocation_method: DiscountAllocationMethod,
    #[serde(rename = "targetSelection")]
    pub target_selection: DiscountTargetSelection,
    #[serde(rename = "targetType")]
    pub target_type: DiscountTargetType,
    pub value: DiscountValue,
}

/// An edge in a discount application connection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscountApplicationEdge {
    pub node: DiscountApplication,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

/// The value of a discount
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscountValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<Money>,
}

/// Paginated list of discount applications
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscountApplicationConnection {
    pub edges: Vec<DiscountApplicationEdge>,
    #[serde(rename = "pageInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_info: Option<PageInfo>,
}

/// An error that occurred during a customer operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerUserError {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<Vec<String>>,
    pub message: String,
    pub code: CustomerUserErrorCode,
}

/// Input for creating a customer access token (login)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerAccessTokenCreateInput {
    pub email: String,
    pub password: String,
}

/// Input for creating a new customer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerCreateInput {
    pub email: String,
    pub password: String,
    #[serde(rename = "firstName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(rename = "lastName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(rename = "acceptsMarketing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepts_marketing: Option<bool>,
}

/// Input for updating a customer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "firstName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(rename = "lastName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(rename = "acceptsMarketing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepts_marketing: Option<bool>,
}

/// Input for resetting a customer password
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerResetInput {
    #[serde(rename = "resetToken")]
    pub reset_token: String,
    pub password: String,
}

/// Input for activating a customer account
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerActivateInput {
    #[serde(rename = "activationToken")]
    pub activation_token: String,
    pub password: String,
}

/// Input for creating or updating a mailing address
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailingAddressInput {
    #[serde(rename = "firstName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(rename = "lastName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub province: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

/// Result of a customer mutation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerMutationResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Customer>,
    #[serde(rename = "customerUserErrors")]
    pub customer_user_errors: Vec<CustomerUserError>,
}

/// Result of an access token mutation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessTokenResult {
    #[serde(rename = "customerAccessToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_access_token: Option<CustomerAccessToken>,
    #[serde(rename = "customerUserErrors")]
    pub customer_user_errors: Vec<CustomerUserError>,
}

/// Result of an address mutation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressMutationResult {
    #[serde(rename = "customerAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_address: Option<MailingAddress>,
    #[serde(rename = "customerUserErrors")]
    pub customer_user_errors: Vec<CustomerUserError>,
}
