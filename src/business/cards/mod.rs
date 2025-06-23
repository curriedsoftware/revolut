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
    client::{Client, ProductionEnvironment},
    errors::ApiResult,
};

pub mod v10 {
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CardProduct {
        code: String,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CardSpendProgram {
        label: String,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Amount {
        amount: f64,
        currency: String,
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

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(rename_all = "snake_case")]
    pub enum CardState {
        #[serde(alias = "CREATED")]
        Created,
        #[serde(alias = "PENDING")]
        Pending,
        #[serde(alias = "ACTIVE")]
        Active,
        #[serde(alias = "FROZEN")]
        Frozen,
        #[serde(alias = "LOCKED")]
        Locked,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CardSensitiveDetails {
        pan: String,
        cvv: String,
        expiry: String,
    }
}

pub async fn list(
    client: &Client<ProductionEnvironment<client::BusinessClient>, BusinessAuthentication>,
) -> ApiResult<Vec<v10::Card>> {
    client
        .request(
            HttpMethod::<()>::Get,
            &client.environment.uri("1.0", "/cards"),
        )
        .await
}
