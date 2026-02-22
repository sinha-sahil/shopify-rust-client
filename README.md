<div align="center">

# shopify-client

**A type-safe, async Rust client for the Shopify Admin API**

[![Crates.io](https://img.shields.io/crates/v/shopify-client.svg)](https://crates.io/crates/shopify-client)
[![Documentation](https://docs.rs/shopify-client/badge.svg)](https://docs.rs/shopify-client)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Shopify API](https://img.shields.io/badge/Shopify%20API-2026--01-green.svg)](https://shopify.dev/docs/api/admin)

[Getting Started](#getting-started) &bull; [API Coverage](#api-coverage) &bull; [Examples](#examples) &bull; [Bulk Operations](#bulk-operations) &bull; [Docs](https://docs.rs/shopify-client)

</div>

---

```rust
let client = ShopifyClient::new(shop_url, access_token, None);

// One client, every API
let order    = client.order.get_with_id(&id).await?;
let products = client.bulk_operation.export_products(None).await?;
let shop     = client.shop.get().await?;
```

## Highlights

| | |
|---|---|
| **9 service modules** | Orders, Subscriptions, Discounts, Cart Transforms, App Installation, Shopify Functions, Shop, Storefront Tokens, Bulk Operations |
| **Bulk Operations** | Prebuilt export templates for products, orders, collections, customers, inventory, and draft orders with typed JSONL parsing |
| **Typed everything** | Strongly typed requests, responses, filters, and JSONL export lines &mdash; no raw strings |
| **Async / await** | Built on `reqwest` with `tokio` &mdash; non-blocking by default |
| **Webhook parsing** | HMAC-verified parsing for customer data, customer redact, and shop redact compliance webhooks |
| **Request callbacks** | Optional before/after hooks for logging, metrics, and observability |

## Getting Started

Add to your `Cargo.toml`:

```toml
[dependencies]
shopify-client = "0.16.0"
```

```rust
use shopify_client::ShopifyClient;

#[tokio::main]
async fn main() {
    let client = ShopifyClient::new(
        "https://your-shop.myshopify.com".to_string(),
        "your-access-token".to_string(),
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
| **`client.shop`** | GraphQL | `get` |
| **`client.storefront_access_token`** | GraphQL | `create`, `delete` |
| **`client.bulk_operation`** | GraphQL | `run_query`, `run_mutation`, `cancel`, `get`, `list`, `create_staged_upload`, `export_*`, `stream_*` |

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
println!("{} ({})", resp.shop.name, resp.shop.plan.display_name);
println!("Owner: {}", resp.shop.account_owner.email);
```

<details>
<summary><strong>More examples: Discounts, Cart Transforms, Metafields</strong></summary>

#### Create Automatic Discount

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

#### Create Cart Transform

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

#### Update Metafields with CAS

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

## Request Callbacks

Add observability with before/after hooks on every request:

```rust
use shopify_client::ShopifyClient;
use std::sync::Arc;

let client = ShopifyClient::new_with_callbacks(
    shop_url, access_token, None,
    Some(Arc::new(|url, body, _headers| {
        println!("-> {} ({} bytes)", url, body.map_or(0, |b| b.len()));
    })),
    Some(Arc::new(|url, resp, _headers| {
        println!("<- {} ({} bytes)", url, resp.len());
    })),
);
```

- Fires on every REST and GraphQL request
- Access token is **never** passed to callbacks
- Panic-safe &mdash; callback panics won't crash your app

## Error Handling

All methods return `Result<T, APIError>`:

```rust
pub enum APIError {
    ServerError { errors: String },  // Shopify returned an error
    FailedToParse,                   // Response couldn't be deserialized
    NetworkError,                    // Connection / timeout failure
}
```

## Project Structure

```
src/
├── lib.rs                       # ShopifyClient entry point
├── types/                       # Public type definitions
│   ├── order.rs                 #   Order types + query enums (OrderStatus, ...)
│   ├── product.rs               #   ProductStatus, ProductQueryParams
│   ├── collection.rs            #   CollectionType, CollectionQueryParams
│   ├── customer.rs              #   CustomerAccountState, CustomerQueryParams
│   ├── inventory.rs             #   InventoryItemQueryParams
│   ├── draft_order.rs           #   DraftOrderFilterStatus, DraftOrderQueryParams
│   ├── subscription.rs          #   Subscription requests & responses
│   ├── discount.rs              #   Discount inputs & responses
│   ├── app_installation.rs      #   Metafield management types
│   ├── cart_transform.rs        #   Cart transform inputs & CAS metafields
│   ├── shop.rs                  #   Shop, StaffMember, ShopPlan, ...
│   ├── shopify_functions.rs     #   Function listing types
│   ├── storefront_access_token.rs  # Token create/delete types
│   └── bulk_operation.rs        #   Bulk ops: core types, JSONL export types
├── services/                    # Internal service implementations
│   ├── order/                   #   REST: get, patch
│   ├── subscription/            #   GraphQL: create, cancel, extend, ...
│   ├── discount/                #   GraphQL: create, update, list
│   ├── app_installation/        #   GraphQL: metafields CRUD
│   ├── cart_transform/          #   GraphQL: create, set metafields
│   ├── shop/                    #   GraphQL: get shop info
│   ├── shopify_functions/       #   GraphQL: list functions
│   ├── storefront_access_token/ #   GraphQL: create, delete tokens
│   └── bulk_operation/          #   GraphQL: queries, mutations, exports
├── webhooks/                    #   Webhook parsing & types
└── common/                      #   Shared HTTP, error types, query filters
```

## Requirements

- **Rust** 2021 edition (1.56+)
- **Dependencies:** `reqwest`, `serde`, `serde_json`, `hmac`, `sha2`, `base64`

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the [MIT License](LICENSE).

---

<div align="center">

*This is an unofficial client library. For official Shopify documentation, visit [shopify.dev](https://shopify.dev/).*

</div>
