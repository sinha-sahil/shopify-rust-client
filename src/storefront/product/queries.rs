use std::sync::OnceLock;

use crate::storefront::fragments::{IMAGE_FRAGMENT, MONEY_FRAGMENT};

const PRODUCT_VARIANT_FRAGMENT: &str = r#"
  fragment ProductVariantFields on ProductVariant {
    id
    title
    availableForSale
    quantityAvailable
    sku
    barcode
    weight
    weightUnit
    requiresShipping
    price {
      ...MoneyFields
    }
    compareAtPrice {
      ...MoneyFields
    }
    image {
      ...ImageFields
    }
    selectedOptions {
      name
      value
    }
  }
"#;

const PRODUCT_FRAGMENT: &str = r#"
  fragment ProductFields on Product {
    id
    handle
    title
    description
    descriptionHtml
    vendor
    productType
    tags
    availableForSale
    createdAt
    updatedAt
    publishedAt
    onlineStoreUrl
    seo {
      title
      description
    }
    priceRange {
      minVariantPrice {
        ...MoneyFields
      }
      maxVariantPrice {
        ...MoneyFields
      }
    }
    compareAtPriceRange {
      minVariantPrice {
        ...MoneyFields
      }
      maxVariantPrice {
        ...MoneyFields
      }
    }
    featuredImage {
      ...ImageFields
    }
    images(first: 10) {
      nodes {
        ...ImageFields
      }
    }
    options {
      id
      name
      optionValues {
        id
        name
      }
    }
    variants(first: 100) {
      nodes {
        ...ProductVariantFields
      }
      pageInfo {
        hasNextPage
        hasPreviousPage
        startCursor
        endCursor
      }
    }
  }
"#;

const GET_PRODUCT_BY_ID: &str = r#"
  query GetProductById($id: ID!) {
    product(id: $id) {
      ...ProductFields
    }
  }
"#;

const GET_PRODUCT_BY_HANDLE: &str = r#"
  query GetProductByHandle($handle: String!) {
    product(handle: $handle) {
      ...ProductFields
    }
  }
"#;

const GET_PRODUCTS: &str = r#"
  query GetProducts(
    $first: Int
    $after: String
    $last: Int
    $before: String
    $reverse: Boolean
    $sortKey: ProductSortKeys
    $query: String
  ) {
    products(
      first: $first
      after: $after
      last: $last
      before: $before
      reverse: $reverse
      sortKey: $sortKey
      query: $query
    ) {
      nodes {
        ...ProductFields
      }
      pageInfo {
        hasNextPage
        hasPreviousPage
        startCursor
        endCursor
      }
    }
  }
"#;

const GET_PRODUCT_RECOMMENDATIONS: &str = r#"
  query GetProductRecommendations($productId: ID!, $intent: ProductRecommendationIntent) {
    productRecommendations(productId: $productId, intent: $intent) {
      ...ProductFields
    }
  }
"#;

pub fn get_product_by_id() -> &'static str {
    static Q: OnceLock<String> = OnceLock::new();
    Q.get_or_init(|| {
        [
            MONEY_FRAGMENT,
            IMAGE_FRAGMENT,
            PRODUCT_VARIANT_FRAGMENT,
            PRODUCT_FRAGMENT,
            GET_PRODUCT_BY_ID,
        ]
        .concat()
    })
}

pub fn get_product_by_handle() -> &'static str {
    static Q: OnceLock<String> = OnceLock::new();
    Q.get_or_init(|| {
        [
            MONEY_FRAGMENT,
            IMAGE_FRAGMENT,
            PRODUCT_VARIANT_FRAGMENT,
            PRODUCT_FRAGMENT,
            GET_PRODUCT_BY_HANDLE,
        ]
        .concat()
    })
}

pub fn get_products() -> &'static str {
    static Q: OnceLock<String> = OnceLock::new();
    Q.get_or_init(|| {
        [
            MONEY_FRAGMENT,
            IMAGE_FRAGMENT,
            PRODUCT_VARIANT_FRAGMENT,
            PRODUCT_FRAGMENT,
            GET_PRODUCTS,
        ]
        .concat()
    })
}

pub fn get_product_recommendations() -> &'static str {
    static Q: OnceLock<String> = OnceLock::new();
    Q.get_or_init(|| {
        [
            MONEY_FRAGMENT,
            IMAGE_FRAGMENT,
            PRODUCT_VARIANT_FRAGMENT,
            PRODUCT_FRAGMENT,
            GET_PRODUCT_RECOMMENDATIONS,
        ]
        .concat()
    })
}
