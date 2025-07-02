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

use unversioned::LocationRequest;

use crate::{
    client::{Body, Client, Environment, HttpMethod},
    errors::ApiResult,
    merchant::client::MerchantAuthentication,
};

pub mod unversioned {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    pub struct LocationRequest {
        pub name: String,
        pub r#type: LocationType,
        pub details: LocationDetails,
    }

    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    #[serde(rename_all = "snake_case")]
    pub enum LocationType {
        Online,
    }

    #[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct LocationDetails {
        pub domain: String,
    }

    #[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Location {
        pub id: String,
        pub name: String,
        pub r#type: String,
        pub details: LocationDetails,
    }
}

pub async fn create<E: Environment>(
    client: &Client<E, MerchantAuthentication>,
    location: &unversioned::LocationRequest,
) -> ApiResult<unversioned::Location> {
    client
        .request(
            HttpMethod::Post {
                body: Some(Body::Json(&location)),
            },
            &client.environment.unversioned_uri("/locations"),
        )
        .await
}

pub async fn list<E: Environment>(
    client: &Client<E, MerchantAuthentication>,
) -> ApiResult<Vec<unversioned::Location>> {
    client
        .request(
            HttpMethod::Get::<()>,
            &client.environment.unversioned_uri("/locations"),
        )
        .await
}

pub async fn retrieve<E: Environment>(
    client: &Client<E, MerchantAuthentication>,
    location_id: &str,
) -> ApiResult<unversioned::Location> {
    client
        .request(
            HttpMethod::Get::<()>,
            &client
                .environment
                .unversioned_uri(&format!("/locations/{location_id}")),
        )
        .await
}

pub async fn update<E: Environment>(
    client: &Client<E, MerchantAuthentication>,
    location_id: &str,
    location: &unversioned::LocationRequest,
) -> ApiResult<unversioned::Location> {
    client
        .request(
            HttpMethod::Patch {
                body: Some(Body::Json(&location)),
            },
            &client
                .environment
                .unversioned_uri(&format!("/locations/{location_id}")),
        )
        .await
}

pub async fn delete<E: Environment>(
    client: &Client<E, MerchantAuthentication>,
    location_id: &str,
) -> ApiResult<()> {
    client
        .request(
            HttpMethod::<()>::Delete,
            &client
                .environment
                .unversioned_uri(&format!("/locations/{location_id}")),
        )
        .await
}
