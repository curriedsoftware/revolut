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

use serde::Deserialize;
use std::fmt::Debug;

/// The error type returned by every API call. It is generic over the backend
/// error's `code` type so each API can pin the concrete shape it actually
/// returns: the Merchant API uses `Error<String>`, the Business API uses
/// `Error<u32>`. Callers therefore match on a `code: Option<String>` or
/// `code: Option<u32>` directly — never on a cross-API union. APIs whose error
/// `code` shape is unconfirmed fall back to the default [`ErrorCode`].
#[derive(Debug, Deserialize)]
pub enum Error<C = ErrorCode> {
    ClientBuilderError(Box<ClientBuilderError>),
    ClientError(Box<ClientError>),
    BackendError(Box<BackendError<C>>),
}

#[derive(Debug, Deserialize)]
pub enum ClientBuilderError {
    MissingEnvironmentVariable(String),
    InvalidSecret,
    CannotInstantiateClient(String),
}

#[derive(Debug, Deserialize)]
pub enum ClientError {
    CannotLogIn(String),
    HttpStatus(u16),
    RequestError(String),
    SerializationError(String),
    ValidationError(String),
    GenericError(String),
}

/// A fallback `code` type for APIs whose error shape we have not pinned to a
/// concrete type. The shape differs per API: the Merchant API returns a string
/// identifier (e.g. `"internal_server_error"`, `"validation"`, per the
/// `Error-v2` schema), while the Business API returns a numeric code (e.g.
/// `2101`) — those are modelled directly as `BackendError<String>` and
/// `BackendError<u32>`, so callers see a `code: Option<String>` or
/// `code: Option<u32>` without matching a cross-API union. This untagged enum
/// remains the default for any not-yet-specialized API, accepting either shape
/// so a `BackendError` never fails to deserialize — which would otherwise mask
/// the real error body behind a decode error.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ErrorCode {
    Numeric(u32),
    Text(String),
}

#[derive(Debug, Deserialize)]
pub struct BackendError<C = ErrorCode> {
    pub code: Option<C>,
    pub error_code: Option<u32>,
    #[serde(rename = "errorId")]
    pub error_id: Option<String>,
    pub errors: Option<Vec<ErrorItem>>,
    pub id: Option<String>,
    pub message: Option<String>,
    pub timestamp: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct ErrorItem {
    pub error_code: String,
    pub message: String,
}

pub type ApiResult<T, C = ErrorCode> = std::result::Result<T, Error<C>>;

impl<C> From<ClientBuilderError> for Error<C> {
    fn from(error: ClientBuilderError) -> Self {
        Error::ClientBuilderError(Box::new(error))
    }
}

impl<C> From<serde_json::Error> for Error<C> {
    fn from(error: serde_json::Error) -> Self {
        Error::ClientError(Box::new(ClientError::SerializationError(format!(
            "{error:?}"
        ))))
    }
}

impl<C> From<reqwest::Error> for Error<C> {
    fn from(error: reqwest::Error) -> Self {
        Error::ClientError(Box::new(ClientError::RequestError(format!("{error:?}"))))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Merchant API errors (`Error-v2`) carry a string `code`, e.g.
    // `{"code":"internal_server_error","message":"...","timestamp":...}`. The
    // Merchant API pins `BackendError<String>`, so callers read the code
    // directly as `Option<String>` without matching a cross-API union.
    #[test]
    fn deserializes_merchant_string_code() {
        let body = r#"{"code":"internal_server_error","message":"An unexpected error occurred","timestamp":1780931703975}"#;
        let err: BackendError<String> =
            serde_json::from_str(body).expect("merchant error must deserialize");
        assert_eq!(err.code.as_deref(), Some("internal_server_error"));
        assert_eq!(err.message.as_deref(), Some("An unexpected error occurred"));
    }

    // Business API errors carry a numeric `code`, e.g.
    // `{"code":2101,"message":"..."}`. The Business API pins `BackendError<u32>`,
    // so callers read the code directly as `Option<u32>`.
    #[test]
    fn deserializes_business_numeric_code() {
        let body = r#"{"code":2101,"message":"'name' is required"}"#;
        let err: BackendError<u32> =
            serde_json::from_str(body).expect("business error must deserialize");
        assert_eq!(err.code, Some(2101));
    }

    // APIs left on the default `ErrorCode` still accept either shape, so a
    // `BackendError` never fails to deserialize behind a decode error.
    #[test]
    fn default_error_code_accepts_either_shape() {
        let numeric: BackendError =
            serde_json::from_str(r#"{"code":2101}"#).expect("numeric must deserialize");
        assert!(matches!(numeric.code, Some(ErrorCode::Numeric(2101))));
        let text: BackendError =
            serde_json::from_str(r#"{"code":"validation"}"#).expect("text must deserialize");
        assert!(matches!(text.code, Some(ErrorCode::Text(ref c)) if c == "validation"));
    }
}
