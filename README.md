<div align="center">

# shopify-client

**Type-safe, async Rust client for the Shopify Admin and Storefront APIs**

[![Crates.io](https://img.shields.io/crates/v/shopify-client.svg)](https://crates.io/crates/shopify-client)
[![Documentation](https://docs.rs/shopify-client/badge.svg)](https://docs.rs/shopify-client)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Shopify API](https://img.shields.io/badge/Shopify%20API-2026--01-green.svg)](https://shopify.dev/docs/api/admin)

[Installation](#installation) &bull; [Admin API](#admin-api) &bull; [Storefront API](#storefront-api) &bull; [Bulk Operations](#bulk-operations) &bull; [Docs](https://docs.rs/shopify-client)

</div>

---

```rust
// Admin API — orders, subscriptions, bulk exports, webhooks, …
let admin = ShopifyClient::new(shop_url.clone(), admin_token, None);
let order = admin.order.get_with_id(&id).await?;

// Storefront API — products, cart, customer, search, … (behind the `storefront` feature)
let store = ShopifyStorefront::new(shop_url, storefront_token, None);
let product = store.product.get_by_handle("my-product").await?;
```

## Highlights

| | |
|---|---|
| **Two clients, one crate** | `ShopifyClient` for the Admin API, `ShopifyStorefront` for the Storefront API — each with its own token, endpoint, and surface. Storefront lives behind a Cargo feature so admin-only apps pay nothing for it. |
| **10 admin service modules** | Orders, Subscriptions, Discounts, Cart Transforms, App Installation, Shopify Functions, Shop, Storefront Tokens, Bulk Operations, Themes |
| **8 storefront service modules** | Products, Collections, Cart, Customer, Search, Content (pages/blogs/articles/menus), Shop, Metaobjects |
| **Bulk operations** | Prebuilt export templates for products, orders, collections, customers, inventory, and draft orders with typed JSONL parsing |
| **Typed everything** | Strongly typed requests, responses, filters, JSONL export lines, GraphQL inputs — no raw strings |
| **Async / await** | Built on `reqwest` with `tokio` — non-blocking by default, one shared connection pool |
| **Webhook parsing** | HMAC-verified parsing for customer data, customer redact, and shop redact compliance webhooks |
| **Request callbacks** | Optional before/after hooks for logging, metrics, and observability |

## Installation

`shopify-client` is a **single crate**. The Admin API is always available; the Storefront API is gated behind the `storefront` Cargo feature.

```toml
[dependencies]

# Admin API only (the common case)
shopify-client = "0.19"

# Admin API + Storefront API
shopify-client = { version = "0.19", features = ["storefront"] }
```

With the `storefront` feature enabled, the Storefront client is available as `shopify_client::storefront`:

```rust
use shopify_client::ShopifyClient;             // Admin
use shopify_client::storefront::ShopifyStorefront; // Storefront (feature-gated)
```

The two clients are fully independent — different access tokens, different endpoints, different rate limits. Construct whichever you need; apps that need both construct one of each.

---

# Admin API

The Admin API is what apps use to manage merchant data: orders, products, subscriptions, discounts, app metafields, bulk exports, and more. Requests use an **Admin access token** sent as the `X-Shopify-Access-Token` header, against the `/admin/api/{version}/graphql.json` (and per-resource REST) endpoints.

## Getting Started

```toml
[dependencies]
shopify-client = "0.19"
```

```rust
use shopify_client::ShopifyClient;

#[tokio::main]
async fn main() {
    let client = ShopifyClient::new(
        "https://your-shop.myshopify.com".to_string(),
        "your-admin-access-token".to_string(),
        None, // defaults to API version 2026-01
    );

    // Every service is a field on the client
    let order = client.order.get_with_id(&"1234567890".to_string()).await;
    let shop  = client.shop.get().await;
}
```

## API Coverage

| Service | Protocol | Operations |
|---------|----------|------------|
| **`client.order`** | REST | `get_with_id`, `get_with_name`, `patch` |
| **`client.subscription`** | GraphQL | `create_recurring`, `create_usage`, `create_combined`, `cancel`, `extend_trial`, `update_capped_amount`, `create_usage_record`, `get_active_subscriptions` |
| **`client.discount`** | GraphQL | `create_automatic_app_discount`, `update_automatic_app_discount`, `get_discount_nodes` |
| **`client.app_installation`** | GraphQL | `get_current`, `set_metafields`, `get_metafield`, `list_metafields` |
| **`client.cart_transform`** | GraphQL | `create`, `set_metafields` |
| **`client.shopify_functions`** | GraphQL | `list` |
| **`client.shop`** | GraphQL | `get`, `get_status` |
| **`client.storefront_access_token`** | GraphQL | `list`, `create`, `delete` |
| **`client.bulk_operation`** | GraphQL | `run_query`, `run_mutation`, `cancel`, `get`, `list`, `create_staged_upload`, `export_*`, `stream_*` |
| **`client.theme`** | GraphQL | `list`, `get_live`, `create_preview` |
| **`client.access_scope`** | REST | `list` |

Admin services live under `shopify_client::admin::*` (also re-exported as `shopify_client::services::*` for back-compat with pre-0.19 releases). Request/response types live under `shopify_client::types::*`.

## Examples

### Orders

```rust
// Get by ID
let resp = client.order.get_with_id(&order_id).await?;
println!("Order: {} - {}", resp.order.name, resp.order.email);

// Get by name
let resp = client.order.get_with_name(&"1001".to_string()).await?;

// Update tags
let patch = PatchOrderRequest {
    order: PatchOrder {
        tags: vec!["processed".into(), "priority".into()],
    },
};
client.order.patch(&order_id, &patch).await?;
```

### Subscriptions

```rust
use shopify_client::types::subscription::*;

let request = CreateRecurringSubscriptionRequest {
    name: "Premium Plan".to_string(),
    price: 29.99,
    currency_code: "USD".to_string(),
    return_url: "https://your-app.com/billing".to_string(),
    interval: Some(AppPricingInterval::Every30Days),
    trial_days: Some(7),
    test: Some(true),
    discount: None,
};

let resp = client.subscription.create_recurring(&request).await?;
println!("Confirm at: {}", resp.confirmation_url);
```

### Shop Info

```rust
let resp = client.shop.get().await?;
println!("{} ({})", resp.shop.name, resp.shop.plan.public_display_name);
println!("Owner: {}", resp.shop.account_owner.email);

// Lightweight status check — useful for health checks / setup wizards
let status = client.shop.get_status().await?;
println!("Setup required: {}", status.shop.setup_required);
```

### Access Scopes

```rust
// Reflects scopes as they stand now, unlike the `scope` string returned at OAuth time
let resp = client.access_scope.list().await?;

for scope in &resp.access_scopes {
    println!("Granted: {}", scope.handle);
}
```

<details>
<summary><strong>More examples: Discounts, Cart Transforms, Metafields, Storefront tokens</strong></summary>

#### Create automatic discount

```rust
use shopify_client::types::discount::DiscountAutomaticAppInput;

let input = DiscountAutomaticAppInput {
    title: "Summer Sale".to_string(),
    function_handle: "my-discount-function".to_string(),
    starts_at: "2024-06-01T00:00:00Z".to_string(),
    ends_at: None,
    combines_with: None,
    discount_classes: None,
    context: None,
    metafields: None,
    applies_on_subscription: None,
    applies_on_one_time_purchase: None,
    recurring_cycle_limit: None,
};
client.discount.create_automatic_app_discount(&input).await?;
```

#### Create cart transform

```rust
use shopify_client::types::cart_transform::{CartTransformCreateInput, MetafieldInput};

let input = CartTransformCreateInput::new()
    .with_function_handle("my-cart-transform".to_string())
    .with_block_on_failure(false)
    .with_metafields(vec![
        MetafieldInput::new(
            "$app".to_string(), "config".to_string(),
            r#"{"bundleDiscount": 10}"#.to_string(), "json".to_string(),
        ),
    ]);
client.cart_transform.create(&input).await?;
```

#### Update metafields with CAS

```rust
use shopify_client::types::cart_transform::MetafieldsSetInput;

let metafields = vec![
    MetafieldsSetInput::new(
        owner_id.clone(), "$app".to_string(), "config".to_string(),
        r#"{"bundleDiscount": 20}"#.to_string(), "json".to_string(),
    ).with_compare_digest(Some("fd6b737...".to_string())),
];
client.cart_transform.set_metafields(&metafields).await?;
```

#### Manage storefront access tokens

The Admin API issues the **Storefront** access tokens used by the Storefront client below.

```rust
// Create a public access token for a custom storefront / SDK
let resp = client.storefront_access_token
    .create(&"My Custom Storefront".to_string())
    .await?;
println!("Token: {}", resp.storefront_access_token.access_token);

// List all existing tokens
let tokens = client.storefront_access_token.list().await?;

// Revoke when no longer needed
client.storefront_access_token.delete(&token_id).await?;
```

</details>

## Bulk Operations

The bulk operations API lets you export or import millions of objects asynchronously. This client provides **two layers**:

| Layer | For | Example |
|-------|-----|---------|
| **Prebuilt templates** | Common exports with zero GraphQL | `client.bulk_operation.export_products(params)` |
| **Raw operations** | Custom queries / mutations | `client.bulk_operation.run_query(graphql, None)` |

### Export Templates

Six prebuilt exports, each with typed filter params and comprehensive field coverage:

| Template | Resource | Children | Filter Struct |
|----------|----------|----------|---------------|
| `export_products` | Products | Variants, Media | `ProductQueryParams` |
| `export_orders` | Orders | LineItems | `OrderQueryParams` |
| `export_collections` | Collections | Products | `CollectionQueryParams` |
| `export_customers` | Customers | Addresses | `CustomerQueryParams` |
| `export_inventory_items` | InventoryItems | Levels | `InventoryItemQueryParams` |
| `export_draft_orders` | DraftOrders | LineItems | `DraftOrderQueryParams` |

### Typed Filters

Build search filters with type safety &mdash; no raw query strings:

```rust
use shopify_client::types::order::*;
use shopify_client::common::query_filter::DateFilter;

let params = OrderQueryParams {
    status: Some(OrderStatus::Open),
    financial_status: Some(OrderFinancialStatus::Paid),
    created_at: Some(DateFilter::OnOrAfter("2025-01-01".to_string())),
    ..Default::default()
};

client.bulk_operation.export_orders(Some(&params)).await?;
```

Available filter primitives:

| Filter | Variants |
|--------|----------|
| `DateFilter` | `Exact`, `Before`, `After`, `OnOrBefore`, `OnOrAfter`, `Range` |
| `NumericFilter<T>` | `Exact`, `GreaterThan`, `LessThan`, `GreaterOrEqual`, `LessOrEqual` |

### Full Export Workflow

Bulk exports are a three-step process: start the export, poll until Shopify finishes, then consume the result. The JSONL result URL points to a temporary file hosted by Shopify that can be multi-GB for large shops (millions of products/orders). The `stream_*` methods download and parse this file in chunks so your app uses constant memory regardless of file size.

```rust
use shopify_client::ShopifyClient;
use shopify_client::types::bulk_operation::{BulkOperationStatus, ProductExportLine};
use shopify_client::types::product::{ProductQueryParams, ProductStatus};

let client = ShopifyClient::new(shop_url, access_token, None);

// 1. Start the export
let params = ProductQueryParams {
    status: Some(ProductStatus::Active),
    ..Default::default()
};
let resp = client.bulk_operation.export_products(Some(&params)).await?;
let op = resp.bulk_operation_run_query.bulk_operation.ok_or("no operation")?;

// 2. Poll until Shopify finishes processing
let url = loop {
    let status = client.bulk_operation.get(&op.id).await?;
    match status.bulk_operation.as_ref().map(|o| &o.status) {
        Some(BulkOperationStatus::Completed) => {
            break status.bulk_operation.and_then(|o| o.url).ok_or("no url")?;
        }
        Some(BulkOperationStatus::Failed) => return Err("export failed".into()),
        _ => tokio::time::sleep(std::time::Duration::from_secs(2)).await,
    }
};

// 3. Stream results — constant memory, processes batch_size items at a time
client.bulk_operation.stream_products(&url, 50, |batch| async move {
    // batch: Vec<ProductExportLine> with up to 50 items
    for item in &batch {
        match item {
            ProductExportLine::Product(p) => println!("Product: {}", p.title),
            ProductExportLine::Variant(v) => println!("  Variant: {:?}", v.sku),
            ProductExportLine::Media(m)   => println!("  Media: {:?}", m.media_content_type),
        }
    }
    Ok(())
}).await?;
```

Each prebuilt export has a matching `stream_*` method that takes a `batch_size` and an async callback:

| Export | Stream | Line Enum |
|--------|--------|-----------|
| `export_products` | `stream_products` | `ProductExportLine` |
| `export_orders` | `stream_orders` | `OrderExportLine` |
| `export_collections` | `stream_collections` | `CollectionExportLine` |
| `export_customers` | `stream_customers` | `CustomerExportLine` |
| `export_inventory_items` | `stream_inventory_items` | `InventoryItemExportLine` |
| `export_draft_orders` | `stream_draft_orders` | `DraftOrderExportLine` |

All `stream_*` methods accumulate up to `batch_size` parsed items before invoking your async callback. The last batch may contain fewer items. Lines that fail to parse are silently skipped.

```rust
// Process 100 products at a time — great for DB writes, API calls, S3 uploads
client.bulk_operation.stream_products(&url, 100, |batch| async move {
    save_to_db(&batch).await?;
    Ok(())
}).await?;
```

### Custom Bulk Queries

For queries the templates don't cover, use `run_query` with your own GraphQL and `stream_jsonl` to consume the result:

```rust
let query = r#"{
  products {
    edges {
      node { id title tags }
    }
  }
}"#;

client.bulk_operation.run_query(query, None).await?;
// ... poll until complete, get the url ...

// Stream raw JSON lines in batches of 100
client.bulk_operation.stream_jsonl(&url, 100, |lines| async move {
    write_to_file(&lines).await?;
    Ok(())
}).await?;
```

If you already have JSONL content in memory (e.g. read from a local file), each export enum has a `parse_line()` method:

```rust
use shopify_client::types::bulk_operation::ProductExportLine;

let parsed = ProductExportLine::parse_line(line)?;
```

## Webhooks

Parse Shopify compliance webhooks with type safety:

```rust
use shopify_client::webhooks::{parse_webhook_with_header, WebhookPayload};

match parse_webhook_with_header(topic_header, payload) {
    Ok(WebhookPayload::CustomersDataRequest(data)) => { /* ... */ }
    Ok(WebhookPayload::CustomersRedact(data))      => { /* ... */ }
    Ok(WebhookPayload::ShopRedact(data))            => { /* ... */ }
    Err(e) => eprintln!("Parse error: {:?}", e),
}
```

HMAC verification helpers live in `shopify_client::webhooks::verify`:

```rust
use shopify_client::webhooks::verify::{verify_hmac, verify_hmac_from_headers};

if !verify_hmac_from_headers(headers, body, app_secret) {
    return Err("invalid webhook signature");
}
```

---

# Storefront API

> Requires the **`storefront`** Cargo feature: `shopify-client = { version = "0.19", features = ["storefront"] }`

The Storefront API is what custom storefronts, BFFs, edge functions, and headless commerce backends use to read the public catalog and manage shopper carts. Requests use a **Storefront access token** (different from the Admin token) sent as `X-Shopify-Storefront-Access-Token`, against the `/api/{version}/graphql.json` endpoint.

> **Where to get a Storefront access token:** create one in the Shopify Admin UI under *Apps → Headless / Custom Storefront*, or programmatically via [`client.storefront_access_token.create`](#manage-storefront-access-tokens) on the Admin API.

## Getting Started

```toml
[dependencies]
shopify-client = { version = "0.19", features = ["storefront"] }
```

```rust
use shopify_client::storefront::ShopifyStorefront;

#[tokio::main]
async fn main() {
    let store = ShopifyStorefront::new(
        "https://your-shop.myshopify.com".to_string(),
        "your-storefront-access-token".to_string(),
        None, // defaults to API version 2026-01
    );

    let product = store.product.get_by_handle("my-product").await;
}
```

## API Coverage

| Service | Field on `ShopifyStorefront` | Operations |
|---------|------------------------------|------------|
| **`store.product`** | Products | `get_by_id`, `get_by_handle`, `get_many`, `get_recommendations` |
| **`store.collection`** | Collections | `get_by_id`, `get_by_handle`, `get_with_products`, `get_many` |
| **`store.cart`** | Cart | `get`, `create`, `add_lines`, `update_lines`, `remove_lines`, `update_note`, `update_attributes`, `update_buyer_identity`, `update_discount_codes`, `add_gift_card_codes` (plus `*_without_customer` variants) |
| **`store.customer`** | Customers | `get`, `login`, `renew_token`, `logout`, `create`, `update`, `recover`, `reset`, `reset_by_url`, `activate`, `activate_by_url`, `create_address`, `update_address`, `delete_address`, `set_default_address` |
| **`store.search`** | Search | `search`, `predictive` |
| **`store.content`** | Pages / Blogs / Articles / Menus | nested: `content.pages.{get_by_id, get_by_handle, get_many}`, `content.blogs.{get_by_id, get_by_handle, get_many, get_with_articles}`, `content.articles.{get_by_id, get_many}`, `content.menus.get_by_handle` |
| **`store.shop`** | Shop info | `get`, `get_localization` |
| **`store.metaobject`** | Metaobjects | `get_by_id`, `get_by_handle`, `get_many` |

All Storefront types — products, carts, customers, response wrappers, input types, sort key enums, etc. — are generated from YAML specs in `types/storefront/` (see [Code Generation](#code-generation)) and live under `shopify_client::storefront::generated::types::*`.

## Examples

### Products

```rust
use shopify_client::storefront::ShopifyStorefront;
use shopify_client::storefront::generated::types::products::{
    GetProductsArgs, GetProductRecommendationsArgs, ProductSortKeys, ProductRecommendationIntent,
};

let store = ShopifyStorefront::new(shop_url, storefront_token, None);

// Single product by handle (most common)
let resp = store.product.get_by_handle("classic-t-shirt").await?;
if let Some(product) = resp.product {
    println!("{} — {}", product.title, product.handle);
}

// Single product by GID
let resp = store.product.get_by_id("gid://shopify/Product/123456").await?;

// Paginated list with filters
let resp = store.product.get_many(GetProductsArgs {
    first: Some(20),
    after: None,
    last: None,
    before: None,
    reverse: Some(false),
    sort_key: Some(ProductSortKeys::BestSelling),
    query: Some("tag:new".to_string()),
}).await?;

if let Some(conn) = resp.products {
    for node in conn.nodes {
        println!("{} ({})", node.title, node.handle);
    }
    if conn.page_info.has_next_page {
        // Use conn.page_info.end_cursor as `after` on the next call
    }
}

// Product recommendations (related, complementary)
let resp = store.product.get_recommendations(GetProductRecommendationsArgs {
    product_id: "gid://shopify/Product/123456".to_string(),
    intent: Some(ProductRecommendationIntent::Related),
}).await?;
```

### Collections

```rust
use shopify_client::storefront::collection::CollectionRef;
use shopify_client::storefront::generated::types::collections::{
    GetCollectionsArgs, GetCollectionProductsArgs, ProductCollectionSortKeys, CollectionSortKeys,
};

// Single collection
let resp = store.collection.get_by_handle("summer-sale").await?;
let resp = store.collection.get_by_id("gid://shopify/Collection/789").await?;

// Collection with paginated products inside it (one round-trip)
let resp = store.collection.get_with_products(
    CollectionRef::Handle("summer-sale".to_string()),
    GetCollectionProductsArgs {
        first: Some(50),
        after: None,
        last: None,
        before: None,
        reverse: None,
        sort_key: Some(ProductCollectionSortKeys::BestSelling),
        filters: None,
    },
).await?;

// Paginated list of all collections
let resp = store.collection.get_many(GetCollectionsArgs {
    first: Some(20),
    after: None,
    last: None,
    before: None,
    reverse: None,
    sort_key: Some(CollectionSortKeys::Title),
    query: None,
}).await?;
```

### Cart

The cart API has 10 mutations plus a `get` query, each with a `*_without_customer` variant for unauthenticated flows that omit `buyerIdentity.customer` from the response.

```rust
use shopify_client::storefront::generated::types::cart::{
    CartInput, CartLineInput, CartLineUpdateInput, CartBuyerIdentityInput,
};
use shopify_client::storefront::generated::types::common::AttributeInput;

// Create an empty cart
let resp = store.cart.create(CartInput {
    lines: None,
    note: None,
    attributes: None,
    buyer_identity: None,
    discount_codes: None,
    gift_card_codes: None,
    metafields: None,
}).await?;
let cart_id = resp.cart_create
    .and_then(|r| r.cart)
    .map(|c| c.id)
    .ok_or("cart creation failed")?;

// Add lines
let resp = store.cart.add_lines(&cart_id, vec![
    CartLineInput {
        merchandise_id: "gid://shopify/ProductVariant/111".to_string(),
        quantity: Some(2),
        attributes: None,
        selling_plan_id: None,
    },
]).await?;

// Update line quantities
store.cart.update_lines(&cart_id, vec![
    CartLineUpdateInput {
        id: "gid://shopify/CartLine/xyz".to_string(),
        quantity: Some(5),
        merchandise_id: None,
        attributes: None,
        selling_plan_id: None,
    },
]).await?;

// Apply a discount code
store.cart.update_discount_codes(&cart_id, vec!["SUMMER10".to_string()]).await?;

// Attach a logged-in customer
store.cart.update_buyer_identity(&cart_id, CartBuyerIdentityInput {
    email: Some("shopper@example.com".to_string()),
    phone: None,
    country_code: None,
    customer_access_token: Some(customer_token),
}).await?;

// Cart attributes (free-form key/value)
store.cart.update_attributes(&cart_id, vec![
    AttributeInput { key: "gift_wrap".to_string(), value: "true".to_string() },
]).await?;

// Read the latest state — `checkout_url` is what you redirect the shopper to
let resp = store.cart.get(&cart_id).await?;
if let Some(cart) = resp.cart {
    println!("Checkout: {}", cart.checkout_url);
}
```

**Unauthenticated flows.** Every cart method has a `*_without_customer` variant that uses a Cart fragment which omits `buyerIdentity.customer`. Use these when you don't have (and don't want to leak the existence of) a customer access token:

```rust
// Same shape as above, but the response Cart never includes buyerIdentity.customer
let resp = store.cart.create_without_customer(CartInput { /* … */ }).await?;
store.cart.add_lines_without_customer(&cart_id, lines).await?;
store.cart.update_discount_codes_without_customer(&cart_id, vec!["SUMMER10".into()]).await?;
```

### Customer

The customer API covers the full account lifecycle — signup, login, password reset, address management, order history.

```rust
use shopify_client::storefront::generated::types::customer::{
    CustomerAccessTokenCreateInput, CustomerCreateInput, CustomerUpdateInput,
    CustomerResetInput, CustomerActivateInput, MailingAddressInput,
};

// Sign up
let resp = store.customer.create(CustomerCreateInput {
    email: "new@shopper.com".to_string(),
    password: "supers3cret".to_string(),
    first_name: Some("Alex".to_string()),
    last_name: Some("Doe".to_string()),
    phone: None,
    accepts_marketing: Some(true),
}).await?;

// Log in — returns an access token + expiry
let resp = store.customer.login(CustomerAccessTokenCreateInput {
    email: "alex@shopper.com".to_string(),
    password: "supers3cret".to_string(),
}).await?;
let token = resp.customer_access_token_create
    .and_then(|r| r.customer_access_token)
    .map(|t| t.access_token)
    .ok_or("login failed")?;

// Renew before expiry
let resp = store.customer.renew_token(&token).await?;

// Fetch the customer + first N addresses + first N orders
let resp = store.customer.get(&token, Some(10), Some(20)).await?;
if let Some(customer) = resp.customer {
    println!("Hello, {}", customer.display_name);
}

// Update profile
store.customer.update(&token, CustomerUpdateInput {
    first_name: Some("Alexandra".to_string()),
    last_name: None,
    email: None,
    phone: None,
    password: None,
    accepts_marketing: None,
}).await?;

// Password recovery → email flow
store.customer.recover("alex@shopper.com").await?;

// After the user clicks the reset link, complete with reset_by_url
let resp = store.customer.reset_by_url(reset_url, "newp@ssword").await?;

// Or reset with an explicit id + reset token (e.g. from a deeplink param)
let resp = store.customer.reset(&customer_id, CustomerResetInput {
    reset_token: reset_token.clone(),
    password: "newp@ssword".to_string(),
}).await?;

// Account activation (for accounts created in admin without a password)
let resp = store.customer.activate(&customer_id, CustomerActivateInput {
    activation_token: activation_token.clone(),
    password: "newp@ssword".to_string(),
}).await?;

// Addresses
store.customer.create_address(&token, MailingAddressInput {
    first_name: Some("Alex".to_string()),
    last_name: Some("Doe".to_string()),
    address1: Some("1 Infinite Loop".to_string()),
    address2: None,
    city: Some("Cupertino".to_string()),
    province: Some("CA".to_string()),
    country: Some("United States".to_string()),
    zip: Some("95014".to_string()),
    phone: None,
    company: None,
}).await?;
store.customer.update_address(&token, &address_id, address).await?;
store.customer.delete_address(&token, &address_id).await?;
store.customer.set_default_address(&token, &address_id).await?;

// Log out (revoke the token)
store.customer.logout(&token).await?;
```

### Search

```rust
use shopify_client::storefront::generated::types::search::{
    SearchArgs, PredictiveSearchArgs, SearchSortKeys, SearchType,
    SearchPrefixQueryType, SearchUnavailableProductsType,
    PredictiveSearchLimitScope, PredictiveSearchType,
};

// Full search (returns Product | Page | Article in a connection)
let resp = store.search.search(SearchArgs {
    query: "summer dress".to_string(),
    first: Some(20),
    after: None,
    last: None,
    before: None,
    reverse: None,
    sort_key: Some(SearchSortKeys::Relevance),
    types: Some(vec![SearchType::Product]),
    product_filters: None,
    prefix: Some(SearchPrefixQueryType::Last),
    unavailable_products: Some(SearchUnavailableProductsType::Hide),
}).await?;

// Predictive search — for instant-results UIs / typeahead
let resp = store.search.predictive(PredictiveSearchArgs {
    query: "sum".to_string(),
    limit: Some(5),
    limit_scope: Some(PredictiveSearchLimitScope::Each),
    types: Some(vec![PredictiveSearchType::Product, PredictiveSearchType::Collection]),
    unavailable_products: Some(SearchUnavailableProductsType::Hide),
}).await?;

if let Some(result) = resp.predictive_search {
    for p in result.products.unwrap_or_default() {
        println!("Product: {}", p.title);
    }
    for c in result.collections.unwrap_or_default() {
        println!("Collection: {}", c.title);
    }
}
```

### Content

The content service is split into four sub-services matching Shopify's content model: pages, blogs, articles, and menus.

```rust
use shopify_client::storefront::content::BlogRef;
use shopify_client::storefront::generated::types::content::{
    GetPagesArgs, GetBlogsArgs, GetArticlesArgs, GetBlogArticlesArgs,
    PageSortKeys, BlogSortKeys, ArticleSortKeys,
};

// Pages
let resp = store.content.pages.get_by_handle("about").await?;
let resp = store.content.pages.get_by_id("gid://shopify/Page/1").await?;
let resp = store.content.pages.get_many(GetPagesArgs {
    first: Some(20),
    after: None, last: None, before: None,
    reverse: None,
    sort_key: Some(PageSortKeys::Title),
    query: None,
}).await?;

// Blogs
let resp = store.content.blogs.get_by_handle("news").await?;
let resp = store.content.blogs.get_many(GetBlogsArgs {
    first: Some(10),
    after: None, last: None, before: None,
    reverse: None,
    sort_key: Some(BlogSortKeys::Handle),
    query: None,
}).await?;

// Blog + its articles in one round-trip
let resp = store.content.blogs.get_with_articles(
    BlogRef::Handle("news".to_string()),
    GetBlogArticlesArgs {
        articles_first: Some(10),
        articles_after: None,
        articles_reverse: Some(true),
        articles_sort_key: Some(ArticleSortKeys::PublishedAt),
    },
).await?;

// Articles (top-level, across all blogs)
let resp = store.content.articles.get_by_id("gid://shopify/Article/42").await?;
let resp = store.content.articles.get_many(GetArticlesArgs {
    first: Some(20),
    after: None, last: None, before: None,
    reverse: Some(true),
    sort_key: Some(ArticleSortKeys::PublishedAt),
    query: Some("tag:featured".to_string()),
}).await?;

// Navigation menus
let resp = store.content.menus.get_by_handle("main-menu").await?;
if let Some(menu) = resp.menu {
    for item in menu.items {
        println!("{} → {:?}", item.title, item.url);
    }
}
```

### Shop & Localization

```rust
let resp = store.shop.get().await?;
if let Some(shop) = resp.shop {
    println!("{}", shop.name);
    println!("Currency: {}", shop.payment_settings.currency_code);
    println!("Ships to: {:?}", shop.ships_to_countries);
}

// Available languages, currencies, and country shopping experiences
let resp = store.shop.get_localization().await?;
if let Some(loc) = resp.localization {
    println!("Current country: {} ({})",
        loc.country.name, loc.country.iso_code);
    for lang in loc.available_languages {
        println!("  {} — {}", lang.iso_code, lang.name);
    }
}
```

### Metaobjects

```rust
use shopify_client::storefront::generated::types::metafields::{
    MetaobjectHandleInput, GetMetaobjectsArgs,
};

// By GID
let resp = store.metaobject.get_by_id("gid://shopify/Metaobject/1").await?;

// By {type, handle} tuple
let resp = store.metaobject.get_by_handle(MetaobjectHandleInput {
    handle: "homepage-banner".to_string(),
    type_: "banner".to_string(),
}).await?;

// All metaobjects of a given type, paginated
let resp = store.metaobject.get_many(GetMetaobjectsArgs {
    type_: "banner".to_string(),
    first: Some(20),
    after: None, last: None, before: None,
    reverse: None,
    sort_key: None,
}).await?;
```

## Storefront vs Admin: when to use which

| Need | API | Service |
|------|-----|---------|
| Display the catalog on a custom storefront | **Storefront** | `store.product`, `store.collection` |
| Manage a shopper's cart and redirect to checkout | **Storefront** | `store.cart` |
| Customer signup / login / address book | **Storefront** | `store.customer` |
| Process an order after it's placed | **Admin** | `client.order` |
| Run a one-off export of all products | **Admin** | `client.bulk_operation.export_products` |
| Charge a subscription | **Admin** | `client.subscription` |
| Define a discount that affects all storefronts | **Admin** | `client.discount` |
| Issue a Storefront access token for a new headless app | **Admin** | `client.storefront_access_token.create` |

---

## Request Callbacks

Both clients support optional before/after hooks on every request — useful for logging, metrics, and tracing.

```rust
use shopify_client::ShopifyClient;
use std::sync::Arc;

let client = ShopifyClient::new_with_callbacks(
    shop_url, admin_token, None,
    Some(Arc::new(|url, body, _headers| {
        println!("-> {} ({} bytes)", url, body.map_or(0, |b| b.len()));
    })),
    Some(Arc::new(|url, resp, _headers| {
        println!("<- {} ({} bytes)", url, resp.len());
    })),
);
```

`ShopifyStorefront::new_with_callbacks` has the exact same signature. Properties:

- Fires on every REST and GraphQL request.
- The shop-level access token is **never** passed to callbacks.
- The request `body` *can* contain other sensitive values that travel as GraphQL variables — most notably a **customer access token** on storefront cart/customer mutations, and passwords on `customerCreate` / `customerReset`. Treat the body as sensitive; avoid logging it verbatim to shared destinations.
- Panic-safe — a panicking callback won't crash the request flow.

## Error Handling

All methods on both clients return `Result<T, APIError>`:

```rust
pub enum APIError {
    ServerError { errors: String },  // Shopify returned a GraphQL/REST error
    FailedToParse,                   // Response couldn't be deserialized
    NetworkError,                    // Connection / timeout failure
}
```

For GraphQL errors, `APIError::ServerError.errors` includes Shopify's error code where available — e.g. `[THROTTLED] Throttled` — so callers can classify failures for retry without parsing free text.

For Storefront mutations that return per-field validation errors (e.g. `customerCreate` with a bad email), check the `customer_user_errors` / `user_errors` field on the response payload — those are part of a *successful* response, not an `APIError`.

## Code Generation

Storefront types are generated from YAML specs in `types/storefront/` via [`type-crafter`](https://www.npmjs.com/package/type-crafter), invoked through `npx`:

```bash
make gen-storefront     # regenerate src/storefront/generated/types/ from the YAML specs
make clean-storefront   # wipe generated output
```

The generated output is **checked into git**, so `cargo build` works without Node installed. Regenerate after editing any `types/storefront/*.yaml` file.

## Project Structure

```
shopify-rust-client/
├── Cargo.toml                       # single [package]; `storefront` feature
├── Makefile                         # `make gen-storefront` regenerates storefront types
├── types/storefront/*.yaml          # source specs for the generated storefront types
└── src/
    ├── lib.rs                       # ShopifyClient
    ├── common/                      # shared infra
    │   ├── types.rs                 #   APIError, RequestCallbacks, PageInfo, …
    │   ├── http.rs                  #   shared reqwest client + GraphQL executors
    │   ├── utils.rs                 #   parse_response helpers
    │   ├── query_filter.rs          #   DateFilter, NumericFilter
    │   └── mod.rs                   #   ServiceContext
    ├── admin/                       # admin services (also re-exported as `services`)
    │   └── <service>/{mod.rs, remote.rs}   # order, subscription, discount, …, bulk_operation
    ├── types/                       # admin request/response types
    ├── webhooks/                    # compliance webhook parsing + HMAC verify
    └── storefront/                  # behind the `storefront` feature
        ├── mod.rs                   #   ShopifyStorefront
        ├── fragments/               #   GraphQL fragments shared across services
        ├── <service>/               #   product, cart, collection, customer, …
        │   ├── mod.rs               #     service struct + async methods
        │   ├── remote.rs            #     internal HTTP calls
        │   └── queries.rs           #     GraphQL query builders
        └── generated/types/         #   type-crafter output (checked in)
```

## Requirements

- **Rust** 2021 edition (1.70+)
- **Always:** `reqwest`, `serde`, `serde_json`, `hmac`, `sha2`, `base64`
- **Only with the `storefront` feature:** `time` (typed `OffsetDateTime` timestamps on generated Storefront types)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. If you change any `types/storefront/*.yaml` spec, run `make gen-storefront` and commit the regenerated output.

## License

This project is licensed under the [MIT License](LICENSE).

---

<div align="center">

*This is an unofficial client library. For official Shopify documentation, visit [shopify.dev](https://shopify.dev/).*

</div>
