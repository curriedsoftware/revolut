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
use std::{
    convert::{self},
    fmt::Debug,
    ops::{ControlFlow, FromResidual, Try},
    process::{ExitCode, Termination},
};

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
    GenericError(String),
}

#[derive(Debug, Deserialize)]
pub struct BackendError {
    code: Option<String>,
    message: Option<String>,
    timestamp: Option<u64>,
}

#[derive(Debug)]
pub enum Result<T> {
    Ok(T),
    Err(Error),
}

impl<T> Try for Result<T> {
    type Output = T;
    type Residual = Result<convert::Infallible>;

    #[inline]
    fn from_output(output: Self::Output) -> Self {
        Result::Ok(output)
    }

    #[inline]
    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            Result::Ok(v) => ControlFlow::Continue(v),
            Result::Err(e) => ControlFlow::Break(Result::Err(e)),
        }
    }
}

impl<T, E: Debug> FromResidual<std::result::Result<convert::Infallible, E>> for Result<T> {
    fn from_residual(residual: std::result::Result<convert::Infallible, E>) -> Result<T> {
        match residual {
            Err(e) => Result::Err(Error::ClientError(ClientError::GenericError(format!(
                "{e:?}"
            )))),
        }
    }
}

impl<T> FromResidual<Result<convert::Infallible>> for Result<T> {
    fn from_residual(residual: Result<convert::Infallible>) -> Result<T> {
        match residual {
            Result::Err(e) => Result::Err(e),
        }
    }
}

impl<T: Termination> Termination for Result<T> {
    fn report(self) -> ExitCode {
        match self {
            Result::Ok(val) => val.report(),
            Result::Err(err) => {
                eprintln!("Error: {err:?}");
                ExitCode::FAILURE
            }
        }
    }
}
