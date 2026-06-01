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

//! [Business transfers API](https://developer.revolut.com/docs/business/transfers).

use crate::{
    business::client::{BusinessAuthentication, Environment, HttpMethod},
    client::{Body, Client},
    errors::ApiResult,
};

pub mod v10 {
    use serde::{Deserialize, Serialize};

    // Reused verbatim from the OpenAPI-generated types: structurally identical.
    pub use crate::business::generated::{ExchangeReason, TransferReason};

    #[derive(Debug, Default, Deserialize, Serialize)]
    pub struct PayRequest {
        pub request_id: String,
        pub account_id: String,
        pub receiver: TransferReceiver,
        pub amont: f64,
        pub currency: Option<String>,
        pub reference: Option<String>,
        pub charge_bearer: Option<String>,
        pub transfer_reason_code: Option<String>,
        pub exchange_reason_code: Option<String>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Pay {
        pub id: String,
        pub state: TransferState,
        pub created_at: String,
        pub completed_at: Option<String>,
    }

    #[derive(Debug, Default, Deserialize, Serialize)]
    pub struct TransferReceiver {
        pub counterparty_id: String,
        pub account_id: Option<String>,
        pub card_id: Option<String>,
    }

    #[derive(Debug, Default, Deserialize, Serialize)]
    pub struct TransferRequest {
        pub request_id: String,
        pub source_account_id: String,
        pub target_account_id: String,
        pub amount: f64,
        pub currency: String,
        pub reference: Option<String>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Transfer {
        pub id: String,
        pub state: TransferState,
        pub created_at: String,
        pub completed_at: Option<String>,
    }

    #[derive(Clone, Debug, Deserialize, strum::Display, Serialize)]
    #[serde(rename_all = "snake_case")]
    pub enum TransferState {
        Created,
        Pending,
        Completed,
        Declined,
        Failed,
        Reverted,
    }
}

pub async fn get_transfer_reasons<E: Environment>(
    client: &Client<E, BusinessAuthentication>,
) -> ApiResult<Vec<v10::TransferReason>> {
    client
        .request(
            HttpMethod::<()>::Get,
            &client.environment.uri("1.0", "/transfer-reasons"),
        )
        .await
}

pub async fn get_exchange_reasons<E: Environment>(
    client: &Client<E, BusinessAuthentication>,
) -> ApiResult<Vec<v10::ExchangeReason>> {
    client
        .request(
            HttpMethod::<()>::Get,
            &client.environment.uri("1.0", "/exchange-reasons"),
        )
        .await
}

pub async fn transfer<E: Environment>(
    client: &Client<E, BusinessAuthentication>,
    transfer_params: &v10::TransferRequest,
) -> ApiResult<v10::Transfer> {
    client
        .request(
            HttpMethod::Post {
                body: Some(Body::Json(&transfer_params)),
            },
            &client.environment.uri("1.0", "/transfer"),
        )
        .await
}

pub async fn pay<E: Environment>(
    client: &Client<E, BusinessAuthentication>,
    pay_params: &v10::PayRequest,
) -> ApiResult<v10::Pay> {
    client
        .request(
            HttpMethod::Post {
                body: Some(Body::Json(&pay_params)),
            },
            &client.environment.uri("1.0", "/pay"),
        )
        .await
}

#[cfg(test)]
mod tests {
    use super::v10::*;

    impl Default for Pay {
        fn default() -> Self {
            Self {
                id: "some-pay-id".to_string(),
                state: Default::default(),
                created_at: "some-created-at".to_string(),
                completed_at: None,
            }
        }
    }

    impl Default for TransferReason {
        fn default() -> Self {
            Self {
                country: "ES".to_string(),
                currency: "EUR".to_string(),
                code: "some-transfer-code".to_string(),
                description: "some-transfer-description".to_string(),
            }
        }
    }

    impl Default for ExchangeReason {
        fn default() -> Self {
            Self {
                code: "some-exchange-reason-code".to_string(),
                name: "some-exchange-reason-name".to_string(),
            }
        }
    }

    impl Default for Transfer {
        fn default() -> Self {
            Self {
                id: "some-transfer-id".to_string(),
                state: Default::default(),
                created_at: "some-created-at".to_string(),
                completed_at: None,
            }
        }
    }

    impl Default for TransferState {
        fn default() -> Self {
            Self::Completed
        }
    }
}
