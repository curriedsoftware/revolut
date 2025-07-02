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
    client::Environment,
    errors::ApiResult,
    merchant::{Client, client, customers, orders},
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

    pub async fn orders(&self) -> ApiResult<Vec<orders::v10::Order>> {
        orders::list(self).await
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

    pub async fn customers(&self) -> ApiResult<Vec<customers::v10::Customer>> {
        customers::list(self).await
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
    ) -> ApiResult<Vec<customers::v10::PaymentMethod>> {
        customers::payment_methods(self, customer_id).await
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
