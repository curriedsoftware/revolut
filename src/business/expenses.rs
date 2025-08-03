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

    #[derive(Clone, Debug, Default)]
    pub struct ListParams {
        pub from: Option<String>,
        pub to: Option<String>,
        pub count: Option<u64>,
        pub state: Option<ExpenseState>,
        pub transaction_type: Option<TransactionType>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Amount {
        amount: Option<f64>,
        currency: Option<String>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Category {
        name: String,
        code: Option<String>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct TaxRate {
        name: String,
        percentage: f64,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ExpenseSplit {
        amount: Amount,
        category: Category,
        tax_rate: TaxRate,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ExpenseSpentAmount {
        amount: f64,
        currency: String,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Expense {
        id: String,
        state: ExpenseState,
        transaction_type: TransactionType,
        description: Option<String>,
        submitted_at: Option<String>,
        completed_at: Option<String>,
        payer: Option<String>,
        merchant: Option<String>,
        transaction_id: Option<String>,
        expense_date: String,
        labels: HashMap<String, Vec<String>>,
        splits: Vec<ExpenseSplit>,
        receipt_ids: Vec<String>,
        spent_amount: ExpenseSpentAmount,
    }

    #[derive(Clone, Debug, Deserialize, strum::Display, Serialize)]
    #[serde(rename_all = "snake_case")]
    #[strum(serialize_all = "snake_case")]
    pub enum ExpenseState {
        #[serde(alias = "MISSING_INFO")]
        MissingInfo,
        #[serde(alias = "AWAITING_REVIEW")]
        AwaitingReview,
        #[serde(alias = "REJECTED")]
        Rejected,
        #[serde(alias = "PENDING_REIMBURSEMENT")]
        PendingReimbursement,
        #[serde(alias = "REFUND_REQUESTED")]
        RefundRequested,
        #[serde(alias = "REFUNDED")]
        Refunded,
        #[serde(alias = "APPROVED")]
        Approved,
        #[serde(alias = "REVERTED")]
        Reverted,
    }

    #[derive(Clone, Debug, Deserialize, strum::Display, Serialize)]
    #[serde(rename_all = "snake_case")]
    #[strum(serialize_all = "snake_case")]
    pub enum TransactionType {
        #[serde(alias = "ATM")]
        Atm,
        #[serde(alias = "CARD_PAYMENT")]
        CardPayment,
        #[serde(alias = "FEE")]
        Fee,
        #[serde(alias = "TRANSFER")]
        Transfer,
        #[serde(alias = "EXTERNAL")]
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
