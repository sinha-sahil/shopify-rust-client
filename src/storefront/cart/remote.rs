use crate::common::ServiceContext;

use crate::common::types::APIError;
use serde_json::json;

use super::queries;
use crate::common::http::execute_storefront_graphql as execute_graphql;
use crate::storefront::generated::types::cart::{
    CartBuyerIdentityInput, CartInput, CartLineInput, CartLineUpdateInput,
};
use crate::storefront::generated::types::common::AttributeInput;
use crate::storefront::generated::types::responses::{
    CartAttributesUpdateResponse, CartBuyerIdentityUpdateResponse, CartCreateResponse,
    CartDiscountCodesUpdateResponse, CartGiftCardCodesAddResponse, CartLinesAddResponse,
    CartLinesRemoveResponse, CartLinesUpdateResponse, CartNoteUpdateResponse, CartResponse,
};

pub async fn get(ctx: &ServiceContext, cart_id: &str) -> Result<CartResponse, APIError> {
    execute_graphql(ctx, queries::get_cart(), json!({ "id": cart_id })).await
}

pub async fn create(
    ctx: &ServiceContext,
    input: CartInput,
) -> Result<CartCreateResponse, APIError> {
    execute_graphql(ctx, queries::cart_create(), json!({ "input": input })).await
}

pub async fn add_lines(
    ctx: &ServiceContext,
    cart_id: &str,
    lines: Vec<CartLineInput>,
) -> Result<CartLinesAddResponse, APIError> {
    execute_graphql(
        ctx,
        queries::cart_lines_add(),
        json!({ "cartId": cart_id, "lines": lines }),
    )
    .await
}

pub async fn update_lines(
    ctx: &ServiceContext,
    cart_id: &str,
    lines: Vec<CartLineUpdateInput>,
) -> Result<CartLinesUpdateResponse, APIError> {
    execute_graphql(
        ctx,
        queries::cart_lines_update(),
        json!({ "cartId": cart_id, "lines": lines }),
    )
    .await
}

pub async fn remove_lines(
    ctx: &ServiceContext,
    cart_id: &str,
    line_ids: Vec<String>,
) -> Result<CartLinesRemoveResponse, APIError> {
    execute_graphql(
        ctx,
        queries::cart_lines_remove(),
        json!({ "cartId": cart_id, "lineIds": line_ids }),
    )
    .await
}

pub async fn update_note(
    ctx: &ServiceContext,
    cart_id: &str,
    note: &str,
) -> Result<CartNoteUpdateResponse, APIError> {
    execute_graphql(
        ctx,
        queries::cart_note_update(),
        json!({ "cartId": cart_id, "note": note }),
    )
    .await
}

pub async fn update_attributes(
    ctx: &ServiceContext,
    cart_id: &str,
    attributes: Vec<AttributeInput>,
) -> Result<CartAttributesUpdateResponse, APIError> {
    execute_graphql(
        ctx,
        queries::cart_attributes_update(),
        json!({ "cartId": cart_id, "attributes": attributes }),
    )
    .await
}

pub async fn update_buyer_identity(
    ctx: &ServiceContext,
    cart_id: &str,
    buyer_identity: CartBuyerIdentityInput,
) -> Result<CartBuyerIdentityUpdateResponse, APIError> {
    execute_graphql(
        ctx,
        queries::cart_buyer_identity_update(),
        json!({ "cartId": cart_id, "buyerIdentity": buyer_identity }),
    )
    .await
}

pub async fn update_discount_codes(
    ctx: &ServiceContext,
    cart_id: &str,
    discount_codes: Vec<String>,
) -> Result<CartDiscountCodesUpdateResponse, APIError> {
    execute_graphql(
        ctx,
        queries::cart_discount_codes_update(),
        json!({ "cartId": cart_id, "discountCodes": discount_codes }),
    )
    .await
}

pub async fn add_gift_card_codes(
    ctx: &ServiceContext,
    cart_id: &str,
    gift_card_codes: Vec<String>,
) -> Result<CartGiftCardCodesAddResponse, APIError> {
    execute_graphql(
        ctx,
        queries::cart_gift_card_codes_add(),
        json!({ "cartId": cart_id, "giftCardCodes": gift_card_codes }),
    )
    .await
}

// Without-customer-scope variants.

pub async fn get_without_customer(
    ctx: &ServiceContext,
    cart_id: &str,
) -> Result<CartResponse, APIError> {
    execute_graphql(
        ctx,
        queries::get_cart_without_customer_scope(),
        json!({ "id": cart_id }),
    )
    .await
}

pub async fn create_without_customer(
    ctx: &ServiceContext,
    input: CartInput,
) -> Result<CartCreateResponse, APIError> {
    execute_graphql(
        ctx,
        queries::cart_create_without_customer_scope(),
        json!({ "input": input }),
    )
    .await
}

pub async fn add_lines_without_customer(
    ctx: &ServiceContext,
    cart_id: &str,
    lines: Vec<CartLineInput>,
) -> Result<CartLinesAddResponse, APIError> {
    execute_graphql(
        ctx,
        queries::cart_lines_add_without_customer_scope(),
        json!({ "cartId": cart_id, "lines": lines }),
    )
    .await
}

pub async fn update_lines_without_customer(
    ctx: &ServiceContext,
    cart_id: &str,
    lines: Vec<CartLineUpdateInput>,
) -> Result<CartLinesUpdateResponse, APIError> {
    execute_graphql(
        ctx,
        queries::cart_lines_update_without_customer_scope(),
        json!({ "cartId": cart_id, "lines": lines }),
    )
    .await
}

pub async fn remove_lines_without_customer(
    ctx: &ServiceContext,
    cart_id: &str,
    line_ids: Vec<String>,
) -> Result<CartLinesRemoveResponse, APIError> {
    execute_graphql(
        ctx,
        queries::cart_lines_remove_without_customer_scope(),
        json!({ "cartId": cart_id, "lineIds": line_ids }),
    )
    .await
}

pub async fn update_note_without_customer(
    ctx: &ServiceContext,
    cart_id: &str,
    note: &str,
) -> Result<CartNoteUpdateResponse, APIError> {
    execute_graphql(
        ctx,
        queries::cart_note_update_without_customer_scope(),
        json!({ "cartId": cart_id, "note": note }),
    )
    .await
}

pub async fn update_attributes_without_customer(
    ctx: &ServiceContext,
    cart_id: &str,
    attributes: Vec<AttributeInput>,
) -> Result<CartAttributesUpdateResponse, APIError> {
    execute_graphql(
        ctx,
        queries::cart_attributes_update_without_customer_scope(),
        json!({ "cartId": cart_id, "attributes": attributes }),
    )
    .await
}

pub async fn update_buyer_identity_without_customer(
    ctx: &ServiceContext,
    cart_id: &str,
    buyer_identity: CartBuyerIdentityInput,
) -> Result<CartBuyerIdentityUpdateResponse, APIError> {
    execute_graphql(
        ctx,
        queries::cart_buyer_identity_update_without_customer_scope(),
        json!({ "cartId": cart_id, "buyerIdentity": buyer_identity }),
    )
    .await
}

pub async fn update_discount_codes_without_customer(
    ctx: &ServiceContext,
    cart_id: &str,
    discount_codes: Vec<String>,
) -> Result<CartDiscountCodesUpdateResponse, APIError> {
    execute_graphql(
        ctx,
        queries::cart_discount_codes_update_without_customer_scope(),
        json!({ "cartId": cart_id, "discountCodes": discount_codes }),
    )
    .await
}

pub async fn add_gift_card_codes_without_customer(
    ctx: &ServiceContext,
    cart_id: &str,
    gift_card_codes: Vec<String>,
) -> Result<CartGiftCardCodesAddResponse, APIError> {
    execute_graphql(
        ctx,
        queries::cart_gift_card_codes_add_without_customer_scope(),
        json!({ "cartId": cart_id, "giftCardCodes": gift_card_codes }),
    )
    .await
}
