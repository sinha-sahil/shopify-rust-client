use std::sync::OnceLock;

use crate::storefront::fragments::{IMAGE_FRAGMENT, MONEY_FRAGMENT};

const COLLECTION_FRAGMENT: &str = r#"
  fragment CollectionFields on Collection {
    id
    handle
    title
    description
    descriptionHtml
    updatedAt
    onlineStoreUrl
    seo {
      title
      description
    }
    image {
      ...ImageFields
    }
  }
"#;

const COLLECTION_PRODUCT_VARIANT_FRAGMENT: &str = r#"
  fragment ProductVariantFields on ProductVariant {
    id
    title
    availableForSale
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

const COLLECTION_PRODUCT_FRAGMENT: &str = r#"
  fragment ProductFields on Product {
    id
    handle
    title
    description
    vendor
    productType
    tags
    availableForSale
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
    seo {
      title
      description
    }
    featuredImage {
      ...ImageFields
    }
    images(first: 5) {
      nodes {
        ...ImageFields
      }
    }
    variants(first: 10) {
      nodes {
        ...ProductVariantFields
      }
    }
  }
"#;

const GET_COLLECTION_BY_ID: &str = r#"
  query GetCollectionById($id: ID!) {
    collection(id: $id) {
      ...CollectionFields
    }
  }
"#;

const GET_COLLECTION_BY_HANDLE: &str = r#"
  query GetCollectionByHandle($handle: String!) {
    collection(handle: $handle) {
      ...CollectionFields
    }
  }
"#;

const GET_COLLECTION_WITH_PRODUCTS: &str = r#"
  query GetCollectionWithProducts(
    $id: ID
    $handle: String
    $first: Int
    $after: String
    $last: Int
    $before: String
    $reverse: Boolean
    $sortKey: ProductCollectionSortKeys
    $filters: [ProductFilter!]
  ) {
    collection(id: $id, handle: $handle) {
      ...CollectionFields
      products(
        first: $first
        after: $after
        last: $last
        before: $before
        reverse: $reverse
        sortKey: $sortKey
        filters: $filters
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
        filters {
          id
          label
          type
          values {
            id
            label
            count
            input
          }
        }
      }
    }
  }
"#;

const GET_COLLECTIONS: &str = r#"
  query GetCollections(
    $first: Int
    $after: String
    $last: Int
    $before: String
    $reverse: Boolean
    $sortKey: CollectionSortKeys
    $query: String
  ) {
    collections(
      first: $first
      after: $after
      last: $last
      before: $before
      reverse: $reverse
      sortKey: $sortKey
      query: $query
    ) {
      nodes {
        ...CollectionFields
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

pub fn get_collection_by_id() -> &'static str {
    static Q: OnceLock<String> = OnceLock::new();
    Q.get_or_init(|| [IMAGE_FRAGMENT, COLLECTION_FRAGMENT, GET_COLLECTION_BY_ID].concat())
}

pub fn get_collection_by_handle() -> &'static str {
    static Q: OnceLock<String> = OnceLock::new();
    Q.get_or_init(|| {
        [
            IMAGE_FRAGMENT,
            COLLECTION_FRAGMENT,
            GET_COLLECTION_BY_HANDLE,
        ]
        .concat()
    })
}

pub fn get_collection_with_products() -> &'static str {
    static Q: OnceLock<String> = OnceLock::new();
    Q.get_or_init(|| {
        [
            MONEY_FRAGMENT,
            IMAGE_FRAGMENT,
            COLLECTION_FRAGMENT,
            COLLECTION_PRODUCT_VARIANT_FRAGMENT,
            COLLECTION_PRODUCT_FRAGMENT,
            GET_COLLECTION_WITH_PRODUCTS,
        ]
        .concat()
    })
}

pub fn get_collections() -> &'static str {
    static Q: OnceLock<String> = OnceLock::new();
    Q.get_or_init(|| [IMAGE_FRAGMENT, COLLECTION_FRAGMENT, GET_COLLECTIONS].concat())
}
