/***
 * Copyright (c) 2025 Rafael Fernández López <ereslibre@curried.software>
 *
 * Permission is hereby granted, free of charge, to any person
 * obtaining a copy of this software and associated documentation
 * files (the "Software"), to deal in the Software without
 * restriction, including without limitation the rights to use, copy,
 * modify, merge, publish, distribute, sublicense, and/or sell copies
 * of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be
 * included in all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 * EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
 * NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
 * BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
 * ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 ***/

//! [Business webhooks API v2](https://developer.revolut.com/docs/business/webhooks-v-2).

use crate::{
    business::client::{self, BusinessAuthentication, Environment, HttpMethod},
    client::{Body, Client, ProductionEnvironment},
    errors::ApiResult,
};

pub mod v20 {
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct WebhookRequest {
        pub url: String,
        pub events: Option<Vec<WebhookEvent>>,
    }

    #[derive(Clone, Debug, Deserialize, strum::Display, Serialize)]
    #[serde(rename_all = "snake_case")]
    #[strum(serialize_all = "snake_case")]
    pub enum WebhookEvent {
        #[serde(alias = "TRANSACTION_CREATED")]
        TransactionCreated,
        #[serde(alias = "TRANSACTION_STATE_CHANGED")]
        TransactionStateChanged,
        #[serde(alias = "TRANSACTION_LINK_CREATED")]
        PayoutLinkCreated,
        #[serde(alias = "TRANSACTION_LINK_STATE_CHANGED")]
        PayoutLinkStateChanged,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct WebhookCreationResponse {
        pub id: String,
        pub url: String,
        pub events: Vec<WebhookEvent>,
        pub signing_secret: String,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Webhook {
        pub id: String,
        pub url: String,
        pub events: Vec<WebhookEvent>,
    }

    #[derive(Debug, Default, Deserialize, Serialize)]
    pub struct RotateWebhookSigningSecretRequest {
        pub expiration_period: Option<String>,
    }

    #[derive(Debug, Default, Deserialize, Serialize)]
    pub struct FailedWebhookEventsListParams {
        pub limit: Option<u64>,
        pub created_before: Option<String>,
    }

    #[derive(Debug, Default, Deserialize, Serialize)]
    pub struct FailedWebhookEvent {
        pub id: String,
        pub created_at: String,
        pub updated_at: String,
        pub webook_id: String,
        pub webhook_url: String,
        pub payload: String,
        pub last_sent_date: Option<String>,
    }
}

impl std::fmt::Display for v20::FailedWebhookEventsListParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let query = [
            ("limit", &self.limit.map(|limit| limit.to_string())),
            ("created_before", &self.created_before),
        ]
        .iter()
        .fold(String::new(), |acc, (key, value)| {
            if let Some(value) = value {
                let value = urlencoding::encode(value);
                if acc.is_empty() {
                    format!("{acc}?{key}={value}")
                } else {
                    format!("{acc}&{key}={value}")
                }
            } else {
                acc
            }
        });
        write!(f, "{query}")
    }
}

pub async fn create<E: Environment>(
    client: &Client<E, BusinessAuthentication>,
    webhook: &v20::WebhookRequest,
) -> ApiResult<v20::WebhookCreationResponse> {
    client
        .request(
            HttpMethod::Post {
                body: Some(Body::Json(&webhook)),
            },
            &client.environment.uri("2.0", "/webhooks"),
        )
        .await
}

pub async fn list<E: Environment>(
    client: &Client<E, BusinessAuthentication>,
) -> ApiResult<Vec<v20::Webhook>> {
    client
        .request(
            HttpMethod::Get::<()>,
            &client.environment.uri("2.0", "/webhooks"),
        )
        .await
}

pub async fn retrieve<E: Environment>(
    client: &Client<E, BusinessAuthentication>,
    webhook_id: &str,
) -> ApiResult<v20::Webhook> {
    client
        .request(
            HttpMethod::Get::<()>,
            &client
                .environment
                .uri("2.0", &format!("/webhooks/{webhook_id}")),
        )
        .await
}

pub async fn update<E: Environment>(
    client: &Client<E, BusinessAuthentication>,
    webhook_id: &str,
    webhook: &v20::WebhookRequest,
) -> ApiResult<v20::Webhook> {
    client
        .request(
            HttpMethod::Patch {
                body: Some(Body::Json(&webhook)),
            },
            &client
                .environment
                .uri("2.0", &format!("/webhooks/{webhook_id}")),
        )
        .await
}

pub async fn delete<E: Environment>(
    client: &Client<E, BusinessAuthentication>,
    webhook_id: &str,
) -> ApiResult<()> {
    client
        .request(
            HttpMethod::Delete::<()>,
            &client
                .environment
                .uri("2.0", &format!("/webhooks/{webhook_id}")),
        )
        .await
}

pub async fn rotate_signing_secret<E: Environment>(
    client: &Client<E, BusinessAuthentication>,
    webhook_id: &str,
    rotate_webhook_signing_secret: &v20::RotateWebhookSigningSecretRequest,
) -> ApiResult<v20::Webhook> {
    client
        .request(
            HttpMethod::Post {
                body: Some(Body::Json(&rotate_webhook_signing_secret)),
            },
            &client.environment.uri(
                "1.0",
                &format!("/webhooks/{webhook_id}/rotate-signing-secret"),
            ),
        )
        .await
}

pub async fn failed_webhook_events<E: Environment>(
    client: &Client<E, BusinessAuthentication>,
    webhook_id: &str,
    list_params: &v20::FailedWebhookEventsListParams,
) -> ApiResult<Vec<v20::FailedWebhookEvent>> {
    client
        .request(
            HttpMethod::Get::<()>,
            &client.environment.uri(
                "2.0",
                &format!("/webhooks/{webhook_id}/failed-events{list_params}"),
            ),
        )
        .await
}
