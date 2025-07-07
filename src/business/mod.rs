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

use client::{Environment, ProductionEnvironment, SandboxEnvironment};

use crate::{client::Client, errors::ApiResult};

pub mod accounts;
pub mod cards;
pub mod client;
pub mod counterparties;
pub mod expenses;
pub mod foreign_exchange;
pub mod payment_drafts;
pub mod payout_links;
pub mod simulations;
pub mod team_members;
pub mod transactions;
pub mod transfers;
pub mod webhooks;

/// Accounts API. Available in sandbox and production environments.
impl<E: Environment> Client<E, client::BusinessAuthentication> {
    pub async fn accounts(&self) -> ApiResult<Vec<accounts::v10::Account>> {
        accounts::list(self).await
    }

    pub async fn account(&self, account_id: &str) -> ApiResult<accounts::v10::Account> {
        accounts::retrieve(self, account_id).await
    }

    pub async fn bank_details(&self, account_id: &str) -> ApiResult<accounts::v10::BankDetails> {
        accounts::bank_details(self, account_id).await
    }
}

/// The Cards API is only available in the production environment.
///
/// Trying to access these endpoints from the sandbox environment will
/// result in a compile error.
impl Client<ProductionEnvironment<client::BusinessClient>, client::BusinessAuthentication> {
    pub async fn cards(&self) -> ApiResult<Vec<cards::v10::Card>> {
        cards::list(self).await
    }

    pub async fn create_card(&self, card: &cards::v10::Card) -> ApiResult<cards::v10::Card> {
        unimplemented!()
    }

    pub async fn card(&self, card_id: &str) -> ApiResult<cards::v10::Card> {
        unimplemented!()
    }

    pub async fn update_card(&self, card: &cards::v10::Card) -> ApiResult<cards::v10::Card> {
        unimplemented!()
    }

    pub async fn terminate_card(&self, card_id: &str) -> ApiResult<()> {
        unimplemented!()
    }

    pub async fn freeze_card(&self, card_id: &str) -> ApiResult<()> {
        unimplemented!()
    }

    pub async fn unfreeze_card(&self, card_id: &str) -> ApiResult<()> {
        unimplemented!()
    }

    pub async fn card_sensitive_details(
        &self,
        card_id: &str,
    ) -> ApiResult<cards::v10::CardSensitiveDetails> {
        unimplemented!()
    }
}

/// Counterparties API. Available in sandbox and production
/// environments.
impl<E: Environment> Client<E, client::BusinessAuthentication> {}

/// The Expenses API is only available in the production environment.
///
/// Trying to access these endpoints from the sandbox environment will
/// result in a compile error.
impl Client<ProductionEnvironment<client::BusinessClient>, client::BusinessAuthentication> {
    pub async fn expenses(&self) -> ApiResult<Vec<expenses::v10::Expense>> {
        expenses::list(self).await
    }

    pub async fn expense(&self, expense_id: &str) -> ApiResult<expenses::v10::Expense> {
        expenses::retrieve(self, expense_id).await
    }

    pub async fn expense_receipt(&self, expense_id: &str, receipt_id: &str) -> ApiResult<Vec<u8>> {
        expenses::expense_receipt(self, expense_id, receipt_id).await
    }
}

/// Foreign Exchange API. Available in sandbox and production
/// environments.
impl<E: Environment> Client<E, client::BusinessAuthentication> {}

/// Payment Drafts API. Available in sandbox and production
/// environments.
impl<E: Environment> Client<E, client::BusinessAuthentication> {}

/// Payout links API. Available in sandbox and production
/// environments.
impl<E: Environment> Client<E, client::BusinessAuthentication> {}

/// The Simulations API is only available in the sandbox environment.
///
/// Trying to access these endpoints from the production environment
/// will result in a compile error.
impl Client<SandboxEnvironment<client::BusinessClient>, client::BusinessAuthentication> {
    pub async fn simulate_transfer_state_update() {
        unimplemented!()
    }

    pub async fn simulate_account_topup() {
        unimplemented!()
    }
}

/// The Team members API is only available in the production environment.
///
/// Trying to access these endpoints from the sandbox environment will
/// result in a compile error.
impl Client<ProductionEnvironment<client::BusinessClient>, client::BusinessAuthentication> {
    pub async fn team_members() {
        unimplemented!()
    }

    pub async fn invite_new_member() {
        unimplemented!()
    }

    pub async fn team_roles() {
        unimplemented!()
    }
}

/// Transactions API. Available in sandbox and production
/// environments.
impl<E: Environment> Client<E, client::BusinessAuthentication> {}

/// Transfers API. Available in sandbox and production environments.
impl<E: Environment> Client<E, client::BusinessAuthentication> {}
