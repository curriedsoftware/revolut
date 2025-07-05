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

//! [Business accounts API](https://developer.revolut.com/docs/business/accounts).

use crate::{
    business::client::{BusinessAuthentication, Environment, HttpMethod},
    client::Client,
    errors::{self, ApiResult},
};

pub mod v10 {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Account {
        pub id: String,
        pub name: Option<String>,
        pub balance: f64,
        pub currency: String,
        pub state: AccountState,
        pub public: bool,
        pub created_at: String,
        pub updated_at: String,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(rename_all = "snake_case")]
    pub enum AccountState {
        #[serde(alias = "ACTIVE")]
        Active,
        #[serde(alias = "INACTIVE")]
        Inactive,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct AccountEstimatedTime {
        unit: String,
        min: Option<u16>,
        max: Option<u16>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct AccountAddress {
        pub street_line1: Option<String>,
        pub street_line2: Option<String>,
        pub region: Option<String>,
        pub city: Option<String>,
        pub country: String,
        pub postcode: String,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct BankDetails {
        pub iban: Option<String>,
        pub bic: Option<String>,
        pub account_no: Option<String>,
        pub sort_code: Option<String>,
        pub routing_number: Option<String>,
        pub beneficiary: String,
        pub beneficiary_address: AccountAddress,
        pub bank_country: Option<String>,
        pub pooled: Option<bool>,
        pub unique_reference: Option<String>,
        pub schemes: Vec<String>,
        pub estimated_time: AccountEstimatedTime,
    }
}

pub async fn list<E: Environment>(
    client: &Client<E, BusinessAuthentication>,
) -> ApiResult<Vec<v10::Account>> {
    client
        .request(
            HttpMethod::<()>::Get,
            &client.environment.uri("1.0", "/accounts"),
        )
        .await
}

pub async fn account<E: Environment>(
    client: &Client<E, BusinessAuthentication>,
    account_id: &str,
) -> ApiResult<v10::Account> {
    client
        .request(
            HttpMethod::<()>::Get,
            &client
                .environment
                .uri("1.0", &format!("/accounts/{account_id}")),
        )
        .await
}

pub async fn bank_details<E: Environment>(
    client: &Client<E, BusinessAuthentication>,
    account_id: &str,
) -> ApiResult<v10::BankDetails> {
    Ok(client
        .request::<Vec<v10::BankDetails>, ()>(
            HttpMethod::<()>::Get,
            &client
                .environment
                .uri("1.0", &format!("/accounts/{account_id}/bank-details")),
        )
        .await?
        .first()
        .ok_or(errors::Error::ClientError(
            errors::ClientError::RequestError("No such account present".to_string()),
        ))?
        .clone())
}
