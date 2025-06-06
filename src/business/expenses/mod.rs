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
    business::client::{BusinessAuthentication, Environment, HttpMethod, ProductionEnvironment},
    client::Client,
    errors::Result,
};

use std::vec::Vec;

pub mod v10 {
    use super::Vec;
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Amount {
        amount: Option<f64>,
        currency: Option<String>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Category {
        name: String,
        code: Option<String>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct TaxRate {
        name: String,
        percentage: f64,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct ExpenseSplit {
        amount: Amount,
        category: Category,
        tax_rate: TaxRate,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct ExpenseSpentAmount {
        amount: f64,
        currency: String,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Expense {
        id: String,
        state: String,
        transaction_type: String,
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
}

pub async fn list(
    client: &Client<ProductionEnvironment, BusinessAuthentication>,
) -> Result<Vec<v10::Expense>> {
    client
        .request(HttpMethod::Get, &client.environment.uri("1.0", "/expenses"))
        .await
}

pub async fn expense(
    client: &Client<ProductionEnvironment, BusinessAuthentication>,
    expense_id: &str,
) -> Result<v10::Expense> {
    client
        .request(
            HttpMethod::Get,
            &client
                .environment
                .uri("1.0", &format!("/expenses/{expense_id}")),
        )
        .await
}

pub async fn expense_receipt(
    client: &Client<ProductionEnvironment, BusinessAuthentication>,
    expense_id: &str,
    receipt_id: &str,
) -> Result<Vec<u8>> {
    client
        .request_raw(
            HttpMethod::Get,
            &client.environment.uri(
                "1.0",
                &format!("/expenses/{expense_id}/receipts/{receipt_id}/content"),
            ),
        )
        .await
}
