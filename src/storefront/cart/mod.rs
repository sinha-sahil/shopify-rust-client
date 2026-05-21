pub mod queries;
pub mod remote;

use crate::common::ServiceContext;

use std::sync::Arc;

use crate::common::types::{APIError, RequestCallbacks};
use crate::storefront::generated::types::cart::{
    CartBuyerIdentityInput, CartInput, CartLineInput, CartLineUpdateInput,
};
use crate::storefront::generated::types::common::AttributeInput;
use crate::storefront::generated::types::responses::{
    CartAttributesUpdateResponse, CartBuyerIdentityUpdateResponse, CartCreateResponse,
    CartDiscountCodesUpdateResponse, CartGiftCardCodesAddResponse, CartLinesAddResponse,
    CartLinesRemoveResponse, CartLinesUpdateResponse, CartNoteUpdateResponse, CartResponse,
};

pub struct Cart {
    pub(crate) ctx: ServiceContext,
}

impl Cart {
    pub fn new(
        shop_url: Arc<String>,
        version: Arc<String>,
        access_token: Arc<String>,
        callbacks: Arc<RequestCallbacks>,
    ) -> Self {
        Self::with_ctx(ServiceContext::new(
            shop_url,
            version,
            access_token,
            callbacks,
        ))
    }

    /// Build the service from a shared `ServiceContext`. Cheaper than `new` at
    /// construction sites that already hold a context (one `Arc` clone per service).
    pub fn with_ctx(ctx: ServiceContext) -> Self {
        Self { ctx }
    }

    pub async fn get(&self, cart_id: &str) -> Result<CartResponse, APIError> {
        remote::get(&self.ctx, cart_id).await
    }

    pub async fn create(&self, input: CartInput) -> Result<CartCreateResponse, APIError> {
        remote::create(&self.ctx, input).await
    }

    pub async fn add_lines(
        &self,
        cart_id: &str,
        lines: Vec<CartLineInput>,
    ) -> Result<CartLinesAddResponse, APIError> {
        remote::add_lines(&self.ctx, cart_id, lines).await
    }

    pub async fn update_lines(
        &self,
        cart_id: &str,
        lines: Vec<CartLineUpdateInput>,
    ) -> Result<CartLinesUpdateResponse, APIError> {
        remote::update_lines(&self.ctx, cart_id, lines).await
    }

    pub async fn remove_lines(
        &self,
        cart_id: &str,
        line_ids: Vec<String>,
    ) -> Result<CartLinesRemoveResponse, APIError> {
        remote::remove_lines(&self.ctx, cart_id, line_ids).await
    }

    pub async fn update_note(
        &self,
        cart_id: &str,
        note: &str,
    ) -> Result<CartNoteUpdateResponse, APIError> {
        remote::update_note(&self.ctx, cart_id, note).await
    }

    pub async fn update_attributes(
        &self,
        cart_id: &str,
        attributes: Vec<AttributeInput>,
    ) -> Result<CartAttributesUpdateResponse, APIError> {
        remote::update_attributes(&self.ctx, cart_id, attributes).await
    }

    pub async fn update_buyer_identity(
        &self,
        cart_id: &str,
        buyer_identity: CartBuyerIdentityInput,
    ) -> Result<CartBuyerIdentityUpdateResponse, APIError> {
        remote::update_buyer_identity(&self.ctx, cart_id, buyer_identity).await
    }

    pub async fn update_discount_codes(
        &self,
        cart_id: &str,
        discount_codes: Vec<String>,
    ) -> Result<CartDiscountCodesUpdateResponse, APIError> {
        remote::update_discount_codes(&self.ctx, cart_id, discount_codes).await
    }

    pub async fn add_gift_card_codes(
        &self,
        cart_id: &str,
        gift_card_codes: Vec<String>,
    ) -> Result<CartGiftCardCodesAddResponse, APIError> {
        remote::add_gift_card_codes(&self.ctx, cart_id, gift_card_codes).await
    }

    // Without-customer-scope variants. These mirror the with-scope methods but
    // use queries that omit `buyerIdentity.customer` from the response, suitable
    // for unauthenticated cart flows.

    pub async fn get_without_customer(&self, cart_id: &str) -> Result<CartResponse, APIError> {
        remote::get_without_customer(&self.ctx, cart_id).await
    }

    pub async fn create_without_customer(
        &self,
        input: CartInput,
    ) -> Result<CartCreateResponse, APIError> {
        remote::create_without_customer(&self.ctx, input).await
    }

    pub async fn add_lines_without_customer(
        &self,
        cart_id: &str,
        lines: Vec<CartLineInput>,
    ) -> Result<CartLinesAddResponse, APIError> {
        remote::add_lines_without_customer(&self.ctx, cart_id, lines).await
    }

    pub async fn update_lines_without_customer(
        &self,
        cart_id: &str,
        lines: Vec<CartLineUpdateInput>,
    ) -> Result<CartLinesUpdateResponse, APIError> {
        remote::update_lines_without_customer(&self.ctx, cart_id, lines).await
    }

    pub async fn remove_lines_without_customer(
        &self,
        cart_id: &str,
        line_ids: Vec<String>,
    ) -> Result<CartLinesRemoveResponse, APIError> {
        remote::remove_lines_without_customer(&self.ctx, cart_id, line_ids).await
    }

    pub async fn update_note_without_customer(
        &self,
        cart_id: &str,
        note: &str,
    ) -> Result<CartNoteUpdateResponse, APIError> {
        remote::update_note_without_customer(&self.ctx, cart_id, note).await
    }

    pub async fn update_attributes_without_customer(
        &self,
        cart_id: &str,
        attributes: Vec<AttributeInput>,
    ) -> Result<CartAttributesUpdateResponse, APIError> {
        remote::update_attributes_without_customer(&self.ctx, cart_id, attributes).await
    }

    pub async fn update_buyer_identity_without_customer(
        &self,
        cart_id: &str,
        buyer_identity: CartBuyerIdentityInput,
    ) -> Result<CartBuyerIdentityUpdateResponse, APIError> {
        remote::update_buyer_identity_without_customer(&self.ctx, cart_id, buyer_identity).await
    }

    pub async fn update_discount_codes_without_customer(
        &self,
        cart_id: &str,
        discount_codes: Vec<String>,
    ) -> Result<CartDiscountCodesUpdateResponse, APIError> {
        remote::update_discount_codes_without_customer(&self.ctx, cart_id, discount_codes).await
    }

    pub async fn add_gift_card_codes_without_customer(
        &self,
        cart_id: &str,
        gift_card_codes: Vec<String>,
    ) -> Result<CartGiftCardCodesAddResponse, APIError> {
        remote::add_gift_card_codes_without_customer(&self.ctx, cart_id, gift_card_codes).await
    }
}
