use std::fmt;

use crate::common::query_filter::DateFilter;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DraftOrderFilterStatus {
    Open,
    InvoiceSent,
    Completed,
}

impl fmt::Display for DraftOrderFilterStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DraftOrderFilterStatus::Open => write!(f, "open"),
            DraftOrderFilterStatus::InvoiceSent => write!(f, "invoice_sent"),
            DraftOrderFilterStatus::Completed => write!(f, "completed"),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct DraftOrderQueryParams {
    pub status: Option<DraftOrderFilterStatus>,
    pub tag: Option<String>,
    pub created_at: Option<DateFilter>,
    pub updated_at: Option<DateFilter>,
}

impl DraftOrderQueryParams {
    pub fn to_query_string(&self) -> Option<String> {
        let mut parts = Vec::new();
        if let Some(v) = &self.status {
            parts.push(format!("status:{}", v));
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
        if parts.is_empty() {
            None
        } else {
            Some(parts.join(" "))
        }
    }
}
