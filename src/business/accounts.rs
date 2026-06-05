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

    // Reused verbatim from the OpenAPI-generated types. These stay in sync
    // automatically when the specs are bumped via `just generate`; we only
    // alias them back to this crate's public names.
    pub use crate::business::generated::{
        AccountBankDetailsItem as BankDetails, BeneficiaryAddress as AccountAddress,
        EstimatedTime as AccountEstimatedTime,
    };

    // Kept hand-written: the spec declares `state` as an inline string enum,
    // which the generator flattens to `String`. We keep the typed enum (and the
    // `Account` struct that embeds it) so this remains correct by construction.
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

    #[derive(Clone, Debug, Deserialize, strum::Display, Serialize)]
    #[serde(rename_all = "snake_case")]
    pub enum AccountState {
        Active,
        Inactive,
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

pub async fn retrieve<E: Environment>(
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
        .ok_or(errors::Error::ClientError(Box::new(
            errors::ClientError::RequestError("No such account present".to_string()),
        )))?
        .clone())
}

#[cfg(test)]
mod tests {
    use crate::business::client::{BusinessAuthenticationBuilder, business_client};

    use super::{v10::*, *};

    impl Default for Account {
        fn default() -> Self {
            Self {
                id: "some-account-id".to_string(),
                name: None,
                balance: 42.42,
                currency: "EUR".to_string(),
                state: Default::default(),
                public: true,
                created_at: "some-created-at".to_string(),
                updated_at: "some-updated-at".to_string(),
            }
        }
    }

    impl Default for AccountState {
        fn default() -> Self {
            Self::Active
        }
    }

    impl Default for AccountEstimatedTime {
        fn default() -> Self {
            Self {
                unit: "days".parse().unwrap(),
                min: None,
                max: None,
            }
        }
    }

    impl Default for AccountAddress {
        fn default() -> Self {
            Self {
                street_line1: None,
                street_line2: None,
                region: None,
                city: None,
                country: "ES".parse().unwrap(),
                postcode: "28810".to_string(),
            }
        }
    }

    impl Default for BankDetails {
        fn default() -> Self {
            Self {
                iban: None,
                bic: None,
                account_no: None,
                sort_code: None,
                routing_number: None,
                beneficiary: "some-beneficiary".to_string(),
                beneficiary_address: Default::default(),
                bank_country: None,
                pooled: None,
                unique_reference: None,
                schemes: Vec::new(),
                estimated_time: Default::default(),
            }
        }
    }

    #[tokio::test]
    async fn check_list_accounts_type() {
        let _: Vec<v10::Account> = list(
            &business_client()
                .with_sandbox_environment()
                .with_authentication(
                    BusinessAuthenticationBuilder::default()
                        .with_dummy_client_assertion()
                        .with_dummy_refresh_token()
                        .build(),
                )
                .build()
                .unwrap(),
        )
        .await
        .unwrap();
    }
}
