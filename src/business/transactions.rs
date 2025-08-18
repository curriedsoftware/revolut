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

//! [Business transactions API](https://developer.revolut.com/docs/business/transactions).

use crate::{
    business::client::{BusinessAuthentication, Environment, HttpMethod},
    client::Client,
    errors::ApiResult,
};

pub mod v10 {
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Default)]
    pub struct ListParams {
        pub from: Option<String>,
        pub to: Option<String>,
        pub account: Option<String>,
        pub count: Option<u64>,
        pub r#type: Option<TransactionType>,
    }

    #[derive(Clone, Debug, Deserialize, strum::Display, Serialize)]
    #[serde(rename_all = "snake_case")]
    #[strum(serialize_all = "snake_case")]
    pub enum TransactionType {
        Atm,
        CardPayment,
        CardRefund,
        CardChargeback,
        CardCredit,
        Exchange,
        Transfer,
        Loan,
        Fee,
        Refund,
        Topup,
        TopupReturn,
        Tax,
        TaxRefund,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Transaction {
        pub id: String,
        pub r#type: TransactionType,
        pub request_id: Option<String>,
        pub state: TransactionState,
        pub reason_code: Option<String>,
        pub created_at: String,
        pub updated_at: String,
        pub completed_at: Option<String>,
        pub scheduled_for: Option<String>,
        pub related_transaction_id: Option<String>,
        pub merchant: Option<TransactionMerchant>,
        pub reference: Option<String>,
        pub legs: Vec<TransactionLeg>,
        pub card: Option<TransactionCard>,
    }

    #[derive(Clone, Debug, Deserialize, strum::Display, Serialize)]
    #[serde(rename_all = "snake_case")]
    pub enum TransactionState {
        Created,
        Pending,
        Completed,
        Declined,
        Failed,
        Reverted,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct TransactionMerchant {
        pub name: Option<String>,
        pub city: Option<String>,
        pub category_code: Option<String>,
        pub country: Option<String>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct TransactionLeg {
        pub leg_id: String,
        pub amount: f64,
        pub fee: Option<f64>,
        pub currency: String,
        pub bill_amount: Option<f64>,
        pub bill_currency: Option<String>,
        pub account_id: String,
        pub counterparty: Option<TransactionCounterparty>,
        pub description: Option<String>,
        pub balance: Option<f64>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct TransactionCounterparty {
        pub account_id: Option<String>,
        pub account_type: TransactionCounterpartyAccountType,
        pub id: Option<String>,
    }

    #[derive(Clone, Debug, Deserialize, strum::Display, Serialize)]
    #[serde(rename_all = "snake_case")]
    pub enum TransactionCounterpartyAccountType {
        #[serde(rename = "self")]
        Self_,
        Revolut,
        External,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct TransactionCard {
        pub id: String,
        pub card_number: String,
        pub first_name: Option<String>,
        pub last_name: Option<String>,
        pub phone: Option<String>,
    }
}

impl std::fmt::Display for v10::ListParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let query = [
            ("from", &self.from),
            ("to", &self.to),
            ("account", &self.account),
            ("count", &self.count.map(|count| count.to_string())),
            ("type", &self.r#type.clone().map(|count| count.to_string())),
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

pub async fn list<E: Environment>(
    client: &Client<E, BusinessAuthentication>,
    list_params: &v10::ListParams,
) -> ApiResult<Vec<v10::Transaction>> {
    client
        .request(
            HttpMethod::<()>::Get,
            &client
                .environment
                .uri("1.0", &format!("/transactions{list_params}")),
        )
        .await
}

pub enum RetrieveParam<'a> {
    Transaction { transaction_id: &'a str },
    Request { request_id: &'a str },
}

pub async fn retrieve<E: Environment>(
    client: &Client<E, BusinessAuthentication>,
    retrieve_param: &RetrieveParam<'_>,
) -> ApiResult<v10::Transaction> {
    let path = match retrieve_param {
        RetrieveParam::Transaction { transaction_id } => {
            format!("/transaction/{transaction_id}")
        }
        RetrieveParam::Request { request_id } => {
            format!("/transaction/{request_id}?id_type=request_id")
        }
    };
    client
        .request(HttpMethod::<()>::Get, &client.environment.uri("1.0", &path))
        .await
}

#[cfg(test)]
mod tests {
    use super::{v10::*, *};

    impl Default for TransactionType {
        fn default() -> Self {
            Self::CardCredit
        }
    }

    impl Default for Transaction {
        fn default() -> Self {
            Self {
                id: "some-transaction-id".to_string(),
                r#type: Default::default(),
                request_id: None,
                state: Default::default(),
                reason_code: None,
                created_at: "some-created-at".to_string(),
                updated_at: "some-updated-at".to_string(),
                completed_at: None,
                scheduled_for: None,
                related_transaction_id: None,
                merchant: None,
                reference: None,
                legs: Vec::new(),
                card: None,
            }
        }
    }

    impl Default for TransactionState {
        fn default() -> Self {
            Self::Completed
        }
    }

    impl Default for TransactionMerchant {
        fn default() -> Self {
            Self {
                name: None,
                city: None,
                category_code: None,
                country: None,
            }
        }
    }

    impl Default for TransactionLeg {
        fn default() -> Self {
            Self {
                leg_id: "some-leg-id".to_string(),
                amount: 42.42,
                fee: None,
                currency: "EUR".to_string(),
                bill_amount: None,
                bill_currency: None,
                account_id: "some-account-id".to_string(),
                counterparty: None,
                description: None,
                balance: None,
            }
        }
    }

    impl Default for TransactionCounterparty {
        fn default() -> Self {
            Self {
                account_id: None,
                account_type: Default::default(),
                id: None,
            }
        }
    }

    impl Default for TransactionCounterpartyAccountType {
        fn default() -> Self {
            Self::External
        }
    }

    impl Default for TransactionCard {
        fn default() -> Self {
            Self {
                id: "some-transaction-card-id".to_string(),
                card_number: "some-card-number".to_string(),
                first_name: None,
                last_name: None,
                phone: None,
            }
        }
    }

    #[test]
    fn check_list_query_parameters() {
        assert_eq!(
            "?type=card_payment",
            v10::ListParams {
                r#type: Some(v10::TransactionType::CardPayment),
                ..Default::default()
            }
            .to_string()
        );
    }
}
