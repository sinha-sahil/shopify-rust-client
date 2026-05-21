use std::sync::OnceLock;

// MONEY_FRAGMENT is brought in only where Customer/Order types actually reference MoneyV2.

const ADDRESS_FRAGMENT: &str = r#"
  fragment AddressFields on MailingAddress {
    id
    firstName
    lastName
    name
    company
    address1
    address2
    city
    province
    provinceCode
    country
    countryCodeV2
    zip
    phone
    formatted
    formattedArea
    latitude
    longitude
  }
"#;

const ORDER_FRAGMENT: &str = r#"
  fragment OrderFields on Order {
    id
    name
    orderNumber
    processedAt
    canceledAt
    cancelReason
    fulfillmentStatus
    financialStatus
    statusUrl
    email
    phone
    currencyCode
    currentTotalPrice {
      ...MoneyFields
    }
    currentSubtotalPrice {
      ...MoneyFields
    }
    currentTotalTax {
      ...MoneyFields
    }
    totalPrice {
      ...MoneyFields
    }
    subtotalPrice {
      ...MoneyFields
    }
    totalShippingPrice {
      ...MoneyFields
    }
    totalTax {
      ...MoneyFields
    }
    totalRefunded {
      ...MoneyFields
    }
    shippingAddress {
      ...AddressFields
    }
    lineItems(first: 50) {
      edges {
        node {
          title
          quantity
          currentQuantity
          originalTotalPrice {
            ...MoneyFields
          }
          discountedTotalPrice {
            ...MoneyFields
          }
          variant {
            id
            title
            sku
            image {
              url
              altText
            }
            product {
              id
              handle
              title
            }
          }
        }
      }
    }
  }
"#;

const CUSTOMER_FRAGMENT: &str = r#"
  fragment CustomerFields on Customer {
    id
    email
    firstName
    lastName
    displayName
    phone
    acceptsMarketing
    createdAt
    updatedAt
    numberOfOrders
    tags
    defaultAddress {
      ...AddressFields
    }
  }
"#;

const CUSTOMER_USER_ERROR_FRAGMENT: &str = r#"
  fragment UserErrorFields on CustomerUserError {
    field
    message
    code
  }
"#;

const CUSTOMER_ACCESS_TOKEN_CREATE: &str = r#"
  mutation CustomerAccessTokenCreate($input: CustomerAccessTokenCreateInput!) {
    customerAccessTokenCreate(input: $input) {
      customerAccessToken {
        accessToken
        expiresAt
      }
      customerUserErrors {
        ...UserErrorFields
      }
    }
  }
"#;

const CUSTOMER_ACCESS_TOKEN_RENEW: &str = r#"
  mutation CustomerAccessTokenRenew($customerAccessToken: String!) {
    customerAccessTokenRenew(customerAccessToken: $customerAccessToken) {
      customerAccessToken {
        accessToken
        expiresAt
      }
      userErrors {
        field
        message
      }
    }
  }
"#;

const CUSTOMER_ACCESS_TOKEN_DELETE: &str = r#"
  mutation CustomerAccessTokenDelete($customerAccessToken: String!) {
    customerAccessTokenDelete(customerAccessToken: $customerAccessToken) {
      deletedAccessToken
      deletedCustomerAccessTokenId
      userErrors {
        field
        message
      }
    }
  }
"#;

const CUSTOMER_CREATE: &str = r#"
  mutation CustomerCreate($input: CustomerCreateInput!) {
    customerCreate(input: $input) {
      customer {
        id
        email
        firstName
        lastName
      }
      customerUserErrors {
        ...UserErrorFields
      }
    }
  }
"#;

const CUSTOMER_UPDATE: &str = r#"
  mutation CustomerUpdate($customerAccessToken: String!, $customer: CustomerUpdateInput!) {
    customerUpdate(customerAccessToken: $customerAccessToken, customer: $customer) {
      customer {
        ...CustomerFields
      }
      customerUserErrors {
        ...UserErrorFields
      }
    }
  }
"#;

const CUSTOMER_RECOVER: &str = r#"
  mutation CustomerRecover($email: String!) {
    customerRecover(email: $email) {
      customerUserErrors {
        ...UserErrorFields
      }
    }
  }
"#;

const CUSTOMER_RESET: &str = r#"
  mutation CustomerReset($id: ID!, $input: CustomerResetInput!) {
    customerReset(id: $id, input: $input) {
      customer {
        id
        email
      }
      customerAccessToken {
        accessToken
        expiresAt
      }
      customerUserErrors {
        ...UserErrorFields
      }
    }
  }
"#;

const CUSTOMER_RESET_BY_URL: &str = r#"
  mutation CustomerResetByUrl($resetUrl: URL!, $password: String!) {
    customerResetByUrl(resetUrl: $resetUrl, password: $password) {
      customer {
        id
        email
      }
      customerAccessToken {
        accessToken
        expiresAt
      }
      customerUserErrors {
        ...UserErrorFields
      }
    }
  }
"#;

const CUSTOMER_ACTIVATE: &str = r#"
  mutation CustomerActivate($id: ID!, $input: CustomerActivateInput!) {
    customerActivate(id: $id, input: $input) {
      customer {
        id
        email
      }
      customerAccessToken {
        accessToken
        expiresAt
      }
      customerUserErrors {
        ...UserErrorFields
      }
    }
  }
"#;

const CUSTOMER_ACTIVATE_BY_URL: &str = r#"
  mutation CustomerActivateByUrl($activationUrl: URL!, $password: String!) {
    customerActivateByUrl(activationUrl: $activationUrl, password: $password) {
      customer {
        id
        email
      }
      customerAccessToken {
        accessToken
        expiresAt
      }
      customerUserErrors {
        ...UserErrorFields
      }
    }
  }
"#;

const CUSTOMER_ADDRESS_CREATE: &str = r#"
  mutation CustomerAddressCreate($customerAccessToken: String!, $address: MailingAddressInput!) {
    customerAddressCreate(customerAccessToken: $customerAccessToken, address: $address) {
      customerAddress {
        ...AddressFields
      }
      customerUserErrors {
        ...UserErrorFields
      }
    }
  }
"#;

const CUSTOMER_ADDRESS_UPDATE: &str = r#"
  mutation CustomerAddressUpdate($customerAccessToken: String!, $id: ID!, $address: MailingAddressInput!) {
    customerAddressUpdate(customerAccessToken: $customerAccessToken, id: $id, address: $address) {
      customerAddress {
        ...AddressFields
      }
      customerUserErrors {
        ...UserErrorFields
      }
    }
  }
"#;

const CUSTOMER_ADDRESS_DELETE: &str = r#"
  mutation CustomerAddressDelete($customerAccessToken: String!, $id: ID!) {
    customerAddressDelete(customerAccessToken: $customerAccessToken, id: $id) {
      deletedCustomerAddressId
      customerUserErrors {
        ...UserErrorFields
      }
    }
  }
"#;

const CUSTOMER_DEFAULT_ADDRESS_UPDATE: &str = r#"
  mutation CustomerDefaultAddressUpdate($customerAccessToken: String!, $addressId: ID!) {
    customerDefaultAddressUpdate(customerAccessToken: $customerAccessToken, addressId: $addressId) {
      customer {
        ...CustomerFields
      }
      customerUserErrors {
        ...UserErrorFields
      }
    }
  }
"#;

const GET_CUSTOMER: &str = r#"
  query GetCustomer(
    $customerAccessToken: String!
    $addressesFirst: Int
    $ordersFirst: Int
  ) {
    customer(customerAccessToken: $customerAccessToken) {
      ...CustomerFields
      addresses(first: $addressesFirst) {
        edges {
          node {
            ...AddressFields
          }
        }
      }
      orders(first: $ordersFirst) {
        edges {
          node {
            ...OrderFields
          }
        }
      }
    }
  }
"#;

pub fn customer_access_token_create() -> &'static str {
    static Q: OnceLock<String> = OnceLock::new();
    Q.get_or_init(|| [CUSTOMER_USER_ERROR_FRAGMENT, CUSTOMER_ACCESS_TOKEN_CREATE].concat())
}

pub fn customer_access_token_renew() -> &'static str {
    static Q: OnceLock<String> = OnceLock::new();
    Q.get_or_init(
        || // No CUSTOMER_USER_ERROR_FRAGMENT here: this mutation returns plain `UserError`
    // (not `CustomerUserError`) and selects `userErrors { field message }` inline,
    // so including the fragment would make it an unused fragment (validation error).
    CUSTOMER_ACCESS_TOKEN_RENEW.to_string(),
    )
}

pub fn customer_access_token_delete() -> &'static str {
    static Q: OnceLock<String> = OnceLock::new();
    Q.get_or_init(|| CUSTOMER_ACCESS_TOKEN_DELETE.to_string())
}

pub fn customer_create() -> &'static str {
    static Q: OnceLock<String> = OnceLock::new();
    Q.get_or_init(|| [CUSTOMER_USER_ERROR_FRAGMENT, CUSTOMER_CREATE].concat())
}

pub fn customer_update() -> &'static str {
    static Q: OnceLock<String> = OnceLock::new();
    Q.get_or_init(
        || // CustomerFields/AddressFields don't reference MoneyV2, so MONEY_FRAGMENT
    // would be an unused fragment (validation error).
    [
        ADDRESS_FRAGMENT,
        CUSTOMER_FRAGMENT,
        CUSTOMER_USER_ERROR_FRAGMENT,
        CUSTOMER_UPDATE,
    ]
    .concat(),
    )
}

pub fn customer_recover() -> &'static str {
    static Q: OnceLock<String> = OnceLock::new();
    Q.get_or_init(|| [CUSTOMER_USER_ERROR_FRAGMENT, CUSTOMER_RECOVER].concat())
}

pub fn customer_reset() -> &'static str {
    static Q: OnceLock<String> = OnceLock::new();
    Q.get_or_init(|| [CUSTOMER_USER_ERROR_FRAGMENT, CUSTOMER_RESET].concat())
}

pub fn customer_reset_by_url() -> &'static str {
    static Q: OnceLock<String> = OnceLock::new();
    Q.get_or_init(|| [CUSTOMER_USER_ERROR_FRAGMENT, CUSTOMER_RESET_BY_URL].concat())
}

pub fn customer_activate() -> &'static str {
    static Q: OnceLock<String> = OnceLock::new();
    Q.get_or_init(|| [CUSTOMER_USER_ERROR_FRAGMENT, CUSTOMER_ACTIVATE].concat())
}

pub fn customer_activate_by_url() -> &'static str {
    static Q: OnceLock<String> = OnceLock::new();
    Q.get_or_init(|| [CUSTOMER_USER_ERROR_FRAGMENT, CUSTOMER_ACTIVATE_BY_URL].concat())
}

pub fn customer_address_create() -> &'static str {
    static Q: OnceLock<String> = OnceLock::new();
    Q.get_or_init(|| {
        [
            ADDRESS_FRAGMENT,
            CUSTOMER_USER_ERROR_FRAGMENT,
            CUSTOMER_ADDRESS_CREATE,
        ]
        .concat()
    })
}

pub fn customer_address_update() -> &'static str {
    static Q: OnceLock<String> = OnceLock::new();
    Q.get_or_init(|| {
        [
            ADDRESS_FRAGMENT,
            CUSTOMER_USER_ERROR_FRAGMENT,
            CUSTOMER_ADDRESS_UPDATE,
        ]
        .concat()
    })
}

pub fn customer_address_delete() -> &'static str {
    static Q: OnceLock<String> = OnceLock::new();
    Q.get_or_init(|| [CUSTOMER_USER_ERROR_FRAGMENT, CUSTOMER_ADDRESS_DELETE].concat())
}

pub fn customer_default_address_update() -> &'static str {
    static Q: OnceLock<String> = OnceLock::new();
    Q.get_or_init(
        || // CustomerFields/AddressFields don't reference MoneyV2, so MONEY_FRAGMENT
    // would be an unused fragment (validation error).
    [
        ADDRESS_FRAGMENT,
        CUSTOMER_FRAGMENT,
        CUSTOMER_USER_ERROR_FRAGMENT,
        CUSTOMER_DEFAULT_ADDRESS_UPDATE,
    ]
    .concat(),
    )
}

pub fn get_customer() -> &'static str {
    static Q: OnceLock<String> = OnceLock::new();
    Q.get_or_init(
        || // OrderFields references MoneyFields heavily, so MONEY_FRAGMENT is required here.
    [
        crate::storefront::fragments::MONEY_FRAGMENT,
        ADDRESS_FRAGMENT,
        ORDER_FRAGMENT,
        CUSTOMER_FRAGMENT,
        GET_CUSTOMER,
    ]
    .concat(),
    )
}
