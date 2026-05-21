use crate::common::ServiceContext;

use crate::common::types::APIError;
use serde_json::json;

use super::queries;
use crate::common::http::execute_storefront_graphql as execute_graphql;
use crate::storefront::generated::types::customer::{
    CustomerAccessTokenCreateInput, CustomerActivateInput, CustomerCreateInput, CustomerResetInput,
    CustomerUpdateInput, MailingAddressInput,
};
use crate::storefront::generated::types::responses::{
    CustomerAccessTokenCreateResponse, CustomerAccessTokenDeleteResponse,
    CustomerAccessTokenRenewResponse, CustomerActivateByUrlResponse, CustomerActivateResponse,
    CustomerAddressCreateResponse, CustomerAddressDeleteResponse, CustomerAddressUpdateResponse,
    CustomerCreateResponse, CustomerDefaultAddressUpdateResponse, CustomerRecoverResponse,
    CustomerResetByUrlResponse, CustomerResetResponse, CustomerResponse, CustomerUpdateResponse,
};

pub async fn get(
    ctx: &ServiceContext,
    customer_access_token: &str,
    addresses_first: i32,
    orders_first: i32,
) -> Result<CustomerResponse, APIError> {
    execute_graphql(
        ctx,
        queries::get_customer(),
        json!({
            "customerAccessToken": customer_access_token,
            "addressesFirst": addresses_first,
            "ordersFirst": orders_first,
        }),
    )
    .await
}

pub async fn login(
    ctx: &ServiceContext,
    input: CustomerAccessTokenCreateInput,
) -> Result<CustomerAccessTokenCreateResponse, APIError> {
    execute_graphql(
        ctx,
        queries::customer_access_token_create(),
        json!({ "input": input }),
    )
    .await
}

pub async fn renew_token(
    ctx: &ServiceContext,
    customer_access_token: &str,
) -> Result<CustomerAccessTokenRenewResponse, APIError> {
    execute_graphql(
        ctx,
        queries::customer_access_token_renew(),
        json!({ "customerAccessToken": customer_access_token }),
    )
    .await
}

pub async fn logout(
    ctx: &ServiceContext,
    customer_access_token: &str,
) -> Result<CustomerAccessTokenDeleteResponse, APIError> {
    execute_graphql(
        ctx,
        queries::customer_access_token_delete(),
        json!({ "customerAccessToken": customer_access_token }),
    )
    .await
}

pub async fn create(
    ctx: &ServiceContext,
    input: CustomerCreateInput,
) -> Result<CustomerCreateResponse, APIError> {
    execute_graphql(ctx, queries::customer_create(), json!({ "input": input })).await
}

pub async fn update(
    ctx: &ServiceContext,
    customer_access_token: &str,
    customer_input: CustomerUpdateInput,
) -> Result<CustomerUpdateResponse, APIError> {
    execute_graphql(
        ctx,
        queries::customer_update(),
        json!({
            "customerAccessToken": customer_access_token,
            "customer": customer_input,
        }),
    )
    .await
}

pub async fn recover(
    ctx: &ServiceContext,
    email: &str,
) -> Result<CustomerRecoverResponse, APIError> {
    execute_graphql(ctx, queries::customer_recover(), json!({ "email": email })).await
}

pub async fn reset(
    ctx: &ServiceContext,
    id: &str,
    input: CustomerResetInput,
) -> Result<CustomerResetResponse, APIError> {
    execute_graphql(
        ctx,
        queries::customer_reset(),
        json!({ "id": id, "input": input }),
    )
    .await
}

pub async fn reset_by_url(
    ctx: &ServiceContext,
    reset_url: &str,
    password: &str,
) -> Result<CustomerResetByUrlResponse, APIError> {
    execute_graphql(
        ctx,
        queries::customer_reset_by_url(),
        json!({ "resetUrl": reset_url, "password": password }),
    )
    .await
}

pub async fn activate(
    ctx: &ServiceContext,
    id: &str,
    input: CustomerActivateInput,
) -> Result<CustomerActivateResponse, APIError> {
    execute_graphql(
        ctx,
        queries::customer_activate(),
        json!({ "id": id, "input": input }),
    )
    .await
}

pub async fn activate_by_url(
    ctx: &ServiceContext,
    activation_url: &str,
    password: &str,
) -> Result<CustomerActivateByUrlResponse, APIError> {
    execute_graphql(
        ctx,
        queries::customer_activate_by_url(),
        json!({ "activationUrl": activation_url, "password": password }),
    )
    .await
}

pub async fn create_address(
    ctx: &ServiceContext,
    customer_access_token: &str,
    address: MailingAddressInput,
) -> Result<CustomerAddressCreateResponse, APIError> {
    execute_graphql(
        ctx,
        queries::customer_address_create(),
        json!({
            "customerAccessToken": customer_access_token,
            "address": address,
        }),
    )
    .await
}

pub async fn update_address(
    ctx: &ServiceContext,
    customer_access_token: &str,
    id: &str,
    address: MailingAddressInput,
) -> Result<CustomerAddressUpdateResponse, APIError> {
    execute_graphql(
        ctx,
        queries::customer_address_update(),
        json!({
            "customerAccessToken": customer_access_token,
            "id": id,
            "address": address,
        }),
    )
    .await
}

pub async fn delete_address(
    ctx: &ServiceContext,
    customer_access_token: &str,
    id: &str,
) -> Result<CustomerAddressDeleteResponse, APIError> {
    execute_graphql(
        ctx,
        queries::customer_address_delete(),
        json!({
            "customerAccessToken": customer_access_token,
            "id": id,
        }),
    )
    .await
}

pub async fn set_default_address(
    ctx: &ServiceContext,
    customer_access_token: &str,
    address_id: &str,
) -> Result<CustomerDefaultAddressUpdateResponse, APIError> {
    execute_graphql(
        ctx,
        queries::customer_default_address_update(),
        json!({
            "customerAccessToken": customer_access_token,
            "addressId": address_id,
        }),
    )
    .await
}
