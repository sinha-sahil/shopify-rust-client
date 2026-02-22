use std::fmt;

use crate::common::query_filter::DateFilter;

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
