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

    #[derive(Debug, Deserialize, PartialEq, Serialize)]
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

    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    #[serde(rename_all = "snake_case")]
    pub enum LineItemType {
        #[serde(alias = "PHYSICAL")]
        Physical,
        #[serde(alias = "SERVICE")]
        Service,
    }

    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    pub struct Quantity {
        pub value: f64,
        pub unit: Option<String>,
    }

    #[derive(Debug, Deserialize, PartialEq, Serialize)]
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

    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    pub struct Discount {
        pub name: String,
        pub amount: u64,
    }

    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    pub struct Tax {
        pub name: String,
        pub amount: u64,
    }

    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    pub struct Passenger {
        pub first_name: String,
        pub last_name: String,
    }

    #[derive(Debug, Deserialize, PartialEq, Serialize)]
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

    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    pub struct Transaction {
        pub id: String,
        pub status: TransactionStatus,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recipient_wallet_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recipient_user_id: Option<String>,
    }

    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    #[serde(rename_all = "snake_case")]
    pub enum TransactionStatus {
        #[serde(alias = "PENDING")]
        Pending,
        #[serde(alias = "FAILED")]
        Failed,
        #[serde(alias = "CANCELLED")]
        Cancelled,
        #[serde(alias = "COMPLETED")]
        Completed,
    }

    #[derive(Debug, Deserialize, PartialEq, Serialize)]
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

    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    pub struct Subseller {
        pub id: String,
        pub name: String,
        pub website: String,
        pub phone: String,
        pub address: Address,
    }

    #[derive(Debug, Deserialize, PartialEq, Serialize)]
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

    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    pub struct Ticket {
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub transferable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub refundability: Option<String>,
    }

    #[derive(Debug, Deserialize, PartialEq, Serialize)]
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

    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    pub struct Guest {
        pub first_name: String,
        pub last_name: String,
    }

    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    pub struct Contact {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub email: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub phone: Option<String>,
    }

    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    pub struct Shipment {
        pub shipping_company_name: String,
        pub tracking_number: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub estimated_delivery_date: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tracking_url: Option<String>,
    }

    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    pub struct Shipping {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub address: Option<Address>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub contact: Option<Contact>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub shipments: Option<Vec<Shipment>>,
    }

    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    #[serde(tag = "type", rename_all = "snake_case")]
    pub enum IndustryData {
        #[serde(alias = "AIRLINE")]
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
        #[serde(alias = "CRYPTO")]
        Crypto {
            transactions: Vec<Transaction>,
            #[serde(skip_serializing_if = "Option::is_none")]
            subseller_mcc: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            subseller_url: Option<String>,
        },
        #[serde(alias = "EVENT")]
        Event {
            booking_id: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            events: Option<Vec<Event>>,
        },
        #[serde(alias = "LODGING")]
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
        #[serde(alias = "MARKETPLACE")]
        Marketplace { subseller: Subseller },
    }

    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    pub struct MerchantOrderData {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub url: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub reference: Option<String>,
    }

    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    pub struct UpcomingPaymentData {
        pub date: String,
        pub payment_method_id: String,
    }

    #[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
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

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(rename_all = "snake_case")]
    pub enum SavedPaymentMethodType {
        #[serde(alias = "CARD")]
        Card,
        #[serde(alias = "REVOLUT_PAY")]
        RevolutPay,
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(rename_all = "snake_case")]
    pub enum SavedPaymentMethodInitiator {
        #[serde(alias = "CUSTOMER")]
        Customer,
        #[serde(alias = "MERCHANT")]
        Merchant,
    }

    #[derive(Debug, Deserialize, Serialize)]
    #[serde(rename_all = "snake_case")]
    pub enum SavedPaymentMethodEnvironment {
        #[serde(alias = "BROWSER")]
        Browser,
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct SavedPaymentMethod {
        r#type: SavedPaymentMethodType,
        id: String,
        initiator: SavedPaymentMethodInitiator,
        environment: String,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub enum Type {
        #[serde(rename = "payment", alias = "PAYMENT")]
        Payment,
        #[serde(rename = "payment_request", alias = "PAYMENT_REQUEST")]
        PaymentRequest,
        #[serde(rename = "refund", alias = "REFUND")]
        Refund,
        #[serde(rename = "chargeback", alias = "CHARGEBACK")]
        Chargeback,
        #[serde(rename = "chargeback_reversal", alias = "CHARGEBACK_REVERSAL")]
        ChargebackReversal,
        #[serde(rename = "credit_reimbursement", alias = "CREDIT_REIMBURSEMENT")]
        CreditReimbursement,
    }

    #[derive(Debug, Deserialize, Serialize)]
    #[serde(rename_all = "snake_case")]
    pub enum State {
        #[serde(alias = "PENDING")]
        Pending,
        #[serde(alias = "PROCESSING")]
        Processing,
        #[serde(alias = "AUTHORISED")]
        Authorised,
        #[serde(alias = "COMPLETED")]
        Completed,
        #[serde(alias = "CANCELLED")]
        Cancelled,
        #[serde(alias = "FAILED")]
        Failed,
    }

    #[derive(Debug, Deserialize, Serialize)]
    #[serde(rename_all = "snake_case")]
    pub enum PaymentType {
        #[serde(alias = "APPLE_PAY")]
        ApplePay,
        #[serde(alias = "CARD")]
        Card,
        #[serde(alias = "GOOGLE_PAY")]
        GooglePay,
        #[serde(alias = "REVOLUT_PAY_CARD")]
        RevolutPayCard,
        #[serde(alias = "REVOLUT_PAY_ACCOUNT")]
        RevolutPayAccount,
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

    #[derive(Debug, Deserialize, Serialize)]
    #[serde(rename_all = "snake_case")]
    pub enum PaymentState {
        #[serde(alias = "PENDING")]
        Pending,
        #[serde(alias = "PROCESSING")]
        Processing,
        #[serde(alias = "AUTHORISED")]
        Authorised,
        #[serde(alias = "COMPLETED")]
        Completed,
        #[serde(alias = "CANCELLED")]
        Cancelled,
        #[serde(alias = "FAILED")]
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

    #[derive(Debug, Deserialize, Serialize)]
    pub enum OrderAuthenticationChallenge {
        #[serde(rename = "three_ds")]
        ThreeDs(OrderAuthenticationChallengeThreeDs),
        #[serde(rename = "three_ds_fingerprint")]
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

    #[derive(Debug, Deserialize, Serialize)]
    #[serde(rename_all = "snake_case")]
    pub enum OrderPaymentState {
        #[serde(alias = "PENDING")]
        Pending,
        #[serde(alias = "AUTHENTICATION_CHALLENGE")]
        AuthenticationChallenge,
        #[serde(alias = "AUTHENTICATION_VERIFIED")]
        AuthenticationVerified,
        #[serde(alias = "AUTHORISATION_STARTED")]
        AuthorisationStarted,
        #[serde(alias = "AUTHORISATION_PASSED")]
        AuthorisationPassed,
        #[serde(alias = "AUTHORISED")]
        Authorised,
        #[serde(alias = "CAPTURE_STARTED")]
        CaptureStarted,
        #[serde(alias = "CAPTURED")]
        Captured,
        #[serde(alias = "REFUND_VALIDATED")]
        RefundValidated,
        #[serde(alias = "REFUND_STARTED")]
        RefundStarted,
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
    #[serde(tag = "type")]
    pub enum OrderPaymentMethod {
        RevolutPay(OrderPaymentMethodRevolutPay),
        Card(OrderPaymentMethodCard),
    }

    #[derive(Debug, Deserialize, Serialize)]
    #[serde(tag = "subtype")]
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

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct RefundRequest {
        pub amount: u64,
        pub currency: String,
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
    // TODO: Implement parameter restrictions based on current state:
    // https://developer.revolut.com/docs/merchant/update-order
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
) -> ApiResult<Vec<v10::Order>> {
    client
        .request(
            HttpMethod::<()>::Get,
            &client.environment.uri("1.0", "/orders"),
        )
        .await
}

pub async fn capture<E: Environment>(
    client: &Client<E, MerchantAuthentication>,
    order_id: &str,
    amount: u64,
) -> ApiResult<v10::Order> {
    #[derive(Clone, Debug, PartialEq, serde::Serialize)]
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
        .request::<v10::Order, ()>(
            HttpMethod::Post { body: None },
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
    #[derive(Clone, Debug, PartialEq, serde::Serialize)]
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
