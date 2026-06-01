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

use serde::Serialize;
use std::clone::Clone;

pub use crate::{
    BusinessClient, CryptoRampClient, MerchantClient, OpenBankingClient,
    errors::{self, ClientBuilderError, Error},
};

use std::{fmt::Debug, marker::PhantomData};

#[derive(Debug)]
pub struct Client<E: Environment, T> {
    pub environment: E,
    pub client: reqwest_middleware::ClientWithMiddleware,
    pub authentication: T,
}

pub struct MissingEnvironment;
pub struct MissingClientAuthentication;

#[derive(Clone)]
pub struct Part<'a> {
    pub contents: &'a [u8],
    pub mime_str: &'a str,
    pub file_name: &'a str,
}

#[derive(Clone)]
pub enum Body<'a, T: Clone + Serialize> {
    Json(&'a T),
    Raw(&'a [u8]),
    Multipart(&'a Vec<Part<'a>>),
}

#[derive(Clone)]
pub enum HttpMethod<'a, T: Clone + Serialize> {
    Get,
    Delete,
    Post { body: Option<Body<'a, T>> },
    Patch { body: Option<Body<'a, T>> },
    Put { body: Option<Body<'a, T>> },
}

/// Builds the underlying [`reqwest::Client`] shared by every API client.
///
/// In non-test builds we rely on reqwest's default rustls platform verifier,
/// which loads the system root certificates.
#[cfg(not(test))]
pub(crate) fn reqwest_client() -> reqwest::Client {
    reqwest::Client::new()
}

/// Builds the underlying [`reqwest::Client`] shared by every API client.
///
/// Under `cfg(test)` the request layer short-circuits before reaching the
/// network (see the `request`/`request_raw` overrides in each client module),
/// so the client never performs a real TLS handshake. We therefore build it
/// with an empty root certificate set via [`tls_certs_only`], which avoids
/// invoking the rustls platform verifier. The platform verifier panics with
/// "No CA certificates were loaded from the system" in sandboxed build
/// environments (e.g. the Nix/devenv CI build) that have no system trust store.
///
/// [`tls_certs_only`]: reqwest::ClientBuilder::tls_certs_only
#[cfg(test)]
pub(crate) fn reqwest_client() -> reqwest::Client {
    reqwest::Client::builder()
        .tls_certs_only(std::iter::empty())
        .build()
        .expect("failed to build test reqwest client")
}

impl<'a, T: Clone + Serialize> From<&HttpMethod<'a, T>> for reqwest::Method {
    fn from(method: &HttpMethod<'a, T>) -> Self {
        match method {
            HttpMethod::Get => reqwest::Method::GET,
            HttpMethod::Delete => reqwest::Method::DELETE,
            HttpMethod::Post { .. } => reqwest::Method::POST,
            HttpMethod::Patch { .. } => reqwest::Method::PATCH,
            HttpMethod::Put { .. } => reqwest::Method::PUT,
        }
    }
}

#[derive(Debug)]
pub struct SandboxEnvironment<C> {
    pub(crate) client_type: PhantomData<C>,
}
#[derive(Debug)]
pub struct ProductionEnvironment<C> {
    pub(crate) client_type: PhantomData<C>,
}

pub trait Environment {
    fn uri(&self, version: &str, path: &str) -> RevolutEndpoint;
    fn unversioned_uri(&self, path: &str) -> RevolutEndpoint;
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
    pub fn with_sandbox_environment(self) -> ClientBuilder<SandboxEnvironment<C>, A, C> {
        ClientBuilder {
            environment: SandboxEnvironment {
                client_type: PhantomData,
            },
            authentication: self.authentication,
            client_type: self.client_type,
        }
    }

    pub fn with_production_environment(self) -> ClientBuilder<ProductionEnvironment<C>, A, C> {
        ClientBuilder {
            environment: ProductionEnvironment {
                client_type: PhantomData,
            },
            authentication: self.authentication,
            client_type: self.client_type,
        }
    }
}
