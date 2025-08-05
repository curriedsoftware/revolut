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
    pub struct Payment {
        pub id: String,
        pub state: PaymentState,
        pub decline_reason: Option<String>,
        pub bank_message: Option<String>,
        pub created_at: String,
        pub updated_at: String,
        pub token: Option<String>,
        pub amount: u64,
        pub currency: Option<String>,
        pub settled_amount: Option<u64>,
        pub payment_method: Option<PaymentMethod>,
        pub authentication_challenge: Option<AuthenticationChallenge>,
        pub billing_address: Option<BillingAddress>,
        pub risk_level: Option<String>,
        pub fees: Option<Vec<Fee>>,
        pub order_id: Option<String>,
    }

    #[derive(Debug, Deserialize, strum::Display, Serialize)]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    #[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
    pub enum PaymentState {
        #[serde(alias = "pending")]
        Pending,
        #[serde(alias = "authentication_challenge")]
        AuthenticationChallenge,
        #[serde(alias = "authentication_verified")]
        AuthenticationVerified,
        #[serde(alias = "authentication_started")]
        AuthorisationStarted,
        #[serde(alias = "authentication_passed")]
        AuthorisationPassed,
        #[serde(alias = "authorised")]
        Authorised,
        #[serde(alias = "capture_started")]
        CaptureStarted,
        #[serde(alias = "captured")]
        Captured,
        #[serde(alias = "refund_validated")]
        RefundValidated,
        #[serde(alias = "cancellation_started")]
        CancellationStarted,
        #[serde(alias = "declining")]
        Declining,
        #[serde(alias = "completing")]
        Completing,
        #[serde(alias = "cancelling")]
        Cancelling,
        #[serde(alias = "failing")]
        Failing,
        #[serde(alias = "completed")]
        Completed,
        #[serde(alias = "declined")]
        Declined,
        #[serde(alias = "soft_declined")]
        SoftDeclined,
        #[serde(alias = "cancelled")]
        Cancelled,
        #[serde(alias = "failed")]
        Failed,
    }

    #[derive(Debug, Deserialize, strum::Display, Serialize)]
    #[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
    #[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
    pub enum PaymentMethod {
        #[serde(alias = "apple_pay")]
        ApplePay(Card),
        #[serde(alias = "card")]
        Card(Card),
        #[serde(alias = "google_pay")]
        GooglePay(Card),
        #[serde(alias = "revolut_pay_card")]
        RevolutPayCard(Card),
        #[serde(alias = "revolut_pay_account")]
        RevolutPayAccount(RevolutPayAccount),
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Card {
        id: Option<String>,
        card_brand: Option<String>,
        funding: Option<String>,
        card_country_code: Option<String>,
        card_bin: Option<String>,
        card_last_four: Option<String>,
        card_expiry: Option<String>,
        cardholder_name: Option<String>,
        checks: Option<Checks>,
        fingerprint: Option<String>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Checks {
        three_ds: Option<ThreeDs>,
        cvv_verification: Option<String>,
        address: Option<String>,
        postcode: Option<String>,
        cardholder: Option<String>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct ThreeDs {
        pub eci: Option<String>,
        pub state: Option<ThreeDsState>,
        pub version: Option<String>,
    }

    #[derive(Debug, Deserialize, strum::Display, Serialize)]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    #[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
    pub enum ThreeDsState {
        #[serde(alias = "verified")]
        Verified,
        #[serde(alias = "failed")]
        Failed,
        #[serde(alias = "challenge")]
        Challenge,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct RevolutPayAccount {
        pub id: String,
        pub fingerprint: Option<String>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct AuthenticationChallenge {
        pub r#type: String,
        pub acs_url: String,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct BillingAddress {
        pub street_line_1: Option<String>,
        pub street_line_2: Option<String>,
        pub region: Option<String>,
        pub city: Option<String>,
        pub country_code: String,
        pub postcode: String,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Fee {
        pub r#type: Option<String>,
        pub amount: Option<u64>,
        pub currency: Option<String>,
    }
}

pub async fn retrieve<E: Environment>(
    client: &Client<E, MerchantAuthentication>,
    payment_id: &str,
) -> ApiResult<unversioned::Payment> {
    client
        .request(
            HttpMethod::<()>::Get,
            &client
                .environment
                .unversioned_uri(&format!("/payments/{payment_id}")),
        )
        .await
}
