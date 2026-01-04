# Shopify Rust Client

A Rust client library for interacting with the Shopify Admin API. Currently focused on order management with plans to support all Shopify Admin APIs (REST and GraphQL).

## Features

### Current Support

- 🔍 **Order Retrieval**: Fetch orders by ID or order name
- ✏️ **Order Updates**: Update order properties (e.g., tags)
- 📦 **Type-Safe**: Strongly typed models for Shopify API responses
- 🚀 **Async/Await**: Built on `reqwest` for asynchronous HTTP requests
- 🔐 **Secure**: Token-based authentication support

### Roadmap

- 🚧 **Full Admin REST API**: Support for Products, Customers, Inventory, Fulfillments, and more
- 🚧 **GraphQL Admin API**: Complete GraphQL API support with query builder
- 🚧 **Webhook Support**: Handle and validate Shopify webhooks
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
use shopify_client::services::order::types::{PatchOrderRequest, PatchOrder};

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

## Project Structure

The library is organized into a modular structure for easy extensibility:

```
src/
├── lib.rs                  # Main client entry point
├── common/                 # Shared utilities and types
│   ├── types.rs           # Common types (APIError, ErrorResp)
│   └── utils.rs           # Utility functions
└── services/              # API service modules
    └── order/             # Order service
        ├── mod.rs         # Order struct with public methods
        ├── remote.rs      # Internal API implementation
        └── types.rs       # Order-specific types
```

### Design Philosophy

- **Client-Based**: Initialize a `ShopifyClient` once and access services through it
- **Service-Oriented**: Each API resource (orders, products, etc.) is its own service module
- **Type Safety**: All API responses are strongly typed with comprehensive models
- **Separation of Concerns**: Public API in `mod.rs`, implementation details in `remote.rs`

## Data Models

The library provides strongly-typed models organized by service:

### Order Types (`services::order::types`)

- **Order**: Complete order information including customer, line items, fulfillments, pricing, and timestamps
- **Customer**: Customer details (ID, email, phone, name, addresses)
- **LineItem**: Product line items with quantities, prices, properties, and fulfillment status
- **OrderFulfillment**: Fulfillment tracking with tracking numbers, URLs, and shipment status
- **Address**: Billing and shipping address information
- **PriceSet**: Multi-currency pricing with shop and presentment money
- **Property**: Custom line item properties
- **PatchOrderRequest** / **PatchOrder**: Request types for updating orders

### Common Types (`common::types`)

- **APIError**: Error handling enum for server errors, parsing failures, and network issues
- **ErrorResp**: Shopify API error response structure
- **WebhookResponse**: Webhook handling response

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
