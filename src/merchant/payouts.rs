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
    client::{Client, Environment, HttpMethod},
    errors::ApiResult,
    merchant::client::MerchantAuthentication,
};

pub mod unversioned {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Payout {
        pub id: String,
        pub state: PayoutState,
        pub created_at: String,
        pub destination_type: PayoutDestinationType,
        pub amount: Option<u64>,
        pub currency: Option<String>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub enum PayoutState {
        Processing,
        Completed,
        Failed,
    }

    #[derive(Debug, Deserialize, Serialize)]
    #[serde(rename_all = "snake_case")]
    pub enum PayoutDestinationType {
        #[serde(alias = "CURRENT_POCKET")]
        CurrentPocket,
        #[serde(alias = "EXTERNAL_BENEFICIARY")]
        ExternalBeneficiary,
    }
}

pub async fn list<E: Environment>(
    client: &Client<E, MerchantAuthentication>,
) -> ApiResult<Vec<unversioned::Payout>> {
    client
        .request(
            HttpMethod::<()>::Get,
            &client.environment.unversioned_uri("/payouts"),
        )
        .await
}

pub async fn retrieve<E: Environment>(
    client: &Client<E, MerchantAuthentication>,
    payout_id: &str,
) -> ApiResult<unversioned::Payout> {
    client
        .request(
            HttpMethod::<()>::Get,
            &client
                .environment
                .unversioned_uri(&format!("/payouts/{payout_id}")),
        )
        .await
}
