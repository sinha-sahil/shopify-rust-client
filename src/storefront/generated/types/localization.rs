use serde::{Deserialize, Serialize};

/// Localization information for the current context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Localization {
    #[serde(rename = "availableCountries")]
    pub available_countries: Vec<Country>,
    #[serde(rename = "availableLanguages")]
    pub available_languages: Vec<Language>,
    pub country: Country,
    pub language: Language,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market: Option<Market>,
}

/// A country for localization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Country {
    /// Two-letter country code (ISO 3166-1 alpha-2)
    #[serde(rename = "isoCode")]
    pub iso_code: String,
    /// Country name
    pub name: String,
    pub currency: Currency,
    #[serde(rename = "availableLanguages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_languages: Option<Vec<Language>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market: Option<Market>,
}

/// A language for localization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Language {
    /// Language code (e.g., EN, FR)
    #[serde(rename = "isoCode")]
    pub iso_code: String,
    /// Language name in English
    pub name: String,
    /// Language name in its own language
    #[serde(rename = "endonymName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endonym_name: Option<String>,
}

/// A currency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Currency {
    /// Three-letter currency code (ISO 4217)
    #[serde(rename = "isoCode")]
    pub iso_code: String,
    /// Currency name
    pub name: String,
    /// Currency symbol
    pub symbol: String,
}

/// A Shopify market
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Market {
    pub id: String,
    pub handle: String,
}
