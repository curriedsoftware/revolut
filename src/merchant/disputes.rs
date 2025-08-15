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

//! [Disputes API](https://developer.revolut.com/docs/merchant/disputes).
//!
//! [^note]: This feature is **not** available in the sandbox
//! environment. Trying to use such a feature using a sandbox client
//! will result in an error at compile time.

use crate::{
    client::{Body, Client, Environment, HttpMethod, Part, ProductionEnvironment},
    errors::ApiResult,
    merchant::client::{self, MerchantAuthentication},
};

pub mod unversioned {
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Default, Debug)]
    pub struct ListParams {
        pub limit: Option<u16>,
        pub from_created_date: Option<String>,
        pub to_created_date: Option<String>,
        pub state: Option<Vec<DisputeState>>,
        pub payment_id: Option<Vec<String>>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Dispute {
        pub id: Option<String>,
        pub state: Option<DisputeState>,
        pub substate: Option<DisputeSubstate>,
        pub created_at: Option<String>,
        pub updated_at: Option<String>,
        pub response_due_date: Option<String>,
        pub reason_code: Option<String>,
        pub reason_description: Option<String>,
        pub amount: Option<u64>,
        pub currency: Option<String>,
        pub payment: Option<Payment>,
    }

    #[cfg(test)]
    impl Default for Dispute {
        fn default() -> Self {
            Self {
                id: None,
                state: None,
                substate: None,
                created_at: None,
                updated_at: None,
                response_due_date: None,
                reason_code: None,
                reason_description: None,
                amount: None,
                currency: None,
                payment: None,
            }
        }
    }

    #[derive(Clone, Debug, Deserialize, strum::Display, Serialize)]
    #[serde(rename_all = "snake_case")]
    #[strum(serialize_all = "snake_case")]
    pub enum DisputeState {
        NeedsResponse,
        UnderReview,
        Won,
        Lost,
    }

    #[cfg(test)]
    impl Default for DisputeState {
        fn default() -> Self {
            Self::Won
        }
    }

    #[derive(Debug, Deserialize, strum::Display, Serialize)]
    #[serde(rename_all = "snake_case")]
    pub enum DisputeSubstate {
        Arbitration,
        LostAccepted,
        LostArbitration,
        LostExpired,
        LostPreArbitration,
        New,
        PreArbitration,
        Representment,
        WonArbitration,
        WonPreArbitration,
        WonRepresentment,
        WonReversal,
    }

    #[cfg(test)]
    impl Default for DisputeSubstate {
        fn default() -> Self {
            Self::New
        }
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Payment {
        pub id: Option<String>,
        pub order_id: Option<String>,
        pub created_at: Option<String>,
        pub arn: Option<String>,
        pub amount: Option<u64>,
        pub currency: Option<String>,
        pub payment_method: Option<PaymentMethod>,
    }

    #[cfg(test)]
    impl Default for Payment {
        fn default() -> Self {
            Self {
                id: None,
                order_id: None,
                created_at: None,
                arn: None,
                amount: None,
                currency: None,
                payment_method: None,
            }
        }
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct PaymentMethod {
        pub r#type: Option<PaymentMethodType>,
        pub card_brand: Option<String>,
        pub card_last_four: Option<String>,
    }

    #[cfg(test)]
    impl Default for PaymentMethod {
        fn default() -> Self {
            Self {
                r#type: None,
                card_brand: None,
                card_last_four: None,
            }
        }
    }

    #[derive(Debug, Deserialize, strum::Display, Serialize)]
    #[serde(rename_all = "snake_case")]
    pub enum PaymentMethodType {
        ApplePay,
        AppleTapToPay,
        Card,
        GooglePay,
        RevolutPayAccount,
        RevolutPayCard,
    }

    #[cfg(test)]
    impl Default for PaymentMethodType {
        fn default() -> Self {
            Self::Card
        }
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Evidence {
        pub id: String,
    }

    #[cfg(test)]
    impl Default for Evidence {
        fn default() -> Self {
            Self {
                id: "some-evidence-id".to_string(),
            }
        }
    }

    pub struct EvidenceRequest<'a> {
        pub file_name: String,
        pub data: EvidenceType<'a>,
    }

    pub enum EvidenceType<'a> {
        PDF(&'a [u8]),
        PNG(&'a [u8]),
        JPEG(&'a [u8]),
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct ChallengeDisputeRequest {
        pub reason: String,
        pub comment: Option<String>,
        pub evidences: Vec<String>,
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
            ("from_created_date", self.from_created_date.clone()),
            ("to_created_date", self.to_created_date.clone()),
            ("limit", limit.clone()),
        ]);

        if let Some(payment_ids) = &self.payment_id {
            for payment_id in payment_ids {
                query.push(("payment_id", Some(payment_id.clone())));
            }
        }

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

pub async fn list(
    client: &Client<ProductionEnvironment<client::MerchantClient>, MerchantAuthentication>,
    list_params: &unversioned::ListParams,
) -> ApiResult<Vec<unversioned::Dispute>> {
    client
        .request(
            HttpMethod::<()>::Get,
            &client
                .environment
                .unversioned_uri(&format!("/disputes{list_params}")),
        )
        .await
}

pub async fn retrieve(
    client: &Client<ProductionEnvironment<client::MerchantClient>, MerchantAuthentication>,
    dispute_id: &str,
) -> ApiResult<unversioned::Dispute> {
    client
        .request(
            HttpMethod::<()>::Get,
            &client
                .environment
                .unversioned_uri(&format!("/disputes/{dispute_id}")),
        )
        .await
}

pub async fn accept(
    client: &Client<ProductionEnvironment<client::MerchantClient>, MerchantAuthentication>,
    dispute_id: &str,
) -> ApiResult<()> {
    client
        .request(
            HttpMethod::Post::<()> { body: None },
            &client
                .environment
                .unversioned_uri(&format!("/disputes/{dispute_id}/accept")),
        )
        .await
}

pub async fn upload_evidence(
    client: &Client<ProductionEnvironment<client::MerchantClient>, MerchantAuthentication>,
    dispute_id: &str,
    evidence: &unversioned::EvidenceRequest<'_>,
) -> ApiResult<unversioned::Evidence> {
    let (evidence_data, evidence_content_type) = match evidence.data {
        unversioned::EvidenceType::JPEG(evidence) => (evidence, "image/jpeg"),
        unversioned::EvidenceType::PDF(evidence) => (evidence, "application/pdf"),
        unversioned::EvidenceType::PNG(evidence) => (evidence, "image/png"),
    };

    client
        .request(
            HttpMethod::Post::<()> {
                body: Some(Body::Multipart(&vec![Part {
                    contents: evidence_data,
                    mime_str: evidence_content_type,
                    file_name: &evidence.file_name,
                }])),
            },
            &client
                .environment
                .unversioned_uri(&format!("/disputes/{dispute_id}/evidences")),
        )
        .await
}

pub async fn challenge(
    client: &Client<ProductionEnvironment<client::MerchantClient>, MerchantAuthentication>,
    dispute_id: &str,
    challenge_dispute: &unversioned::ChallengeDisputeRequest,
) -> ApiResult<()> {
    client
        .request(
            HttpMethod::Post {
                body: Some(Body::Json(&challenge_dispute)),
            },
            &client
                .environment
                .unversioned_uri(&format!("/disputes/{dispute_id}/challenge")),
        )
        .await
}
