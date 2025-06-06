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
    errors::Result,
    merchant::{client, orders, Client},
};

use super::orders::v10;

/// Orders API. Available in sandbox and production environments.
impl<E: Environment> Client<E, client::MerchantAuthentication> {
    pub async fn create_order(&self, order: &v10::OrderRequest) -> Result<v10::Order> {
        orders::create(self, order).await
    }

    pub async fn order(&self, order_id: &str) -> Result<v10::Order> {
        orders::retrieve(self, order_id).await
    }

    pub async fn update_order(
        &self,
        order_id: &str,
        order: &v10::OrderRequest,
    ) -> Result<v10::Order> {
        orders::update(self, order_id, order).await
    }

    pub async fn capture_order(&self, order_id: &str, amount: u64) -> Result<v10::Order> {
        orders::capture(self, order_id, amount).await
    }

    pub async fn orders(&self) -> Result<Vec<v10::Order>> {
        orders::list(self).await
    }

    pub async fn cancel_order(&self, order_id: &str) -> Result<v10::Order> {
        orders::cancel(self, order_id).await
    }
}
