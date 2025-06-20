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

#[derive(Debug, Deserialize)]
pub enum Error {
    ClientBuilderError(ClientBuilderError),
    ClientError(ClientError),
    BackendError(BackendError),
}

#[derive(Debug, Deserialize)]
pub enum ClientBuilderError {
    MissingEnvironmentVariable(String),
    CannotInstantiateClient(String),
}

#[derive(Debug, Deserialize)]
pub enum ClientError {
    CannotLogIn(String),
    RequestError(String),
    SerializationError(String),
    GenericError(String),
}

#[derive(Debug, Deserialize)]
pub struct BackendError {
    code: Option<String>,
    error_code: Option<String>,
    #[serde(rename = "errorId")]
    error_id: Option<String>,
    errors: Option<Vec<ErrorItem>>,
    id: Option<String>,
    message: Option<String>,
    timestamp: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct ErrorItem {
    error_code: String,
    message: String,
}

pub type ApiResult<T> = std::result::Result<T, Error>;

impl From<ClientBuilderError> for Error {
    fn from(error: ClientBuilderError) -> Self {
        Error::ClientBuilderError(error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::ClientError(ClientError::SerializationError(format!("{error:?}")))
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::ClientError(ClientError::RequestError(format!("{error:?}")))
    }
}
