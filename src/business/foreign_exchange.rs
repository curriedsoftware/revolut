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

//! [Business foreign exchange API](https://developer.revolut.com/docs/business/foreign-exchange).

use crate::{
    business::client::BusinessAuthentication,
    client::{Body, Client, Environment, HttpMethod},
    errors::{self, ApiResult},
};

use serde_json::json;

pub mod v10 {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize)]
    pub struct ExchangeRateParams {
        pub from: String,
        pub amount: Option<f64>,
        pub to: String,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ExchangeRate {
        pub from: AmountWithCurrency,
        pub to: AmountWithCurrency,
        pub fee: AmountWithCurrency,
        pub rate_date: String,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct AmountWithCurrency {
        pub amount: Option<f64>,
        pub currency: Option<String>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ExchangeRequest {
        pub from: ExchangeFromTo,
        pub to: ExchangeFromTo,
        pub reference: Option<String>,
        pub request_id: String,
        pub exchange_reason_code: Option<String>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ExchangeFromTo {
        pub account_id: String,
        pub currency: String,
        pub amount: Option<f64>,
    }

    #[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Exchange {
        pub id: Option<String>,
        pub r#type: Option<String>,
        pub reason_code: Option<String>,
        pub created_at: Option<String>,
        pub completed_at: Option<String>,
        pub state: Option<ExchangeState>,
    }

    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    #[serde(rename_all = "snake_case")]
    pub enum ExchangeState {
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

impl std::fmt::Display for v10::ExchangeRateParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let query = format!("?from={}&to={}", self.from, self.to);
        if let Some(amount) = self.amount {
            write!(f, "{query}&amount={amount}")
        } else {
            write!(f, "{query}")
        }
    }
}

pub async fn get<E: Environment>(
    client: &Client<E, BusinessAuthentication>,
    get_params: v10::ExchangeRateParams,
) -> ApiResult<Vec<v10::ExchangeRate>> {
    client
        .request(
            HttpMethod::<()>::Get,
            &client
                .environment
                .uri("1.0", &format!("/rate{}", get_params)),
        )
        .await
}

pub async fn exchange<E: Environment>(
    client: &Client<E, BusinessAuthentication>,
    exchange: v10::ExchangeRequest,
) -> ApiResult<v10::Exchange> {
    client
        .request(
            HttpMethod::Post {
                body: Some(Body::Json(&exchange)),
            },
            &client.environment.uri("1.0", &format!("/exchange")),
        )
        .await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_documented_examples() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(
            serde_json::from_value::<v10::Exchange>(json!(
                {
                    "id": "630f9c2e-2e74-a06d-ab61-deb7ggkkd6cb",
                    "state": "completed",
                    "created_at": "2022-08-31T17:36:46.656876Z",
                    "completed_at": "2022-08-31T17:36:46.657239Z"
                }
            ))?,
            v10::Exchange {
                id: Some("630f9c2e-2e74-a06d-ab61-deb7ggkkd6cb".to_string()),
                state: Some(v10::ExchangeState::Completed),
                created_at: Some("2022-08-31T17:36:46.656876Z".to_string()),
                completed_at: Some("2022-08-31T17:36:46.657239Z".to_string()),
                ..Default::default()
            },
        );

        Ok(())
    }
}
