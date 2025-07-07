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

    #[derive(Debug, Deserialize, strum::Display, Serialize)]
    #[serde(rename_all = "snake_case")]
    #[strum(serialize_all = "snake_case")]
    pub enum DisputeState {
        #[serde(alias = "NEEDS_RESPONSE")]
        NeedsResponse,
        #[serde(alias = "UNDER_REVIEW")]
        UnderReview,
        #[serde(alias = "WON")]
        Won,
        #[serde(alias = "LOST")]
        Lost,
    }

    #[derive(Debug, Deserialize, strum::Display, Serialize)]
    #[serde(rename_all = "snake_case")]
    #[strum(serialize_all = "snake_case")]
    pub enum DisputeSubstate {
        #[serde(alias = "ARBITRATION")]
        Arbitration,
        #[serde(alias = "LOST_ACCEPTED")]
        LostAccepted,
        #[serde(alias = "LOST_ARBITRATION")]
        LostArbitration,
        #[serde(alias = "LOST_EXPIRED")]
        LostExpired,
        #[serde(alias = "LOST_PRE_ARBITRATION")]
        LostPreArbitration,
        #[serde(alias = "NEW")]
        New,
        #[serde(alias = "PRE_ARBITRATION")]
        PreArbitration,
        #[serde(alias = "REPRESENTMENT")]
        Representment,
        #[serde(alias = "WON_ARBITRATION")]
        WonArbitration,
        #[serde(alias = "WON_PRE_ARBITRATION")]
        WonPreArbitration,
        #[serde(alias = "WON_REPRESENTMENT")]
        WonRepresentment,
        #[serde(alias = "WON_REVERSAL")]
        WonReversal,
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

    #[derive(Debug, Deserialize, Serialize)]
    pub struct PaymentMethod {
        pub r#type: Option<PaymentMethodType>,
        pub card_brand: Option<String>,
        pub card_last_four: Option<String>,
    }

    #[derive(Debug, Deserialize, strum::Display, Serialize)]
    #[serde(rename_all = "snake_case")]
    #[strum(serialize_all = "snake_case")]
    pub enum PaymentMethodType {
        #[serde(alias = "APPLE_PAY")]
        ApplePay,
        #[serde(alias = "APPLE_TAP_TO_PAY")]
        AppleTapToPay,
        #[serde(alias = "CARD")]
        Card,
        #[serde(alias = "GOOGLE_PAY")]
        GooglePay,
        #[serde(alias = "REVOLUT_PAY_ACCOUNT")]
        RevolutPayAccount,
        #[serde(alias = "REVOLUT_PAY_CARD")]
        RevolutPayCard,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Evidence {
        pub id: String,
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

pub async fn list(
    client: &Client<ProductionEnvironment<client::MerchantClient>, MerchantAuthentication>,
) -> ApiResult<unversioned::Dispute> {
    client
        .request(
            HttpMethod::<()>::Get,
            &client.environment.unversioned_uri("/disputes"),
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
    evidence: unversioned::EvidenceRequest<'_>,
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
