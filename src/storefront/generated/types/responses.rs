use super::cart::Cart;
use super::cart::CartMutationResult;
use super::collections::Collection;
use super::collections::CollectionConnection;
use super::content::Article;
use super::content::ArticleConnection;
use super::content::Blog;
use super::content::BlogConnection;
use super::content::Menu;
use super::content::Page;
use super::content::PageConnection;
use super::customer::AccessTokenResult;
use super::customer::AddressMutationResult;
use super::customer::Customer;
use super::customer::CustomerAccessToken;
use super::customer::CustomerMutationResult;
use super::customer::CustomerUserError;
use super::errors::UserError;
use super::localization::Localization;
use super::metafields::Metaobject;
use super::metafields::MetaobjectConnection;
use super::products::Product;
use super::products::ProductConnection;
use super::products::ProductVariant;
use super::search::PredictiveSearchResult;
use super::search::SearchResultItemConnection;
use super::shop::Shop;
use serde::{Deserialize, Serialize};

/// GraphQL response containing a single product
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<Product>,
}

/// GraphQL response containing product connection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub products: Option<ProductConnection>,
}

/// GraphQL response containing product recommendations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductRecommendationsResponse {
    #[serde(rename = "productRecommendations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_recommendations: Option<Vec<Product>>,
}

/// GraphQL response containing product variants resolved by id
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductVariantsResponse {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub nodes: Vec<Option<ProductVariant>>,
}

/// GraphQL response containing a single collection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection: Option<Collection>,
}

/// GraphQL response containing collection connection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collections: Option<CollectionConnection>,
}

/// GraphQL response containing a cart
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cart: Option<Cart>,
}

/// GraphQL response for cartCreate mutation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartCreateResponse {
    #[serde(rename = "cartCreate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cart_create: Option<CartMutationResult>,
}

/// GraphQL response for cartLinesAdd mutation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartLinesAddResponse {
    #[serde(rename = "cartLinesAdd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cart_lines_add: Option<CartMutationResult>,
}

/// GraphQL response for cartLinesUpdate mutation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartLinesUpdateResponse {
    #[serde(rename = "cartLinesUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cart_lines_update: Option<CartMutationResult>,
}

/// GraphQL response for cartLinesRemove mutation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartLinesRemoveResponse {
    #[serde(rename = "cartLinesRemove")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cart_lines_remove: Option<CartMutationResult>,
}

/// GraphQL response for cartNoteUpdate mutation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartNoteUpdateResponse {
    #[serde(rename = "cartNoteUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cart_note_update: Option<CartMutationResult>,
}

/// GraphQL response for cartAttributesUpdate mutation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartAttributesUpdateResponse {
    #[serde(rename = "cartAttributesUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cart_attributes_update: Option<CartMutationResult>,
}

/// GraphQL response for cartBuyerIdentityUpdate mutation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartBuyerIdentityUpdateResponse {
    #[serde(rename = "cartBuyerIdentityUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cart_buyer_identity_update: Option<CartMutationResult>,
}

/// GraphQL response for cartDiscountCodesUpdate mutation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartDiscountCodesUpdateResponse {
    #[serde(rename = "cartDiscountCodesUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cart_discount_codes_update: Option<CartMutationResult>,
}

/// GraphQL response for cartGiftCardCodesAdd mutation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartGiftCardCodesAddResponse {
    #[serde(rename = "cartGiftCardCodesAdd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cart_gift_card_codes_add: Option<CartMutationResult>,
}

/// GraphQL response containing a customer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Customer>,
}

/// GraphQL response for customerAccessTokenCreate mutation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerAccessTokenCreateResponse {
    #[serde(rename = "customerAccessTokenCreate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_access_token_create: Option<AccessTokenResult>,
}

/// GraphQL response for customerAccessTokenRenew mutation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerAccessTokenRenewResponse {
    #[serde(rename = "customerAccessTokenRenew")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_access_token_renew: Option<AccessTokenRenewResult>,
}

/// Result of customerAccessTokenRenew mutation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessTokenRenewResult {
    #[serde(rename = "customerAccessToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_access_token: Option<CustomerAccessToken>,
    #[serde(rename = "userErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_errors: Option<Vec<UserError>>,
}

/// GraphQL response for customerAccessTokenDelete mutation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerAccessTokenDeleteResponse {
    #[serde(rename = "customerAccessTokenDelete")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_access_token_delete: Option<CustomerAccessTokenDeleteResult>,
}

/// Result of customerAccessTokenDelete mutation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerAccessTokenDeleteResult {
    #[serde(rename = "deletedAccessToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_access_token: Option<String>,
    #[serde(rename = "deletedCustomerAccessTokenId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_customer_access_token_id: Option<String>,
    #[serde(rename = "userErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_errors: Option<Vec<UserError>>,
}

/// GraphQL response for customerCreate mutation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerCreateResponse {
    #[serde(rename = "customerCreate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_create: Option<CustomerMutationResult>,
}

/// GraphQL response for customerUpdate mutation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerUpdateResponse {
    #[serde(rename = "customerUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_update: Option<CustomerMutationResult>,
}

/// GraphQL response for customerRecover mutation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerRecoverResponse {
    #[serde(rename = "customerRecover")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_recover: Option<CustomerRecoverResult>,
}

/// Result of customerRecover mutation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerRecoverResult {
    #[serde(rename = "customerUserErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_user_errors: Option<Vec<CustomerUserError>>,
}

/// GraphQL response for customerReset mutation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerResetResponse {
    #[serde(rename = "customerReset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_reset: Option<AccessTokenResult>,
}

/// GraphQL response for customerResetByUrl mutation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerResetByUrlResponse {
    #[serde(rename = "customerResetByUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_reset_by_url: Option<AccessTokenResult>,
}

/// GraphQL response for customerActivate mutation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerActivateResponse {
    #[serde(rename = "customerActivate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_activate: Option<AccessTokenResult>,
}

/// GraphQL response for customerActivateByUrl mutation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerActivateByUrlResponse {
    #[serde(rename = "customerActivateByUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_activate_by_url: Option<AccessTokenResult>,
}

/// GraphQL response for customerAddressCreate mutation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerAddressCreateResponse {
    #[serde(rename = "customerAddressCreate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_address_create: Option<AddressMutationResult>,
}

/// GraphQL response for customerAddressUpdate mutation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerAddressUpdateResponse {
    #[serde(rename = "customerAddressUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_address_update: Option<AddressMutationResult>,
}

/// GraphQL response for customerAddressDelete mutation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerAddressDeleteResponse {
    #[serde(rename = "customerAddressDelete")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_address_delete: Option<CustomerAddressDeleteResult>,
}

/// Result of customerAddressDelete mutation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerAddressDeleteResult {
    #[serde(rename = "deletedCustomerAddressId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_customer_address_id: Option<String>,
    #[serde(rename = "customerUserErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_user_errors: Option<Vec<CustomerUserError>>,
}

/// GraphQL response for customerDefaultAddressUpdate mutation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerDefaultAddressUpdateResponse {
    #[serde(rename = "customerDefaultAddressUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_default_address_update: Option<CustomerMutationResult>,
}

/// GraphQL response for search query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<SearchResultItemConnection>,
}

/// GraphQL response for predictiveSearch query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictiveSearchResponse {
    #[serde(rename = "predictiveSearch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predictive_search: Option<PredictiveSearchResult>,
}

/// GraphQL response containing a page
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<Page>,
}

/// GraphQL response containing page connection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PagesResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pages: Option<PageConnection>,
}

/// GraphQL response containing a blog
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blog: Option<Blog>,
}

/// GraphQL response containing blog connection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blogs: Option<BlogConnection>,
}

/// GraphQL response containing an article
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub article: Option<Article>,
}

/// GraphQL response containing article connection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticlesResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub articles: Option<ArticleConnection>,
}

/// GraphQL response containing a menu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub menu: Option<Menu>,
}

/// GraphQL response containing a metaobject
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaobjectResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metaobject: Option<Metaobject>,
}

/// GraphQL response containing metaobject connection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaobjectsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metaobjects: Option<MetaobjectConnection>,
}

/// GraphQL response containing shop info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShopResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shop: Option<Shop>,
}

/// GraphQL response containing localization info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalizationResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub localization: Option<Localization>,
}
