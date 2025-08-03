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

use crate::{
    client::{Environment, ProductionEnvironment},
    errors::ApiResult,
    merchant::{
        Client, apple_pay, client, customers, disputes, locations, orders, other, payments,
        payouts, report_runs, webhooks,
    },
};

/// Orders API. Available in sandbox and production environments.
impl<E: Environment> Client<E, client::MerchantAuthentication> {
    pub async fn create_order(
        &self,
        order: &orders::v10::OrderRequest,
    ) -> ApiResult<orders::v10::Order> {
        orders::create(self, order).await
    }

    pub async fn order(&self, order_id: &str) -> ApiResult<orders::v10::Order> {
        orders::retrieve(self, order_id).await
    }

    pub async fn update_order(
        &self,
        order_id: &str,
        order: &orders::v10::OrderRequest,
    ) -> ApiResult<orders::v10::Order> {
        orders::update(self, order_id, order).await
    }

    pub async fn capture_order(
        &self,
        order_id: &str,
        amount: u64,
    ) -> ApiResult<orders::v10::Order> {
        orders::capture(self, order_id, amount).await
    }

    pub async fn orders(
        &self,
        list_params: &orders::v10::ListParams,
    ) -> ApiResult<Vec<orders::v10::Order>> {
        orders::list(self, list_params).await
    }

    pub async fn cancel_order(&self, order_id: &str) -> ApiResult<orders::v10::Order> {
        orders::cancel(self, order_id).await
    }

    pub async fn refund_order(
        &self,
        order_id: &str,
        refund: &orders::v10::RefundRequest,
    ) -> ApiResult<orders::v10::Order> {
        orders::refund(self, order_id, refund).await
    }

    pub async fn pay_order(
        &self,
        order_id: &str,
        saved_payment_method: &orders::v10::SavedPaymentMethod,
    ) -> ApiResult<orders::v10::OrderPayment> {
        orders::pay(self, order_id, saved_payment_method).await
    }

    pub async fn order_payments(
        &self,
        order_id: &str,
    ) -> ApiResult<Vec<orders::v10::OrderPayment>> {
        orders::payment_list(self, order_id).await
    }
}

/// Customers API. Available in sandbox and production environments.
impl<E: Environment> Client<E, client::MerchantAuthentication> {
    pub async fn create_customer(
        &self,
        customer: &customers::v10::CustomerRequest,
    ) -> ApiResult<customers::v10::Customer> {
        customers::create(self, customer).await
    }

    pub async fn customers(
        &self,
        list_params: &customers::v10::ListParams,
    ) -> ApiResult<Vec<customers::v10::Customer>> {
        customers::list(self, list_params).await
    }

    pub async fn customer(&self, customer_id: &str) -> ApiResult<customers::v10::Customer> {
        customers::retrieve(self, customer_id).await
    }

    pub async fn update_customer(
        &self,
        customer_id: &str,
        customer: &customers::v10::CustomerRequest,
    ) -> ApiResult<customers::v10::Customer> {
        customers::update(self, customer_id, customer).await
    }

    pub async fn delete_customer(&self, customer_id: &str) -> ApiResult<()> {
        customers::delete(self, customer_id).await
    }

    pub async fn payment_methods(
        &self,
        customer_id: &str,
        list_params: &customers::v10::PaymentMethodListParams,
    ) -> ApiResult<Vec<customers::v10::PaymentMethod>> {
        customers::payment_methods(self, customer_id, list_params).await
    }

    pub async fn payment_method(
        &self,
        customer_id: &str,
        payment_method_id: &str,
    ) -> ApiResult<customers::v10::PaymentMethod> {
        customers::payment_method(self, customer_id, payment_method_id).await
    }

    pub async fn update_payment_method(
        &self,
        customer_id: &str,
        payment_method_id: &str,
        payment_method: &customers::v10::PaymentMethodRequest,
    ) -> ApiResult<customers::v10::PaymentMethod> {
        customers::update_payment_method(self, customer_id, payment_method_id, payment_method).await
    }

    pub async fn delete_payment_method(
        &self,
        customer_id: &str,
        payment_method_id: &str,
    ) -> ApiResult<()> {
        customers::delete_payment_method(self, customer_id, payment_method_id).await
    }
}

// Payments API. Available in sandbox and production environments.
impl<E: Environment> Client<E, client::MerchantAuthentication> {
    pub async fn payment_details(
        &self,
        payment_id: &str,
    ) -> ApiResult<payments::unversioned::Payment> {
        payments::retrieve(self, payment_id).await
    }
}

// Payouts API. Available in sandbox and production environments.
impl<E: Environment> Client<E, client::MerchantAuthentication> {
    pub async fn payouts(
        &self,
        list_params: &payouts::unversioned::ListParams,
    ) -> ApiResult<Vec<payouts::unversioned::Payout>> {
        payouts::list(self, list_params).await
    }

    pub async fn payout_details(&self, payout_id: &str) -> ApiResult<payouts::unversioned::Payout> {
        payouts::retrieve(self, payout_id).await
    }
}

// Disputes API. Available only in the production environment.
///
/// Trying to access these endpoints from the sandbox environment will
/// result in a compile error.
impl Client<ProductionEnvironment<client::MerchantClient>, client::MerchantAuthentication> {
    pub async fn disputes(
        &self,
        list_params: &disputes::unversioned::ListParams,
    ) -> ApiResult<Vec<disputes::unversioned::Dispute>> {
        disputes::list(self, list_params).await
    }

    pub async fn dispute(&self, dispute_id: &str) -> ApiResult<disputes::unversioned::Dispute> {
        disputes::retrieve(self, dispute_id).await
    }

    pub async fn accept_dispute(&self, dispute_id: &str) -> ApiResult<()> {
        disputes::accept(self, dispute_id).await
    }

    pub async fn upload_dispute_evidence(
        &self,
        dispute_id: &str,
        evidence: &disputes::unversioned::EvidenceRequest<'_>,
    ) -> ApiResult<disputes::unversioned::Evidence> {
        disputes::upload_evidence(self, dispute_id, evidence).await
    }

    pub async fn challenge_dispute(
        &self,
        dispute_id: &str,
        challenge_dispute: &disputes::unversioned::ChallengeDisputeRequest,
    ) -> ApiResult<()> {
        disputes::challenge(self, dispute_id, challenge_dispute).await
    }
}

// Report runs API. Available in sandbox and production environments.
impl<E: Environment> Client<E, client::MerchantAuthentication> {
    pub async fn create_report_run(
        &self,
        order: &report_runs::unversioned::ReportRunRequest,
    ) -> ApiResult<report_runs::unversioned::ReportRun> {
        report_runs::create(self, order).await
    }

    pub async fn report_run_details(
        &self,
        report_run_id: &str,
    ) -> ApiResult<report_runs::unversioned::ReportRun> {
        report_runs::retrieve(self, report_run_id).await
    }

    pub async fn download_report_run(&self, report_run_id: &str) -> ApiResult<Vec<u8>> {
        report_runs::download(self, report_run_id).await
    }
}

// Webhooks API. Available in sandbox and production environments.
impl<E: Environment> Client<E, client::MerchantAuthentication> {
    pub async fn create_webhook(
        &self,
        webhook: &webhooks::v10::WebhookRequest,
    ) -> ApiResult<webhooks::v10::Webhook> {
        webhooks::create(self, webhook).await
    }

    pub async fn webhooks(&self) -> ApiResult<Vec<webhooks::v10::Webhook>> {
        webhooks::list(self).await
    }

    pub async fn webhook(&self, webhook_id: &str) -> ApiResult<webhooks::v10::Webhook> {
        webhooks::retrieve(self, webhook_id).await
    }

    pub async fn delete_webhook(&self, webhook_id: &str) -> ApiResult<()> {
        webhooks::delete(self, webhook_id).await
    }

    pub async fn rotate_webhook_signing_secret(
        &self,
        webhook_id: &str,
        rotate_webhook_signing_secret: &webhooks::v10::RotateWebhookSigningSecretRequest,
    ) -> ApiResult<webhooks::v10::Webhook> {
        webhooks::rotate_signing_secret(self, webhook_id, rotate_webhook_signing_secret).await
    }
}

// Locations API. Available in sandbox and production environments.
impl<E: Environment> Client<E, client::MerchantAuthentication> {
    pub async fn create_location(
        &self,
        location: &locations::unversioned::LocationRequest,
    ) -> ApiResult<locations::unversioned::Location> {
        locations::create(self, location).await
    }

    pub async fn locations(&self) -> ApiResult<Vec<locations::unversioned::Location>> {
        locations::list(self).await
    }

    pub async fn location(&self, location_id: &str) -> ApiResult<locations::unversioned::Location> {
        locations::retrieve(self, location_id).await
    }

    pub async fn update_location(
        &self,
        location_id: &str,
        location: &locations::unversioned::LocationRequest,
    ) -> ApiResult<locations::unversioned::Location> {
        locations::update(self, location_id, location).await
    }

    pub async fn delete_location(&self, location_id: &str) -> ApiResult<()> {
        locations::delete(self, location_id).await
    }
}

// Apple Pay API. Available in sandbox and production environments.
impl<E: Environment> Client<E, client::MerchantAuthentication> {
    pub async fn register_domain_for_apple_pay(
        &self,
        domain: &apple_pay::unversioned::RegisterDomainRequest,
    ) -> ApiResult<()> {
        apple_pay::register(self, domain).await
    }

    pub async fn unregister_domain_for_apple_pay(
        &self,
        domain: &apple_pay::unversioned::UnregisterDomainRequest,
    ) -> ApiResult<()> {
        apple_pay::unregister(self, domain).await
    }
}

// "Other" API. Available in sandbox and production environments.
impl<E: Environment> Client<E, client::MerchantAuthentication> {
    pub async fn register_address_validation_endpoint_for_fast_checkout(
        &self,
        address_validation_endpoint: &other::unversioned::RegisterAddressValidationEndpointForFastCheckoutRequest,
    ) -> ApiResult<other::unversioned::RegisterAddressValidationEndpointForFastCheckout> {
        other::register_address_validation_endpoint_for_fast_checkout(
            self,
            address_validation_endpoint,
        )
        .await
    }

    pub async fn retrieve_synchronous_webhook_list(
        &self,
    ) -> ApiResult<Vec<other::unversioned::RegisterAddressValidationEndpointForFastCheckout>> {
        other::retrieve_synchronous_webhook_list(self).await
    }

    pub async fn delete_synchronous_webhook(&self, synchronous_webhook_id: &str) -> ApiResult<()> {
        other::delete_synchronous_webhook(self, synchronous_webhook_id).await
    }
}
