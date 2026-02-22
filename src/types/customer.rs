use std::fmt;

use crate::common::query_filter::{DateFilter, NumericFilter};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CustomerAccountState {
    Enabled,
    Disabled,
    Invited,
    Declined,
}

impl fmt::Display for CustomerAccountState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CustomerAccountState::Enabled => write!(f, "enabled"),
            CustomerAccountState::Disabled => write!(f, "disabled"),
            CustomerAccountState::Invited => write!(f, "invited"),
            CustomerAccountState::Declined => write!(f, "declined"),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct CustomerQueryParams {
    pub state: Option<CustomerAccountState>,
    pub tag: Option<String>,
    pub email: Option<String>,
    pub country: Option<String>,
    pub created_at: Option<DateFilter>,
    pub updated_at: Option<DateFilter>,
    pub orders_count: Option<NumericFilter<i64>>,
    pub total_spent: Option<NumericFilter<f64>>,
    pub accepts_marketing: Option<bool>,
}

impl CustomerQueryParams {
    pub fn to_query_string(&self) -> Option<String> {
        let mut parts = Vec::new();
        if let Some(v) = &self.state {
            parts.push(format!("state:{}", v));
        }
        if let Some(v) = &self.tag {
            parts.push(format!("tag:{}", v));
        }
        if let Some(v) = &self.email {
            parts.push(format!("email:{}", v));
        }
        if let Some(v) = &self.country {
            parts.push(format!("country:{}", v));
        }
        if let Some(v) = &self.created_at {
            parts.push(format!("created_at:{}", v));
        }
        if let Some(v) = &self.updated_at {
            parts.push(format!("updated_at:{}", v));
        }
        if let Some(v) = &self.orders_count {
            parts.push(format!("orders_count:{}", v));
        }
        if let Some(v) = &self.total_spent {
            parts.push(format!("total_spent:{}", v));
        }
        if let Some(v) = &self.accepts_marketing {
            parts.push(format!("accepts_marketing:{}", v));
        }
        if parts.is_empty() {
            None
        } else {
            Some(parts.join(" "))
        }
    }
}
