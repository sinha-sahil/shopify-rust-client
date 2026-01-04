mod remote;
pub mod types;

use std::sync::Arc;

use crate::common::types::APIError;
use types::GetOrderResp;

pub struct Order {
    pub shop_url: Arc<String>,
    pub version: Arc<String>,
    pub access_token: Arc<String>,
}

impl Order {
    pub fn new(shop_url: Arc<String>, version: Arc<String>, access_token: Arc<String>) -> Self {
        Order {
            shop_url,
            version,
            access_token,
        }
    }

    pub async fn get_with_id(&self, order_id: &String) -> Result<GetOrderResp, APIError> {
        remote::get_order_with_id(&self.shop_url, &self.version, &self.access_token, order_id).await
    }

    pub async fn get_with_name(
        &self,
        order_name: &String,
    ) -> Result<types::OrderQueryResp, APIError> {
        remote::get_order_with_name(
            &self.shop_url,
            &self.version,
            order_name,
            &self.access_token,
        )
        .await
    }

    pub async fn patch(
        &self,
        order_id: &String,
        patch_request: &types::PatchOrderRequest,
    ) -> Result<GetOrderResp, APIError> {
        remote::patch_order(
            &self.shop_url,
            &self.version,
            &self.access_token,
            order_id,
            patch_request,
        )
        .await
    }
}
