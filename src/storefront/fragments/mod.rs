// Fragments shared across more than one service's queries. Service-specific
// fragments live in that service's `queries.rs`.

pub const MONEY_FRAGMENT: &str = r#"
  fragment MoneyFields on MoneyV2 {
    amount
    currencyCode
  }
"#;

pub const IMAGE_FRAGMENT: &str = r#"
  fragment ImageFields on Image {
    id
    url
    altText
    width
    height
  }
"#;

pub const SEO_FRAGMENT: &str = r#"
  fragment SEOFields on SEO {
    title
    description
  }
"#;
