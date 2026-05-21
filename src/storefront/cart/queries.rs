use std::sync::OnceLock;

use crate::storefront::fragments::MONEY_FRAGMENT;

const CART_LINE_FRAGMENT: &str = r#"
  fragment CartLineFields on CartLine {
    id
    quantity
    merchandise {
      ... on ProductVariant {
        id
        title
        sku
        image {
          url
          altText
        }
        price {
          ...MoneyFields
        }
        compareAtPrice {
          ...MoneyFields
        }
        product {
          id
          handle
          title
          vendor
        }
        selectedOptions {
          name
          value
        }
      }
    }
    attributes {
      key
      value
    }
    cost {
      totalAmount {
        ...MoneyFields
      }
      subtotalAmount {
        ...MoneyFields
      }
      amountPerQuantity {
        ...MoneyFields
      }
      compareAtAmountPerQuantity {
        ...MoneyFields
      }
    }
    discountAllocations {
      discountedAmount {
        ...MoneyFields
      }
    }
  }
"#;

const CART_FRAGMENT: &str = r#"
  fragment CartFields on Cart {
    id
    checkoutUrl
    createdAt
    updatedAt
    note
    totalQuantity
    attributes {
      key
      value
    }
    buyerIdentity {
      email
      phone
      countryCode
      customer {
        id
        email
        firstName
        lastName
      }
    }
    discountCodes {
      code
      applicable
    }
    discountAllocations {
      discountedAmount {
        ...MoneyFields
      }
    }
    cost {
      totalAmount {
        ...MoneyFields
      }
      subtotalAmount {
        ...MoneyFields
      }
      checkoutChargeAmount {
        ...MoneyFields
      }
    }
    lines(first: 100) {
      nodes {
        ...CartLineFields
      }
    }
  }
"#;
// totalTaxAmount and totalDutyAmount were removed from CartCost selection — those fields
// are deprecated in the Storefront API and no longer return data. The generated `CartCost`
// type still has them as `Option<Money>`; they will simply deserialize as `None`.

const CART_USER_ERROR_FRAGMENT: &str = r#"
  fragment UserErrorFields on CartUserError {
    field
    message
    code
  }
"#;

// Cart fragment without customer scope (omits buyerIdentity.customer).
const CART_WITHOUT_CUSTOMER_SCOPE_FRAGMENT: &str = r#"
  fragment CartFieldsWithoutCustomerScope on Cart {
    id
    checkoutUrl
    createdAt
    updatedAt
    note
    totalQuantity
    attributes {
      key
      value
    }
    buyerIdentity {
      email
      phone
      countryCode
    }
    discountCodes {
      code
      applicable
    }
    discountAllocations {
      discountedAmount {
        ...MoneyFields
      }
    }
    cost {
      totalAmount {
        ...MoneyFields
      }
      subtotalAmount {
        ...MoneyFields
      }
      checkoutChargeAmount {
        ...MoneyFields
      }
    }
    lines(first: 100) {
      nodes {
        ...CartLineFields
      }
    }
  }
"#;
// Same deprecation note as CART_FRAGMENT — totalTaxAmount/totalDutyAmount removed.

const GET_CART: &str = r#"
  query GetCart($id: ID!) {
    cart(id: $id) {
      ...CartFields
    }
  }
"#;

const CART_CREATE: &str = r#"
  mutation CartCreate($input: CartInput) {
    cartCreate(input: $input) {
      cart {
        ...CartFields
      }
      userErrors {
        ...UserErrorFields
      }
    }
  }
"#;

const CART_LINES_ADD: &str = r#"
  mutation CartLinesAdd($cartId: ID!, $lines: [CartLineInput!]!) {
    cartLinesAdd(cartId: $cartId, lines: $lines) {
      cart {
        ...CartFields
      }
      userErrors {
        ...UserErrorFields
      }
    }
  }
"#;

const CART_LINES_UPDATE: &str = r#"
  mutation CartLinesUpdate($cartId: ID!, $lines: [CartLineUpdateInput!]!) {
    cartLinesUpdate(cartId: $cartId, lines: $lines) {
      cart {
        ...CartFields
      }
      userErrors {
        ...UserErrorFields
      }
    }
  }
"#;

const CART_LINES_REMOVE: &str = r#"
  mutation CartLinesRemove($cartId: ID!, $lineIds: [ID!]!) {
    cartLinesRemove(cartId: $cartId, lineIds: $lineIds) {
      cart {
        ...CartFields
      }
      userErrors {
        ...UserErrorFields
      }
    }
  }
"#;

const CART_NOTE_UPDATE: &str = r#"
  mutation CartNoteUpdate($cartId: ID!, $note: String!) {
    cartNoteUpdate(cartId: $cartId, note: $note) {
      cart {
        ...CartFields
      }
      userErrors {
        ...UserErrorFields
      }
    }
  }
"#;

const CART_ATTRIBUTES_UPDATE: &str = r#"
  mutation CartAttributesUpdate($cartId: ID!, $attributes: [AttributeInput!]!) {
    cartAttributesUpdate(cartId: $cartId, attributes: $attributes) {
      cart {
        ...CartFields
      }
      userErrors {
        ...UserErrorFields
      }
    }
  }
"#;

const CART_BUYER_IDENTITY_UPDATE: &str = r#"
  mutation CartBuyerIdentityUpdate($cartId: ID!, $buyerIdentity: CartBuyerIdentityInput!) {
    cartBuyerIdentityUpdate(cartId: $cartId, buyerIdentity: $buyerIdentity) {
      cart {
        ...CartFields
      }
      userErrors {
        ...UserErrorFields
      }
    }
  }
"#;

const CART_DISCOUNT_CODES_UPDATE: &str = r#"
  mutation CartDiscountCodesUpdate($cartId: ID!, $discountCodes: [String!]!) {
    cartDiscountCodesUpdate(cartId: $cartId, discountCodes: $discountCodes) {
      cart {
        ...CartFields
      }
      userErrors {
        ...UserErrorFields
      }
    }
  }
"#;

const CART_GIFT_CARD_CODES_ADD: &str = r#"
  mutation CartGiftCardCodesAdd($cartId: ID!, $giftCardCodes: [String!]!) {
    cartGiftCardCodesAdd(cartId: $cartId, giftCardCodes: $giftCardCodes) {
      cart {
        ...CartFields
      }
      userErrors {
        ...UserErrorFields
      }
    }
  }
"#;

// Without-customer-scope variants
const GET_CART_WITHOUT_CUSTOMER_SCOPE: &str = r#"
  query GetCartWithoutCustomerScope($id: ID!) {
    cart(id: $id) {
      ...CartFieldsWithoutCustomerScope
    }
  }
"#;

const CART_CREATE_WITHOUT_CUSTOMER_SCOPE: &str = r#"
  mutation CartCreateWithoutCustomerScope($input: CartInput) {
    cartCreate(input: $input) {
      cart {
        ...CartFieldsWithoutCustomerScope
      }
      userErrors {
        ...UserErrorFields
      }
    }
  }
"#;

const CART_LINES_ADD_WITHOUT_CUSTOMER_SCOPE: &str = r#"
  mutation CartLinesAddWithoutCustomerScope($cartId: ID!, $lines: [CartLineInput!]!) {
    cartLinesAdd(cartId: $cartId, lines: $lines) {
      cart {
        ...CartFieldsWithoutCustomerScope
      }
      userErrors {
        ...UserErrorFields
      }
    }
  }
"#;

const CART_LINES_UPDATE_WITHOUT_CUSTOMER_SCOPE: &str = r#"
  mutation CartLinesUpdateWithoutCustomerScope($cartId: ID!, $lines: [CartLineUpdateInput!]!) {
    cartLinesUpdate(cartId: $cartId, lines: $lines) {
      cart {
        ...CartFieldsWithoutCustomerScope
      }
      userErrors {
        ...UserErrorFields
      }
    }
  }
"#;

const CART_LINES_REMOVE_WITHOUT_CUSTOMER_SCOPE: &str = r#"
  mutation CartLinesRemoveWithoutCustomerScope($cartId: ID!, $lineIds: [ID!]!) {
    cartLinesRemove(cartId: $cartId, lineIds: $lineIds) {
      cart {
        ...CartFieldsWithoutCustomerScope
      }
      userErrors {
        ...UserErrorFields
      }
    }
  }
"#;

const CART_NOTE_UPDATE_WITHOUT_CUSTOMER_SCOPE: &str = r#"
  mutation CartNoteUpdateWithoutCustomerScope($cartId: ID!, $note: String!) {
    cartNoteUpdate(cartId: $cartId, note: $note) {
      cart {
        ...CartFieldsWithoutCustomerScope
      }
      userErrors {
        ...UserErrorFields
      }
    }
  }
"#;

const CART_ATTRIBUTES_UPDATE_WITHOUT_CUSTOMER_SCOPE: &str = r#"
  mutation CartAttributesUpdateWithoutCustomerScope($cartId: ID!, $attributes: [AttributeInput!]!) {
    cartAttributesUpdate(cartId: $cartId, attributes: $attributes) {
      cart {
        ...CartFieldsWithoutCustomerScope
      }
      userErrors {
        ...UserErrorFields
      }
    }
  }
"#;

const CART_BUYER_IDENTITY_UPDATE_WITHOUT_CUSTOMER_SCOPE: &str = r#"
  mutation CartBuyerIdentityUpdateWithoutCustomerScope($cartId: ID!, $buyerIdentity: CartBuyerIdentityInput!) {
    cartBuyerIdentityUpdate(cartId: $cartId, buyerIdentity: $buyerIdentity) {
      cart {
        ...CartFieldsWithoutCustomerScope
      }
      userErrors {
        ...UserErrorFields
      }
    }
  }
"#;

const CART_DISCOUNT_CODES_UPDATE_WITHOUT_CUSTOMER_SCOPE: &str = r#"
  mutation CartDiscountCodesUpdateWithoutCustomerScope($cartId: ID!, $discountCodes: [String!]!) {
    cartDiscountCodesUpdate(cartId: $cartId, discountCodes: $discountCodes) {
      cart {
        ...CartFieldsWithoutCustomerScope
      }
      userErrors {
        ...UserErrorFields
      }
    }
  }
"#;

const CART_GIFT_CARD_CODES_ADD_WITHOUT_CUSTOMER_SCOPE: &str = r#"
  mutation CartGiftCardCodesAddWithoutCustomerScope($cartId: ID!, $giftCardCodes: [String!]!) {
    cartGiftCardCodesAdd(cartId: $cartId, giftCardCodes: $giftCardCodes) {
      cart {
        ...CartFieldsWithoutCustomerScope
      }
      userErrors {
        ...UserErrorFields
      }
    }
  }
"#;

fn with_customer_prefix() -> [&'static str; 4] {
    [
        MONEY_FRAGMENT,
        CART_LINE_FRAGMENT,
        CART_FRAGMENT,
        CART_USER_ERROR_FRAGMENT,
    ]
}

fn without_customer_prefix() -> [&'static str; 4] {
    [
        MONEY_FRAGMENT,
        CART_LINE_FRAGMENT,
        CART_WITHOUT_CUSTOMER_SCOPE_FRAGMENT,
        CART_USER_ERROR_FRAGMENT,
    ]
}

fn assemble(fragments: [&'static str; 4], op: &'static str) -> String {
    let mut out =
        String::with_capacity(fragments.iter().map(|s| s.len()).sum::<usize>() + op.len());
    for fragment in fragments.iter() {
        out.push_str(fragment);
    }
    out.push_str(op);
    out
}

pub fn get_cart() -> &'static str {
    static Q: OnceLock<String> = OnceLock::new();
    Q.get_or_init(|| [MONEY_FRAGMENT, CART_LINE_FRAGMENT, CART_FRAGMENT, GET_CART].concat())
}

pub fn cart_create() -> &'static str {
    static Q: OnceLock<String> = OnceLock::new();
    Q.get_or_init(|| assemble(with_customer_prefix(), CART_CREATE))
}

pub fn cart_lines_add() -> &'static str {
    static Q: OnceLock<String> = OnceLock::new();
    Q.get_or_init(|| assemble(with_customer_prefix(), CART_LINES_ADD))
}

pub fn cart_lines_update() -> &'static str {
    static Q: OnceLock<String> = OnceLock::new();
    Q.get_or_init(|| assemble(with_customer_prefix(), CART_LINES_UPDATE))
}

pub fn cart_lines_remove() -> &'static str {
    static Q: OnceLock<String> = OnceLock::new();
    Q.get_or_init(|| assemble(with_customer_prefix(), CART_LINES_REMOVE))
}

pub fn cart_note_update() -> &'static str {
    static Q: OnceLock<String> = OnceLock::new();
    Q.get_or_init(|| assemble(with_customer_prefix(), CART_NOTE_UPDATE))
}

pub fn cart_attributes_update() -> &'static str {
    static Q: OnceLock<String> = OnceLock::new();
    Q.get_or_init(|| assemble(with_customer_prefix(), CART_ATTRIBUTES_UPDATE))
}

pub fn cart_buyer_identity_update() -> &'static str {
    static Q: OnceLock<String> = OnceLock::new();
    Q.get_or_init(|| assemble(with_customer_prefix(), CART_BUYER_IDENTITY_UPDATE))
}

pub fn cart_discount_codes_update() -> &'static str {
    static Q: OnceLock<String> = OnceLock::new();
    Q.get_or_init(|| assemble(with_customer_prefix(), CART_DISCOUNT_CODES_UPDATE))
}

pub fn cart_gift_card_codes_add() -> &'static str {
    static Q: OnceLock<String> = OnceLock::new();
    Q.get_or_init(|| assemble(with_customer_prefix(), CART_GIFT_CARD_CODES_ADD))
}

pub fn get_cart_without_customer_scope() -> &'static str {
    static Q: OnceLock<String> = OnceLock::new();
    Q.get_or_init(|| {
        [
            MONEY_FRAGMENT,
            CART_LINE_FRAGMENT,
            CART_WITHOUT_CUSTOMER_SCOPE_FRAGMENT,
            GET_CART_WITHOUT_CUSTOMER_SCOPE,
        ]
        .concat()
    })
}

pub fn cart_create_without_customer_scope() -> &'static str {
    static Q: OnceLock<String> = OnceLock::new();
    Q.get_or_init(|| {
        assemble(
            without_customer_prefix(),
            CART_CREATE_WITHOUT_CUSTOMER_SCOPE,
        )
    })
}

pub fn cart_lines_add_without_customer_scope() -> &'static str {
    static Q: OnceLock<String> = OnceLock::new();
    Q.get_or_init(|| {
        assemble(
            without_customer_prefix(),
            CART_LINES_ADD_WITHOUT_CUSTOMER_SCOPE,
        )
    })
}

pub fn cart_lines_update_without_customer_scope() -> &'static str {
    static Q: OnceLock<String> = OnceLock::new();
    Q.get_or_init(|| {
        assemble(
            without_customer_prefix(),
            CART_LINES_UPDATE_WITHOUT_CUSTOMER_SCOPE,
        )
    })
}

pub fn cart_lines_remove_without_customer_scope() -> &'static str {
    static Q: OnceLock<String> = OnceLock::new();
    Q.get_or_init(|| {
        assemble(
            without_customer_prefix(),
            CART_LINES_REMOVE_WITHOUT_CUSTOMER_SCOPE,
        )
    })
}

pub fn cart_note_update_without_customer_scope() -> &'static str {
    static Q: OnceLock<String> = OnceLock::new();
    Q.get_or_init(|| {
        assemble(
            without_customer_prefix(),
            CART_NOTE_UPDATE_WITHOUT_CUSTOMER_SCOPE,
        )
    })
}

pub fn cart_attributes_update_without_customer_scope() -> &'static str {
    static Q: OnceLock<String> = OnceLock::new();
    Q.get_or_init(|| {
        assemble(
            without_customer_prefix(),
            CART_ATTRIBUTES_UPDATE_WITHOUT_CUSTOMER_SCOPE,
        )
    })
}

pub fn cart_buyer_identity_update_without_customer_scope() -> &'static str {
    static Q: OnceLock<String> = OnceLock::new();
    Q.get_or_init(|| {
        assemble(
            without_customer_prefix(),
            CART_BUYER_IDENTITY_UPDATE_WITHOUT_CUSTOMER_SCOPE,
        )
    })
}

pub fn cart_discount_codes_update_without_customer_scope() -> &'static str {
    static Q: OnceLock<String> = OnceLock::new();
    Q.get_or_init(|| {
        assemble(
            without_customer_prefix(),
            CART_DISCOUNT_CODES_UPDATE_WITHOUT_CUSTOMER_SCOPE,
        )
    })
}

pub fn cart_gift_card_codes_add_without_customer_scope() -> &'static str {
    static Q: OnceLock<String> = OnceLock::new();
    Q.get_or_init(|| {
        assemble(
            without_customer_prefix(),
            CART_GIFT_CARD_CODES_ADD_WITHOUT_CUSTOMER_SCOPE,
        )
    })
}
