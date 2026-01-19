use crate::{
    common::types::APIError,
    types::subscription::{
        ActiveSubscriptionsResp, CancelSubscriptionResp, CreateCombinedSubscriptionRequest,
        CreateRecurringSubscriptionRequest, CreateSubscriptionResp, CreateUsageRecordRequest,
        CreateUsageRecordResp, CreateUsageSubscriptionRequest, ExtendTrialResp, MoneyInput,
        UpdateCappedAmountResp, AppPricingInterval,
    },
};
use serde_json::json;

#[derive(serde::Serialize)]
struct GraphQLRequest {
    query: String,
    variables: serde_json::Value,
}

#[derive(serde::Deserialize)]
struct GraphQLResponse<T> {
    data: Option<T>,
    errors: Option<Vec<GraphQLError>>,
}

#[derive(serde::Deserialize, Debug)]
struct GraphQLError {
    message: String,
}

async fn execute_graphql<T: serde::de::DeserializeOwned>(
    shop_url: &String,
    version: &String,
    access_token: &String,
    query: String,
    variables: serde_json::Value,
) -> Result<T, APIError> {
    let endpoint = format!("{}/admin/api/{}/graphql.json", shop_url, version);

    let request_body = GraphQLRequest { query, variables };

    let client = reqwest::Client::new();
    let response = client
        .post(&endpoint)
        .header("X-Shopify-Access-Token", access_token)
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await;

    match response {
        Ok(resp) => {
            let graphql_response = resp
                .json::<GraphQLResponse<T>>()
                .await
                .map_err(|_| APIError::FailedToParse)?;

            if let Some(errors) = graphql_response.errors {
                let error_messages: Vec<String> =
                    errors.iter().map(|e| e.message.clone()).collect();
                return Err(APIError::ServerError {
                    errors: error_messages.join(", "),
                });
            }

            graphql_response
                .data
                .ok_or(APIError::FailedToParse)
        }
        Err(_) => Err(APIError::NetworkError),
    }
}

pub async fn create_recurring_subscription(
    shop_url: &String,
    version: &String,
    access_token: &String,
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
    "#.to_string();

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

    // Add discount if provided
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

    execute_graphql(shop_url, version, access_token, query, variables).await
}

pub async fn create_usage_subscription(
    shop_url: &String,
    version: &String,
    access_token: &String,
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
    "#.to_string();

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

    execute_graphql(shop_url, version, access_token, query, variables).await
}

pub async fn create_combined_subscription(
    shop_url: &String,
    version: &String,
    access_token: &String,
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
    "#.to_string();

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

    // Add discount if provided
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

    execute_graphql(shop_url, version, access_token, query, variables).await
}

pub async fn cancel_subscription(
    shop_url: &String,
    version: &String,
    access_token: &String,
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
    "#.to_string();

    let variables = json!({
        "id": subscription_id,
        "prorate": prorate
    });

    execute_graphql(shop_url, version, access_token, query, variables).await
}

pub async fn extend_trial(
    shop_url: &String,
    version: &String,
    access_token: &String,
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
    "#.to_string();

    let variables = json!({
        "id": subscription_id,
        "days": days
    });

    execute_graphql(shop_url, version, access_token, query, variables).await
}

pub async fn update_capped_amount(
    shop_url: &String,
    version: &String,
    access_token: &String,
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
    "#.to_string();

    let variables = json!({
        "id": line_item_id,
        "cappedAmount": {
            "amount": capped_amount.amount,
            "currencyCode": capped_amount.currency_code
        }
    });

    execute_graphql(shop_url, version, access_token, query, variables).await
}

pub async fn create_usage_record(
    shop_url: &String,
    version: &String,
    access_token: &String,
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
    "#.to_string();

    let variables = json!({
        "subscriptionLineItemId": request.subscription_line_item_id,
        "price": {
            "amount": request.price,
            "currencyCode": request.currency_code
        },
        "description": request.description,
        "idempotencyKey": request.idempotency_key
    });

    execute_graphql(shop_url, version, access_token, query, variables).await
}

pub async fn get_active_subscriptions(
    shop_url: &String,
    version: &String,
    access_token: &String,
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
    "#.to_string();

    let variables = json!({});

    execute_graphql(shop_url, version, access_token, query, variables).await
}
