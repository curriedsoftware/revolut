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

//! [Business simulations API](https://developer.revolut.com/docs/business/simulations).
//!
//! [^note]: This feature is **only** available in the sandbox
//! environment. Trying to use such a feature using a production
//! client will result in an error at compile time.

use crate::{
    business::client::{self, BusinessAuthentication, Environment, HttpMethod},
    client::{Body, Client, SandboxEnvironment},
    errors::ApiResult,
};

pub mod v10 {
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, strum::Display)]
    #[strum(serialize_all = "snake_case")]
    pub enum TransferStateRequest {
        Complete,
        Revert,
        Decline,
        Fail,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct TransferStateUpdate {
        pub id: String,
        pub state: TransferState,
        pub created_at: String,
        pub completed_at: Option<String>,
    }

    #[derive(Clone, Debug, Deserialize, strum::Display, Serialize)]
    #[serde(rename_all = "snake_case")]
    #[strum(serialize_all = "snake_case")]
    pub enum TransferState {
        #[serde(alias = "COMPLETED")]
        Completed,
        #[serde(alias = "REVERTED")]
        Reverted,
        #[serde(alias = "DECLINED")]
        Declined,
        #[serde(alias = "FAILED")]
        Failed,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct TopUpRequest {
        pub account_id: String,
        pub amount: f64,
        pub currency: String,
        pub reference: Option<String>,
        pub state: Option<TopUpState>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct TopUp {
        pub id: String,
        pub state: TopUpState,
        pub created_at: String,
        pub completed_at: Option<String>,
    }

    #[derive(Clone, Debug, Deserialize, strum::Display, Serialize)]
    #[serde(rename_all = "snake_case")]
    #[strum(serialize_all = "snake_case")]
    pub enum TopUpState {
        #[serde(alias = "PENDING")]
        Pending,
        #[serde(alias = "COMPLETED")]
        Completed,
        #[serde(alias = "REVERTED")]
        Reverted,
        #[serde(alias = "FAILED")]
        Failed,
    }
}

pub async fn transfer_state_update(
    client: &Client<SandboxEnvironment<client::BusinessClient>, BusinessAuthentication>,
    id: &str,
    state: &v10::TransferStateRequest,
) -> ApiResult<v10::TransferStateUpdate> {
    client
        .request(
            HttpMethod::Post::<()> { body: None },
            &client
                .environment
                .uri("1.0", &format!("/sandbox/transactions/{id}/{state}")),
        )
        .await
}

pub async fn account_top_up(
    client: &Client<SandboxEnvironment<client::BusinessClient>, BusinessAuthentication>,
    top_up: &v10::TopUpRequest,
) -> ApiResult<v10::TopUp> {
    client
        .request(
            HttpMethod::Post {
                body: Some(Body::Json(top_up)),
            },
            &client.environment.uri("1.0", "/sandbox/topup"),
        )
        .await
}
