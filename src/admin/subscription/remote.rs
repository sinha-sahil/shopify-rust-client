use crate::common::ServiceContext;
use crate::{
    common::{http::execute_graphql, types::APIError},
    types::subscription::{
        ActiveSubscriptionsResp, AppPricingInterval, CancelSubscriptionResp,
        CreateCombinedSubscriptionRequest, CreateRecurringSubscriptionRequest,
        CreateSubscriptionResp, CreateUsageRecordRequest, CreateUsageRecordResp,
        CreateUsageSubscriptionRequest, ExtendTrialResp, MoneyInput, UpdateCappedAmountResp,
    },
};

use serde_json::json;

pub async fn create_recurring_subscription(
    ctx: &ServiceContext,
    request: &CreateRecurringSubscriptionRequest,
) -> Result<CreateSubscriptionResp, APIError> {
    let interval = request.interval.unwrap_or(AppPricingInterval::Every30Days);
    let test = request.test.unwrap_or(false);

    let query = r#"
        mutation appSubscriptionCreate($name: String!, $returnUrl: URL!, $lineItems: [AppSubscriptionLineItemInput!]!, $test: Boolean, $trialDays: Int) {
            appSubscriptionCreate(name: $name, returnUrl: $returnUrl, lineItems: $lineItems, test: $test, trialDays: $trialDays) {
                appSubscription {
                    id
                    name
                    status
                    lineItems {
                        id
                        plan {
                            pricingDetails {
                                __typename
                                ... on AppRecurringPricing {
                                    price {
                                        amount
                                        currencyCode
                                    }
                                    interval
                                }
                            }
                        }
                    }
                    createdAt
                    currentPeriodEnd
                    returnUrl
                    trialDays
                    test
                }
                confirmationUrl
                userErrors {
                    field
                    message
                }
            }
        }
    "#;

    let mut line_item = json!({
        "plan": {
            "appRecurringPricingDetails": {
                "price": {
                    "amount": request.price,
                    "currencyCode": request.currency_code
                },
                "interval": interval
            }
        }
    });

    if let Some(discount) = &request.discount {
        line_item["plan"]["appRecurringPricingDetails"]["discount"] = json!(discount);
    }

    let variables = json!({
        "name": request.name,
        "returnUrl": request.return_url,
        "lineItems": [line_item],
        "test": test,
        "trialDays": request.trial_days
    });

    execute_graphql(ctx, query, variables).await
}

pub async fn create_usage_subscription(
    ctx: &ServiceContext,
    request: &CreateUsageSubscriptionRequest,
) -> Result<CreateSubscriptionResp, APIError> {
    let test = request.test.unwrap_or(false);

    let query = r#"
        mutation appSubscriptionCreate($name: String!, $returnUrl: URL!, $lineItems: [AppSubscriptionLineItemInput!]!, $test: Boolean, $trialDays: Int) {
            appSubscriptionCreate(name: $name, returnUrl: $returnUrl, lineItems: $lineItems, test: $test, trialDays: $trialDays) {
                appSubscription {
                    id
                    name
                    status
                    lineItems {
                        id
                        plan {
                            pricingDetails {
                                __typename
                                ... on AppUsagePricing {
                                    cappedAmount {
                                        amount
                                        currencyCode
                                    }
                                    terms
                                    balanceUsed {
                                        amount
                                        currencyCode
                                    }
                                    interval
                                }
                            }
                        }
                    }
                    createdAt
                    currentPeriodEnd
                    returnUrl
                    trialDays
                    test
                }
                confirmationUrl
                userErrors {
                    field
                    message
                }
            }
        }
    "#;

    let variables = json!({
        "name": request.name,
        "returnUrl": request.return_url,
        "lineItems": [{
            "plan": {
                "appUsagePricingDetails": {
                    "cappedAmount": {
                        "amount": request.capped_amount,
                        "currencyCode": request.currency_code
                    },
                    "terms": request.terms
                }
            }
        }],
        "test": test,
        "trialDays": request.trial_days
    });

    execute_graphql(ctx, query, variables).await
}

pub async fn create_combined_subscription(
    ctx: &ServiceContext,
    request: &CreateCombinedSubscriptionRequest,
) -> Result<CreateSubscriptionResp, APIError> {
    let interval = request.interval.unwrap_or(AppPricingInterval::Every30Days);
    let test = request.test.unwrap_or(false);

    let query = r#"
        mutation appSubscriptionCreate($name: String!, $returnUrl: URL!, $lineItems: [AppSubscriptionLineItemInput!]!, $test: Boolean, $trialDays: Int) {
            appSubscriptionCreate(name: $name, returnUrl: $returnUrl, lineItems: $lineItems, test: $test, trialDays: $trialDays) {
                appSubscription {
                    id
                    name
                    status
                    lineItems {
                        id
                        plan {
                            pricingDetails {
                                __typename
                                ... on AppRecurringPricing {
                                    price {
                                        amount
                                        currencyCode
                                    }
                                    interval
                                }
                                ... on AppUsagePricing {
                                    cappedAmount {
                                        amount
                                        currencyCode
                                    }
                                    terms
                                    balanceUsed {
                                        amount
                                        currencyCode
                                    }
                                    interval
                                }
                            }
                        }
                    }
                    createdAt
                    currentPeriodEnd
                    returnUrl
                    trialDays
                    test
                }
                confirmationUrl
                userErrors {
                    field
                    message
                }
            }
        }
    "#;

    let mut recurring_line_item = json!({
        "plan": {
            "appRecurringPricingDetails": {
                "price": {
                    "amount": request.recurring_price,
                    "currencyCode": request.recurring_currency_code
                },
                "interval": interval
            }
        }
    });

    if let Some(discount) = &request.discount {
        recurring_line_item["plan"]["appRecurringPricingDetails"]["discount"] = json!(discount);
    }

    let usage_line_item = json!({
        "plan": {
            "appUsagePricingDetails": {
                "cappedAmount": {
                    "amount": request.capped_amount,
                    "currencyCode": request.usage_currency_code
                },
                "terms": request.terms
            }
        }
    });

    let variables = json!({
        "name": request.name,
        "returnUrl": request.return_url,
        "lineItems": [recurring_line_item, usage_line_item],
        "test": test,
        "trialDays": request.trial_days
    });

    execute_graphql(ctx, query, variables).await
}

pub async fn cancel_subscription(
    ctx: &ServiceContext,
    subscription_id: &String,
    prorate: bool,
) -> Result<CancelSubscriptionResp, APIError> {
    let query = r#"
        mutation appSubscriptionCancel($id: ID!, $prorate: Boolean) {
            appSubscriptionCancel(id: $id, prorate: $prorate) {
                appSubscription {
                    id
                    name
                    status
                    lineItems {
                        id
                        plan {
                            pricingDetails {
                                __typename
                                ... on AppRecurringPricing {
                                    price {
                                        amount
                                        currencyCode
                                    }
                                    interval
                                }
                                ... on AppUsagePricing {
                                    cappedAmount {
                                        amount
                                        currencyCode
                                    }
                                    terms
                                    balanceUsed {
                                        amount
                                        currencyCode
                                    }
                                    interval
                                }
                            }
                        }
                    }
                }
                userErrors {
                    field
                    message
                }
            }
        }
    "#;

    let variables = json!({
        "id": subscription_id,
        "prorate": prorate
    });

    execute_graphql(ctx, query, variables).await
}

pub async fn extend_trial(
    ctx: &ServiceContext,
    subscription_id: &String,
    days: i32,
) -> Result<ExtendTrialResp, APIError> {
    let query = r#"
        mutation appSubscriptionTrialExtend($id: ID!, $days: Int!) {
            appSubscriptionTrialExtend(id: $id, days: $days) {
                appSubscription {
                    id
                    name
                    status
                    trialDays
                }
                userErrors {
                    field
                    message
                }
            }
        }
    "#;

    let variables = json!({
        "id": subscription_id,
        "days": days
    });

    execute_graphql(ctx, query, variables).await
}

pub async fn update_capped_amount(
    ctx: &ServiceContext,
    line_item_id: &String,
    capped_amount: &MoneyInput,
) -> Result<UpdateCappedAmountResp, APIError> {
    let query = r#"
        mutation appSubscriptionLineItemUpdate($id: ID!, $cappedAmount: MoneyInput!) {
            appSubscriptionLineItemUpdate(id: $id, cappedAmount: $cappedAmount) {
                appSubscription {
                    id
                    name
                    status
                    lineItems {
                        id
                        plan {
                            pricingDetails {
                                __typename
                                ... on AppUsagePricing {
                                    cappedAmount {
                                        amount
                                        currencyCode
                                    }
                                    terms
                                    balanceUsed {
                                        amount
                                        currencyCode
                                    }
                                    interval
                                }
                            }
                        }
                    }
                }
                userErrors {
                    field
                    message
                }
            }
        }
    "#;

    let variables = json!({
        "id": line_item_id,
        "cappedAmount": {
            "amount": capped_amount.amount,
            "currencyCode": capped_amount.currency_code
        }
    });

    execute_graphql(ctx, query, variables).await
}

pub async fn create_usage_record(
    ctx: &ServiceContext,
    request: &CreateUsageRecordRequest,
) -> Result<CreateUsageRecordResp, APIError> {
    let query = r#"
        mutation appUsageRecordCreate($subscriptionLineItemId: ID!, $price: MoneyInput!, $description: String!, $idempotencyKey: String) {
            appUsageRecordCreate(subscriptionLineItemId: $subscriptionLineItemId, price: $price, description: $description, idempotencyKey: $idempotencyKey) {
                appUsageRecord {
                    id
                    description
                    price {
                        amount
                        currencyCode
                    }
                    createdAt
                }
                userErrors {
                    field
                    message
                }
            }
        }
    "#;

    let variables = json!({
        "subscriptionLineItemId": request.subscription_line_item_id,
        "price": {
            "amount": request.price,
            "currencyCode": request.currency_code
        },
        "description": request.description,
        "idempotencyKey": request.idempotency_key
    });

    execute_graphql(ctx, query, variables).await
}

pub async fn get_active_subscriptions(
    ctx: &ServiceContext,
) -> Result<ActiveSubscriptionsResp, APIError> {
    let query = r#"
        query {
            currentAppInstallation {
                activeSubscriptions {
                    id
                    name
                    status
                    lineItems {
                        id
                        plan {
                            pricingDetails {
                                __typename
                                ... on AppRecurringPricing {
                                    price {
                                        amount
                                        currencyCode
                                    }
                                    interval
                                }
                                ... on AppUsagePricing {
                                    cappedAmount {
                                        amount
                                        currencyCode
                                    }
                                    terms
                                    balanceUsed {
                                        amount
                                        currencyCode
                                    }
                                    interval
                                }
                            }
                        }
                    }
                    createdAt
                    currentPeriodEnd
                    returnUrl
                    trialDays
                    test
                }
            }
        }
    "#;

    let variables = json!({});

    execute_graphql(ctx, query, variables).await
}
