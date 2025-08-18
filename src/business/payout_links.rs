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

//! [Business payout links API](https://developer.revolut.com/docs/business/payout-links).

use crate::{
    business::client::{BusinessAuthentication, Environment, HttpMethod},
    client::{Body, Client},
    errors::ApiResult,
};

pub mod v10 {
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Default)]
    pub struct ListParams {
        pub state: Option<Vec<PayoutLinkState>>,
        pub created_before: Option<String>,
        pub limit: Option<u16>,
    }

    #[derive(Clone, Debug, Deserialize, strum::Display, Serialize)]
    #[serde(rename_all = "snake_case")]
    #[strum(serialize_all = "snake_case")]
    pub enum PayoutLinkState {
        Created,
        Failed,
        Awaiting,
        Active,
        Expired,
        Cancelled,
        Processing,
        Processed,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct PayoutLink {
        pub id: String,
        pub state: PayoutLinkState,
        pub created_at: String,
        pub updated_at: String,
        pub counterparty_name: String,
        pub save_counterparty: bool,
        pub request_id: String,
        pub expiry_date: Option<String>,
        pub payout_method: Vec<PayoutMethod>,
        pub account_id: String,
        pub amount: f64,
        pub currency: String,
        pub url: Option<String>,
        pub reference: String,
        pub transfer_reason_code: Option<String>,
        pub counterparty_id: Option<String>,
        pub transaction_id: Option<String>,
        pub cancellation_reason: Option<CancellationReason>,
    }

    #[derive(Clone, Debug, Deserialize, strum::Display, Serialize)]
    #[serde(rename_all = "snake_case")]
    pub enum PayoutMethod {
        Revolut,
        BankAccount,
        Card,
    }

    #[derive(Clone, Debug, Deserialize, strum::Display, Serialize)]
    #[serde(rename_all = "snake_case")]
    pub enum CancellationReason {
        TooManyNameCheckAttempts,
    }

    #[derive(Debug, Default, Deserialize, Serialize)]
    pub struct PayoutLinkRequest {
        pub counterparty_name: String,
        pub save_counterparty: Option<bool>,
        pub request_id: String,
        pub account_id: String,
        pub amount: f64,
        pub currency: String,
        pub reference: String,
        pub payout_methods: Option<Vec<PayoutMethod>>,
        pub expiry_period: Option<String>,
        pub transfer_reason_code: Option<String>,
    }
}

impl std::fmt::Display for v10::ListParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let state = self.state.clone();

        let mut query = state
            .unwrap_or_default()
            .into_iter()
            .map(|state| ("state", Some(state.to_string())))
            .collect::<Vec<(&str, Option<String>)>>();

        let limit = self
            .limit
            .map(|limit| std::string::ToString::to_string(&limit));

        query.extend(vec![
            ("created_before", self.created_before.clone()),
            ("limit", limit.clone()),
        ]);

        let query = query.iter().fold(String::new(), |acc, (key, value)| {
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

pub async fn list<E: Environment>(
    client: &Client<E, BusinessAuthentication>,
    list_params: &v10::ListParams,
) -> ApiResult<Vec<v10::PayoutLink>> {
    client
        .request(
            HttpMethod::<()>::Get,
            &client
                .environment
                .uri("1.0", &format!("/payout-links{list_params}")),
        )
        .await
}

pub async fn retrieve<E: Environment>(
    client: &Client<E, BusinessAuthentication>,
    payout_link_id: &str,
) -> ApiResult<v10::PayoutLink> {
    client
        .request(
            HttpMethod::<()>::Get,
            &client
                .environment
                .uri("1.0", &format!("/payout-links/{payout_link_id}")),
        )
        .await
}

pub async fn create<E: Environment>(
    client: &Client<E, BusinessAuthentication>,
    payout_link: &v10::PayoutLinkRequest,
) -> ApiResult<v10::PayoutLink> {
    client
        .request(
            HttpMethod::Post {
                body: Some(Body::Json(&payout_link)),
            },
            &client.environment.uri("1.0", "/payout-links"),
        )
        .await
}

pub async fn cancel<E: Environment>(
    client: &Client<E, BusinessAuthentication>,
    payout_link_id: &str,
) -> ApiResult<v10::PayoutLink> {
    client
        .request(
            HttpMethod::Post::<()> { body: None },
            &client
                .environment
                .uri("1.0", &format!("/payout-links/{payout_link_id}/cancel")),
        )
        .await
}

#[cfg(test)]
mod tests {
    use super::{v10::*, *};

    impl Default for PayoutLinkState {
        fn default() -> Self {
            Self::Active
        }
    }

    impl Default for PayoutLink {
        fn default() -> Self {
            Self {
                id: "some-payout-link-id".to_string(),
                state: Default::default(),
                created_at: "some-created-at".to_string(),
                updated_at: "some-updated-at".to_string(),
                counterparty_name: "some-counterparty-name".to_string(),
                save_counterparty: false,
                request_id: "some-payout-request-id".to_string(),
                expiry_date: None,
                payout_method: Vec::new(),
                account_id: "some-account-id".to_string(),
                amount: 42.42,
                currency: "EUR".to_string(),
                url: None,
                reference: "some-payout-link-reference".to_string(),
                transfer_reason_code: None,
                counterparty_id: None,
                transaction_id: None,
                cancellation_reason: None,
            }
        }
    }

    impl Default for PayoutMethod {
        fn default() -> Self {
            PayoutMethod::Card
        }
    }

    impl Default for CancellationReason {
        fn default() -> Self {
            Self::TooManyNameCheckAttempts
        }
    }

    #[test]
    fn check_list_query_parameters() {
        assert_eq!(
            "?state=created",
            v10::ListParams {
                state: Some(vec![v10::PayoutLinkState::Created]),
                ..Default::default()
            }
            .to_string()
        );
        assert_eq!(
            "?state=created&state=active",
            v10::ListParams {
                state: Some(vec![
                    v10::PayoutLinkState::Created,
                    v10::PayoutLinkState::Active,
                ]),
                ..Default::default()
            }
            .to_string()
        );
    }
}
