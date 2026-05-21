use std::sync::OnceLock;

use crate::storefront::fragments::{IMAGE_FRAGMENT, MONEY_FRAGMENT};

const SEARCH_PRODUCT_FRAGMENT: &str = r#"
  fragment SearchProductFields on Product {
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
    variants(first: 5) {
      nodes {
        id
        title
        availableForSale
        price {
          ...MoneyFields
        }
      }
    }
  }
"#;

const SEARCH_COLLECTION_FRAGMENT: &str = r#"
  fragment SearchCollectionFields on Collection {
    id
    handle
    title
    description
    seo {
      title
      description
    }
    image {
      ...ImageFields
    }
  }
"#;

const SEARCH_PAGE_FRAGMENT: &str = r#"
  fragment SearchPageFields on Page {
    id
    handle
    title
    body
    bodySummary
  }
"#;

const SEARCH_ARTICLE_FRAGMENT: &str = r#"
  fragment SearchArticleFields on Article {
    id
    handle
    title
    excerpt
    blog {
      id
      title
      handle
    }
    image {
      ...ImageFields
    }
  }
"#;

// SearchResultItem union covers Article | Page | Product only (NOT Collection).
const SEARCH: &str = r#"
  query Search(
    $query: String!
    $first: Int
    $after: String
    $last: Int
    $before: String
    $reverse: Boolean
    $sortKey: SearchSortKeys
    $types: [SearchType!]
    $productFilters: [ProductFilter!]
    $prefix: SearchPrefixQueryType
    $unavailableProducts: SearchUnavailableProductsType
  ) {
    search(
      query: $query
      first: $first
      after: $after
      last: $last
      before: $before
      reverse: $reverse
      sortKey: $sortKey
      types: $types
      productFilters: $productFilters
      prefix: $prefix
      unavailableProducts: $unavailableProducts
    ) {
      nodes {
        __typename
        ... on Product {
          ...SearchProductFields
        }
        ... on Page {
          ...SearchPageFields
        }
        ... on Article {
          ...SearchArticleFields
        }
      }
      pageInfo {
        hasNextPage
        hasPreviousPage
        startCursor
        endCursor
      }
      totalCount
      productFilters {
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
"#;

const PREDICTIVE_SEARCH: &str = r#"
  query PredictiveSearch(
    $query: String!
    $limit: Int
    $limitScope: PredictiveSearchLimitScope
    $types: [PredictiveSearchType!]
    $unavailableProducts: SearchUnavailableProductsType
  ) {
    predictiveSearch(
      query: $query
      limit: $limit
      limitScope: $limitScope
      types: $types
      unavailableProducts: $unavailableProducts
    ) {
      products {
        ...SearchProductFields
      }
      collections {
        ...SearchCollectionFields
      }
      pages {
        ...SearchPageFields
      }
      articles {
        ...SearchArticleFields
      }
      queries {
        text
        styledText
      }
    }
  }
"#;

pub fn search() -> &'static str {
    static Q: OnceLock<String> = OnceLock::new();
    Q.get_or_init(|| {
        [
            MONEY_FRAGMENT,
            IMAGE_FRAGMENT,
            SEARCH_PRODUCT_FRAGMENT,
            SEARCH_PAGE_FRAGMENT,
            SEARCH_ARTICLE_FRAGMENT,
            SEARCH,
        ]
        .concat()
    })
}

pub fn predictive_search() -> &'static str {
    static Q: OnceLock<String> = OnceLock::new();
    Q.get_or_init(|| {
        [
            MONEY_FRAGMENT,
            IMAGE_FRAGMENT,
            SEARCH_PRODUCT_FRAGMENT,
            SEARCH_COLLECTION_FRAGMENT,
            SEARCH_PAGE_FRAGMENT,
            SEARCH_ARTICLE_FRAGMENT,
            PREDICTIVE_SEARCH,
        ]
        .concat()
    })
}
