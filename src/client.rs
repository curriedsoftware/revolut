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

use std::cell::RefCell;

pub use crate::{
    errors::{self, ClientBuilderError, Error, Result},
    BusinessClient, MerchantClient, OpenBankingClient,
};

use std::{fmt::Debug, marker::PhantomData};

#[derive(Debug)]
pub struct OpenBankingAuthentication {}

#[derive(Debug)]
pub struct Client<E, T> {
    pub environment: E,
    pub client: reqwest::Client,
    pub authentication: T,
    pub access_token_expires_at: RefCell<Option<chrono::DateTime<chrono::Utc>>>,
    pub access_token: RefCell<Option<String>>,
}

pub struct MissingEnvironment;
pub struct MissingClientAuthentication;

#[derive(PartialEq)]
pub enum HttpMethod<'a> {
    Get,
    Delete,
    Post { body: &'a str },
    Patch { body: &'a str },
    Put { body: &'a str },
}

#[derive(Debug)]
pub struct SandboxEnvironment;
#[derive(Debug)]
pub struct ProductionEnvironment;

pub trait Environment {
    fn uri(&self, version: &str, path: &str) -> RevolutEndpoint;
}

#[derive(Debug)]
pub struct RevolutEndpoint(pub String);

impl<'a: 'b, 'b> From<&'a RevolutEndpoint> for &'b str {
    fn from(endpoint: &'a RevolutEndpoint) -> &'b str {
        &endpoint.0
    }
}

pub struct ClientBuilder<E, A, C> {
    pub(crate) environment: E,
    pub(crate) authentication: A,
    pub(crate) client_type: PhantomData<C>,
}

impl<A, C> ClientBuilder<MissingEnvironment, A, C> {
    pub fn with_sandbox_environment(self) -> ClientBuilder<SandboxEnvironment, A, C> {
        ClientBuilder {
            environment: SandboxEnvironment,
            authentication: self.authentication,
            client_type: self.client_type,
        }
    }

    pub fn with_production_environment(self) -> ClientBuilder<ProductionEnvironment, A, C> {
        ClientBuilder {
            environment: ProductionEnvironment,
            authentication: self.authentication,
            client_type: self.client_type,
        }
    }
}

impl Environment for SandboxEnvironment {
    fn uri(&self, version: &str, path: &str) -> RevolutEndpoint {
        RevolutEndpoint(format!(
            "{}{}{}",
            "https://sandbox-b2b.revolut.com/api/", version, path
        ))
    }
}

impl Environment for ProductionEnvironment {
    fn uri(&self, version: &str, path: &str) -> RevolutEndpoint {
        RevolutEndpoint(format!(
            "{}{}{}",
            "https://b2b.revolut.com/api/", version, path
        ))
    }
}
