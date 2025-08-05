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

pub mod v10 {
    use {
        serde::{Deserialize, Serialize},
        std::collections::HashMap,
    };

    #[derive(Clone, Debug, Default)]
    pub struct ListParams {
        pub limit: Option<u16>,
        pub created_before: Option<String>,
        pub from_created_date: Option<String>,
        pub to_created_date: Option<String>,
        pub customer_id: Option<String>,
        pub email: Option<String>,
        pub merchant_order_ext_ref: Option<String>,
        pub state: Option<Vec<State>>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Customer {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub full_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub phone: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub email: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub date_of_birth: Option<String>,
    }

    #[derive(Debug, Deserialize, strum::Display, Serialize)]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    #[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
    pub enum LineItemType {
        #[serde(alias = "physical")]
        Physical,
        #[serde(alias = "service")]
        Service,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Quantity {
        pub value: f64,
        pub unit: Option<String>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct LineItem {
        pub name: String,
        pub r#type: LineItemType,
        pub quantity: Quantity,
        pub unit_price_amount: u64,
        pub total_amount: u64,
        pub external_id: Option<String>,
        pub discounts: Option<Vec<Discount>>,
        pub taxes: Option<Vec<Tax>>,
        pub image_urls: Option<Vec<String>>,
        pub description: Option<String>,
        pub url: Option<String>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Discount {
        pub name: String,
        pub amount: u64,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Tax {
        pub name: String,
        pub amount: u64,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Passenger {
        pub first_name: String,
        pub last_name: String,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct JourneyLeg {
        pub sequence: String,
        pub departure_airport_code: String,
        pub arrival_airport_code: String,
        pub flight_number: Option<String>,
        pub fare_base_code: Option<String>,
        pub travel_date: String,
        pub airline_name: String,
        pub airline_code: String,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Transaction {
        pub id: String,
        pub status: TransactionStatus,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recipient_wallet_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recipient_user_id: Option<String>,
    }

    #[derive(Debug, Deserialize, strum::Display, Serialize)]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    #[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
    pub enum TransactionStatus {
        #[serde(alias = "pending")]
        Pending,
        #[serde(alias = "failed")]
        Failed,
        #[serde(alias = "cancelled")]
        Cancelled,
        #[serde(alias = "completed")]
        Completed,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Address {
        pub street_line_1: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub street_line_2: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub region: Option<String>,
        pub city: String,
        pub country_code: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub country_subdivision_code: Option<String>,
        pub postcode: String,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Subseller {
        pub id: String,
        pub name: String,
        pub website: String,
        pub phone: String,
        pub address: Address,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Location {
        pub street_line_1: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub street_line_2: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub region: Option<String>,
        pub city: String,
        pub country_code: String,
        pub postcode: String,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Ticket {
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub transferable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub refundability: Option<String>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Event {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub start_date: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub end_date: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub supplier: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub supplier_payment_date: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub location: Option<Location>,
        pub category: String,
        pub market: String,
        pub tickets: Vec<Ticket>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Guest {
        pub first_name: String,
        pub last_name: String,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Contact {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub email: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub phone: Option<String>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Shipment {
        pub shipping_company_name: String,
        pub tracking_number: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub estimated_delivery_date: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tracking_url: Option<String>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Shipping {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub address: Option<Address>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub contact: Option<Contact>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub shipments: Option<Vec<Shipment>>,
    }

    #[derive(Debug, Deserialize, strum::Display, Serialize)]
    #[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
    #[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
    pub enum IndustryData {
        #[serde(alias = "airline")]
        Airline {
            booking_id: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            fulfillment_date: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            tickets_purchase: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            tickets_type: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            crs_code: Option<String>,
            ticket_change_indicator: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            refundability: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            passengers: Option<Vec<Passenger>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            journey_legs: Option<Vec<JourneyLeg>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            booking_url: Option<String>,
        },
        #[serde(alias = "crypto")]
        Crypto {
            transactions: Vec<Transaction>,
            #[serde(skip_serializing_if = "Option::is_none")]
            subseller_mcc: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            subseller_url: Option<String>,
        },
        #[serde(alias = "event")]
        Event {
            booking_id: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            events: Option<Vec<Event>>,
        },
        #[serde(alias = "lodging")]
        Lodging {
            booking_id: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            check_in_date: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            check_out_date: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            supplier_payment_date: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            category: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            booking_type: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            refundability: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            location: Option<Location>,
            #[serde(skip_serializing_if = "Option::is_none")]
            guests: Option<Vec<Guest>>,
        },
        #[serde(alias = "marketplace")]
        Marketplace { subseller: Subseller },
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct MerchantOrderData {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub url: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub reference: Option<String>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct UpcomingPaymentData {
        pub date: String,
        pub payment_method_id: String,
    }

    #[derive(Debug, Default, Deserialize, Serialize)]
    pub struct OrderRequest {
        pub amount: u64,
        pub currency: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub settlement_currency: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub customer: Option<Customer>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub enforce_challenge: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub line_items: Option<Vec<LineItem>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub shipping: Option<Vec<Shipping>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub capture_mode: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub cancel_authorised_after: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub location_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub metadata: Option<HashMap<String, String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub industry_data: Option<IndustryData>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub merchant_order_data: Option<MerchantOrderData>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub upcoming_payment_data: Option<UpcomingPaymentData>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub redirect_url: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub statement_descriptor_suffix: Option<String>,
    }

    #[derive(Clone, Debug, Deserialize, strum::Display, Serialize)]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    #[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
    pub enum SavedPaymentMethodType {
        #[serde(alias = "card")]
        Card,
        #[serde(alias = "revolut_pay")]
        RevolutPay,
    }

    #[derive(Clone, Debug, Deserialize, strum::Display, Serialize)]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    #[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
    pub enum SavedPaymentMethodInitiator {
        #[serde(alias = "customer")]
        Customer,
        #[serde(alias = "merchant")]
        Merchant,
    }

    #[derive(Debug, Deserialize, strum::Display, Serialize)]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    #[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
    pub enum SavedPaymentMethodEnvironment {
        #[serde(alias = "browser")]
        Browser,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SavedPaymentMethod {
        r#type: SavedPaymentMethodType,
        id: String,
        initiator: SavedPaymentMethodInitiator,
        environment: String,
    }

    #[derive(Debug, Deserialize, strum::Display, Serialize)]
    #[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
    pub enum Type {
        #[serde(alias = "payment")]
        Payment,
        #[serde(alias = "payment_request")]
        PaymentRequest,
        #[serde(alias = "refund")]
        Refund,
        #[serde(alias = "chargeback")]
        Chargeback,
        #[serde(alias = "chargeback_reversal")]
        ChargebackReversal,
        #[serde(alias = "credit_reimbursement")]
        CreditReimbursement,
    }

    #[derive(Clone, Debug, Deserialize, strum::Display, Serialize)]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    #[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
    pub enum State {
        #[serde(alias = "pending")]
        Pending,
        #[serde(alias = "processing")]
        Processing,
        #[serde(alias = "authorised")]
        Authorised,
        #[serde(alias = "completed")]
        Completed,
        #[serde(alias = "cancelled")]
        Cancelled,
        #[serde(alias = "failed")]
        Failed,
    }

    #[derive(Debug, Deserialize, strum::Display, Serialize)]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    #[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
    pub enum PaymentType {
        #[serde(alias = "apple_pay")]
        ApplePay,
        #[serde(alias = "card")]
        Card,
        #[serde(alias = "google_pay")]
        GooglePay,
        #[serde(alias = "revolut_pay_card")]
        RevolutPayCard,
        #[serde(alias = "revolut_pay_account")]
        RevolutPayAccount,
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
    pub struct Checks {
        pub three_ds: Option<ThreeDs>,
        pub cvv_verification: Option<String>,
        pub address: Option<String>,
        pub postcode: Option<String>,
        pub cardholder: Option<String>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct PaymentMethod {
        pub id: Option<String>,
        pub r#type: PaymentType,
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
    pub struct AuthenticationChallenge {
        pub r#type: String,
        pub acs_url: String,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Fee {
        pub r#type: Option<String>,
        pub amount: Option<u64>,
        pub currency: Option<String>,
    }

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
        pub settled_amount: Option<u32>,
        pub settled_currency: Option<String>,
        pub payment_method: Option<PaymentMethod>,
        pub authentication_challenge: Option<AuthenticationChallenge>,
        pub billing_address: Option<Address>,
        pub risk_level: Option<String>,
        pub fees: Option<Vec<Fee>>,
    }

    #[derive(Debug, Deserialize, strum::Display, Serialize)]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    #[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
    pub enum PaymentState {
        #[serde(alias = "pending")]
        Pending,
        #[serde(alias = "processing")]
        Processing,
        #[serde(alias = "authorised")]
        Authorised,
        #[serde(alias = "completed")]
        Completed,
        #[serde(alias = "cancelled")]
        Cancelled,
        #[serde(alias = "failed")]
        Failed,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Order {
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub token: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub r#type: Option<Type>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub state: Option<State>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub created_at: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub updated_at: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub capture_mode: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub cancel_authorised_after: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub amount: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub outstanding_amount: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub refunded_amount: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub currency: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub settlement_currency: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub customer: Option<Customer>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub payments: Option<Payment>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub location_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub metadata: Option<HashMap<String, String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub industry_data: Option<IndustryData>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub merchant_order_data: Option<MerchantOrderData>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub upcoming_payment_data: Option<UpcomingPaymentData>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub checkout_url: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub redirect_url: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub shipping: Option<Shipping>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub enforce_challenge: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub line_items: Option<Vec<LineItem>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub statement_descriptor_suffix: Option<String>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct OrderAuthenticationChallengeThreeDs {
        pub r#type: String,
        pub acs_url: String,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct OrderAuthenticationChallengeThreeDsFingerprint {
        pub r#type: String,
        pub fingerprint_url: String,
        pub fingerprint_data: String,
    }

    #[derive(Debug, Deserialize, strum::Display, Serialize)]
    #[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
    pub enum OrderAuthenticationChallenge {
        #[serde(alias = "three_ds")]
        ThreeDs(OrderAuthenticationChallengeThreeDs),
        #[serde(alias = "three_ds_fingerprint")]
        ThreeDsFingerprint(OrderAuthenticationChallengeThreeDsFingerprint),
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct OrderPayment {
        pub id: String,
        pub order_id: String,
        pub payment_method: OrderPaymentMethod,
        pub state: Option<OrderPaymentState>,
        pub authentication_challenge: Option<OrderAuthenticationChallenge>,
    }

    #[derive(Debug, Deserialize, strum::Display, Serialize)]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    #[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
    pub enum OrderPaymentState {
        #[serde(alias = "pending")]
        Pending,
        #[serde(alias = "authentication_challenge")]
        AuthenticationChallenge,
        #[serde(alias = "authentication_verified")]
        AuthenticationVerified,
        #[serde(alias = "authorisation_started")]
        AuthorisationStarted,
        #[serde(alias = "authorisation_passed")]
        AuthorisationPassed,
        #[serde(alias = "authorised")]
        Authorised,
        #[serde(alias = "capture_started")]
        CaptureStarted,
        #[serde(alias = "captured")]
        Captured,
        #[serde(alias = "refund_validated")]
        RefundValidated,
        #[serde(alias = "refund_started")]
        RefundStarted,
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
    #[serde(tag = "type")]
    #[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
    pub enum OrderPaymentMethod {
        RevolutPay(OrderPaymentMethodRevolutPay),
        Card(OrderPaymentMethodCard),
    }

    #[derive(Debug, Deserialize, strum::Display, Serialize)]
    #[serde(tag = "subtype")]
    #[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
    pub enum OrderPaymentMethodRevolutPay {
        Account(OrderPaymentMethodRevolutPayAccount),
        Card(OrderPaymentMethodRevolutPayCard),
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct OrderPaymentMethodCard {
        pub id: Option<String>,
        pub brand: Option<String>,
        pub last_four: Option<String>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct OrderPaymentMethodRevolutPayAccount {
        pub id: Option<String>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct OrderPaymentMethodRevolutPayCard {
        pub id: Option<String>,
        pub brand: Option<String>,
        pub last_four: Option<String>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct RefundRequest {
        pub amount: u64,
        pub currency: String,
    }
}

impl std::fmt::Display for v10::ListParams {
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
            ("created_before", self.created_before.clone()),
            ("from_created_date", self.from_created_date.clone()),
            ("to_created_date", self.to_created_date.clone()),
            ("customer_id", self.customer_id.clone()),
            ("email", self.email.clone()),
            (
                "merchant_order_ext_ref",
                self.merchant_order_ext_ref.clone(),
            ),
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

pub async fn create<E: Environment>(
    client: &Client<E, MerchantAuthentication>,
    order: &v10::OrderRequest,
) -> ApiResult<v10::Order> {
    client
        .request(
            HttpMethod::Post {
                body: Some(Body::Json(&order)),
            },
            &client.environment.unversioned_uri("/orders"),
        )
        .await
}

pub async fn retrieve<E: Environment>(
    client: &Client<E, MerchantAuthentication>,
    order_id: &str,
) -> ApiResult<v10::Order> {
    client
        .request(
            HttpMethod::<()>::Get,
            &client
                .environment
                .unversioned_uri(&format!("/orders/{order_id}")),
        )
        .await
}

pub async fn update<E: Environment>(
    client: &Client<E, MerchantAuthentication>,
    order_id: &str,
    order: &v10::OrderRequest,
) -> ApiResult<v10::Order> {
    client
        .request(
            HttpMethod::Patch {
                body: Some(Body::Json(&order)),
            },
            &client
                .environment
                .unversioned_uri(&format!("/orders/{order_id}")),
        )
        .await
}

pub async fn list<E: Environment>(
    client: &Client<E, MerchantAuthentication>,
    list_params: &v10::ListParams,
) -> ApiResult<Vec<v10::Order>> {
    client
        .request(
            HttpMethod::<()>::Get,
            &client
                .environment
                .uri("1.0", &format!("/orders{list_params}")),
        )
        .await
}

pub async fn capture<E: Environment>(
    client: &Client<E, MerchantAuthentication>,
    order_id: &str,
    amount: u64,
) -> ApiResult<v10::Order> {
    #[derive(Clone, Debug, serde::Serialize)]
    struct Amount {
        amount: u64,
    }

    client
        .request(
            HttpMethod::Post {
                body: Some(Body::Json(&Amount { amount })),
            },
            &client
                .environment
                .unversioned_uri(&format!("/orders/{order_id}/capture")),
        )
        .await
}

pub async fn cancel<E: Environment>(
    client: &Client<E, MerchantAuthentication>,
    order_id: &str,
) -> ApiResult<v10::Order> {
    client
        .request(
            HttpMethod::Post::<()> { body: None },
            &client
                .environment
                .unversioned_uri(&format!("/orders/{order_id}/cancel")),
        )
        .await
}

pub async fn refund<E: Environment>(
    client: &Client<E, MerchantAuthentication>,
    order_id: &str,
    refund: &v10::RefundRequest,
) -> ApiResult<v10::Order> {
    client
        .request(
            HttpMethod::Post {
                body: Some(Body::Json(refund)),
            },
            &client
                .environment
                .unversioned_uri(&format!("/orders/{order_id}/refund")),
        )
        .await
}

pub async fn pay<E: Environment>(
    client: &Client<E, MerchantAuthentication>,
    order_id: &str,
    saved_payment_method: &v10::SavedPaymentMethod,
) -> ApiResult<v10::OrderPayment> {
    #[derive(Clone, Debug, serde::Serialize)]
    struct SavedPaymentMethod<'a> {
        saved_payment_method: &'a v10::SavedPaymentMethod,
    }

    client
        .request(
            HttpMethod::Post {
                body: Some(Body::Json(&SavedPaymentMethod {
                    saved_payment_method,
                })),
            },
            &client
                .environment
                .unversioned_uri(&format!("/orders/{order_id}/payments")),
        )
        .await
}

pub async fn payment_list<E: Environment>(
    client: &Client<E, MerchantAuthentication>,
    order_id: &str,
) -> ApiResult<Vec<v10::OrderPayment>> {
    client
        .request(
            HttpMethod::<()>::Get,
            &client
                .environment
                .unversioned_uri(&format!("/orders/{order_id}/payments")),
        )
        .await
}
