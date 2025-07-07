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
    client::{Body, Client, Environment, HttpMethod},
    errors::ApiResult,
    merchant::client::MerchantAuthentication,
};

pub mod v10 {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Default, Deserialize, Serialize)]
    pub struct CustomerRequest {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub full_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub business_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub email: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub phone: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub date_of_birth: Option<String>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Customer {
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub full_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub business_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub phone: Option<String>,
        pub created_at: String,
        pub updated_at: String,
        pub email: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub date_of_birth: Option<String>,
    }

    #[derive(Debug, Deserialize, strum::Display, Serialize)]
    #[serde(rename_all = "snake_case")]
    #[strum(serialize_all = "snake_case")]
    pub enum PaymentMethodType {
        #[serde(alias = "CARD")]
        Card,
        #[serde(alias = "REVOLUT_PAY")]
        RevolutPay,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct PaymentMethod {
        pub id: String,
        pub r#type: PaymentMethodType,
        pub saved_for: Option<String>,
        pub method_details: Option<PaymentMethodDetails>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct PaymentMethodDetails {
        pub bin: Option<String>,
        pub last4: Option<String>,
        pub expiry_month: Option<u8>,
        pub expiry_year: Option<u8>,
        pub cardholder_name: Option<String>,
        pub billing_address: Option<BillingAddress>,
        pub brand: Option<String>,
        pub funding: Option<String>,
        pub issuer: Option<String>,
        pub issuer_country: Option<String>,
        pub created_at: Option<String>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct BillingAddress {
        pub street_line_1: Option<String>,
        pub street_line_2: Option<String>,
        pub post_code: Option<String>,
        pub city: Option<String>,
        pub region: Option<String>,
        pub country_code: Option<String>,
    }

    #[derive(Clone, Debug, Deserialize, strum::Display, Serialize)]
    #[serde(rename_all = "snake_case")]
    #[strum(serialize_all = "snake_case")]
    pub enum PaymentMethodSavedForRequest {
        #[serde(alias = "CUSTOMER")]
        Customer,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct PaymentMethodRequest {
        saved_for: PaymentMethodSavedForRequest,
    }
}

pub async fn create<E: Environment>(
    client: &Client<E, MerchantAuthentication>,
    customer: &v10::CustomerRequest,
) -> ApiResult<v10::Customer> {
    client
        .request(
            HttpMethod::Post {
                body: Some(Body::Json(&customer)),
            },
            &client.environment.uri("1.0", "/customers"),
        )
        .await
}

pub async fn list<E: Environment>(
    client: &Client<E, MerchantAuthentication>,
) -> ApiResult<Vec<v10::Customer>> {
    client
        .request(
            HttpMethod::<()>::Get,
            &client.environment.uri("1.0", "/customers"),
        )
        .await
}

pub async fn retrieve<E: Environment>(
    client: &Client<E, MerchantAuthentication>,
    customer_id: &str,
) -> ApiResult<v10::Customer> {
    client
        .request(
            HttpMethod::<()>::Get,
            &client
                .environment
                .uri("1.0", &format!("/customers/{customer_id}")),
        )
        .await
}

pub async fn update<E: Environment>(
    client: &Client<E, MerchantAuthentication>,
    customer_id: &str,
    customer: &v10::CustomerRequest,
) -> ApiResult<v10::Customer> {
    client
        .request(
            HttpMethod::Patch {
                body: Some(Body::Json(&customer)),
            },
            &client
                .environment
                .uri("1.0", &format!("/customers/{customer_id}")),
        )
        .await
}

pub async fn delete<E: Environment>(
    client: &Client<E, MerchantAuthentication>,
    customer_id: &str,
) -> ApiResult<()> {
    client
        .request(
            HttpMethod::<()>::Delete,
            &client
                .environment
                .uri("1.0", &format!("/customers/{customer_id}")),
        )
        .await
}

pub async fn payment_methods<E: Environment>(
    client: &Client<E, MerchantAuthentication>,
    customer_id: &str,
) -> ApiResult<Vec<v10::PaymentMethod>> {
    client
        .request(
            HttpMethod::<()>::Get,
            &client
                .environment
                .uri("1.0", &format!("/customers/{customer_id}/payment-methods")),
        )
        .await
}

pub async fn payment_method<E: Environment>(
    client: &Client<E, MerchantAuthentication>,
    customer_id: &str,
    payment_method_id: &str,
) -> ApiResult<v10::PaymentMethod> {
    client
        .request(
            HttpMethod::<()>::Get,
            &client.environment.uri(
                "1.0",
                &format!("/customers/{customer_id}/payment-methods/{payment_method_id}"),
            ),
        )
        .await
}

pub async fn update_payment_method<E: Environment>(
    client: &Client<E, MerchantAuthentication>,
    customer_id: &str,
    payment_method_id: &str,
    payment_method: &v10::PaymentMethodRequest,
) -> ApiResult<v10::PaymentMethod> {
    client
        .request(
            HttpMethod::Patch {
                body: Some(Body::Json(payment_method)),
            },
            &client.environment.uri(
                "1.0",
                &format!("/customers/{customer_id}/payment-methods/{payment_method_id}"),
            ),
        )
        .await
}

pub async fn delete_payment_method<E: Environment>(
    client: &Client<E, MerchantAuthentication>,
    customer_id: &str,
    payment_method_id: &str,
) -> ApiResult<()> {
    client
        .request(
            HttpMethod::<()>::Delete,
            &client.environment.uri(
                "1.0",
                &format!("/customers/{customer_id}/payment-methods/{payment_method_id}"),
            ),
        )
        .await
}
