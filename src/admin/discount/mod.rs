pub mod remote;

use crate::common::ServiceContext;

use std::sync::Arc;

use crate::{
    common::types::{APIError, RequestCallbacks},
    types::discount::{
        CodeDiscountNodeByCodeResp, DiscountAutomaticActivateResp, DiscountAutomaticAppCreateResp,
        DiscountAutomaticAppInput, DiscountAutomaticAppUpdateInput, DiscountAutomaticAppUpdateResp,
        DiscountAutomaticDeactivateResp, DiscountAutomaticDeleteResp, DiscountCodeActivateResp,
        DiscountCodeAppCreateResp, DiscountCodeAppInput, DiscountCodeAppUpdateInput,
        DiscountCodeAppUpdateResp, DiscountCodeDeactivateResp, DiscountCodeDeleteResp,
        DiscountNodesResp, GetDiscountMetafieldResp, GetDiscountNodeResp, NativeDiscountNodesResp,
    },
};

pub struct Discount {
    pub(crate) ctx: ServiceContext,
}

impl Discount {
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

    pub async fn create_automatic_app_discount(
        &self,
        input: &DiscountAutomaticAppInput,
    ) -> Result<DiscountAutomaticAppCreateResp, APIError> {
        remote::create_automatic_app_discount(&self.ctx, input).await
    }

    pub async fn update_automatic_app_discount(
        &self,
        input: &DiscountAutomaticAppUpdateInput,
    ) -> Result<DiscountAutomaticAppUpdateResp, APIError> {
        remote::update_automatic_app_discount(&self.ctx, input).await
    }

    pub async fn list_discounts(
        &self,
        first: Option<i32>,
        after: Option<String>,
        query: Option<String>,
    ) -> Result<DiscountNodesResp, APIError> {
        remote::list_discounts(&self.ctx, first, after, query).await
    }

    pub async fn get_discount_by_id(
        &self,
        id: &str,
        first: Option<i32>,
        after: Option<String>,
    ) -> Result<GetDiscountNodeResp, APIError> {
        remote::get_discount_by_id(&self.ctx, id, first, after).await
    }

    pub async fn get_discount_metafield(
        &self,
        id: &str,
        namespace: &str,
        key: &str,
    ) -> Result<GetDiscountMetafieldResp, APIError> {
        remote::get_discount_metafield(&self.ctx, id, namespace, key).await
    }

    /// One page of the shop's active or scheduled Shopify-native discounts (automatic and code).
    pub async fn list_native_discounts(
        &self,
        first: i32,
        after: Option<String>,
    ) -> Result<NativeDiscountNodesResp, APIError> {
        remote::list_native_discounts(&self.ctx, first, after).await
    }

    pub async fn get_code_discount_by_code(
        &self,
        code: &str,
    ) -> Result<CodeDiscountNodeByCodeResp, APIError> {
        remote::get_code_discount_by_code(&self.ctx, code).await
    }

    pub async fn create_code_app_discount(
        &self,
        input: &DiscountCodeAppInput,
    ) -> Result<DiscountCodeAppCreateResp, APIError> {
        remote::create_code_app_discount(&self.ctx, input).await
    }

    pub async fn update_code_app_discount(
        &self,
        input: &DiscountCodeAppUpdateInput,
    ) -> Result<DiscountCodeAppUpdateResp, APIError> {
        remote::update_code_app_discount(&self.ctx, input).await
    }

    pub async fn delete_automatic_discount(
        &self,
        id: &str,
    ) -> Result<DiscountAutomaticDeleteResp, APIError> {
        remote::delete_automatic_discount(&self.ctx, id).await
    }

    pub async fn delete_code_discount(&self, id: &str) -> Result<DiscountCodeDeleteResp, APIError> {
        remote::delete_code_discount(&self.ctx, id).await
    }

    pub async fn activate_automatic_discount(
        &self,
        id: &str,
    ) -> Result<DiscountAutomaticActivateResp, APIError> {
        remote::activate_automatic_discount(&self.ctx, id).await
    }

    pub async fn deactivate_automatic_discount(
        &self,
        id: &str,
    ) -> Result<DiscountAutomaticDeactivateResp, APIError> {
        remote::deactivate_automatic_discount(&self.ctx, id).await
    }

    pub async fn activate_code_discount(
        &self,
        id: &str,
    ) -> Result<DiscountCodeActivateResp, APIError> {
        remote::activate_code_discount(&self.ctx, id).await
    }

    pub async fn deactivate_code_discount(
        &self,
        id: &str,
    ) -> Result<DiscountCodeDeactivateResp, APIError> {
        remote::deactivate_code_discount(&self.ctx, id).await
    }
}

#[cfg(test)]
mod tests {
    use crate::types::discount::{
        CodeDiscount, CodeDiscountNodeByCodeResp, DiscountCodeAppCreateResp, DiscountCodeAppInput,
        DiscountCodeDeactivateResp,
    };

    #[test]
    fn a_code_lookup_parses_an_app_discount_with_its_function() {
        let resp: CodeDiscountNodeByCodeResp = serde_json::from_str(
            r#"{"codeDiscountNodeByCode":{"id":"gid://shopify/DiscountCodeNode/1","codeDiscount":{"__typename":"DiscountCodeApp","title":"BZ20","status":"ACTIVE","appDiscountType":{"appKey":"k","functionId":"fn-1","title":null,"description":null}}}}"#,
        )
        .expect("a code discount lookup");

        let node = resp.code_discount_node_by_code.expect("a node");
        assert_eq!(node.id, "gid://shopify/DiscountCodeNode/1");
        match node.code_discount {
            CodeDiscount::DiscountCodeApp(app) => {
                assert_eq!(app.app_discount_type.function_id, "fn-1")
            }
            other => panic!("expected an app discount, got {other:?}"),
        }
    }

    #[test]
    fn a_code_lookup_parses_a_native_discount_and_a_miss() {
        let resp: CodeDiscountNodeByCodeResp = serde_json::from_str(
            r#"{"codeDiscountNodeByCode":{"id":"gid://shopify/DiscountCodeNode/2","codeDiscount":{"__typename":"DiscountCodeBasic","title":"SAVE8","status":"ACTIVE"}}}"#,
        )
        .expect("a code discount lookup");
        assert!(matches!(
            resp.code_discount_node_by_code
                .expect("a node")
                .code_discount,
            CodeDiscount::DiscountCodeBasic(_)
        ));

        let miss: CodeDiscountNodeByCodeResp =
            serde_json::from_str(r#"{"codeDiscountNodeByCode":null}"#).expect("a miss");
        assert!(miss.code_discount_node_by_code.is_none());
    }

    #[test]
    fn a_code_app_create_and_a_deactivate_parse() {
        let created: DiscountCodeAppCreateResp = serde_json::from_str(
            r#"{"discountCodeAppCreate":{"codeAppDiscount":{"discountId":"gid://shopify/DiscountCodeNode/3","title":"t","status":"ACTIVE"},"userErrors":[]}}"#,
        )
        .expect("a create response");
        assert_eq!(
            created
                .discount_code_app_create
                .code_app_discount
                .expect("a discount")
                .discount_id,
            "gid://shopify/DiscountCodeNode/3"
        );

        let deactivated: DiscountCodeDeactivateResp = serde_json::from_str(
            r#"{"discountCodeDeactivate":{"codeDiscountNode":{"id":"x"},"userErrors":[{"field":null,"message":"gone","code":null}]}}"#,
        )
        .expect("a deactivate response");
        assert_eq!(
            deactivated.discount_code_deactivate.user_errors[0].message,
            "gone"
        );
    }

    #[test]
    fn native_discounts_parse_and_app_discounts_read_as_other() {
        use crate::types::discount::{NativeDiscount, NativeDiscountNodesResp};
        let resp: NativeDiscountNodesResp = serde_json::from_str(
            r#"{"discountNodes":{"nodes":[
                {"id":"gid://shopify/DiscountCodeNode/1","discount":{"__typename":"DiscountCodeBasic","title":"SAVE10","status":"ACTIVE","codes":{"nodes":[{"code":"SAVE10"}]},"minimumRequirement":{"greaterThanOrEqualToSubtotal":{"amount":"500.0","currencyCode":"INR"}},"customerGets":{"value":{"percentage":0.1}}}},
                {"id":"gid://shopify/DiscountAutomaticNode/2","discount":{"__typename":"DiscountAutomaticFreeShipping","title":"Ship","status":"SCHEDULED","minimumRequirement":{"greaterThanOrEqualToQuantity":"3"}}},
                {"id":"gid://shopify/DiscountAutomaticNode/3","discount":{"__typename":"DiscountAutomaticApp"}}
            ],"pageInfo":{"hasNextPage":false,"endCursor":null}}}"#,
        )
        .expect("a native discount list");
        let nodes = &resp.discount_nodes.nodes;
        assert!(matches!(
            nodes[0].discount,
            NativeDiscount::DiscountCodeBasic(_)
        ));
        assert!(matches!(
            nodes[1].discount,
            NativeDiscount::DiscountAutomaticFreeShipping(_)
        ));
        assert!(matches!(nodes[2].discount, NativeDiscount::Other));
        assert!(!resp.discount_nodes.page_info.has_next_page);
    }

    #[test]
    fn a_code_app_input_uses_shopify_names_and_omits_unset_dates() {
        let input = DiscountCodeAppInput::new("t".into(), "BZ".into(), "fn-1".into())
            .with_starts_at("2026-01-01T00:00:00Z".into());
        let value = serde_json::to_value(&input).expect("serializable");
        assert_eq!(value["functionId"], "fn-1");
        assert_eq!(value["startsAt"], "2026-01-01T00:00:00Z");
        assert!(value.get("endsAt").is_none());
    }
}
