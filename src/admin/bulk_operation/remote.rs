use crate::common::ServiceContext;
use crate::{
    common::{http::execute_graphql, types::APIError},
    types::bulk_operation::{
        CancelResp, GetBulkOperationResp, ListBulkOperationsParams, ListBulkOperationsResp,
        RunMutationResp, RunQueryResp, StagedUploadInput, StagedUploadsCreateResp,
    },
};

use serde_json::json;

// region: Core Operations

pub async fn run_query(
    ctx: &ServiceContext,
    query_string: &str,
    group_objects: Option<bool>,
) -> Result<RunQueryResp, APIError> {
    let query = r#"
        mutation bulkOperationRunQuery($query: String!) {
            bulkOperationRunQuery(query: $query) {
                bulkOperation {
                    id
                    status
                    errorCode
                    createdAt
                    objectCount
                    rootObjectCount
                    url
                    query
                    type
                }
                userErrors {
                    code
                    field
                    message
                }
            }
        }
    "#;

    let mut query_with_group = query_string.to_string();
    if let Some(true) = group_objects {
        // Wrap the query to enable object grouping if not already present
        if !query_with_group.contains("groupObjects") {
            query_with_group = query_string.to_string();
        }
    }

    let variables = json!({
        "query": query_with_group
    });

    execute_graphql(ctx, query, variables).await
}

pub async fn run_mutation(
    ctx: &ServiceContext,
    mutation: &str,
    staged_upload_path: &str,
    client_identifier: Option<&str>,
) -> Result<RunMutationResp, APIError> {
    let query = r#"
        mutation bulkOperationRunMutation($mutation: String!, $stagedUploadPath: String!, $clientIdentifier: String) {
            bulkOperationRunMutation(mutation: $mutation, stagedUploadPath: $stagedUploadPath, clientIdentifier: $clientIdentifier) {
                bulkOperation {
                    id
                    status
                    errorCode
                    createdAt
                    objectCount
                    rootObjectCount
                    url
                    type
                }
                userErrors {
                    code
                    field
                    message
                }
            }
        }
    "#;

    let variables = json!({
        "mutation": mutation,
        "stagedUploadPath": staged_upload_path,
        "clientIdentifier": client_identifier
    });

    execute_graphql(ctx, query, variables).await
}

pub async fn cancel(ctx: &ServiceContext, id: &str) -> Result<CancelResp, APIError> {
    let query = r#"
        mutation bulkOperationCancel($id: ID!) {
            bulkOperationCancel(id: $id) {
                bulkOperation {
                    id
                    status
                    errorCode
                }
                userErrors {
                    field
                    message
                }
            }
        }
    "#;

    let variables = json!({ "id": id });

    execute_graphql(ctx, query, variables).await
}

pub async fn get(ctx: &ServiceContext, id: &str) -> Result<GetBulkOperationResp, APIError> {
    let query = format!(
        r#"
        query {{
            node(id: "{}") {{
                ... on BulkOperation {{
                    id
                    status
                    errorCode
                    createdAt
                    completedAt
                    objectCount
                    rootObjectCount
                    fileSize
                    url
                    partialDataUrl
                    query
                    type
                }}
            }}
        }}
    "#,
        id
    );

    let variables = json!({});

    execute_graphql(ctx, &query, variables).await
}

pub async fn list(
    ctx: &ServiceContext,
    params: &ListBulkOperationsParams,
) -> Result<ListBulkOperationsResp, APIError> {
    let mut args = Vec::new();

    if let Some(first) = params.first {
        args.push(format!("first: {}", first));
    }
    if let Some(after) = &params.after {
        args.push(format!("after: \"{}\"", after));
    }
    if let Some(last) = params.last {
        args.push(format!("last: {}", last));
    }
    if let Some(before) = &params.before {
        args.push(format!("before: \"{}\"", before));
    }
    if let Some(reverse) = params.reverse {
        args.push(format!("reverse: {}", reverse));
    }
    if let Some(sort_key) = &params.sort_key {
        let key_str = serde_json::to_string(sort_key).unwrap_or_default();
        args.push(format!("sortKey: {}", key_str.trim_matches('"')));
    }

    let mut query_filters = Vec::new();
    if let Some(status) = &params.status {
        let status_str = serde_json::to_string(status).unwrap_or_default();
        query_filters.push(format!("status:{}", status_str.trim_matches('"')));
    }
    if let Some(op_type) = &params.operation_type {
        let type_str = serde_json::to_string(op_type).unwrap_or_default();
        query_filters.push(format!("type:{}", type_str.trim_matches('"')));
    }
    if !query_filters.is_empty() {
        args.push(format!("query: \"{}\"", query_filters.join(" ")));
    }

    let args_str = if args.is_empty() {
        String::new()
    } else {
        format!("({})", args.join(", "))
    };

    let query = format!(
        r#"
        query {{
            bulkOperations{} {{
                nodes {{
                    id
                    status
                    errorCode
                    createdAt
                    completedAt
                    objectCount
                    rootObjectCount
                    fileSize
                    url
                    query
                    type
                }}
                pageInfo {{
                    hasNextPage
                    hasPreviousPage
                    startCursor
                    endCursor
                }}
            }}
        }}
    "#,
        args_str
    );

    let variables = json!({});

    execute_graphql(ctx, &query, variables).await
}

pub async fn create_staged_upload(
    ctx: &ServiceContext,
    input: &[StagedUploadInput],
) -> Result<StagedUploadsCreateResp, APIError> {
    let query = r#"
        mutation stagedUploadsCreate($input: [StagedUploadInput!]!) {
            stagedUploadsCreate(input: $input) {
                stagedTargets {
                    url
                    resourceUrl
                    parameters {
                        name
                        value
                    }
                }
                userErrors {
                    field
                    message
                }
            }
        }
    "#;

    let variables = json!({ "input": input });

    execute_graphql(ctx, query, variables).await
}

// endregion

// region: Export Template Queries

fn build_export_query(resource: &str, query_body: &str, filter: Option<&str>) -> String {
    let mut args = Vec::new();

    if resource == "inventoryItems" {
        args.push("first: 1".to_string());
    }
    if let Some(f) = filter {
        args.push(format!("query: \"{}\"", f));
    }

    let args_str = if args.is_empty() {
        String::new()
    } else {
        format!("({})", args.join(", "))
    };

    format!(
        r#"{{
  {}{} {{
    edges {{
      node {{
        {}
      }}
    }}
  }}
}}"#,
        resource, args_str, query_body
    )
}

pub fn products_query(filter: Option<&str>) -> String {
    let body = r#"id
        title
        handle
        descriptionHtml
        status
        vendor
        productType
        tags
        isGiftCard
        hasOnlyDefaultVariant
        templateSuffix
        onlineStoreUrl
        createdAt
        updatedAt
        publishedAt
        seo {
          title
          description
        }
        priceRangeV2 {
          minVariantPrice {
            amount
            currencyCode
          }
          maxVariantPrice {
            amount
            currencyCode
          }
        }
        options {
          id
          name
          values
        }
        category {
          id
          name
          fullName
          isLeaf
          isRoot
          isArchived
          level
          parentId
          ancestorIds
          childrenIds
        }
        featuredMedia {
          preview {
            image {
              url
              altText
              width
              height
            }
          }
        }
        variants {
          edges {
            node {
              id
              title
              sku
              price
              compareAtPrice
              barcode
              position
              inventoryQuantity
              taxable
              taxCode
              availableForSale
              selectedOptions {
                name
                value
              }
              inventoryItem {
                id
                tracked
                requiresShipping
                unitCost {
                  amount
                  currencyCode
                }
              }
              image {
                url
                altText
                width
                height
              }
            }
          }
        }
        media {
          edges {
            node {
              mediaContentType
              status
              alt
              preview {
                image {
                  url
                  altText
                  width
                  height
                }
              }
              ... on MediaImage {
                id
                mimeType
                image {
                  url
                  altText
                  width
                  height
                }
              }
              ... on Video {
                id
                filename
                sources {
                  url
                  format
                  width
                  height
                  mimeType
                }
              }
              ... on ExternalVideo {
                id
                embedUrl
                host
              }
              ... on Model3d {
                id
                filename
                originalSource {
                  url
                  format
                  filesize
                }
              }
            }
          }
        }"#;

    build_export_query("products", body, filter)
}

pub fn orders_query(filter: Option<&str>) -> String {
    let body = r#"id
        name
        email
        phone
        note
        tags
        displayFinancialStatus
        displayFulfillmentStatus
        cancelledAt
        cancelReason
        closedAt
        createdAt
        updatedAt
        processedAt
        test
        confirmed
        taxesIncluded
        currencyCode
        presentmentCurrencyCode
        subtotalPriceSet {
          shopMoney { amount currencyCode }
          presentmentMoney { amount currencyCode }
        }
        totalPriceSet {
          shopMoney { amount currencyCode }
          presentmentMoney { amount currencyCode }
        }
        totalDiscountsSet {
          shopMoney { amount currencyCode }
          presentmentMoney { amount currencyCode }
        }
        totalTaxSet {
          shopMoney { amount currencyCode }
          presentmentMoney { amount currencyCode }
        }
        totalShippingPriceSet {
          shopMoney { amount currencyCode }
          presentmentMoney { amount currencyCode }
        }
        totalRefundedSet {
          shopMoney { amount currencyCode }
          presentmentMoney { amount currencyCode }
        }
        totalWeight
        customer {
          id
          email
          firstName
          lastName
        }
        shippingAddress {
          address1 address2 city province provinceCode
          country countryCodeV2 zip phone
          firstName lastName company name
          latitude longitude
        }
        billingAddress {
          address1 address2 city province provinceCode
          country countryCodeV2 zip phone
          firstName lastName company name
          latitude longitude
        }
        sourceName
        fulfillable
        requiresShipping
        riskLevel
        discountCodes
        lineItems {
          edges {
            node {
              id
              title
              name
              sku
              quantity
              variantTitle
              vendor
              product { id }
              variant { id }
              originalUnitPriceSet {
                shopMoney { amount currencyCode }
                presentmentMoney { amount currencyCode }
              }
              discountedUnitPriceSet {
                shopMoney { amount currencyCode }
                presentmentMoney { amount currencyCode }
              }
              discountedTotalSet {
                shopMoney { amount currencyCode }
                presentmentMoney { amount currencyCode }
              }
              totalDiscountSet {
                shopMoney { amount currencyCode }
                presentmentMoney { amount currencyCode }
              }
              taxLines {
                title
                rate
                ratePercentage
                priceSet {
                  shopMoney { amount currencyCode }
                  presentmentMoney { amount currencyCode }
                }
              }
              requiresShipping
              taxable
              fulfillableQuantity
              fulfillmentStatus
              customAttributes { key value }
              duties {
                id
                harmonizedSystemCode
                price {
                  shopMoney { amount currencyCode }
                  presentmentMoney { amount currencyCode }
                }
                taxLines {
                  title rate ratePercentage
                  priceSet {
                    shopMoney { amount currencyCode }
                    presentmentMoney { amount currencyCode }
                  }
                }
              }
            }
          }
        }"#;

    build_export_query("orders", body, filter)
}

pub fn collections_query(filter: Option<&str>) -> String {
    let body = r#"id
        title
        handle
        descriptionHtml
        sortOrder
        templateSuffix
        productsCount {
          count
          precision
        }
        updatedAt
        seo {
          title
          description
        }
        image {
          url
          altText
          width
          height
        }
        products {
          edges {
            node {
              id
              title
              handle
              status
              vendor
              productType
            }
          }
        }"#;

    build_export_query("collections", body, filter)
}

pub fn customers_query(filter: Option<&str>) -> String {
    let body = r#"id
        defaultEmailAddress {
          emailAddress
        }
        firstName
        lastName
        displayName
        defaultPhoneNumber {
          phoneNumber
        }
        note
        tags
        state
        taxExempt
        verifiedEmail
        locale
        numberOfOrders
        amountSpent {
          amount
          currencyCode
        }
        createdAt
        updatedAt
        defaultAddress {
          address1 address2 city province provinceCode
          country countryCodeV2 zip phone
          firstName lastName company name
          latitude longitude
        }
        image {
          url
          altText
          width
          height
        }
        addresses {
          id
          address1 address2 city province provinceCode
          country countryCodeV2 zip phone
          firstName lastName company name
        }"#;

    build_export_query("customers", body, filter)
}

pub fn inventory_items_query(filter: Option<&str>) -> String {
    let body = r#"id
        sku
        tracked
        requiresShipping
        countryCodeOfOrigin
        provinceCodeOfOrigin
        harmonizedSystemCode
        createdAt
        updatedAt
        unitCost {
          amount
          currencyCode
        }
        variant {
          id
        }
        inventoryLevels {
          edges {
            node {
              id
              quantities(names: ["available"]) {
                name
                quantity
              }
              location {
                id
                name
              }
              updatedAt
            }
          }
        }"#;

    build_export_query("inventoryItems", body, filter)
}

pub fn draft_orders_query(filter: Option<&str>) -> String {
    let body = r#"id
        name
        email
        phone
        note2
        tags
        status
        currencyCode
        taxExempt
        taxesIncluded
        createdAt
        updatedAt
        completedAt
        invoiceSentAt
        subtotalPriceSet {
          shopMoney { amount currencyCode }
          presentmentMoney { amount currencyCode }
        }
        totalPriceSet {
          shopMoney { amount currencyCode }
          presentmentMoney { amount currencyCode }
        }
        totalTaxSet {
          shopMoney { amount currencyCode }
          presentmentMoney { amount currencyCode }
        }
        totalDiscountsSet {
          shopMoney { amount currencyCode }
          presentmentMoney { amount currencyCode }
        }
        totalShippingPriceSet {
          shopMoney { amount currencyCode }
          presentmentMoney { amount currencyCode }
        }
        customer {
          id
          email
          firstName
          lastName
        }
        shippingAddress {
          address1 address2 city province provinceCode
          country countryCodeV2 zip phone
          firstName lastName company name
          latitude longitude
        }
        billingAddress {
          address1 address2 city province provinceCode
          country countryCodeV2 zip phone
          firstName lastName company name
          latitude longitude
        }
        order { id }
        lineItems {
          edges {
            node {
              id
              title
              name
              sku
              quantity
              variantTitle
              vendor
              product { id }
              variant { id }
              originalUnitPriceSet {
                shopMoney { amount currencyCode }
                presentmentMoney { amount currencyCode }
              }
              discountedUnitPriceSet {
                shopMoney { amount currencyCode }
                presentmentMoney { amount currencyCode }
              }
              discountedTotalSet {
                shopMoney { amount currencyCode }
                presentmentMoney { amount currencyCode }
              }
              totalDiscountSet {
                shopMoney { amount currencyCode }
                presentmentMoney { amount currencyCode }
              }
              requiresShipping
              taxable
              customAttributes { key value }
            }
          }
        }"#;

    build_export_query("draftOrders", body, filter)
}

// endregion
