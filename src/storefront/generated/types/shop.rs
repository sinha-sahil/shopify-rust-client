use super::common::Image;
use super::common::Money;
use super::metafields::Metafield;
use serde::{Deserialize, Serialize};

/// Shop configuration and information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shop {
    /// Globally unique identifier
    pub id: String,
    /// Shop name
    pub name: String,
    /// Shop description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Primary domain of the Online Store
    #[serde(rename = "primaryDomain")]
    pub primary_domain: Domain,
    /// Shop branding configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand: Option<Brand>,
    /// Payment settings for the shop
    #[serde(rename = "paymentSettings")]
    pub payment_settings: PaymentSettings,
    /// Country codes the shop ships to
    #[serde(rename = "shipsToCountries")]
    pub ships_to_countries: Vec<String>,
    /// Money format string (when currency not specified)
    #[serde(rename = "moneyFormat")]
    pub money_format: String,
    /// Shop refund policy
    #[serde(rename = "refundPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_policy: Option<ShopPolicy>,
    /// Shop privacy policy
    #[serde(rename = "privacyPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_policy: Option<ShopPolicy>,
    /// Shop shipping policy
    #[serde(rename = "shippingPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_policy: Option<ShopPolicy>,
    /// Shop terms of service
    #[serde(rename = "termsOfService")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_of_service: Option<ShopPolicy>,
    /// Shop subscription policy (has default value)
    #[serde(rename = "subscriptionPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_policy: Option<ShopPolicyWithDefault>,
    /// URL for customer account (if vanity domain exists)
    #[serde(rename = "customerAccountUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_account_url: Option<String>,
    /// A custom field associated with the shop
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metafield: Option<Metafield>,
    /// List of custom fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metafields: Option<Vec<Metafield>>,
    /// Shop Pay Installments pricing information
    #[serde(rename = "shopPayInstallmentsPricing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shop_pay_installments_pricing: Option<ShopPayInstallmentsPricing>,
    /// Social login providers for customer accounts
    #[serde(rename = "socialLoginProviders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub social_login_providers: Option<Vec<SocialLoginProvider>>,
}

/// A shop domain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Domain {
    /// Domain host (e.g., example.myshopify.com)
    pub host: String,
    /// Full URL (e.g., <https://example.myshopify.com>)
    pub url: String,
}

/// Shop brand information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Brand {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<BrandMediaImage>,
    #[serde(rename = "coverImage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_image: Option<BrandMediaImage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colors: Option<BrandColors>,
    #[serde(rename = "shortDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slogan: Option<String>,
}

/// A media image for brand assets (logo, cover)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrandMediaImage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<Image>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt: Option<String>,
}

/// Brand color settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrandColors {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary: Option<Vec<BrandColorGroup>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary: Option<Vec<BrandColorGroup>>,
}

/// A group of brand colors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrandColorGroup {
    /// Background color (hex)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<String>,
    /// Foreground color (hex)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foreground: Option<String>,
}

/// A shop policy configured by the merchant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShopPolicy {
    /// Globally unique identifier
    pub id: String,
    /// Policy title
    pub title: String,
    /// Policy handle
    pub handle: String,
    /// Policy text (max 64kb)
    pub body: String,
    /// Public URL to the policy
    pub url: String,
}

/// A shop policy with a default value (e.g., subscription policy)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShopPolicyWithDefault {
    /// Unique ID (null for default policy)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Policy title
    pub title: String,
    /// Policy handle
    pub handle: String,
    /// Policy text (max 64kb)
    pub body: String,
    /// Public URL to the policy
    pub url: String,
}

/// Shop Pay Installments pricing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShopPayInstallmentsPricing {
    /// Available financing plans
    #[serde(rename = "financingPlans")]
    pub financing_plans: Vec<ShopPayInstallmentsFinancingPlan>,
    /// Maximum price to qualify for financing
    #[serde(rename = "maxPrice")]
    pub max_price: Money,
    /// Minimum price to qualify for financing
    #[serde(rename = "minPrice")]
    pub min_price: Money,
}

/// A Shop Pay Installments financing plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShopPayInstallmentsFinancingPlan {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub terms: Vec<ShopPayInstallmentsFinancingPlanTerm>,
}

/// A term for a financing plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShopPayInstallmentsFinancingPlanTerm {
    #[serde(rename = "loanType")]
    pub loan_type: ShopPayInstallmentsLoanType,
    /// Payment amount per installment
    #[serde(rename = "paymentAmount")]
    pub payment_amount: Money,
    /// Annual percentage rate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apr: Option<f64>,
    /// Number of installments
    #[serde(rename = "installmentsCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installments_count: Option<i32>,
}

/// The loan type for Shop Pay Installments
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ShopPayInstallmentsLoanType {
    #[serde(rename = "SPLIT_PAY")]
    SPLITPAY,
    #[serde(rename = "INTEREST_FREE")]
    INTERESTFREE,
    #[serde(rename = "INTEREST_BEARING")]
    INTERESTBEARING,
}

/// A social login provider for customer accounts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialLoginProvider {
    pub r#type: SocialLoginProviderType,
    /// Client ID for the provider
    #[serde(rename = "clientId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
}

/// Type of social login provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SocialLoginProviderType {
    #[serde(rename = "GOOGLE")]
    GOOGLE,
    #[serde(rename = "FACEBOOK")]
    FACEBOOK,
    #[serde(rename = "APPLE")]
    APPLE,
}

/// Shop payment settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentSettings {
    /// Shop country code
    #[serde(rename = "countryCode")]
    pub country_code: String,
    /// Shop currency code
    #[serde(rename = "currencyCode")]
    pub currency_code: String,
    #[serde(rename = "acceptedCardBrands")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepted_card_brands: Option<Vec<AcceptedCardBrandsItem>>,
    /// URL for card vaulting
    #[serde(rename = "cardVaultUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_vault_url: Option<String>,
    /// Enabled presentment currencies
    #[serde(rename = "enabledPresentmentCurrencies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_presentment_currencies: Option<Vec<String>>,
    #[serde(rename = "shopifyPaymentsAccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shopify_payments_account_id: Option<String>,
    #[serde(rename = "supportedDigitalWallets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_digital_wallets: Option<Vec<SupportedDigitalWalletsItem>>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AcceptedCardBrandsItem {
    #[serde(rename = "VISA")]
    VISA,
    #[serde(rename = "MASTERCARD")]
    MASTERCARD,
    #[serde(rename = "DISCOVER")]
    DISCOVER,
    #[serde(rename = "AMERICAN_EXPRESS")]
    AMERICANEXPRESS,
    #[serde(rename = "DINERS_CLUB")]
    DINERSCLUB,
    #[serde(rename = "JCB")]
    JCB,
    #[serde(rename = "EFTPOS_AU")]
    EFTPOSAU,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SupportedDigitalWalletsItem {
    #[serde(rename = "APPLE_PAY")]
    APPLEPAY,
    #[serde(rename = "GOOGLE_PAY")]
    GOOGLEPAY,
    #[serde(rename = "SHOPIFY_PAY")]
    SHOPIFYPAY,
}
