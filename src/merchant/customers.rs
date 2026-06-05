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

    // Reused verbatim from the OpenAPI-generated types. These stay in sync
    // automatically when the specs are bumped via `just generate`; we only
    // alias them back to this crate's public names. Each endpoint returns a
    // distinct generated shape, so we expose them directly rather than wrap
    // them in hand-written copies that would silently drift from the spec:
    //   - create               -> `CustomerCreationV2` / `CustomerCreated`
    //   - list                 -> paginated `Customers`
    //   - retrieve / update    -> `CustomerUpdateV2` / `CustomerV3`
    //   - payment methods       -> `CustomerPaymentMethodsV2` / `PaymentMethodV4`
    pub use crate::merchant::generated::{
        CustomerCreated, CustomerCreationV2, CustomerPaymentMethodsV2, CustomerUpdateV2,
        CustomerV3, Customers, PaymentMethodV4,
    };

    #[derive(Clone, Debug, Default)]
    pub struct ListParams {
        pub limit: Option<u16>,
        pub page: Option<u64>,
    }

    #[derive(Clone, Debug, Default)]
    pub struct PaymentMethodListParams {
        pub only_merchant: Option<bool>,
    }

    // SCREAMING_SNAKE_CASE
    #[derive(Clone, Debug, Deserialize, strum::Display, Serialize)]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    pub enum PaymentMethodSavedForRequest {
        Customer,
    }

    // Thin request wrapper kept on purpose: the generated `Payment-Method-Update`
    // types `saved_for` as a free-form `Option<String>`, whereas we restrict it
    // to the values this client supports.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct PaymentMethodRequest {
        saved_for: PaymentMethodSavedForRequest,
    }
}

impl std::fmt::Display for v10::ListParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let query = [
            ("limit", &self.limit.map(|limit| limit.to_string())),
            ("page", &self.page.map(|page| page.to_string())),
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

impl std::fmt::Display for v10::PaymentMethodListParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let query = [(
            "only_merchant",
            if self.only_merchant == Some(true) {
                Some("true")
            } else {
                None
            },
        )]
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

pub async fn create<E: Environment>(
    client: &Client<E, MerchantAuthentication>,
    customer: &v10::CustomerCreationV2,
) -> ApiResult<v10::CustomerCreated> {
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
    list_params: &v10::ListParams,
) -> ApiResult<v10::Customers> {
    client
        .request(
            HttpMethod::<()>::Get,
            &client
                .environment
                .uri("1.0", &format!("/customers{list_params}")),
        )
        .await
}

pub async fn retrieve<E: Environment>(
    client: &Client<E, MerchantAuthentication>,
    customer_id: &str,
) -> ApiResult<v10::CustomerV3> {
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
    customer: &v10::CustomerUpdateV2,
) -> ApiResult<v10::CustomerV3> {
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
    list_params: &v10::PaymentMethodListParams,
) -> ApiResult<v10::CustomerPaymentMethodsV2> {
    client
        .request(
            HttpMethod::<()>::Get,
            &client.environment.uri(
                "1.0",
                &format!("/customers/{customer_id}/payment-methods{list_params}"),
            ),
        )
        .await
}

pub async fn payment_method<E: Environment>(
    client: &Client<E, MerchantAuthentication>,
    customer_id: &str,
    payment_method_id: &str,
) -> ApiResult<v10::PaymentMethodV4> {
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
) -> ApiResult<v10::PaymentMethodV4> {
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

#[cfg(test)]
mod tests {
    use super::v10::*;

    impl Default for CustomerV3 {
        fn default() -> Self {
            Self {
                id: "00000000-0000-0000-0000-000000000000".parse().unwrap(),
                full_name: None,
                phone: None,
                created_at: "2025-06-11T15:28:36Z".parse().unwrap(),
                updated_at: "2025-07-11T15:28:36Z".parse().unwrap(),
                email: "some-email@example.com".parse().unwrap(),
                payment_methods: vec![],
            }
        }
    }

    impl Default for CustomerCreated {
        fn default() -> Self {
            Self {
                id: "00000000-0000-0000-0000-000000000000".parse().unwrap(),
                full_name: None,
                phone: None,
                created_at: "2025-06-11T15:28:36Z".parse().unwrap(),
                updated_at: "2025-07-11T15:28:36Z".parse().unwrap(),
                email: "some-email@example.com".parse().unwrap(),
            }
        }
    }

    impl Default for Customers {
        fn default() -> Self {
            Self {
                next_page_token: None,
                customers: vec![],
            }
        }
    }

    impl Default for CustomerPaymentMethodsV2 {
        fn default() -> Self {
            Self {
                payment_methods: vec![],
            }
        }
    }

    impl Default for PaymentMethodV4 {
        fn default() -> Self {
            use crate::merchant::generated::{PaymentMethodTypeV2, RevolutPayV2};
            Self::RevolutPayV2(RevolutPayV2 {
                id: "00000000-0000-0000-0000-000000000000".parse().unwrap(),
                type_: PaymentMethodTypeV2::RevolutPay,
                saved_for: None,
                created_at: "2025-06-11T15:28:36Z".parse().unwrap(),
            })
        }
    }
}
