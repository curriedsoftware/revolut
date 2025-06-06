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
    errors::{self, Result},
};

pub mod v10 {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Account {
        id: String,
        name: Option<String>,
        balance: f64,
        currency: String,
        state: String,
        public: bool,
        created_at: String,
        updated_at: String,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct AccountEstimatedTime {
        unit: String,
        min: Option<u16>,
        max: Option<u16>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct AccountAddress {
        street_line1: Option<String>,
        street_line2: Option<String>,
        region: Option<String>,
        city: Option<String>,
        country: String,
        postcode: String,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct BankDetails {
        iban: Option<String>,
        bic: Option<String>,
        account_no: Option<String>,
        sort_code: Option<String>,
        routing_number: Option<String>,
        beneficiary: String,
        beneficiary_address: AccountAddress,
        bank_country: Option<String>,
        pooled: Option<bool>,
        unique_reference: Option<String>,
        schemes: Vec<String>,
        estimated_time: AccountEstimatedTime,
    }
}

pub async fn list<E: Environment>(
    client: &Client<E, BusinessAuthentication>,
) -> Result<Vec<v10::Account>> {
    client
        .request(HttpMethod::Get, &client.environment.uri("1.0", "/accounts"))
        .await
}

pub async fn account<E: Environment>(
    client: &Client<E, BusinessAuthentication>,
    account_id: &str,
) -> Result<v10::Account> {
    client
        .request(
            HttpMethod::Get,
            &client
                .environment
                .uri("1.0", &format!("/accounts/{account_id}")),
        )
        .await
}

pub async fn bank_details<E: Environment>(
    client: &Client<E, BusinessAuthentication>,
    account_id: &str,
) -> Result<v10::BankDetails> {
    Ok(client
        .request::<Vec<v10::BankDetails>>(
            HttpMethod::Get,
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
