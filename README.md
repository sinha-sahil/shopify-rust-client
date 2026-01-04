# Shopify Rust Client

A Rust client library for interacting with the Shopify Admin API. Currently focused on order management with plans to support all Shopify Admin APIs (REST and GraphQL).

## Features

### Current Support

- 🔍 **Order Retrieval**: Fetch orders by ID or order name
- ✏️ **Order Updates**: Update order properties (e.g., tags)
- 🪝 **Webhook Support**: Parse customer and shop compliance webhooks
- 📦 **Type-Safe**: Strongly typed models for Shopify API responses
- 🚀 **Async/Await**: Built on `reqwest` for asynchronous HTTP requests
- 🔐 **Secure**: Token-based authentication support

### Roadmap

- 🚧 **Full Admin REST API**: Support for Products, Customers, Inventory, Fulfillments, and more
- 🚧 **GraphQL Admin API**: Complete GraphQL API support with query builder
- 🚧 **Additional Webhooks**: Support for more webhook topics beyond compliance
- 🚧 **Rate Limiting**: Built-in request throttling and retry logic

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
shopify-client = "0.1.0"
```

Or install directly:

```bash
cargo add shopify-client
```

## Public API

The library exposes three main modules for public use:

- **`ShopifyClient`** - Main client for making API calls
- **`types`** - All type definitions organized by resource (e.g., `types::order`)
- **`webhooks`** - Webhook parsing utilities and types

```rust
use shopify_client::ShopifyClient;           // Main client
use shopify_client::types::order::*;         // Order types
use shopify_client::webhooks::*;             // Webhook utilities
```

## Usage

### Initialize the Client

```rust
use shopify_client::ShopifyClient;

let client = ShopifyClient::new(
    "https://your-shop.myshopify.com".to_string(),
    "your-access-token".to_string(),
    None, // Optional API version, defaults to "2024-07"
);
```

### Get Order by ID

```rust
use shopify_client::ShopifyClient;

#[tokio::main]
async fn main() {
    let client = ShopifyClient::new(
        "https://your-shop.myshopify.com".to_string(),
        "your-access-token".to_string(),
        None,
    );

    let order_id = "1234567890".to_string();

    match client.order.get_with_id(&order_id).await {
        Ok(response) => {
            println!("Order: {:?}", response.order);
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
        }
    }
}
```

### Get Order by Name

```rust
use shopify_client::ShopifyClient;

#[tokio::main]
async fn main() {
    let client = ShopifyClient::new(
        "https://your-shop.myshopify.com".to_string(),
        "your-access-token".to_string(),
        None,
    );

    let order_name = "1001".to_string(); // Without the # prefix

    match client.order.get_with_name(&order_name).await {
        Ok(response) => {
            println!("Orders found: {}", response.orders.len());
            for order in response.orders {
                println!("Order {}: {}", order.name, order.id);
            }
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
        }
    }
}
```

### Update Order (Patch)

```rust
use shopify_client::ShopifyClient;
use shopify_client::types::order::{PatchOrderRequest, PatchOrder};

#[tokio::main]
async fn main() {
    let client = ShopifyClient::new(
        "https://your-shop.myshopify.com".to_string(),
        "your-access-token".to_string(),
        None,
    );

    let order_id = "1234567890".to_string();

    let patch_request = PatchOrderRequest {
        order: PatchOrder {
            tags: vec!["processed".to_string(), "priority".to_string()],
        },
    };

    match client.order.patch(&order_id, &patch_request).await {
        Ok(response) => {
            println!("Order updated: {:?}", response.order);
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
        }
    }
}
```

### Parse Webhooks

```rust
use shopify_client::webhooks::{parse_webhook_with_header, WebhookPayload};

fn handle_webhook(topic_header: &str, payload: &str) {
    match parse_webhook_with_header(topic_header, payload) {
        Ok(WebhookPayload::CustomersDataRequest(data)) => {
            println!("Customer data request for shop: {}", data.shop_domain);
            println!("Customer: {:?}", data.customer);
            println!("Orders requested: {:?}", data.orders_requested);
        }
        Ok(WebhookPayload::CustomersRedact(data)) => {
            println!("Customer redact request for shop: {}", data.shop_domain);
            println!("Customer to redact: {:?}", data.customer);
            println!("Orders to redact: {:?}", data.orders_to_redact);
        }
        Ok(WebhookPayload::ShopRedact(data)) => {
            println!("Shop redact request for shop: {}", data.shop_domain);
            println!("Shop ID: {}", data.shop_id);
        }
        Err(e) => {
            eprintln!("Failed to parse webhook: {:?}", e);
        }
    }
}
```

## Project Structure

The library is organized into a modular structure for easy extensibility:

```
src/
├── lib.rs                  # Main client entry point and exports
├── types/                  # Public type definitions
│   ├── mod.rs             # Type module exports
│   └── order.rs           # Order-related types
├── webhooks/              # Public webhook parsing module
│   ├── mod.rs             # Webhook parsing functions
│   └── types.rs           # Webhook payload types
├── common/                # Internal shared utilities (private)
│   ├── mod.rs
│   ├── types.rs           # Common types (APIError, ErrorResp)
│   └── utils.rs           # Utility functions
└── services/              # Internal API services (private)
    └── order/             # Order service implementation
        ├── mod.rs         # Order struct with public methods
        └── remote.rs      # Internal API implementation
```

### Design Philosophy

- **Client-Based**: Initialize a `ShopifyClient` once and access services through it
- **Clean Public API**: Three public modules - `ShopifyClient`, `types`, and `webhooks`
- **Encapsulation**: All internal implementation (`common`, `services`) is private
- **Service-Oriented**: Each API resource (orders, products, etc.) is its own service module
- **Clear Separation**: Services for outgoing API calls, webhooks for incoming data parsing
- **Type Safety**: All API responses are strongly typed with comprehensive models

## Data Models

The library provides strongly-typed models organized by resource:

### Order Types (`types::order`)

- **Order**: Complete order information including customer, line items, fulfillments, pricing, and timestamps
- **Customer**: Customer details (ID, email, phone, name, addresses)
- **LineItem**: Product line items with quantities, prices, properties, and fulfillment status
- **OrderFulfillment**: Fulfillment tracking with tracking numbers, URLs, and shipment status
- **Address**: Billing and shipping address information
- **PriceSet**: Multi-currency pricing with shop and presentment money
- **Property**: Custom line item properties
- **PatchOrderRequest** / **PatchOrder**: Request types for updating orders

### Webhook Types (`webhooks::types`)

- **CustomersDataRequestPayload**: Customer data request webhook payload
- **CustomersRedactPayload**: Customer redaction webhook payload
- **ShopRedactPayload**: Shop redaction webhook payload
- **WebhookPayload**: Enum containing all webhook payload types
- **WebhookParseError**: Error type for webhook parsing failures
- **WebhookCustomer**: Customer information in webhook payloads

## Error Handling

The library uses a custom `APIError` enum for comprehensive error handling:

```rust
pub enum APIError {
    ServerError { errors: String },
    FailedToParse,
    NetworkError,
}
```

Errors are returned through `Result<T, APIError>` for easy error propagation and handling.

## Authentication

This client requires a Shopify Admin API access token. You can obtain one by:

1. Creating a custom app in your Shopify admin panel
2. Generating an Admin API access token
3. Granting the necessary permissions (e.g., `read_orders`, `write_orders`)

## API Version

Currently uses Shopify Admin API version **2024-07**.

## Requirements

- Rust 1.56 or later (2021 edition)
- Dependencies:
  - `reqwest` (with JSON support)
  - `serde` (with derive feature)
  - `serde_json`

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under your preferred license.

## Disclaimer

This is an unofficial Shopify client library. For official SDKs and documentation, visit [Shopify's Developer Documentation](https://shopify.dev/).
