use std::sync::OnceLock;

const SHOP_FRAGMENT: &str = r#"
  fragment ShopFields on Shop {
    id
    name
    description
    moneyFormat
    brand {
      shortDescription
      slogan
      logo {
        image {
          url
          altText
        }
      }
      squareLogo {
        image {
          url
          altText
        }
      }
      coverImage {
        image {
          url
          altText
        }
      }
      colors {
        primary {
          background
          foreground
        }
        secondary {
          background
          foreground
        }
      }
    }
    primaryDomain {
      url
      host
    }
    paymentSettings {
      countryCode
      currencyCode
      acceptedCardBrands
      enabledPresentmentCurrencies
      supportedDigitalWallets
    }
    shipsToCountries
    refundPolicy {
      id
      body
      handle
      title
      url
    }
    privacyPolicy {
      id
      body
      handle
      title
      url
    }
    termsOfService {
      id
      body
      handle
      title
      url
    }
    shippingPolicy {
      id
      body
      handle
      title
      url
    }
    subscriptionPolicy {
      id
      body
      handle
      title
      url
    }
  }
"#;

const LOCALIZATION_FRAGMENT: &str = r#"
  fragment LocalizationFields on Localization {
    availableCountries {
      isoCode
      name
      currency {
        isoCode
        name
        symbol
      }
      availableLanguages {
        isoCode
        name
        endonymName
      }
    }
    availableLanguages {
      isoCode
      name
      endonymName
    }
    country {
      isoCode
      name
      currency {
        isoCode
        name
        symbol
      }
    }
    language {
      isoCode
      name
      endonymName
    }
  }
"#;
// `Localization.market` is intentionally omitted from the selection above: the field
// is deprecated and scheduled for removal in a future Storefront API version.
// The generated `Localization` type still has it as `Option<Market>`; it deserializes
// as `None`.

const GET_SHOP: &str = r#"
  query GetShop {
    shop {
      ...ShopFields
    }
  }
"#;

const GET_LOCALIZATION: &str = r#"
  query GetLocalization {
    localization {
      ...LocalizationFields
    }
  }
"#;

pub fn get_shop() -> &'static str {
    static Q: OnceLock<String> = OnceLock::new();
    Q.get_or_init(|| [SHOP_FRAGMENT, GET_SHOP].concat())
}

pub fn get_localization() -> &'static str {
    static Q: OnceLock<String> = OnceLock::new();
    Q.get_or_init(|| [LOCALIZATION_FRAGMENT, GET_LOCALIZATION].concat())
}
