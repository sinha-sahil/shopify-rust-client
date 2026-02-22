use crate::common::query_filter::DateFilter;

#[derive(Debug, Clone, Default)]
pub struct InventoryItemQueryParams {
    pub sku: Option<String>,
    pub created_at: Option<DateFilter>,
    pub updated_at: Option<DateFilter>,
}

impl InventoryItemQueryParams {
    pub fn to_query_string(&self) -> Option<String> {
        let mut parts = Vec::new();
        if let Some(v) = &self.sku {
            parts.push(format!("sku:{}", v));
        }
        if let Some(v) = &self.created_at {
            parts.push(format!("created_at:{}", v));
        }
        if let Some(v) = &self.updated_at {
            parts.push(format!("updated_at:{}", v));
        }
        if parts.is_empty() {
            None
        } else {
            Some(parts.join(" "))
        }
    }
}
