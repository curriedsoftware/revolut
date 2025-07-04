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
    client::{Body, Client, Environment, HttpMethod, ProductionEnvironment},
    errors::ApiResult,
    merchant::client::{self, MerchantAuthentication},
};

pub mod unversioned {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Default, Deserialize, Serialize)]
    pub struct RegisterDomainRequest {
        pub domain: String,
    }

    #[derive(Debug, Default, Deserialize, Serialize)]
    pub struct UnregisterDomainRequest {
        pub domain: String,
        pub reason: String,
    }
}

pub async fn register(
    client: &Client<ProductionEnvironment<client::MerchantClient>, MerchantAuthentication>,
    domain: &unversioned::RegisterDomainRequest,
) -> ApiResult<()> {
    client
        .request(
            HttpMethod::Post {
                body: Some(Body::Json(&domain)),
            },
            &client
                .environment
                .unversioned_uri("/apple-pay/domains/register"),
        )
        .await
}

pub async fn unregister(
    client: &Client<ProductionEnvironment<client::MerchantClient>, MerchantAuthentication>,
    domain: &unversioned::UnregisterDomainRequest,
) -> ApiResult<()> {
    client
        .request(
            HttpMethod::Post {
                body: Some(Body::Json(&domain)),
            },
            &client
                .environment
                .unversioned_uri("/apple-pay/domains/unregister"),
        )
        .await
}
