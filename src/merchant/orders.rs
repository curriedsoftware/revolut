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
        pub state: Option<Vec<OrderStateListItem>>,
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
    #[serde(rename_all = "snake_case")]
    pub enum LineItemType {
        Physical,
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
    #[serde(rename_all = "snake_case")]
    pub enum TransactionStatus {
        Pending,
        Failed,
        Cancelled,
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
    #[serde(rename_all = "snake_case", tag = "type")]
    pub enum IndustryData {
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
        Crypto {
            transactions: Vec<Transaction>,
            #[serde(skip_serializing_if = "Option::is_none")]
            subseller_mcc: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            subseller_url: Option<String>,
        },
        Event {
            booking_id: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            events: Option<Vec<Event>>,
        },
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
        Marketplace {
            subseller: Subseller,
        },
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

    #[derive(Debug, Deserialize, Serialize)]
    pub struct MerchantOrderData {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub url: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub reference: Option<String>,
    }

    #[derive(Clone, Debug, Deserialize, strum::Display, Serialize)]
    #[serde(rename_all = "snake_case")]
    pub enum SavedPaymentMethodType {
        Card,
        RevolutPay,
    }

    #[derive(Clone, Debug, Deserialize, strum::Display, Serialize)]
    #[serde(rename_all = "snake_case")]
    pub enum SavedPaymentMethodInitiator {
        Customer,
        Merchant,
    }

    #[derive(Debug, Deserialize, strum::Display, Serialize)]
    #[serde(rename_all = "snake_case")]
    pub enum SavedPaymentMethodEnvironment {
        Browser,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SavedPaymentMethodReq {
        r#type: SavedPaymentMethodType,
        id: String,
        initiator: SavedPaymentMethodInitiator,
        // Revolut rejects this for merchant-initiated (off-session) payments —
        // "Value for field 'environment' is not in correct format" — it must be
        // omitted there, and is only expected for customer-initiated payments.
        // Optional so callers can leave it out.
        #[serde(skip_serializing_if = "Option::is_none")]
        environment: Option<String>,
    }

    impl SavedPaymentMethodReq {
        /// Build a reference to a payment method already saved for a customer, so
        /// an order can be charged against it. Pass `environment = None` for
        /// merchant-initiated charges (the only kind that omits it).
        pub fn new(
            r#type: SavedPaymentMethodType,
            id: String,
            initiator: SavedPaymentMethodInitiator,
            environment: Option<String>,
        ) -> Self {
            Self {
                r#type,
                id,
                initiator,
                environment,
            }
        }
    }

    // snake_case
    #[derive(Debug, Deserialize, strum::Display, Serialize)]
    #[serde(rename_all = "snake_case")]
    pub enum OrderType {
        Payment,
        PaymentRequest,
        Refund,
        Chargeback,
        ChargebackReversal,
        CreditReimbursement,
    }

    // SCREAMING_SNAKE_CASE
    #[derive(Debug, Deserialize, strum::Display, Serialize)]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    pub enum OrderTypeListItem {
        Payment,
        PaymentRequest,
        Refund,
        Chargeback,
        ChargebackReversal,
        CreditReimbursement,
    }

    // snake_case
    #[derive(Clone, Debug, Deserialize, strum::Display, Serialize)]
    #[serde(rename_all = "snake_case")]
    #[strum(serialize_all = "snake_case")]
    pub enum State {
        Pending,
        Processing,
        Authorised,
        Completed,
        Cancelled,
        Failed,
    }

    // SCREAMING_SNAKE_CASE
    #[derive(Clone, Debug, Deserialize, strum::Display, strum::EnumString, Serialize)]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    #[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
    pub enum OrderStateListItem {
        Pending,
        Processing,
        Authorised,
        Completed,
        Cancelled,
        Failed,
    }

    #[derive(Debug, Deserialize, strum::Display, Serialize)]
    #[serde(rename_all = "snake_case")]
    pub enum PaymentType {
        ApplePay,
        Card,
        GooglePay,
        RevolutPayCard,
        RevolutPayAccount,
    }

    /// The 3-D Secure `version` is returned inconsistently by Revolut: as a bare
    /// number (e.g. `2`) for retrieved orders, and as a string (e.g. `"2"`)
    /// elsewhere. Accept either and normalise to a string so deserialization of
    /// the surrounding order does not fail.
    fn deserialize_opt_string_or_number<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Option::<serde_json::Value>::deserialize(deserializer)?;
        Ok(value.and_then(|v| match v {
            serde_json::Value::Null => None,
            serde_json::Value::String(s) => Some(s),
            serde_json::Value::Number(n) => Some(n.to_string()),
            other => Some(other.to_string()),
        }))
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct ThreeDs {
        pub eci: Option<String>,
        pub state: Option<ThreeDsState>,
        #[serde(default, deserialize_with = "deserialize_opt_string_or_number")]
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
    pub struct Checks {
        pub three_ds: Option<ThreeDs>,
        pub cvv_verification: Option<String>,
        pub address: Option<String>,
        pub postcode: Option<String>,
        pub cardholder: Option<String>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    #[serde(rename_all = "snake_case", tag = "type")]
    pub enum PaymentMethod {
        ApplePay(PaymentMethodApplePay),
        Card(PaymentMethodCard),
        GooglePay(PaymentMethodGooglePay),
        RevolutPayCard(PaymentMethodRevolutPayCard),
        RevolutPayAccount(PaymentMethodRevolutPayAccount),
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct PaymentMethodApplePay {
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
    pub struct PaymentMethodCard {
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
    pub struct PaymentMethodGooglePay {
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
    pub struct PaymentMethodRevolutPayCard {
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
    pub struct PaymentMethodRevolutPayAccount {
        pub id: Option<String>,
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
        RefundStarted,
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

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Order {
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub token: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub r#type: Option<OrderType>,
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
        pub payments: Option<Vec<Payment>>,
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
    pub struct OrderListItem {
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub token: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub r#type: Option<OrderTypeListItem>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub state: Option<OrderStateListItem>,
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
        pub payments: Option<Vec<Payment>>,
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
    #[serde(rename_all = "snake_case")]
    pub enum OrderAuthenticationChallenge {
        ThreeDs(OrderAuthenticationChallengeThreeDs),
        ThreeDsFingerprint(OrderAuthenticationChallengeThreeDsFingerprint),
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct OrderPayment {
        pub id: String,
        pub order_id: String,
        pub payment_method: Option<OrderPaymentMethod>,
        pub state: Option<OrderPaymentState>,
        pub authentication_challenge: Option<OrderAuthenticationChallenge>,
    }

    #[derive(Debug, Deserialize, strum::Display, Serialize)]
    #[serde(rename_all = "snake_case")]
    pub enum OrderPaymentState {
        Pending,
        AuthenticationChallenge,
        AuthenticationVerified,
        AuthorisationStarted,
        AuthorisationPassed,
        Authorised,
        CaptureStarted,
        Captured,
        RefundValidated,
        RefundStarted,
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
    #[serde(rename_all = "snake_case", tag = "type")]
    pub enum OrderPaymentMethod {
        RevolutPay(OrderPaymentMethodRevolutPay),
        Card(OrderPaymentMethodCard),
    }

    #[derive(Debug, Deserialize, strum::Display, Serialize)]
    #[serde(rename_all = "snake_case", tag = "subtype")]
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
) -> ApiResult<Vec<v10::OrderListItem>> {
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
    #[derive(Clone, Debug, Default, serde::Serialize)]
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
    saved_payment_method: &v10::SavedPaymentMethodReq,
) -> ApiResult<v10::OrderPayment> {
    #[derive(Clone, Debug, serde::Serialize)]
    struct SavedPaymentMethod<'a> {
        saved_payment_method: &'a v10::SavedPaymentMethodReq,
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

#[cfg(test)]
mod tests {
    use super::{v10::*, *};
    use crate::merchant::client::{MerchantAuthenticationBuilder, merchant_client};

    impl Default for Customer {
        fn default() -> Self {
            Self {
                id: None,
                full_name: None,
                phone: None,
                email: None,
                date_of_birth: None,
            }
        }
    }

    impl Default for LineItemType {
        fn default() -> Self {
            Self::Physical
        }
    }

    impl Default for Quantity {
        fn default() -> Self {
            Self {
                value: 1.0,
                unit: None,
            }
        }
    }

    impl Default for LineItem {
        fn default() -> Self {
            Self {
                name: "some-line-item".to_string(),
                r#type: Default::default(),
                quantity: Default::default(),
                unit_price_amount: 4242,
                total_amount: 4242,
                external_id: None,
                discounts: None,
                taxes: None,
                image_urls: None,
                description: None,
                url: None,
            }
        }
    }

    impl Default for Discount {
        fn default() -> Self {
            Self {
                name: "some-discount".to_string(),
                amount: 2121,
            }
        }
    }

    impl Default for Tax {
        fn default() -> Self {
            Self {
                name: "some-tax".to_string(),
                amount: 2121,
            }
        }
    }

    impl Default for Passenger {
        fn default() -> Self {
            Self {
                first_name: "some-passenger-first-name".to_string(),
                last_name: "some-passenger-last-name".to_string(),
            }
        }
    }

    impl Default for JourneyLeg {
        fn default() -> Self {
            Self {
                sequence: "some-journey".to_string(),
                departure_airport_code: "MAD".to_string(),
                arrival_airport_code: "JFK".to_string(),
                flight_number: None,
                fare_base_code: None,
                travel_date: "some-travel-date".to_string(),
                airline_name: "some-airline-name".to_string(),
                airline_code: "some-airline-code".to_string(),
            }
        }
    }

    impl Default for Transaction {
        fn default() -> Self {
            Self {
                id: "some-transaction".to_string(),
                status: Default::default(),
                recipient_wallet_id: None,
                recipient_user_id: None,
            }
        }
    }

    impl Default for TransactionStatus {
        fn default() -> Self {
            Self::Completed
        }
    }

    impl Default for Address {
        fn default() -> Self {
            Self {
                street_line_1: "some-street-line".to_string(),
                street_line_2: None,
                region: None,
                city: "some-city".to_string(),
                country_code: "ES".to_string(),
                country_subdivision_code: None,
                postcode: "28830".to_string(),
            }
        }
    }

    impl Default for Subseller {
        fn default() -> Self {
            Self {
                id: "some-subseller-id".to_string(),
                name: "some-subseller-name".to_string(),
                website: "some-subseller-website".to_string(),
                phone: "some-subseller-phone".to_string(),
                address: Default::default(),
            }
        }
    }

    impl Default for Location {
        fn default() -> Self {
            Self {
                street_line_1: "some-street-line".to_string(),
                street_line_2: None,
                region: None,
                city: "some-city".to_string(),
                country_code: "ES".to_string(),
                postcode: "28830".to_string(),
            }
        }
    }

    impl Default for Ticket {
        fn default() -> Self {
            Self {
                id: "some-ticket-id".to_string(),
                transferable: None,
                refundability: None,
            }
        }
    }

    impl Default for Event {
        fn default() -> Self {
            Self {
                start_date: None,
                end_date: None,
                supplier: None,
                supplier_payment_date: None,
                name: None,
                location: None,
                category: "some-category".to_string(),
                market: "some-market".to_string(),
                tickets: Vec::new(),
            }
        }
    }

    impl Default for Guest {
        fn default() -> Self {
            Self {
                first_name: "some-guest-first-name".to_string(),
                last_name: "some-guest-last-name".to_string(),
            }
        }
    }

    impl Default for Contact {
        fn default() -> Self {
            Self {
                name: None,
                email: None,
                phone: None,
            }
        }
    }

    impl Default for Shipment {
        fn default() -> Self {
            Self {
                shipping_company_name: "some-shipping-company-name".to_string(),
                tracking_number: "some-tracking-number".to_string(),
                estimated_delivery_date: None,
                tracking_url: None,
            }
        }
    }

    impl Default for Shipping {
        fn default() -> Self {
            Self {
                address: None,
                contact: None,
                shipments: None,
            }
        }
    }

    impl Default for IndustryData {
        fn default() -> Self {
            Self::Event {
                booking_id: Default::default(),
                events: Default::default(),
            }
        }
    }

    impl Default for UpcomingPaymentData {
        fn default() -> Self {
            Self {
                date: "some-date".to_string(),
                payment_method_id: "some-payment-method-id".to_string(),
            }
        }
    }

    impl Default for SavedPaymentMethodEnvironment {
        fn default() -> Self {
            Self::Browser
        }
    }

    impl Default for OrderType {
        fn default() -> Self {
            Self::Payment
        }
    }

    impl Default for OrderTypeListItem {
        fn default() -> Self {
            Self::Payment
        }
    }

    impl Default for State {
        fn default() -> Self {
            Self::Completed
        }
    }

    impl Default for OrderStateListItem {
        fn default() -> Self {
            Self::Completed
        }
    }

    impl Default for PaymentType {
        fn default() -> Self {
            Self::Card
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

    impl Default for PaymentMethod {
        fn default() -> Self {
            Self::Card(Default::default())
        }
    }

    impl Default for PaymentMethodApplePay {
        fn default() -> Self {
            Self {
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

    impl Default for PaymentMethodCard {
        fn default() -> Self {
            Self {
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

    impl Default for PaymentMethodRevolutPayCard {
        fn default() -> Self {
            Self {
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

    impl Default for PaymentMethodRevolutPayAccount {
        fn default() -> Self {
            Self {
                id: None,
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

    impl Default for Fee {
        fn default() -> Self {
            Self {
                r#type: None,
                amount: None,
                currency: None,
            }
        }
    }

    impl Default for Payment {
        fn default() -> Self {
            Self {
                id: "some-payment-id".to_string(),
                state: Default::default(),
                decline_reason: None,
                bank_message: None,
                created_at: "some-created-at".to_string(),
                updated_at: "some-updated-at".to_string(),
                token: None,
                amount: 4242,
                currency: None,
                settled_amount: None,
                settled_currency: None,
                payment_method: None,
                authentication_challenge: None,
                billing_address: None,
                risk_level: None,
                fees: None,
            }
        }
    }

    impl Default for PaymentState {
        fn default() -> Self {
            Self::Completed
        }
    }

    impl Default for Order {
        fn default() -> Self {
            Self {
                id: "some-order-id".to_string(),
                token: None,
                r#type: None,
                state: None,
                created_at: None,
                updated_at: None,
                description: None,
                capture_mode: None,
                cancel_authorised_after: None,
                amount: None,
                outstanding_amount: None,
                refunded_amount: None,
                currency: None,
                settlement_currency: None,
                customer: None,
                payments: None,
                location_id: None,
                metadata: None,
                industry_data: None,
                merchant_order_data: None,
                upcoming_payment_data: None,
                checkout_url: None,
                redirect_url: None,
                shipping: None,
                enforce_challenge: None,
                line_items: None,
                statement_descriptor_suffix: None,
            }
        }
    }

    impl Default for OrderListItem {
        fn default() -> Self {
            Self {
                id: "some-order-list-item".to_string(),
                token: None,
                r#type: None,
                state: None,
                created_at: None,
                updated_at: None,
                description: None,
                capture_mode: None,
                cancel_authorised_after: None,
                amount: None,
                outstanding_amount: None,
                refunded_amount: None,
                currency: None,
                settlement_currency: None,
                customer: None,
                payments: None,
                location_id: None,
                metadata: None,
                industry_data: None,
                merchant_order_data: None,
                upcoming_payment_data: None,
                checkout_url: None,
                redirect_url: None,
                shipping: None,
                enforce_challenge: None,
                line_items: None,
                statement_descriptor_suffix: None,
            }
        }
    }

    impl Default for OrderAuthenticationChallengeThreeDs {
        fn default() -> Self {
            Self {
                r#type: "some-order-authentication-challenge-three-ds".to_string(),
                acs_url: "some-acs-url".to_string(),
            }
        }
    }

    impl Default for OrderAuthenticationChallengeThreeDsFingerprint {
        fn default() -> Self {
            Self {
                r#type: "some-order-authentication-challenge-three-ds-fingerprint".to_string(),
                fingerprint_url: "some-fingerprint-url".to_string(),
                fingerprint_data: "some-fingerprint-data".to_string(),
            }
        }
    }

    impl Default for OrderAuthenticationChallenge {
        fn default() -> Self {
            Self::ThreeDs(Default::default())
        }
    }

    impl Default for OrderPayment {
        fn default() -> Self {
            Self {
                id: "some-order-payment-id".to_string(),
                order_id: "some-order-payment-order-id".to_string(),
                payment_method: None,
                state: None,
                authentication_challenge: None,
            }
        }
    }

    impl Default for OrderPaymentState {
        fn default() -> Self {
            Self::Completed
        }
    }

    impl Default for OrderPaymentMethod {
        fn default() -> Self {
            Self::Card(Default::default())
        }
    }

    impl Default for OrderPaymentMethodRevolutPay {
        fn default() -> Self {
            Self::Card(Default::default())
        }
    }

    impl Default for OrderPaymentMethodCard {
        fn default() -> Self {
            Self {
                id: None,
                brand: None,
                last_four: None,
            }
        }
    }

    impl Default for OrderPaymentMethodRevolutPayAccount {
        fn default() -> Self {
            Self { id: None }
        }
    }

    impl Default for OrderPaymentMethodRevolutPayCard {
        fn default() -> Self {
            Self {
                id: None,
                brand: None,
                last_four: None,
            }
        }
    }

    impl Default for RefundRequest {
        fn default() -> Self {
            Self {
                amount: 4242,
                currency: "EUR".to_string(),
            }
        }
    }

    #[test]
    fn check_list_orders_casing() {
        assert!(
            serde_json::from_value::<Vec<v10::OrderListItem>>(serde_json::json!([
                {
                    "id": "6849a0a4-ef38-a9ba-9ac2-d6ef5d1997af",
                    "type": "PAYMENT",
                    "state": "FAILED",
                    "created_at": "2025-06-11T15:28:36.339668Z",
                    "updated_at": "2025-07-11T15:28:36.956369Z",
                    "capture_mode": "AUTOMATIC",
                    "metadata": {}
                }
            ]))
            .is_ok()
        )
    }

    #[test]
    fn check_list_query_parameters_casing() {
        assert_eq!(
            "?state=PENDING",
            v10::ListParams {
                state: Some(vec![v10::OrderStateListItem::Pending]),
                ..Default::default()
            }
            .to_string()
        );
        assert_eq!(
            "?state=COMPLETED&state=FAILED",
            v10::ListParams {
                state: Some(vec![
                    v10::OrderStateListItem::Completed,
                    v10::OrderStateListItem::Failed,
                ]),
                ..Default::default()
            }
            .to_string()
        );
    }

    #[tokio::test]
    async fn check_list_orders_type() {
        let _: Vec<v10::OrderListItem> = list(
            &merchant_client()
                .with_sandbox_environment()
                .with_authentication(
                    MerchantAuthenticationBuilder::default()
                        .with_dummy_secret_key()
                        .build(),
                )
                .build()
                .unwrap(),
            &Default::default(),
        )
        .await
        .unwrap();
    }

    #[tokio::test]
    async fn check_get_order_type() {
        let _: v10::Order = retrieve(
            &merchant_client()
                .with_sandbox_environment()
                .with_authentication(
                    MerchantAuthenticationBuilder::default()
                        .with_dummy_secret_key()
                        .build(),
                )
                .build()
                .unwrap(),
            "some-order-id",
        )
        .await
        .unwrap();
    }

    #[test]
    fn check_get_order_casing() {
        assert!(
            serde_json::from_value::<v10::Order>(serde_json::json!(
                {
                    "id": "6849a0a4-ef38-a9ba-9ac2-d6ef5d1997af",
                    "type": "payment",
                    "state": "failed",
                    "created_at": "2025-06-11T15:28:36.339668Z",
                    "updated_at": "2025-07-11T15:28:36.956369Z",
                    "capture_mode": "automatic",
                    "amount": 9990,
                    "outstanding_amount": 9990,
                    "currency": "EUR",
                    "enforce_challenge": "automatic"
                }
            ))
            .is_ok()
        )
    }
}
