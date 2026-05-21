pub mod remote;

use crate::common::ServiceContext;

use std::future::Future;
use std::sync::Arc;

use crate::{
    common::types::{APIError, RequestCallbacks},
    types::{
        bulk_operation::{
            CancelResp, CollectionExportLine, CustomerExportLine, DraftOrderExportLine,
            GetBulkOperationResp, InventoryItemExportLine, ListBulkOperationsParams,
            ListBulkOperationsResp, OrderExportLine, ProductExportLine, RunMutationResp,
            RunQueryResp, StagedUploadInput, StagedUploadsCreateResp,
        },
        collection::CollectionQueryParams,
        customer::CustomerQueryParams,
        draft_order::DraftOrderQueryParams,
        inventory::InventoryItemQueryParams,
        order::OrderQueryParams,
        product::ProductQueryParams,
    },
};

pub struct BulkOperation {
    pub(crate) ctx: ServiceContext,
}

impl BulkOperation {
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

    // region: Raw Operations

    pub async fn run_query(
        &self,
        query: &str,
        group_objects: Option<bool>,
    ) -> Result<RunQueryResp, APIError> {
        remote::run_query(&self.ctx, query, group_objects).await
    }

    pub async fn run_mutation(
        &self,
        mutation: &str,
        staged_upload_path: &str,
        client_identifier: Option<&str>,
    ) -> Result<RunMutationResp, APIError> {
        remote::run_mutation(&self.ctx, mutation, staged_upload_path, client_identifier).await
    }

    pub async fn cancel(&self, id: &str) -> Result<CancelResp, APIError> {
        remote::cancel(&self.ctx, id).await
    }

    pub async fn get(&self, id: &str) -> Result<GetBulkOperationResp, APIError> {
        remote::get(&self.ctx, id).await
    }

    pub async fn list(
        &self,
        params: &ListBulkOperationsParams,
    ) -> Result<ListBulkOperationsResp, APIError> {
        remote::list(&self.ctx, params).await
    }

    pub async fn create_staged_upload(
        &self,
        input: &[StagedUploadInput],
    ) -> Result<StagedUploadsCreateResp, APIError> {
        remote::create_staged_upload(&self.ctx, input).await
    }

    // endregion

    // region: Export Templates

    pub async fn export_products(
        &self,
        params: Option<&ProductQueryParams>,
    ) -> Result<RunQueryResp, APIError> {
        let filter = params.and_then(|p| p.to_query_string());
        let query = remote::products_query(filter.as_deref());
        self.run_query(&query, None).await
    }

    pub async fn export_orders(
        &self,
        params: Option<&OrderQueryParams>,
    ) -> Result<RunQueryResp, APIError> {
        let filter = params.and_then(|p| p.to_query_string());
        let query = remote::orders_query(filter.as_deref());
        self.run_query(&query, None).await
    }

    pub async fn export_collections(
        &self,
        params: Option<&CollectionQueryParams>,
    ) -> Result<RunQueryResp, APIError> {
        let filter = params.and_then(|p| p.to_query_string());
        let query = remote::collections_query(filter.as_deref());
        self.run_query(&query, None).await
    }

    pub async fn export_customers(
        &self,
        params: Option<&CustomerQueryParams>,
    ) -> Result<RunQueryResp, APIError> {
        let filter = params.and_then(|p| p.to_query_string());
        let query = remote::customers_query(filter.as_deref());
        self.run_query(&query, None).await
    }

    pub async fn export_inventory_items(
        &self,
        params: Option<&InventoryItemQueryParams>,
    ) -> Result<RunQueryResp, APIError> {
        let filter = params.and_then(|p| p.to_query_string());
        let query = remote::inventory_items_query(filter.as_deref());
        self.run_query(&query, None).await
    }

    pub async fn export_draft_orders(
        &self,
        params: Option<&DraftOrderQueryParams>,
    ) -> Result<RunQueryResp, APIError> {
        let filter = params.and_then(|p| p.to_query_string());
        let query = remote::draft_orders_query(filter.as_deref());
        self.run_query(&query, None).await
    }

    // endregion

    // region: Streaming JSONL

    pub async fn stream_jsonl<F, Fut>(
        &self,
        url: &str,
        batch_size: usize,
        on_batch: F,
    ) -> Result<(), APIError>
    where
        F: FnMut(Vec<String>) -> Fut,
        Fut: Future<Output = Result<(), APIError>>,
    {
        download_and_batch(url, batch_size, |line| Some(line.to_string()), on_batch).await
    }

    pub async fn stream_products<F, Fut>(
        &self,
        url: &str,
        batch_size: usize,
        on_batch: F,
    ) -> Result<(), APIError>
    where
        F: FnMut(Vec<ProductExportLine>) -> Fut,
        Fut: Future<Output = Result<(), APIError>>,
    {
        download_and_batch(
            url,
            batch_size,
            |line| ProductExportLine::parse_line(line).ok(),
            on_batch,
        )
        .await
    }

    pub async fn stream_orders<F, Fut>(
        &self,
        url: &str,
        batch_size: usize,
        on_batch: F,
    ) -> Result<(), APIError>
    where
        F: FnMut(Vec<OrderExportLine>) -> Fut,
        Fut: Future<Output = Result<(), APIError>>,
    {
        download_and_batch(
            url,
            batch_size,
            |line| OrderExportLine::parse_line(line).ok(),
            on_batch,
        )
        .await
    }

    pub async fn stream_collections<F, Fut>(
        &self,
        url: &str,
        batch_size: usize,
        on_batch: F,
    ) -> Result<(), APIError>
    where
        F: FnMut(Vec<CollectionExportLine>) -> Fut,
        Fut: Future<Output = Result<(), APIError>>,
    {
        download_and_batch(
            url,
            batch_size,
            |line| CollectionExportLine::parse_line(line).ok(),
            on_batch,
        )
        .await
    }

    pub async fn stream_customers<F, Fut>(
        &self,
        url: &str,
        batch_size: usize,
        on_batch: F,
    ) -> Result<(), APIError>
    where
        F: FnMut(Vec<CustomerExportLine>) -> Fut,
        Fut: Future<Output = Result<(), APIError>>,
    {
        download_and_batch(
            url,
            batch_size,
            |line| CustomerExportLine::parse_line(line).ok(),
            on_batch,
        )
        .await
    }

    pub async fn stream_inventory_items<F, Fut>(
        &self,
        url: &str,
        batch_size: usize,
        on_batch: F,
    ) -> Result<(), APIError>
    where
        F: FnMut(Vec<InventoryItemExportLine>) -> Fut,
        Fut: Future<Output = Result<(), APIError>>,
    {
        download_and_batch(
            url,
            batch_size,
            |line| InventoryItemExportLine::parse_line(line).ok(),
            on_batch,
        )
        .await
    }

    pub async fn stream_draft_orders<F, Fut>(
        &self,
        url: &str,
        batch_size: usize,
        on_batch: F,
    ) -> Result<(), APIError>
    where
        F: FnMut(Vec<DraftOrderExportLine>) -> Fut,
        Fut: Future<Output = Result<(), APIError>>,
    {
        download_and_batch(
            url,
            batch_size,
            |line| DraftOrderExportLine::parse_line(line).ok(),
            on_batch,
        )
        .await
    }

    // endregion
}

async fn download_and_batch<T, P, F, Fut>(
    url: &str,
    batch_size: usize,
    mut parse: P,
    mut on_batch: F,
) -> Result<(), APIError>
where
    P: FnMut(&str) -> Option<T>,
    F: FnMut(Vec<T>) -> Fut,
    Fut: Future<Output = Result<(), APIError>>,
{
    let mut response = reqwest::get(url)
        .await
        .map_err(|_| APIError::NetworkError)?;

    let mut buffer = String::new();
    let mut batch: Vec<T> = Vec::with_capacity(batch_size);

    while let Some(chunk) = response.chunk().await.map_err(|_| APIError::NetworkError)? {
        buffer.push_str(&String::from_utf8_lossy(&chunk));

        while let Some(pos) = buffer.find('\n') {
            let line = &buffer[..pos];
            let line = line.trim();
            if !line.is_empty() {
                if let Some(item) = parse(line) {
                    batch.push(item);
                    if batch.len() >= batch_size {
                        on_batch(std::mem::take(&mut batch)).await?;
                        batch = Vec::with_capacity(batch_size);
                    }
                }
            }
            buffer = buffer[pos + 1..].to_string();
        }
    }

    let remaining = buffer.trim();
    if !remaining.is_empty() {
        if let Some(item) = parse(remaining) {
            batch.push(item);
        }
    }

    if !batch.is_empty() {
        on_batch(batch).await?;
    }

    Ok(())
}
