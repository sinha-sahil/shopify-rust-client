use std::fmt;

use crate::common::query_filter::DateFilter;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollectionType {
    Smart,
    Custom,
}

impl fmt::Display for CollectionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CollectionType::Smart => write!(f, "smart"),
            CollectionType::Custom => write!(f, "custom"),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct CollectionQueryParams {
    pub collection_type: Option<CollectionType>,
    pub title: Option<String>,
    pub updated_at: Option<DateFilter>,
    pub published_at: Option<DateFilter>,
}

impl CollectionQueryParams {
    pub fn to_query_string(&self) -> Option<String> {
        let mut parts = Vec::new();
        if let Some(v) = &self.collection_type {
            parts.push(format!("collection_type:{}", v));
        }
        if let Some(v) = &self.title {
            parts.push(format!("title:{}", v));
        }
        if let Some(v) = &self.updated_at {
            parts.push(format!("updated_at:{}", v));
        }
        if let Some(v) = &self.published_at {
            parts.push(format!("published_at:{}", v));
        }
        if parts.is_empty() {
            None
        } else {
            Some(parts.join(" "))
        }
    }
}
