#[derive(serde::Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReturnLineItemInput {
    pub fulfillment_line_item_id: String,
    pub quantity: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_reason_definition: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_note: Option<String>,
}

#[derive(serde::Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReturnProcessLineInput {
    pub return_line_item_id: String,
    pub dispositions: Vec<ReturnLineItemDisposition>,
}

#[derive(serde::Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReturnLineItemDisposition {
    #[serde(rename = "type")]
    pub disposition_type: ReturnDispositionType,
    pub location_id: String,
    pub quantity: i32,
}

#[derive(serde::Serialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReturnDispositionType {
    Restocked,
    NotRestocked,
    Damaged,
    Fraudulent,
    Missing,
}

#[derive(serde::Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderTransactionInput {
    pub amount: String,
    pub currency: String,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct ReturnRef {
    pub id: String,
    pub status: String,
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReturnUserError {
    #[serde(default)]
    pub field: Option<Vec<String>>,
    pub message: String,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct ReturnRequestResp {
    #[serde(rename = "returnRequest")]
    pub return_request: ReturnRequestPayload,
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReturnRequestPayload {
    #[serde(rename = "return")]
    pub return_ref: Option<ReturnRef>,
    #[serde(default)]
    pub user_errors: Vec<ReturnUserError>,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct ReturnProcessResp {
    #[serde(rename = "returnProcess")]
    pub return_process: ReturnProcessPayload,
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReturnProcessPayload {
    #[serde(rename = "return")]
    pub return_ref: Option<ReturnRef>,
    #[serde(default)]
    pub user_errors: Vec<ReturnUserError>,
}
