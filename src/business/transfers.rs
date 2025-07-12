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
    business::client::{self, BusinessAuthentication, Environment, HttpMethod},
    client::{Body, Client, ProductionEnvironment},
    errors::ApiResult,
};

pub mod v10 {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize)]
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

    #[derive(Debug, Deserialize, Serialize)]
    pub struct TransferReceiver {
        pub counterparty_id: String,
        pub account_id: Option<String>,
        pub card_id: Option<String>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct TransferReason {
        pub country: String,
        pub currency: String,
        pub code: String,
        pub description: String,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct ExchangeReason {
        pub code: String,
        pub name: String,
    }

    #[derive(Debug, Deserialize, Serialize)]
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
    #[strum(serialize_all = "snake_case")]
    pub enum TransferState {
        #[serde(alias = "CREATED")]
        Created,
        #[serde(alias = "PENDING")]
        Pending,
        #[serde(alias = "COMPLETED")]
        Completed,
        #[serde(alias = "DECLINED")]
        Declined,
        #[serde(alias = "FAILED")]
        Failed,
        #[serde(alias = "REVERTED")]
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
