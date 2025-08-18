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

pub mod unversioned {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize)]
    pub struct RegisterAddressValidationEndpointForFastCheckoutRequest {
        pub event_type: RegisterAddressValidationEndpointForFastCheckoutEventType,
        pub url: String,
        pub location_id: Option<String>,
    }

    #[derive(Debug, Deserialize, strum::Display, Serialize)]
    pub enum RegisterAddressValidationEndpointForFastCheckoutEventType {
        #[serde(rename = "fast_checkout.validate_address")]
        ValidateAddress,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct RegisterAddressValidationEndpointForFastCheckout {
        pub id: String,
        pub signing_key: String,
        pub url: String,
        pub event_type: RegisterAddressValidationEndpointForFastCheckoutEventType,
        pub location_id: Option<String>,
    }
}

pub async fn register_address_validation_endpoint_for_fast_checkout<E: Environment>(
    client: &Client<E, MerchantAuthentication>,
    address_validation_endpoint: &unversioned::RegisterAddressValidationEndpointForFastCheckoutRequest,
) -> ApiResult<unversioned::RegisterAddressValidationEndpointForFastCheckout> {
    client
        .request(
            HttpMethod::Post {
                body: Some(Body::Json(&address_validation_endpoint)),
            },
            &client.environment.unversioned_uri("/synchronous-webhooks"),
        )
        .await
}

pub async fn retrieve_synchronous_webhook_list<E: Environment>(
    client: &Client<E, MerchantAuthentication>,
) -> ApiResult<Vec<unversioned::RegisterAddressValidationEndpointForFastCheckout>> {
    client
        .request(
            HttpMethod::Get::<()>,
            &client.environment.unversioned_uri("/synchronous-webhooks"),
        )
        .await
}

pub async fn delete_synchronous_webhook<E: Environment>(
    client: &Client<E, MerchantAuthentication>,
    synchronous_webhook_id: &str,
) -> ApiResult<()> {
    client
        .request(
            HttpMethod::Delete::<()>,
            &client
                .environment
                .unversioned_uri(&format!("/synchronous-webooks/{synchronous_webhook_id}")),
        )
        .await
}

#[cfg(test)]
mod tests {
    use super::unversioned::*;

    impl Default for RegisterAddressValidationEndpointForFastCheckout {
        fn default() -> Self {
            Self {
                id: "some-register-addres-vaildation-endpoint-for-fast-checkout".to_string(),
                signing_key: "some-signing-key".to_string(),
                url: "some-url".to_string(),
                event_type: Default::default(),
                location_id: None,
            }
        }
    }

    impl Default for RegisterAddressValidationEndpointForFastCheckoutEventType {
        fn default() -> Self {
            Self::ValidateAddress
        }
    }
}
