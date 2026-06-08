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

use crate::client::Client;

/// The error type returned by every Business API call. The Business API reports
/// its error `code` as a numeric identifier (e.g. `2101`), so it is pinned to
/// `errors::Error<u32>` — callers read `code: Option<u32>` directly and never
/// see a string Merchant-style code.
pub type Error = crate::errors::Error<u32>;
/// The result type returned by every Business API call. See [`Error`].
pub type ApiResult<T> = crate::errors::ApiResult<T, u32>;

pub mod accounts;
pub mod cards;
pub mod client;
pub mod counterparties;
pub mod expenses;
pub mod foreign_exchange;
pub mod generated;
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
    pub async fn cards(
        &self,
        list_params: &cards::v10::ListParams,
    ) -> ApiResult<Vec<cards::v10::Card>> {
        cards::list(self, list_params).await
    }

    pub async fn create_card(
        &self,
        card: &cards::v10::CreateCardParams,
    ) -> ApiResult<cards::v10::Card> {
        cards::create(self, card).await
    }

    pub async fn card(&self, card_id: &str) -> ApiResult<cards::v10::Card> {
        cards::retrieve(self, card_id).await
    }

    pub async fn update_card(
        &self,
        card_id: &str,
        card: &cards::v10::UpdateCardParams,
    ) -> ApiResult<cards::v10::Card> {
        cards::update(self, card_id, card).await
    }

    pub async fn terminate_card(&self, card_id: &str) -> ApiResult<()> {
        cards::terminate(self, card_id).await
    }

    pub async fn freeze_card(&self, card_id: &str) -> ApiResult<()> {
        cards::freeze(self, card_id).await
    }

    pub async fn unfreeze_card(&self, card_id: &str) -> ApiResult<()> {
        cards::unfreeze(self, card_id).await
    }

    pub async fn card_sensitive_details(
        &self,
        card_id: &str,
    ) -> ApiResult<cards::v10::CardSensitiveDetails> {
        cards::sensitive_details(self, card_id).await
    }
}

/// Counterparties API. Available only in production environments.
///
/// Trying to access these endpoints from the sandbox environment will
/// result in a compile error.
impl Client<ProductionEnvironment<client::BusinessClient>, client::BusinessAuthentication> {
    pub async fn counterparties(
        &self,
        list_params: &counterparties::v10::ListParams,
    ) -> ApiResult<Vec<counterparties::v10::Counterparty>> {
        counterparties::list(self, list_params).await
    }

    pub async fn counterparty(
        &self,
        counterparty_id: &str,
    ) -> ApiResult<counterparties::v10::Counterparty> {
        counterparties::retrieve(self, counterparty_id).await
    }

    pub async fn delete_counterparty(&self, counterparty_id: &str) -> ApiResult<()> {
        counterparties::delete(self, counterparty_id).await
    }

    pub async fn create_counterparty(
        &self,
        counterparty: &counterparties::v10::CounterpartyRequest,
    ) -> ApiResult<counterparties::v10::Counterparty> {
        counterparties::create(self, counterparty).await
    }

    pub async fn validate_account_name(
        &self,
        account_name: &counterparties::v10::AccountNameRequest,
    ) -> ApiResult<counterparties::v10::AccountName> {
        counterparties::validate_account_name(self, account_name).await
    }
}

/// The Expenses API is only available in the production environment.
///
/// Trying to access these endpoints from the sandbox environment will
/// result in a compile error.
impl Client<ProductionEnvironment<client::BusinessClient>, client::BusinessAuthentication> {
    pub async fn expenses(
        &self,
        list_params: &expenses::v10::ListParams,
    ) -> ApiResult<Vec<expenses::v10::Expense>> {
        expenses::list(self, list_params).await
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
impl<E: Environment> Client<E, client::BusinessAuthentication> {
    pub async fn foreign_exchange(
        &self,
        get_params: &foreign_exchange::v10::ExchangeRateGetParams,
    ) -> ApiResult<Vec<foreign_exchange::v10::ExchangeRate>> {
        foreign_exchange::get(self, get_params).await
    }

    pub async fn exchange(
        &self,
        exchange: &foreign_exchange::v10::ExchangeRequest,
    ) -> ApiResult<foreign_exchange::v10::Exchange> {
        foreign_exchange::exchange(self, exchange).await
    }
}

/// Payment Drafts API. Available in sandbox and production
/// environments.
impl<E: Environment> Client<E, client::BusinessAuthentication> {
    pub async fn payment_drafts(&self) -> ApiResult<payment_drafts::v10::PaymentDraft> {
        payment_drafts::list(self).await
    }

    pub async fn create_payment_draft(
        &self,
        payment_draft: &payment_drafts::v10::PaymentDraftRequest,
    ) -> ApiResult<payment_drafts::v10::CreatePaymentDraft> {
        payment_drafts::create(self, payment_draft).await
    }

    pub async fn payment_draft(
        &self,
        payment_draft_id: &str,
    ) -> ApiResult<payment_drafts::v10::PaymentDraftDetails> {
        payment_drafts::retrieve(self, payment_draft_id).await
    }

    pub async fn delete_payment_draft(&self, payment_draft_id: &str) -> ApiResult<()> {
        payment_drafts::delete(self, payment_draft_id).await
    }
}

/// Payout links API. Available in sandbox and production
/// environments.
impl<E: Environment> Client<E, client::BusinessAuthentication> {
    pub async fn payout_links(
        &self,
        list_params: &payout_links::v10::ListParams,
    ) -> ApiResult<Vec<payout_links::v10::PayoutLink>> {
        payout_links::list(self, list_params).await
    }

    pub async fn payout_link(
        &self,
        payout_link_id: &str,
    ) -> ApiResult<payout_links::v10::PayoutLink> {
        payout_links::retrieve(self, payout_link_id).await
    }

    pub async fn create_payout_link(
        &self,
        payout_link: &payout_links::v10::PayoutLinkRequest,
    ) -> ApiResult<payout_links::v10::PayoutLink> {
        payout_links::create(self, payout_link).await
    }

    pub async fn cancel_payout_link(
        &self,
        payout_link_id: &str,
    ) -> ApiResult<payout_links::v10::PayoutLink> {
        payout_links::cancel(self, payout_link_id).await
    }
}

/// The Simulations API is only available in the sandbox environment.
///
/// Trying to access these endpoints from the production environment
/// will result in a compile error.
impl Client<SandboxEnvironment<client::BusinessClient>, client::BusinessAuthentication> {
    pub async fn simulate_transfer_state_update(
        &self,
        id: &str,
        state: &simulations::v10::TransferStateRequest,
    ) -> ApiResult<simulations::v10::TransferStateUpdate> {
        simulations::transfer_state_update(self, id, state).await
    }

    pub async fn simulate_account_topup(
        &self,
        top_up: &simulations::v10::TopUpRequest,
    ) -> ApiResult<simulations::v10::TopUp> {
        simulations::account_top_up(self, top_up).await
    }
}

/// The Team members API is only available in the production environment.
///
/// Trying to access these endpoints from the sandbox environment will
/// result in a compile error.
impl Client<ProductionEnvironment<client::BusinessClient>, client::BusinessAuthentication> {
    pub async fn team_members(
        &self,
        list_params: &team_members::v10::ListParams,
    ) -> ApiResult<Vec<team_members::v10::TeamMember>> {
        team_members::list(self, list_params).await
    }

    pub async fn invite_new_member(
        &self,
        member_invite: &team_members::v10::TeamMemberInviteRequest,
    ) -> ApiResult<team_members::v10::TeamMemberInvite> {
        team_members::invite_new_member(self, member_invite).await
    }

    pub async fn team_roles(
        &self,
        list_params: &team_members::v10::ListParams,
    ) -> ApiResult<Vec<team_members::v10::TeamRole>> {
        team_members::list_team_roles(self, list_params).await
    }
}

/// Transactions API. Available in sandbox and production
/// environments.
impl<E: Environment> Client<E, client::BusinessAuthentication> {
    pub async fn transaction_list(
        &self,
        list_params: &transactions::v10::ListParams,
    ) -> ApiResult<Vec<transactions::v10::Transaction>> {
        transactions::list(self, list_params).await
    }

    pub async fn retrieve(
        &self,
        retrieve_param: &transactions::RetrieveParam<'_>,
    ) -> ApiResult<transactions::v10::Transaction> {
        transactions::retrieve(self, retrieve_param).await
    }
}

/// Transfers API. Available in sandbox and production environments.
impl<E: Environment> Client<E, client::BusinessAuthentication> {
    pub async fn get_transfer_reasons(&self) -> ApiResult<Vec<transfers::v10::TransferReason>> {
        transfers::get_transfer_reasons(self).await
    }

    pub async fn get_exchange_reasons(&self) -> ApiResult<Vec<transfers::v10::ExchangeReason>> {
        transfers::get_exchange_reasons(self).await
    }

    pub async fn transfer(
        &self,
        transfer_params: &transfers::v10::TransferRequest,
    ) -> ApiResult<transfers::v10::Transfer> {
        transfers::transfer(self, transfer_params).await
    }

    pub async fn pay(
        &self,
        pay_params: &transfers::v10::PayRequest,
    ) -> ApiResult<transfers::v10::Pay> {
        transfers::pay(self, pay_params).await
    }
}

/// Webhooks API. Available in sandbox and production environments.
impl<E: Environment> Client<E, client::BusinessAuthentication> {
    pub async fn create_webhook(
        &self,
        webhook: &webhooks::v2::WebhookRequest,
    ) -> ApiResult<webhooks::v2::WebhookWithSigningSecret> {
        webhooks::v2::create(self, webhook).await
    }

    pub async fn webhooks(&self) -> ApiResult<Vec<webhooks::v2::Webhook>> {
        webhooks::v2::list(self).await
    }

    pub async fn webhook(&self, webhook_id: &str) -> ApiResult<webhooks::v2::Webhook> {
        webhooks::v2::retrieve(self, webhook_id).await
    }

    pub async fn update_webhook(
        &self,
        webhook_id: &str,
        webhook: &webhooks::v2::WebhookRequest,
    ) -> ApiResult<webhooks::v2::Webhook> {
        webhooks::v2::update(self, webhook_id, webhook).await
    }

    pub async fn delete_webhook(&self, webhook_id: &str) -> ApiResult<()> {
        webhooks::v2::delete(self, webhook_id).await
    }

    pub async fn rotate_signing_secret(
        &self,
        webhook_id: &str,
        rotate_webhook_signing_secret: &webhooks::v2::RotateWebhookSigningSecretRequest,
    ) -> ApiResult<webhooks::v2::WebhookWithSigningSecret> {
        webhooks::v2::rotate_signing_secret(self, webhook_id, rotate_webhook_signing_secret).await
    }

    pub async fn failed_webhook_events(
        &self,
        webhook_id: &str,
        list_params: &webhooks::v2::ListParams,
    ) -> ApiResult<Vec<webhooks::v2::FailedWebhookEvent>> {
        webhooks::v2::failed_webhook_events(self, webhook_id, list_params).await
    }
}
