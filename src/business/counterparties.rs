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

//! [Business counterparties API](https://developer.revolut.com/docs/business/counterparties).
//!
//! [^note]: This feature is **limited** in the sandbox
//! environment. Read more about it in the Revolut documentation.

use crate::{
    business::client::{self, BusinessAuthentication, Environment, HttpMethod},
    client::{Body, Client, ProductionEnvironment},
    errors::ApiResult,
};

pub mod v10 {
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Default)]
    pub struct ListParams {
        pub name: Option<String>,
        pub account_no: Option<String>,
        pub sort_code: Option<String>,
        pub iban: Option<String>,
        pub bic: Option<String>,
        pub created_before: Option<String>,
        pub limit: Option<u16>,
    }

    #[derive(Clone, Debug, Default, Deserialize, Serialize)]
    pub struct CounterpartyRequest {
        pub company_name: Option<String>,
        pub profile_type: Option<CounterpartyProfileType>,
        pub name: Option<String>,
        pub individual_name: Option<IndividualName>,
        pub bank_country: Option<String>,
        pub currency: Option<String>,
        pub revtag: Option<String>,
        pub account_no: Option<String>,
        pub iban: Option<String>,
        pub sort_code: Option<String>,
        pub routing_number: Option<String>,
        pub bic: Option<String>,
        pub clabe: Option<String>,
        pub ifsc: Option<String>,
        pub bsb_code: Option<String>,
        pub address: Option<CounterpartyAddress>,
    }

    #[derive(Clone, Debug, Deserialize, strum::Display, Serialize)]
    #[serde(rename_all = "snake_case")]
    pub enum CounterpartyProfileType {
        Personal,
        Business,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct IndividualName {
        pub first_name: Option<String>,
        pub last_name: Option<String>,
    }

    #[derive(Clone, Debug, Default, Deserialize, Serialize)]
    pub struct CounterpartyAddress {
        pub street_line1: Option<String>,
        pub street_line2: Option<String>,
        pub region: Option<String>,
        pub city: Option<String>,
        pub country: String,
        pub postcode: String,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Counterparty {
        pub id: String,
        pub name: String,
        pub revtag: Option<String>,
        pub profile_type: Option<String>,
        pub country: Option<String>,
        pub state: CounterpartyState,
        pub created_at: String,
        pub updated_at: String,
        pub accounts: Vec<CounterpartyAccount>,
        pub cards: Vec<CounterpartyCard>,
    }

    #[derive(Clone, Debug, Deserialize, strum::Display, Serialize)]
    #[serde(rename_all = "snake_case")]
    pub enum CounterpartyState {
        Created,
        Draft,
        Deleted,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CounterpartyAccount {
        pub id: String,
        pub name: Option<String>,
        pub bank_country: Option<String>,
        pub currency: String,
        pub r#type: CounterpartyAccountType,
        pub account_no: Option<String>,
        pub iban: Option<String>,
        pub sort_code: Option<String>,
        pub routing_number: Option<String>,
        pub bic: Option<String>,
        pub clabe: Option<String>,
        pub ifsc: Option<String>,
        pub bsb_code: Option<String>,
        pub recipient_charges: Option<CounterpartyAccountRecipientCharges>, // deprecated
    }

    #[derive(Clone, Debug, Deserialize, strum::Display, Serialize)]
    #[serde(rename_all = "snake_case")]
    pub enum CounterpartyAccountRecipientCharges {
        No,
        Expected,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CounterpartyCard {
        pub id: String,
        pub name: String,
        pub last_digits: String,
        pub scheme: CounterpartyCardScheme,
        pub country: String,
        pub currency: String,
    }

    #[derive(Clone, Debug, Deserialize, strum::Display, Serialize)]
    #[serde(rename_all = "snake_case")]
    pub enum CounterpartyCardScheme {
        Visa,
        Mastercard,
    }

    #[derive(Clone, Debug, Deserialize, strum::Display, Serialize)]
    #[serde(rename_all = "snake_case")]
    pub enum CounterpartyAccountType {
        Revolut,
        External,
    }

    #[derive(Clone, Debug, Default, Deserialize, Serialize)]
    pub struct AccountNameRequest {
        pub account_no: String,
        pub sort_code: String,
        pub company_name: Option<String>,
        pub individual_name: Option<IndividualName>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct AccountName {
        pub result_code: String,
        pub reason: Option<AccountNameReason>,
        pub company_name: Option<String>,
        pub individual_name: Option<IndividualName>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct AccountNameReason {
        pub r#type: Option<AccountNameReasonType>,
        pub code: Option<AccountNameReasonCode>,
    }

    #[derive(Clone, Debug, Deserialize, strum::Display, Serialize)]
    #[serde(rename_all = "snake_case")]
    pub enum AccountNameReasonType {
        UkCop,
    }

    #[derive(Clone, Debug, Deserialize, strum::Display, Serialize)]
    #[serde(rename_all = "snake_case")]
    pub enum AccountNameReasonCode {
        CloseMatch,
        IndividualAccountNameMatched,
        CompanyAccountNameMatched,
        IndividualAccountCloseMatch,
        CompanyAccountCloseMatch,
        NotMatched,
        AccountDoesNotExist,
        AccountSwitched,
        CannotBeChecked,
    }
}

impl std::fmt::Display for v10::ListParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let query = [
            ("name", &self.name),
            ("account_no", &self.account_no),
            ("sort_code", &self.sort_code),
            ("iban", &self.iban),
            ("bic", &self.bic),
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
) -> ApiResult<Vec<v10::Counterparty>> {
    client
        .request(
            HttpMethod::<()>::Get,
            &client
                .environment
                .uri("1.0", &format!("/counterparties{list_params}")),
        )
        .await
}

pub async fn retrieve(
    client: &Client<ProductionEnvironment<client::BusinessClient>, BusinessAuthentication>,
    counterparty_id: &str,
) -> ApiResult<v10::Counterparty> {
    client
        .request(
            HttpMethod::<()>::Get,
            &client
                .environment
                .uri("1.0", &format!("/counterparty/{counterparty_id}")),
        )
        .await
}

pub async fn delete(
    client: &Client<ProductionEnvironment<client::BusinessClient>, BusinessAuthentication>,
    counterparty_id: &str,
) -> ApiResult<()> {
    client
        .request(
            HttpMethod::<()>::Delete,
            &client
                .environment
                .uri("1.0", &format!("/counterparty/{counterparty_id}")),
        )
        .await
}

pub async fn create(
    client: &Client<ProductionEnvironment<client::BusinessClient>, BusinessAuthentication>,
    counterparty: &v10::CounterpartyRequest,
) -> ApiResult<v10::Counterparty> {
    client
        .request(
            HttpMethod::Post {
                body: Some(Body::Json(&counterparty)),
            },
            &client.environment.uri("1.0", "/counterparty"),
        )
        .await
}

pub async fn validate_account_name(
    client: &Client<ProductionEnvironment<client::BusinessClient>, BusinessAuthentication>,
    account_name: &v10::AccountNameRequest,
) -> ApiResult<v10::AccountName> {
    client
        .request(
            HttpMethod::Post {
                body: Some(Body::Json(&account_name)),
            },
            &client.environment.uri("1.0", "/account-name-validation"),
        )
        .await
}

#[cfg(test)]
mod tests {
    use super::v10::*;

    impl Default for IndividualName {
        fn default() -> Self {
            Self {
                first_name: None,
                last_name: None,
            }
        }
    }

    impl Default for Counterparty {
        fn default() -> Self {
            Self {
                id: "some-counterparty-id".to_string(),
                name: "some-name".to_string(),
                revtag: None,
                profile_type: None,
                country: None,
                state: Default::default(),
                created_at: "some-created-at".to_string(),
                updated_at: "some-updated-at".to_string(),
                accounts: Vec::new(),
                cards: Vec::new(),
            }
        }
    }

    impl Default for CounterpartyState {
        fn default() -> Self {
            Self::Created
        }
    }

    impl Default for CounterpartyAccount {
        fn default() -> Self {
            Self {
                id: "some-counterparty-account-id".to_string(),
                name: None,
                bank_country: None,
                currency: "EUR".to_string(),
                r#type: Default::default(),
                account_no: None,
                iban: None,
                sort_code: None,
                routing_number: None,
                bic: None,
                clabe: None,
                ifsc: None,
                bsb_code: None,
                recipient_charges: None,
            }
        }
    }

    impl Default for CounterpartyAccountRecipientCharges {
        fn default() -> Self {
            Self::No
        }
    }

    impl Default for CounterpartyCard {
        fn default() -> Self {
            Self {
                id: "some-counterparty-card-id".to_string(),
                name: "some-name".to_string(),
                last_digits: "some-last-digits".to_string(),
                scheme: Default::default(),
                country: "ES".to_string(),
                currency: "EUR".to_string(),
            }
        }
    }

    impl Default for CounterpartyCardScheme {
        fn default() -> Self {
            Self::Visa
        }
    }

    impl Default for CounterpartyAccountType {
        fn default() -> Self {
            Self::External
        }
    }

    impl Default for AccountName {
        fn default() -> Self {
            Self {
                result_code: "some-result-code".to_string(),
                reason: None,
                company_name: None,
                individual_name: None,
            }
        }
    }

    impl Default for AccountNameReason {
        fn default() -> Self {
            Self {
                r#type: None,
                code: None,
            }
        }
    }

    impl Default for AccountNameReasonType {
        fn default() -> Self {
            Self::UkCop
        }
    }

    impl Default for AccountNameReasonCode {
        fn default() -> Self {
            Self::IndividualAccountNameMatched
        }
    }
}
