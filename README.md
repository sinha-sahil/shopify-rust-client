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

### Get Order by ID

```rust
use shopify_client::remote::get_order_with_id;

#[tokio::main]
async fn main() {
    let shop_url = "https://your-shop.myshopify.com".to_string();
    let order_id = "1234567890".to_string();
    let access_token = "your-access-token".to_string();

    match get_order_with_id(&shop_url, &order_id, &access_token).await {
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
use shopify_client::remote::get_order_with_name;

#[tokio::main]
async fn main() {
    let shop_url = "https://your-shop.myshopify.com".to_string();
    let order_name = "1001".to_string(); // Without the # prefix
    let access_token = "your-access-token".to_string();

    match get_order_with_name(&shop_url, &order_name, &access_token).await {
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
use shopify_client::remote::patch_order;
use shopify_client::types::{PatchOrderRequest, PatchOrder};

#[tokio::main]
async fn main() {
    let shop_url = "https://your-shop.myshopify.com".to_string();
    let order_id = "1234567890".to_string();
    let access_token = "your-access-token".to_string();

    let patch_request = PatchOrderRequest {
        order: PatchOrder {
            tags: vec!["processed".to_string(), "priority".to_string()],
        },
    };

    match patch_order(&shop_url, &order_id, &access_token, &patch_request).await {
        Ok(response) => {
            println!("Order updated: {:?}", response.order);
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
        }
    }
}
```

## Data Models

The library provides strongly-typed models for Shopify API responses:

- **Order**: Complete order information including customer, line items, fulfillments, and pricing
- **Customer**: Customer details (ID, email, phone, name)
- **LineItem**: Product line items with quantities, prices, and properties
- **Fulfillment**: Fulfillment tracking information
- **PriceSet**: Multi-currency pricing information

## Error Handling

The library uses a custom `APIError` enum for error handling:

```rust
pub enum APIError {
    ServerError { errors: String },
    FailedToParse,
    NetworkError,
}
```

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
