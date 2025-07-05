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
    business::client::{BusinessAuthentication, Environment, HttpMethod},
    client::Client,
    errors::{self, ApiResult},
};

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
