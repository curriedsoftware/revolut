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

use crate::{
    client::{Body, Client, Environment, HttpMethod},
    errors::ApiResult,
    merchant::client::MerchantAuthentication,
};

pub mod v10 {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Default, Deserialize, Serialize)]
    pub struct WebhookRequest {
        pub url: String,
        pub events: Vec<WebhookEvent>,
    }

    #[derive(Debug, Deserialize, strum::Display, Serialize)]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    #[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
    pub enum WebhookEvent {
        #[serde(alias = "order_completed")]
        OrderCompleted,
        #[serde(alias = "order_authorised")]
        OrderAuthorised,
        #[serde(alias = "order_cancelled")]
        OrderCancelled,
        #[serde(alias = "order_payment_authenticated")]
        OrderPaymentAuthenticated,
        #[serde(alias = "order_payment_declined")]
        OrderPaymentDeclined,
        #[serde(alias = "order_payment_failed")]
        OrderPaymentFailed,
        #[serde(alias = "payout_initiated")]
        PayoutInitiated,
        #[serde(alias = "payout_completed")]
        PayoutCompleted,
        #[serde(alias = "payout_failed")]
        PayoutFailed,
        #[serde(alias = "dispute_action_required")]
        DisputeActionRequired,
        #[serde(alias = "dispute_under_review")]
        DisputeUnderReview,
        #[serde(alias = "dispute_won")]
        DisputeWon,
        #[serde(alias = "dispute_lost")]
        DisputeLost,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Webhook {
        pub id: String,
        pub url: Option<String>,
        pub events: Option<Vec<WebhookEvent>>,
        pub signing_secret: String,
    }

    #[derive(Debug, Default, Deserialize, Serialize)]
    pub struct RotateWebhookSigningSecretRequest {
        pub expiration_period: Option<String>,
    }
}

pub async fn create<E: Environment>(
    client: &Client<E, MerchantAuthentication>,
    webhook: &v10::WebhookRequest,
) -> ApiResult<v10::Webhook> {
    client
        .request(
            HttpMethod::Post {
                body: Some(Body::Json(&webhook)),
            },
            &client.environment.uri("1.0", "/webhooks"),
        )
        .await
}

pub async fn list<E: Environment>(
    client: &Client<E, MerchantAuthentication>,
) -> ApiResult<Vec<v10::Webhook>> {
    client
        .request(
            HttpMethod::Get::<()>,
            &client.environment.uri("1.0", "/webhooks"),
        )
        .await
}

pub async fn retrieve<E: Environment>(
    client: &Client<E, MerchantAuthentication>,
    webhook_id: &str,
) -> ApiResult<v10::Webhook> {
    client
        .request(
            HttpMethod::Get::<()>,
            &client
                .environment
                .uri("1.0", &format!("/webhooks/{webhook_id}")),
        )
        .await
}

pub async fn update<E: Environment>(
    client: &Client<E, MerchantAuthentication>,
    webhook_id: &str,
    webhook: &v10::WebhookRequest,
) -> ApiResult<v10::Webhook> {
    client
        .request(
            HttpMethod::Put {
                body: Some(Body::Json(&webhook)),
            },
            &client
                .environment
                .uri("1.0", &format!("/webhooks/{webhook_id}")),
        )
        .await
}

pub async fn delete<E: Environment>(
    client: &Client<E, MerchantAuthentication>,
    webhook_id: &str,
) -> ApiResult<()> {
    client
        .request(
            HttpMethod::Delete::<()>,
            &client
                .environment
                .uri("1.0", &format!("/webhooks/{webhook_id}")),
        )
        .await
}

pub async fn rotate_signing_secret<E: Environment>(
    client: &Client<E, MerchantAuthentication>,
    webhook_id: &str,
    rotate_webhook_signing_secret: &v10::RotateWebhookSigningSecretRequest,
) -> ApiResult<v10::Webhook> {
    client
        .request(
            HttpMethod::Post {
                body: Some(Body::Json(&rotate_webhook_signing_secret)),
            },
            &client
                .environment
                .uri("1.0", &format!("/webhooks/{webhook_id}")),
        )
        .await
}
