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

//! [Business cards API](https://developer.revolut.com/docs/business/cards).
//!
//! [^note]: This feature is **not** available in the sandbox
//! environment. Trying to use such a feature using a sandbox client
//! will result in an error at compile time.

use crate::{
    business::client::{self, BusinessAuthentication, Environment, HttpMethod},
    client::{Body, Client, ProductionEnvironment},
    errors::ApiResult,
};

pub mod v10 {
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Default)]
    pub struct ListParams {
        pub created_before: Option<String>,
        pub limit: Option<u16>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CardProduct {
        code: String,
    }

    #[cfg(test)]
    impl Default for CardProduct {
        fn default() -> Self {
            Self {
                code: "some-card-product-code".to_string(),
            }
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CardSpendProgram {
        label: String,
    }

    #[cfg(test)]
    impl Default for CardSpendProgram {
        fn default() -> Self {
            Self {
                label: "some-card-spend-program-label".to_string(),
            }
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Amount {
        amount: f64,
        currency: String,
    }

    #[cfg(test)]
    impl Default for Amount {
        fn default() -> Self {
            Self {
                amount: 42.42,
                currency: "EUR".to_string(),
            }
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CardSpendingLimits {
        single: Option<Amount>,
        day: Option<Amount>,
        week: Option<Amount>,
        month: Option<Amount>,
        quarter: Option<Amount>,
        year: Option<Amount>,
        all_time: Option<Amount>,
    }

    #[cfg(test)]
    impl Default for CardSpendingLimits {
        fn default() -> Self {
            Self {
                single: None,
                day: None,
                week: None,
                month: None,
                quarter: None,
                year: None,
                all_time: None,
            }
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Card {
        id: String,
        last_digits: String,
        expiry: String,
        state: CardState,
        label: Option<String>,
        r#virtual: bool,
        product: Option<CardProduct>,
        accounts: Vec<String>,
        categories: Option<Vec<String>>,
        spend_program: Option<CardSpendProgram>,
        spending_limits: Option<CardSpendingLimits>,
        holder_id: Option<String>,
        created_at: String,
        updated_at: String,
    }

    #[cfg(test)]
    impl Default for Card {
        fn default() -> Self {
            Self {
                id: "some-card-id".to_string(),
                last_digits: "some-last-digits".to_string(),
                expiry: "some-expiry".to_string(),
                state: Default::default(),
                label: None,
                r#virtual: true,
                product: None,
                accounts: Vec::new(),
                categories: None,
                spend_program: None,
                spending_limits: None,
                holder_id: None,
                created_at: "some-created-at".to_string(),
                updated_at: "some-updated-at".to_string(),
            }
        }
    }

    #[derive(Clone, Debug, Deserialize, strum::Display, Serialize)]
    #[serde(rename_all = "snake_case")]
    pub enum CardState {
        Created,
        Pending,
        Active,
        Frozen,
        Locked,
    }

    #[cfg(test)]
    impl Default for CardState {
        fn default() -> Self {
            Self::Active
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CardSensitiveDetails {
        pan: String,
        cvv: String,
        expiry: String,
    }

    #[cfg(test)]
    impl Default for CardSensitiveDetails {
        fn default() -> Self {
            Self {
                pan: "some-pan".to_string(),
                cvv: "some-cvv".to_string(),
                expiry: "some-expiry".to_string(),
            }
        }
    }

    #[derive(Clone, Debug, Default, Deserialize, Serialize)]
    pub struct CreateCardParams {
        pub request_id: String,
        pub r#virtual: bool,
        pub holder_id: String,
        pub label: Option<String>,
        pub accounts: Option<Vec<String>>,
        pub categories: Option<Vec<String>>,
        pub spending_limits: Option<CardSpendingLimits>,
    }

    #[derive(Clone, Debug, Default, Deserialize, Serialize)]
    pub struct UpdateCardParams {
        pub label: Option<String>,
        pub categories: Option<Vec<String>>,
        pub spending_limits: Option<CardSpendingLimits>,
    }
}

impl std::fmt::Display for v10::ListParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let query = [
            ("created_before", &self.created_before),
            ("limit", &self.limit.map(|limit| limit.to_string())),
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

pub async fn list(
    client: &Client<ProductionEnvironment<client::BusinessClient>, BusinessAuthentication>,
    list_params: &v10::ListParams,
) -> ApiResult<Vec<v10::Card>> {
    client
        .request(
            HttpMethod::<()>::Get,
            &client
                .environment
                .uri("1.0", &format!("/cards{list_params}")),
        )
        .await
}

pub async fn create(
    client: &Client<ProductionEnvironment<client::BusinessClient>, BusinessAuthentication>,
    card: &v10::CreateCardParams,
) -> ApiResult<v10::Card> {
    client
        .request(
            HttpMethod::Post {
                body: Some(Body::Json(&card)),
            },
            &client.environment.uri("1.0", "/cards"),
        )
        .await
}

pub async fn retrieve(
    client: &Client<ProductionEnvironment<client::BusinessClient>, BusinessAuthentication>,
    card_id: &str,
) -> ApiResult<v10::Card> {
    client
        .request(
            HttpMethod::Get::<()>,
            &client.environment.uri("1.0", &format!("/cards/{card_id}")),
        )
        .await
}

pub async fn update(
    client: &Client<ProductionEnvironment<client::BusinessClient>, BusinessAuthentication>,
    card_id: &str,
    card: &v10::UpdateCardParams,
) -> ApiResult<v10::Card> {
    client
        .request(
            HttpMethod::Patch {
                body: Some(Body::Json(&card)),
            },
            &client.environment.uri("1.0", &format!("/cards/{card_id}")),
        )
        .await
}

pub async fn terminate(
    client: &Client<ProductionEnvironment<client::BusinessClient>, BusinessAuthentication>,
    card_id: &str,
) -> ApiResult<()> {
    client
        .request(
            HttpMethod::Delete::<()>,
            &client.environment.uri("1.0", &format!("/cards/{card_id}")),
        )
        .await
}

pub async fn freeze(
    client: &Client<ProductionEnvironment<client::BusinessClient>, BusinessAuthentication>,
    card_id: &str,
) -> ApiResult<()> {
    client
        .request(
            HttpMethod::Post::<()> { body: None },
            &client
                .environment
                .uri("1.0", &format!("/cards/{card_id}/freeze")),
        )
        .await
}

pub async fn unfreeze(
    client: &Client<ProductionEnvironment<client::BusinessClient>, BusinessAuthentication>,
    card_id: &str,
) -> ApiResult<()> {
    client
        .request(
            HttpMethod::Post::<()> { body: None },
            &client
                .environment
                .uri("1.0", &format!("/cards/{card_id}/unfreeze")),
        )
        .await
}

pub async fn sensitive_details(
    client: &Client<ProductionEnvironment<client::BusinessClient>, BusinessAuthentication>,
    card_id: &str,
) -> ApiResult<v10::CardSensitiveDetails> {
    client
        .request(
            HttpMethod::Get::<()>,
            &client
                .environment
                .uri("1.0", &format!("/cards/{card_id}/sensitive-details")),
        )
        .await
}
