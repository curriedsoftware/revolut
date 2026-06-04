/***
 * Copyright (c) 2026 Rafael Fernández López <ereslibre@curried.software>
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
    errors::{ApiResult, ClientError, Error},
    merchant::client::MerchantAuthentication,
};

pub mod v10 {
    // A hand-written, validated wrapper for the ISO 8601 duration fields the
    // generated types model as plain `String`s. See the `Durations` section
    // below.
    pub use super::Iso8601Duration;

    // Reused verbatim from the OpenAPI-generated types. These stay in sync
    // automatically when the specs are bumped via `just generate`; we only
    // alias them back to this crate's public names.
    pub use crate::merchant::generated::{
        Subscription, SubscriptionCreation, SubscriptionCycle, SubscriptionCycleState,
        SubscriptionCycles, SubscriptionPaymentMethodType, SubscriptionPlan,
        SubscriptionPlanCreation, SubscriptionPlanPhase, SubscriptionPlanPhaseCreation,
        SubscriptionPlanState, SubscriptionPlanVariation, SubscriptionPlanVariationCreation,
        SubscriptionPlans, SubscriptionState, SubscriptionUpdate, Subscriptions,
    };
}

// ───────────────────────────── Subscription plans ─────────────────────────────

pub async fn create_plan<E: Environment>(
    client: &Client<E, MerchantAuthentication>,
    plan: &v10::SubscriptionPlanCreation,
) -> ApiResult<v10::SubscriptionPlan> {
    if let Some(trial_duration) = &plan.trial_duration {
        validate_iso8601_duration("trial_duration", trial_duration)?;
    }
    for variation in &plan.variations {
        for phase in &variation.phases {
            validate_iso8601_duration("cycle_duration", &phase.cycle_duration)?;
        }
    }
    client
        .request(
            HttpMethod::Post {
                body: Some(Body::Json(&plan)),
            },
            &client.environment.unversioned_uri("/subscription-plans"),
        )
        .await
}

pub async fn plans<E: Environment>(
    client: &Client<E, MerchantAuthentication>,
) -> ApiResult<v10::SubscriptionPlans> {
    client
        .request(
            HttpMethod::<()>::Get,
            &client.environment.unversioned_uri("/subscription-plans"),
        )
        .await
}

pub async fn plan<E: Environment>(
    client: &Client<E, MerchantAuthentication>,
    plan_id: &str,
) -> ApiResult<v10::SubscriptionPlan> {
    client
        .request(
            HttpMethod::<()>::Get,
            &client
                .environment
                .unversioned_uri(&format!("/subscription-plans/{plan_id}")),
        )
        .await
}

// ─────────────────────────────── Subscriptions ───────────────────────────────

pub async fn create<E: Environment>(
    client: &Client<E, MerchantAuthentication>,
    subscription: &v10::SubscriptionCreation,
) -> ApiResult<v10::Subscription> {
    if let Some(trial_duration) = &subscription.trial_duration {
        validate_iso8601_duration("trial_duration", trial_duration)?;
    }
    client
        .request(
            HttpMethod::Post {
                body: Some(Body::Json(&subscription)),
            },
            &client.environment.unversioned_uri("/subscriptions"),
        )
        .await
}

pub async fn list<E: Environment>(
    client: &Client<E, MerchantAuthentication>,
) -> ApiResult<v10::Subscriptions> {
    client
        .request(
            HttpMethod::<()>::Get,
            &client.environment.unversioned_uri("/subscriptions"),
        )
        .await
}

pub async fn retrieve<E: Environment>(
    client: &Client<E, MerchantAuthentication>,
    subscription_id: &str,
) -> ApiResult<v10::Subscription> {
    client
        .request(
            HttpMethod::<()>::Get,
            &client
                .environment
                .unversioned_uri(&format!("/subscriptions/{subscription_id}")),
        )
        .await
}

pub async fn update<E: Environment>(
    client: &Client<E, MerchantAuthentication>,
    subscription_id: &str,
    update: &v10::SubscriptionUpdate,
) -> ApiResult<v10::Subscription> {
    client
        .request(
            HttpMethod::Patch {
                body: Some(Body::Json(&update)),
            },
            &client
                .environment
                .unversioned_uri(&format!("/subscriptions/{subscription_id}")),
        )
        .await
}

pub async fn cancel<E: Environment>(
    client: &Client<E, MerchantAuthentication>,
    subscription_id: &str,
) -> ApiResult<()> {
    client
        .request(
            HttpMethod::Post::<()> { body: None },
            &client
                .environment
                .unversioned_uri(&format!("/subscriptions/{subscription_id}/cancel")),
        )
        .await
}

pub async fn cycles<E: Environment>(
    client: &Client<E, MerchantAuthentication>,
    subscription_id: &str,
) -> ApiResult<v10::SubscriptionCycles> {
    client
        .request(
            HttpMethod::<()>::Get,
            &client
                .environment
                .unversioned_uri(&format!("/subscriptions/{subscription_id}/cycles")),
        )
        .await
}

// ─────────────────────────────── Durations ───────────────────────────────
//
// The Revolut API expects `cycle_duration` and `trial_duration` to be ISO 8601
// durations (e.g. `P1M`, `P7D`, `PT1H30M`). The OpenAPI-generated types model
// them as plain `String`s, which we leave untouched. To push correctness as
// close to the value's construction as possible, callers should build durations
// through `Iso8601Duration`:
//
//   * `iso8601_duration!("P1M")` validates the literal at *compile time* — a
//     malformed literal fails the build, so it can never reach the request.
//   * `Iso8601Duration::parse(runtime_value)` validates at *construction time*
//     for values only known at runtime, surfacing the error where the value
//     enters the program rather than deep inside the request call.
//
// An `Iso8601Duration` converts into the `String` the generated structs expect
// via `.into()`. The generated fields stay raw `String`s, so `create_plan` and
// `create` keep a thin runtime guard (`validate_iso8601_duration`) as a last
// line of defense for callers that hand-roll the field with a bare string.

/// An ISO 8601 duration (e.g. `P1M`, `P7D`, `PT1H30M`), validated on the way in.
///
/// Build one with the [`iso8601_duration!`](crate::iso8601_duration) macro for a
/// compile-time-checked literal, or [`Iso8601Duration::parse`] /
/// [`TryFrom`]/[`FromStr`] for a runtime value. Convert into the `String` the
/// generated request types expect with `.into()`.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Iso8601Duration(String);

impl Iso8601Duration {
    /// Validates `value`, returning a [`ClientError::ValidationError`] if it is
    /// not a syntactically valid ISO 8601 duration.
    pub fn parse(value: impl Into<String>) -> ApiResult<Self> {
        let value = value.into();
        if is_valid_iso8601_duration(&value) {
            Ok(Self(value))
        } else {
            Err(Error::ClientError(Box::new(ClientError::ValidationError(
                format!(
                    "expected an ISO 8601 duration (e.g. `P1M`, `P7D`, `PT1H30M`), got `{value}`"
                ),
            ))))
        }
    }

    /// Wraps a literal already proven valid by [`iso8601_duration!`]. Not meant
    /// to be called directly — use the macro, which checks the literal first.
    #[doc(hidden)]
    pub fn from_static(value: &'static str) -> Self {
        Self(value.to_string())
    }

    /// The underlying duration string.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the wrapper, returning the underlying `String`.
    pub fn into_string(self) -> String {
        self.0
    }
}

impl From<Iso8601Duration> for String {
    fn from(duration: Iso8601Duration) -> Self {
        duration.0
    }
}

impl std::fmt::Display for Iso8601Duration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::str::FromStr for Iso8601Duration {
    type Err = Error;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

impl TryFrom<String> for Iso8601Duration {
    type Error = Error;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl TryFrom<&str> for Iso8601Duration {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

/// Builds an [`Iso8601Duration`] from a string literal, rejecting malformed
/// literals at compile time.
///
/// ```
/// # use revolut::iso8601_duration;
/// let monthly = iso8601_duration!("P1M");
/// ```
///
/// A malformed literal fails to compile:
///
/// ```compile_fail
/// # use revolut::iso8601_duration;
/// let nope = iso8601_duration!("two weeks");
/// ```
#[macro_export]
macro_rules! iso8601_duration {
    ($literal:literal) => {{
        const _: () = ::core::assert!(
            $crate::merchant::subscriptions::is_valid_iso8601_duration($literal),
            ::core::concat!(
                "`",
                $literal,
                "` is not a valid ISO 8601 duration (e.g. `P1M`, `P7D`, `PT1H30M`)"
            )
        );
        $crate::merchant::subscriptions::Iso8601Duration::from_static($literal)
    }};
}

fn validate_iso8601_duration(field: &str, value: &str) -> ApiResult<()> {
    if is_valid_iso8601_duration(value) {
        Ok(())
    } else {
        Err(Error::ClientError(Box::new(ClientError::ValidationError(
            format!(
                "`{field}` must be an ISO 8601 duration (e.g. `P1M`, `P7D`, `PT1H30M`), got `{value}`"
            ),
        ))))
    }
}

/// Returns `true` if `value` is a syntactically valid ISO 8601 duration.
///
/// Accepts the date/time form (`P[nY][nM][nD][T[nH][nM][nS]]`) and the week
/// form (`PnW`), requiring at least one component and rejecting designators
/// that are duplicated, out of order, or missing a preceding number.
///
/// This is a `const fn` so [`iso8601_duration!`] can evaluate it at compile
/// time; it is `pub` only so the macro can name it, and is not part of the
/// stable surface.
#[doc(hidden)]
pub const fn is_valid_iso8601_duration(value: &str) -> bool {
    let bytes = value.as_bytes();
    let len = bytes.len();
    // The shortest valid duration is two bytes (`P` plus one digit-led group),
    // and it must start with the `P` designator.
    if len < 2 || bytes[0] != b'P' {
        return false;
    }

    // Week form (`PnW`) is mutually exclusive with the date/time form.
    if bytes[len - 1] == b'W' {
        // Need at least one digit between the `P` and the `W`.
        if len < 3 {
            return false;
        }
        let mut i = 1;
        while i < len - 1 {
            if !is_ascii_digit(bytes[i]) {
                return false;
            }
            i += 1;
        }
        return true;
    }

    // Locate the `T` separating the date and time parts, if present.
    let mut t_index = len; // `len` sentinel: no `T` found.
    let mut i = 1;
    while i < len {
        if bytes[i] == b'T' {
            t_index = i;
            break;
        }
        i += 1;
    }

    // Date part is `bytes[1..date_end]`; time part, when present, is
    // `bytes[time_start..len]`.
    let date_end = t_index;
    let has_time = t_index != len;
    let time_start = t_index + 1;

    // A `T` with no time components after it is invalid.
    if has_time && time_start >= len {
        return false;
    }

    let mut saw_component = false;
    if date_end > 1 {
        if !valid_components(bytes, 1, date_end, b"YMD") {
            return false;
        }
        saw_component = true;
    }
    if has_time {
        if !valid_components(bytes, time_start, len, b"HMS") {
            return false;
        }
        saw_component = true;
    }

    saw_component
}

const fn is_ascii_digit(byte: u8) -> bool {
    byte >= b'0' && byte <= b'9'
}

/// Checks that `bytes[start..end]` is a run of `<digits><designator>` groups
/// whose designators appear, each at most once, in the order given by
/// `designators`.
const fn valid_components(bytes: &[u8], start: usize, end: usize, designators: &[u8]) -> bool {
    let mut next = 0;
    let mut has_digits = false;
    let mut i = start;
    while i < end {
        let ch = bytes[i];
        if is_ascii_digit(ch) {
            has_digits = true;
            i += 1;
            continue;
        }
        if !has_digits {
            return false; // designator without a preceding number
        }
        // Find `ch` among the designators still allowed (`designators[next..]`).
        let mut j = next;
        let mut found = false;
        while j < designators.len() {
            if designators[j] == ch {
                next = j + 1;
                found = true;
                break;
            }
            j += 1;
        }
        if !found {
            return false; // unknown, duplicated, or out-of-order designator
        }
        has_digits = false;
        i += 1;
    }
    !has_digits // trailing digits without a designator are invalid
}

#[cfg(test)]
mod tests {
    use super::{v10::*, *};
    use crate::merchant::client::{MerchantAuthenticationBuilder, merchant_client};

    // The `#[cfg(test)]` `request` returns `Default::default()` for an empty
    // secret key, so every generated response type returned by this module
    // needs a `Default` test helper. The generated types stay pristine; these
    // impls live only in the hand-written consumption module under `cfg(test)`.

    impl Default for SubscriptionPlanState {
        fn default() -> Self {
            Self::Active
        }
    }

    impl Default for SubscriptionPlan {
        fn default() -> Self {
            Self {
                id: "some-plan-id".to_string(),
                name: "some-plan".to_string(),
                trial_duration: None,
                state: Default::default(),
                created_at: "2025-06-11T15:28:36.339668Z".to_string(),
                updated_at: "2025-07-11T15:28:36.956369Z".to_string(),
                variations: Vec::new(),
            }
        }
    }

    impl Default for SubscriptionPlans {
        fn default() -> Self {
            Self {
                next_page_token: None,
                subscription_plans: Vec::new(),
            }
        }
    }

    impl Default for SubscriptionState {
        fn default() -> Self {
            Self::Pending
        }
    }

    impl Default for SubscriptionPaymentMethodType {
        fn default() -> Self {
            Self::Automatic
        }
    }

    impl Default for Subscription {
        fn default() -> Self {
            Self {
                id: "some-subscription-id".to_string(),
                external_reference: None,
                state: Default::default(),
                customer_id: "some-customer-id".to_string(),
                plan_id: "some-plan-id".to_string(),
                plan_variation_id: "some-plan-variation-id".to_string(),
                payment_method_type: Default::default(),
                payment_method_id: None,
                created_at: "2025-06-11T15:28:36.339668Z".to_string(),
                updated_at: "2025-07-11T15:28:36.956369Z".to_string(),
                start_date: None,
                current_cycle_id: "some-cycle-id".to_string(),
                trial_duration: None,
                trial_end_date: None,
                setup_order_id: None,
            }
        }
    }

    impl Default for Subscriptions {
        fn default() -> Self {
            Self {
                next_page_token: None,
                subscriptions: Vec::new(),
            }
        }
    }

    impl Default for SubscriptionCycles {
        fn default() -> Self {
            Self {
                next_page_token: None,
                cycles: Vec::new(),
            }
        }
    }

    fn test_client() -> Client<impl Environment, MerchantAuthentication> {
        merchant_client()
            .with_sandbox_environment()
            .with_authentication(
                MerchantAuthenticationBuilder::default()
                    .with_dummy_secret_key()
                    .build(),
            )
            .build()
            .unwrap()
    }

    #[test]
    fn accepts_valid_durations() {
        for value in [
            "P1Y",
            "P1M",
            "P7D",
            "P1Y2M10D",
            "PT1H",
            "PT30M",
            "PT1H30M",
            "P1DT12H",
            "P1Y2M3DT4H5M6S",
            "P2W",
        ] {
            assert!(
                is_valid_iso8601_duration(value),
                "expected `{value}` to be valid"
            );
        }
    }

    #[test]
    fn rejects_invalid_durations() {
        for value in [
            "", "P", "1M", "PM", "PT", "P1H", "PT1Y", "P1M2Y", "P1MM", "P1.5M", "P-1M", "P1W2D",
            "1Y", "P1S",
        ] {
            assert!(
                !is_valid_iso8601_duration(value),
                "expected `{value}` to be invalid"
            );
        }
    }

    #[test]
    fn iso8601_duration_macro_builds_validated_value() {
        let monthly = crate::iso8601_duration!("P1M");
        assert_eq!(monthly.as_str(), "P1M");
        // Converts into the `String` the generated request types expect.
        let as_string: String = monthly.into();
        assert_eq!(as_string, "P1M");
    }

    #[test]
    fn iso8601_duration_parse_accepts_valid_and_rejects_invalid() {
        assert_eq!(
            Iso8601Duration::parse("PT1H30M").unwrap().as_str(),
            "PT1H30M"
        );
        assert!(matches!(
            Iso8601Duration::parse("two weeks"),
            Err(Error::ClientError(e)) if matches!(*e, ClientError::ValidationError(_))
        ));
        // `FromStr` / `TryFrom` route through the same validation.
        assert!("P7D".parse::<Iso8601Duration>().is_ok());
        assert!(Iso8601Duration::try_from("nope").is_err());
    }

    #[tokio::test]
    async fn create_plan_rejects_non_iso8601_cycle_duration() {
        let plan = SubscriptionPlanCreation {
            name: "some-plan".to_string(),
            trial_duration: None,
            variations: vec![SubscriptionPlanVariationCreation {
                phases: vec![SubscriptionPlanPhaseCreation {
                    ordinal: 1,
                    cycle_duration: "1 month".to_string(),
                    cycle_count: None,
                    amount: 4242,
                    currency: "EUR".to_string(),
                    subscription_items: None,
                }],
            }],
        };
        assert!(matches!(
            create_plan(&test_client(), &plan).await,
            Err(Error::ClientError(e)) if matches!(*e, ClientError::ValidationError(_))
        ));
    }

    #[tokio::test]
    async fn create_rejects_non_iso8601_trial_duration() {
        let subscription = SubscriptionCreation {
            plan_variation_id: "some-plan-variation-id".to_string(),
            customer_id: "some-customer-id".to_string(),
            external_reference: None,
            setup_order_redirect_url: None,
            trial_duration: Some("two weeks".to_string()),
        };
        assert!(matches!(
            create(&test_client(), &subscription).await,
            Err(Error::ClientError(e)) if matches!(*e, ClientError::ValidationError(_))
        ));
    }

    #[tokio::test]
    async fn check_plans_type() {
        let _: SubscriptionPlans = plans(&test_client()).await.unwrap();
    }

    #[tokio::test]
    async fn check_plan_type() {
        let _: SubscriptionPlan = plan(&test_client(), "some-plan-id").await.unwrap();
    }

    #[tokio::test]
    async fn check_list_type() {
        let _: Subscriptions = list(&test_client()).await.unwrap();
    }

    #[tokio::test]
    async fn check_retrieve_type() {
        let _: Subscription = retrieve(&test_client(), "some-subscription-id")
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn check_cycles_type() {
        let _: SubscriptionCycles = cycles(&test_client(), "some-subscription-id")
            .await
            .unwrap();
    }

    #[test]
    fn check_subscription_casing() {
        assert!(
            serde_json::from_value::<Subscription>(serde_json::json!(
                {
                    "id": "6849a0a4-ef38-a9ba-9ac2-d6ef5d1997af",
                    "state": "active",
                    "customer_id": "some-customer-id",
                    "plan_id": "some-plan-id",
                    "plan_variation_id": "some-plan-variation-id",
                    "payment_method_type": "automatic",
                    "created_at": "2025-06-11T15:28:36.339668Z",
                    "updated_at": "2025-07-11T15:28:36.956369Z",
                    "current_cycle_id": "some-cycle-id"
                }
            ))
            .is_ok()
        )
    }
}
