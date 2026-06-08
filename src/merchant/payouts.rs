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
    merchant::ApiResult,
    merchant::client::MerchantAuthentication,
};

pub mod unversioned {
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Default)]
    pub struct ListParams {
        pub currency: Option<String>,
        pub limit: Option<u16>,
        pub from_created_date: Option<String>,
        pub to_created_date: Option<String>,
        pub state: Option<Vec<PayoutState>>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Payout {
        pub id: String,
        pub state: PayoutState,
        pub created_at: String,
        pub destination_type: PayoutDestinationType,
        pub amount: Option<u64>,
        pub currency: Option<String>,
    }

    #[derive(Clone, Debug, Deserialize, strum::Display, Serialize)]
    #[serde(rename_all = "snake_case")]
    #[strum(serialize_all = "snake_case")]
    pub enum PayoutState {
        Processing,
        Completed,
        Failed,
    }

    #[derive(Debug, Deserialize, strum::Display, Serialize)]
    #[serde(rename_all = "snake_case")]
    pub enum PayoutDestinationType {
        CurrentPocket,
        ExternalBeneficiary,
    }
}

impl std::fmt::Display for unversioned::ListParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let state = self.state.clone();

        let mut query = state
            .unwrap_or_default()
            .into_iter()
            .map(|state| ("state", Some(state.to_string())))
            .collect::<Vec<(&str, Option<String>)>>();

        let limit = self
            .limit
            .map(|limit| std::string::ToString::to_string(&limit));

        query.extend(vec![
            ("currency", self.currency.clone()),
            ("from_created_date", self.from_created_date.clone()),
            ("to_created_date", self.to_created_date.clone()),
            ("limit", limit.clone()),
        ]);

        let query = query.iter().fold(String::new(), |acc, (key, value)| {
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

pub async fn list<E: Environment>(
    client: &Client<E, MerchantAuthentication>,
    list_params: &unversioned::ListParams,
) -> ApiResult<Vec<unversioned::Payout>> {
    client
        .request(
            HttpMethod::<()>::Get,
            &client
                .environment
                .unversioned_uri(&format!("/payouts{list_params}")),
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

#[cfg(test)]
mod tests {
    use super::unversioned::*;

    impl Default for Payout {
        fn default() -> Self {
            Self {
                id: "some-payout-id".to_string(),
                state: Default::default(),
                created_at: "some-date".to_string(),
                destination_type: Default::default(),
                amount: None,
                currency: None,
            }
        }
    }

    impl Default for PayoutState {
        fn default() -> Self {
            Self::Completed
        }
    }

    impl Default for PayoutDestinationType {
        fn default() -> Self {
            Self::ExternalBeneficiary
        }
    }
}
