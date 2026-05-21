pub mod queries;
pub mod remote;

use crate::common::ServiceContext;

use std::sync::Arc;

use crate::common::types::{APIError, RequestCallbacks};
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

pub struct Customer {
    pub(crate) ctx: ServiceContext,
}

impl Customer {
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

    pub async fn get(
        &self,
        customer_access_token: &str,
        addresses_first: Option<i32>,
        orders_first: Option<i32>,
    ) -> Result<CustomerResponse, APIError> {
        remote::get(
            &self.ctx,
            customer_access_token,
            addresses_first.unwrap_or(10),
            orders_first.unwrap_or(10),
        )
        .await
    }

    pub async fn login(
        &self,
        input: CustomerAccessTokenCreateInput,
    ) -> Result<CustomerAccessTokenCreateResponse, APIError> {
        remote::login(&self.ctx, input).await
    }

    pub async fn renew_token(
        &self,
        customer_access_token: &str,
    ) -> Result<CustomerAccessTokenRenewResponse, APIError> {
        remote::renew_token(&self.ctx, customer_access_token).await
    }

    pub async fn logout(
        &self,
        customer_access_token: &str,
    ) -> Result<CustomerAccessTokenDeleteResponse, APIError> {
        remote::logout(&self.ctx, customer_access_token).await
    }

    pub async fn create(
        &self,
        input: CustomerCreateInput,
    ) -> Result<CustomerCreateResponse, APIError> {
        remote::create(&self.ctx, input).await
    }

    pub async fn update(
        &self,
        customer_access_token: &str,
        customer: CustomerUpdateInput,
    ) -> Result<CustomerUpdateResponse, APIError> {
        remote::update(&self.ctx, customer_access_token, customer).await
    }

    pub async fn recover(&self, email: &str) -> Result<CustomerRecoverResponse, APIError> {
        remote::recover(&self.ctx, email).await
    }

    pub async fn reset(
        &self,
        id: &str,
        input: CustomerResetInput,
    ) -> Result<CustomerResetResponse, APIError> {
        remote::reset(&self.ctx, id, input).await
    }

    pub async fn reset_by_url(
        &self,
        reset_url: &str,
        password: &str,
    ) -> Result<CustomerResetByUrlResponse, APIError> {
        remote::reset_by_url(&self.ctx, reset_url, password).await
    }

    pub async fn activate(
        &self,
        id: &str,
        input: CustomerActivateInput,
    ) -> Result<CustomerActivateResponse, APIError> {
        remote::activate(&self.ctx, id, input).await
    }

    pub async fn activate_by_url(
        &self,
        activation_url: &str,
        password: &str,
    ) -> Result<CustomerActivateByUrlResponse, APIError> {
        remote::activate_by_url(&self.ctx, activation_url, password).await
    }

    pub async fn create_address(
        &self,
        customer_access_token: &str,
        address: MailingAddressInput,
    ) -> Result<CustomerAddressCreateResponse, APIError> {
        remote::create_address(&self.ctx, customer_access_token, address).await
    }

    pub async fn update_address(
        &self,
        customer_access_token: &str,
        id: &str,
        address: MailingAddressInput,
    ) -> Result<CustomerAddressUpdateResponse, APIError> {
        remote::update_address(&self.ctx, customer_access_token, id, address).await
    }

    pub async fn delete_address(
        &self,
        customer_access_token: &str,
        id: &str,
    ) -> Result<CustomerAddressDeleteResponse, APIError> {
        remote::delete_address(&self.ctx, customer_access_token, id).await
    }

    pub async fn set_default_address(
        &self,
        customer_access_token: &str,
        address_id: &str,
    ) -> Result<CustomerDefaultAddressUpdateResponse, APIError> {
        remote::set_default_address(&self.ctx, customer_access_token, address_id).await
    }
}
