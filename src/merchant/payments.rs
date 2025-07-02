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

    #[derive(Debug, Deserialize, Serialize)]
    #[serde(rename_all = "snake_case")]
    pub enum PaymentState {
        #[serde(alias = "PENDING")]
        Pending,
        #[serde(alias = "AUTHENTICATION_CHALLENGE")]
        AuthenticationChallenge,
        #[serde(alias = "AUTHENTICATION_VERIFIED")]
        AuthenticationVerified,
        #[serde(alias = "AUTHENTICATION_STARTED")]
        AuthorisationStarted,
        #[serde(alias = "AUTHENTICATION_PASSED")]
        AuthorisationPassed,
        #[serde(alias = "AUTHORISED")]
        Authorised,
        #[serde(alias = "CAPTURE_STARTED")]
        CaptureStarted,
        #[serde(alias = "CAPTURED")]
        Captured,
        #[serde(alias = "REFUND_VALIDATED")]
        RefundValidated,
        #[serde(alias = "CANCELLATION_STARTED")]
        CancellationStarted,
        #[serde(alias = "DECLINING")]
        Declining,
        #[serde(alias = "COMPLETING")]
        Completing,
        #[serde(alias = "CANCELLING")]
        Cancelling,
        #[serde(alias = "FAILING")]
        Failing,
        #[serde(alias = "COMPLETED")]
        Completed,
        #[serde(alias = "DECLINED")]
        Declined,
        #[serde(alias = "SOFT_DECLINED")]
        SoftDeclined,
        #[serde(alias = "CANCELLED")]
        Cancelled,
        #[serde(alias = "FAILED")]
        Failed,
    }

    #[derive(Debug, Deserialize, Serialize)]
    #[serde(tag = "type", rename_all = "snake_case")]
    pub enum PaymentMethod {
        #[serde(alias = "APPLE_PAY")]
        ApplePay(Card),
        #[serde(alias = "CARD")]
        Card(Card),
        #[serde(alias = "GOOGLE_PAY")]
        GooglePay(Card),
        #[serde(alias = "REVOLUT_PAY_CARD")]
        RevolutPayCard(Card),
        #[serde(alias = "REVOLUT_PAY_ACCOUNT")]
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

    #[derive(Debug, Deserialize, Serialize)]
    #[serde(rename_all = "snake_case")]
    pub enum ThreeDsState {
        #[serde(alias = "VERIFIED")]
        Verified,
        #[serde(alias = "FAILED")]
        Failed,
        #[serde(alias = "CHALLENGE")]
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
