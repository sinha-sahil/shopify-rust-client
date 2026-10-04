use std::fmt;

use crate::common::query_filter::DateFilter;
use crate::common::types::Connection;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProductStatus {
    Active,
    Archived,
    Draft,
}

impl fmt::Display for ProductStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProductStatus::Active => write!(f, "active"),
            ProductStatus::Archived => write!(f, "archived"),
            ProductStatus::Draft => write!(f, "draft"),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ProductQueryParams {
    pub status: Option<ProductStatus>,
    pub product_type: Option<String>,
    pub vendor: Option<String>,
    pub tag: Option<String>,
    pub created_at: Option<DateFilter>,
    pub updated_at: Option<DateFilter>,
    pub published_at: Option<DateFilter>,
    pub gift_card: Option<bool>,
}

impl ProductQueryParams {
    pub fn to_query_string(&self) -> Option<String> {
        let mut parts = Vec::new();
        if let Some(v) = &self.status {
            parts.push(format!("status:{}", v));
        }
        if let Some(v) = &self.product_type {
            parts.push(format!("product_type:{}", v));
        }
        if let Some(v) = &self.vendor {
            parts.push(format!("vendor:{}", v));
        }
        if let Some(v) = &self.tag {
            parts.push(format!("tag:{}", v));
        }
        if let Some(v) = &self.created_at {
            parts.push(format!("created_at:{}", v));
        }
        if let Some(v) = &self.updated_at {
            parts.push(format!("updated_at:{}", v));
        }
        if let Some(v) = &self.published_at {
            parts.push(format!("published_at:{}", v));
        }
        if let Some(v) = &self.gift_card {
            parts.push(format!("gift_card:{}", v));
        }
        if parts.is_empty() {
            None
        } else {
            Some(parts.join(" "))
        }
    }
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProductVariantResp {
    pub product_variant: Option<ProductVariant>,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct ProductVariant {
    pub id: String,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub media: Option<Connection<MediaPreview>>,
    pub product: VariantProduct,
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VariantProduct {
    pub id: String,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub featured_media: Option<MediaPreview>,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct MediaPreview {
    pub preview: Option<MediaPreviewImage>,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct MediaPreviewImage {
    pub image: Option<MediaImage>,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct MediaImage {
    pub url: String,
}

impl ProductVariant {
    pub fn image_url(&self) -> Option<&str> {
        let own = self
            .media
            .as_ref()
            .and_then(|media| media.nodes.first())
            .and_then(MediaPreview::url);
        own.or_else(|| {
            self.product
                .featured_media
                .as_ref()
                .and_then(MediaPreview::url)
        })
    }
}

impl MediaPreview {
    fn url(&self) -> Option<&str> {
        self.preview
            .as_ref()
            .and_then(|preview| preview.image.as_ref())
            .map(|image| image.url.as_str())
    }
}

#[cfg(test)]
mod variant_tests {
    use super::*;
    use serde_json::json;

    fn variant(media: serde_json::Value) -> ProductVariant {
        serde_json::from_value(json!({
            "id": "gid://shopify/ProductVariant/1",
            "title": "L / Black",
            "media": { "nodes": media, "pageInfo": { "hasNextPage": false } },
            "product": {
                "id": "gid://shopify/Product/1",
                "title": "Linen shirt",
                "featuredMedia": { "preview": { "image": { "url": "https://cdn/product.jpg" } } }
            }
        }))
        .expect("parses")
    }

    #[test]
    fn a_variant_shows_its_own_image_first() {
        let own =
            variant(json!([{ "preview": { "image": { "url": "https://cdn/variant.jpg" } } }]));
        assert_eq!(own.image_url(), Some("https://cdn/variant.jpg"));
    }

    #[test]
    fn a_variant_without_media_falls_back_to_the_product_image() {
        assert_eq!(
            variant(json!([])).image_url(),
            Some("https://cdn/product.jpg")
        );
    }

    #[test]
    fn an_id_only_variant_still_parses() {
        let bare: ProductVariant = serde_json::from_value(
            json!({ "id": "gid://shopify/ProductVariant/1", "product": { "id": "gid://shopify/Product/1" } }),
        )
        .expect("parses");
        assert_eq!(bare.image_url(), None);
    }
}
