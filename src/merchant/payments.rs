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
    #[serde(rename_all = "snake_case")]
    pub enum PaymentState {
        Pending,
        AuthenticationChallenge,
        AuthenticationVerified,
        AuthorisationStarted,
        AuthorisationPassed,
        Authorised,
        CaptureStarted,
        Captured,
        RefundValidated,
        CancellationStarted,
        Declining,
        Completing,
        Cancelling,
        Failing,
        Completed,
        Declined,
        SoftDeclined,
        Cancelled,
        Failed,
    }

    #[derive(Debug, Deserialize, strum::Display, Serialize)]
    #[serde(tag = "type", rename_all = "snake_case")]
    pub enum PaymentMethod {
        ApplePay(Card),
        Card(Card),
        GooglePay(Card),
        RevolutPayCard(Card),
        RevolutPayAccount(RevolutPayAccount),
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Card {
        pub id: Option<String>,
        pub card_brand: Option<String>,
        pub funding: Option<String>,
        pub card_country_code: Option<String>,
        pub card_bin: Option<String>,
        pub card_last_four: Option<String>,
        pub card_expiry: Option<String>,
        pub cardholder_name: Option<String>,
        pub checks: Option<Checks>,
        pub fingerprint: Option<String>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Checks {
        pub three_ds: Option<ThreeDs>,
        pub cvv_verification: Option<String>,
        pub address: Option<String>,
        pub postcode: Option<String>,
        pub cardholder: Option<String>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct ThreeDs {
        pub eci: Option<String>,
        pub state: Option<ThreeDsState>,
        pub version: Option<String>,
    }

    #[derive(Debug, Deserialize, strum::Display, Serialize)]
    #[serde(rename_all = "snake_case")]
    pub enum ThreeDsState {
        Verified,
        Failed,
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

#[cfg(test)]
mod tests {
    use super::unversioned::*;

    impl Default for Payment {
        fn default() -> Self {
            Self {
                id: "some-payment-id".to_string(),
                state: Default::default(),
                decline_reason: None,
                bank_message: None,
                created_at: "some-date".to_string(),
                updated_at: "some-date".to_string(),
                token: None,
                amount: 4242,
                currency: None,
                settled_amount: None,
                payment_method: None,
                authentication_challenge: None,
                billing_address: None,
                risk_level: None,
                fees: None,
                order_id: None,
            }
        }
    }

    impl Default for PaymentState {
        fn default() -> Self {
            Self::Completed
        }
    }

    impl Default for PaymentMethod {
        fn default() -> PaymentMethod {
            PaymentMethod::Card(Default::default())
        }
    }

    impl Default for Card {
        fn default() -> Self {
            Card {
                id: None,
                card_brand: None,
                funding: None,
                card_country_code: None,
                card_bin: None,
                card_last_four: None,
                card_expiry: None,
                cardholder_name: None,
                checks: None,
                fingerprint: None,
            }
        }
    }

    impl Default for Checks {
        fn default() -> Self {
            Self {
                three_ds: None,
                cvv_verification: None,
                address: None,
                postcode: None,
                cardholder: None,
            }
        }
    }

    impl Default for ThreeDs {
        fn default() -> Self {
            Self {
                eci: None,
                state: None,
                version: None,
            }
        }
    }

    impl Default for ThreeDsState {
        fn default() -> Self {
            Self::Verified
        }
    }

    impl Default for RevolutPayAccount {
        fn default() -> Self {
            Self {
                id: "some-revolut-pay-account-id".to_string(),
                fingerprint: None,
            }
        }
    }

    impl Default for AuthenticationChallenge {
        fn default() -> Self {
            Self {
                r#type: "some-authentication-challenge-type".to_string(),
                acs_url: "some-acs-url".to_string(),
            }
        }
    }

    impl Default for BillingAddress {
        fn default() -> Self {
            Self {
                street_line_1: None,
                street_line_2: None,
                region: None,
                city: None,
                country_code: "ES".to_string(),
                postcode: "28830".to_string(),
            }
        }
    }

    impl Default for Fee {
        fn default() -> Self {
            Self {
                r#type: None,
                amount: None,
                currency: None,
            }
        }
    }
}
