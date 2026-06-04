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

//! [Business expenses API](https://developer.revolut.com/docs/business/expenses).
//!
//! [^note]: This feature is **not** available in the sandbox
//! environment. Trying to use such a feature using a sandbox client
//! will result in an error at compile time.

use crate::{
    business::client::{
        self, BusinessAuthentication, Environment, HttpMethod, ProductionEnvironment,
    },
    client::Client,
    errors::ApiResult,
};

use std::vec::Vec;

pub mod v10 {
    use super::Vec;
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    // Reused verbatim from the OpenAPI-generated types: structurally identical.
    // `Amount` matches the spec's `AmountWithCurrency`.
    pub use crate::business::generated::{AmountWithCurrency as Amount, Category};

    #[derive(Clone, Debug, Default)]
    pub struct ListParams {
        pub from: Option<String>,
        pub to: Option<String>,
        pub count: Option<u64>,
        pub state: Option<ExpenseState>,
        pub transaction_type: Option<TransactionType>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct TaxRate {
        pub name: String,
        pub percentage: f64,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ExpenseSplit {
        pub amount: Amount,
        pub category: Category,
        pub tax_rate: TaxRate,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ExpenseSpentAmount {
        pub amount: f64,
        pub currency: String,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Expense {
        pub id: String,
        pub state: ExpenseState,
        pub transaction_type: TransactionType,
        pub description: Option<String>,
        pub submitted_at: Option<String>,
        pub completed_at: Option<String>,
        pub payer: Option<String>,
        pub merchant: Option<String>,
        pub transaction_id: Option<String>,
        pub expense_date: String,
        pub labels: HashMap<String, Vec<String>>,
        pub splits: Vec<ExpenseSplit>,
        pub receipt_ids: Vec<String>,
        pub spent_amount: ExpenseSpentAmount,
    }

    #[derive(Clone, Debug, Deserialize, strum::Display, Serialize)]
    #[serde(rename_all = "snake_case")]
    #[strum(serialize_all = "snake_case")]
    pub enum ExpenseState {
        MissingInfo,
        AwaitingReview,
        Rejected,
        PendingReimbursement,
        RefundRequested,
        Refunded,
        Approved,
        Reverted,
    }

    #[derive(Clone, Debug, Deserialize, strum::Display, Serialize)]
    #[serde(rename_all = "snake_case")]
    #[strum(serialize_all = "snake_case")]
    pub enum TransactionType {
        Atm,
        CardPayment,
        Fee,
        Transfer,
        External,
    }
}

impl std::fmt::Display for v10::ListParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let query = [
            ("from", &self.from),
            ("to", &self.to),
            ("count", &self.count.map(|count| count.to_string())),
            ("state", &self.state.clone().map(|state| state.to_string())),
            (
                "transaction_type",
                &self
                    .transaction_type
                    .clone()
                    .map(|transaction_type| transaction_type.to_string()),
            ),
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
) -> ApiResult<Vec<v10::Expense>> {
    client
        .request(
            HttpMethod::<()>::Get,
            &client
                .environment
                .uri("1.0", &format!("/expenses{list_params}")),
        )
        .await
}

pub async fn retrieve(
    client: &Client<ProductionEnvironment<client::BusinessClient>, BusinessAuthentication>,
    expense_id: &str,
) -> ApiResult<v10::Expense> {
    client
        .request(
            HttpMethod::<()>::Get,
            &client
                .environment
                .uri("1.0", &format!("/expenses/{expense_id}")),
        )
        .await
}

pub async fn expense_receipt(
    client: &Client<ProductionEnvironment<client::BusinessClient>, BusinessAuthentication>,
    expense_id: &str,
    receipt_id: &str,
) -> ApiResult<Vec<u8>> {
    client
        .request_raw(
            HttpMethod::<()>::Get,
            &client.environment.uri(
                "1.0",
                &format!("/expenses/{expense_id}/receipts/{receipt_id}/content"),
            ),
        )
        .await
}

#[cfg(test)]
mod tests {
    use super::v10::*;
    use std::collections::HashMap;

    impl Default for Amount {
        fn default() -> Self {
            Self {
                amount: None,
                currency: None,
            }
        }
    }

    impl Default for Category {
        fn default() -> Self {
            Self {
                id: "some-category-id".to_string(),
                name: "some-category-name".to_string(),
                code: None,
            }
        }
    }

    impl Default for TaxRate {
        fn default() -> Self {
            Self {
                name: "some-tax-rate-name".to_string(),
                percentage: 21.21,
            }
        }
    }

    impl Default for ExpenseSplit {
        fn default() -> Self {
            Self {
                amount: Default::default(),
                category: Default::default(),
                tax_rate: Default::default(),
            }
        }
    }

    impl Default for ExpenseSpentAmount {
        fn default() -> Self {
            Self {
                amount: 21.21,
                currency: "EUR".to_string(),
            }
        }
    }

    impl Default for Expense {
        fn default() -> Self {
            Self {
                id: "some-expense-id".to_string(),
                state: Default::default(),
                transaction_type: Default::default(),
                description: None,
                submitted_at: None,
                completed_at: None,
                payer: None,
                merchant: None,
                transaction_id: None,
                expense_date: "some-expense-date".to_string(),
                labels: HashMap::new(),
                splits: Vec::new(),
                receipt_ids: Vec::new(),
                spent_amount: Default::default(),
            }
        }
    }

    impl Default for ExpenseState {
        fn default() -> Self {
            Self::Approved
        }
    }

    impl Default for TransactionType {
        fn default() -> Self {
            Self::CardPayment
        }
    }
}
