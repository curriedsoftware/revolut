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

//! [Business payment drafts API](https://developer.revolut.com/docs/business/payment-drafts).

use crate::{
    business::client::BusinessAuthentication,
    client::{Body, Client, Environment, HttpMethod},
    errors::ApiResult,
};

pub mod v10 {
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct PaymentDraft {
        pub payment_orders: Vec<PaymentOrder>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct PaymentOrder {
        pub id: String,
        pub scheduled_for: Option<String>,
        pub title: Option<String>,
        pub payments_count: u64,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Payment {
        pub id: String,
        pub amount: Amount,
        pub currency: Option<String>,
        pub account_id: String,
        pub receiver: String,
        pub state: PaymentState,
        pub reason: Option<String>,
        pub error_message: Option<String>,
        pub current_charge_options: CurrentChargeOptions,
        pub reference: Option<String>,
    }

    #[derive(Clone, Debug, Default, Deserialize, Serialize)]
    pub struct PaymentDraftRequest {
        pub title: Option<String>,
        pub schedule_for: Option<String>,
        pub payments: Vec<PaymentRequest>,
    }

    #[derive(Clone, Debug, Default, Deserialize, Serialize)]
    pub struct PaymentRequest {
        pub account_id: String,
        pub receiver: PaymentReceiver,
        pub amount: f64,
        pub currency: String,
        pub reference: String,
    }

    #[derive(Clone, Debug, Default, Deserialize, Serialize)]
    pub struct PaymentReceiver {
        pub counterparty_id: Option<String>,
        pub account_id: Option<String>,
        pub card_id: Option<String>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CreatePaymentDraft {
        pub id: String,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct PaymentDraftDetails {
        pub scheduled_for: Option<String>,
        pub title: Option<String>,
        pub payments: Vec<Payment>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Amount {
        pub amount: Option<f64>,
        pub currency: Option<String>,
    }

    // SCREAMING_SNAKE_CASE
    #[derive(Clone, Debug, Deserialize, strum::Display, Serialize)]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    pub enum PaymentState {
        Created,
        Pending,
        Completed,
        Reverted,
        Declined,
        Cancelled,
        Failed,
        Deleted,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CurrentChargeOptions {
        pub from: Amount,
        pub to: Amount,
        pub rate: Option<String>,
        pub fee: Option<Fee>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Fee {
        pub amount: Option<f64>,
        pub currency: Option<String>,
    }
}

pub async fn list<E: Environment>(
    client: &Client<E, BusinessAuthentication>,
) -> ApiResult<v10::PaymentDraft> {
    client
        .request(
            HttpMethod::<()>::Get,
            &client.environment.uri("1.0", "/payment-drafts"),
        )
        .await
}

pub async fn create<E: Environment>(
    client: &Client<E, BusinessAuthentication>,
    payment_draft: &v10::PaymentDraftRequest,
) -> ApiResult<v10::CreatePaymentDraft> {
    client
        .request(
            HttpMethod::Post {
                body: Some(Body::Json(&payment_draft)),
            },
            &client.environment.uri("1.0", "/payment-drafts"),
        )
        .await
}

pub async fn retrieve<E: Environment>(
    client: &Client<E, BusinessAuthentication>,
    payment_draft_id: &str,
) -> ApiResult<v10::PaymentDraftDetails> {
    client
        .request(
            HttpMethod::<()>::Get,
            &client
                .environment
                .uri("1.0", &format!("/payment-drafts/{payment_draft_id}")),
        )
        .await
}

pub async fn delete<E: Environment>(
    client: &Client<E, BusinessAuthentication>,
    payment_draft_id: &str,
) -> ApiResult<()> {
    client
        .request(
            HttpMethod::<()>::Delete,
            &client
                .environment
                .uri("1.0", &format!("/payment-drafts/{payment_draft_id}")),
        )
        .await
}

#[cfg(test)]
mod tests {
    use super::v10::*;

    impl Default for PaymentDraft {
        fn default() -> Self {
            Self {
                payment_orders: Vec::new(),
            }
        }
    }

    impl Default for PaymentOrder {
        fn default() -> Self {
            Self {
                id: "some-payment-order-id".to_string(),
                scheduled_for: None,
                title: None,
                payments_count: 1,
            }
        }
    }

    impl Default for Payment {
        fn default() -> Self {
            Self {
                id: "some-payment-id".to_string(),
                amount: Default::default(),
                currency: None,
                account_id: "some-account-id".to_string(),
                receiver: "some-receiver".to_string(),
                state: Default::default(),
                reason: None,
                error_message: None,
                current_charge_options: Default::default(),
                reference: None,
            }
        }
    }

    impl Default for CreatePaymentDraft {
        fn default() -> Self {
            Self {
                id: "some-payment-draft".to_string(),
            }
        }
    }

    impl Default for PaymentDraftDetails {
        fn default() -> Self {
            Self {
                scheduled_for: None,
                title: None,
                payments: Vec::new(),
            }
        }
    }

    impl Default for Amount {
        fn default() -> Self {
            Self {
                amount: None,
                currency: None,
            }
        }
    }

    impl Default for PaymentState {
        fn default() -> Self {
            Self::Completed
        }
    }

    impl Default for CurrentChargeOptions {
        fn default() -> Self {
            Self {
                from: Default::default(),
                to: Default::default(),
                rate: None,
                fee: None,
            }
        }
    }

    impl Default for Fee {
        fn default() -> Self {
            Self {
                amount: None,
                currency: None,
            }
        }
    }
}
