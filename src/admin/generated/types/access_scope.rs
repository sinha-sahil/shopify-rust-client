use super::common::AccessScope;
use serde::{Deserialize, Serialize};

/// Response for GET /admin/oauth/access_scopes.json
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessScopesResp {
    pub access_scopes: Vec<AccessScope>,
}
