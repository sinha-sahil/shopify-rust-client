use serde::{Deserialize, Serialize};

/// A language the shop has added
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShopLocale {
    /// ISO code, such as en or pt-BR
    pub locale: String,
    pub name: String,
    /// The shop default language
    pub primary: bool,
    /// Shown to customers on the storefront
    pub published: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListShopLocalesResp {
    #[serde(rename = "shopLocales")]
    pub shop_locales: Vec<ShopLocale>,
}
