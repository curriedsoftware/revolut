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

use serde::de::DeserializeOwned;
use std::{cell::RefCell, fmt::Debug, marker::PhantomData};

use crate::{
    client::{
        self, Client, ClientBuilder, Environment, HttpMethod, MerchantClient,
        MissingClientAuthentication, MissingEnvironment, ProductionEnvironment, RevolutEndpoint,
        SandboxEnvironment,
    },
    errors::{self, Result},
};

pub fn merchant_client(
) -> ClientBuilder<MissingEnvironment, MissingClientAuthentication, MerchantClient> {
    ClientBuilder {
        environment: MissingEnvironment,
        authentication: MissingClientAuthentication,
        client_type: PhantomData,
    }
}

impl Environment for SandboxEnvironment<MerchantClient> {
    fn uri(&self, version: &str, path: &str) -> RevolutEndpoint {
        RevolutEndpoint(format!(
            "{}{}{}",
            "https://sandbox-merchant.revolut.com/api/", version, path
        ))
    }
}

impl Environment for ProductionEnvironment<MerchantClient> {
    fn uri(&self, version: &str, path: &str) -> RevolutEndpoint {
        RevolutEndpoint(format!(
            "{}{}{}",
            "https://merchant.revolut.com/api/", version, path
        ))
    }
}

pub struct MerchantAuthenticationBuilder {}

impl Default for MerchantAuthenticationBuilder {
    fn default() -> MerchantAuthenticationBuilder {
        MerchantAuthenticationBuilder {}
    }
}

impl MerchantAuthenticationBuilder {
    pub fn build(self) -> MerchantAuthentication {
        MerchantAuthentication {
            secret_key: String::new(),
        }
    }
}

#[derive(Debug)]
pub struct MerchantAuthentication {
    secret_key: String,
}

impl<E> ClientBuilder<E, MissingClientAuthentication, MerchantClient> {
    pub fn with_authentication(
        self,
        authentication: MerchantAuthentication,
    ) -> ClientBuilder<E, MerchantAuthentication, MerchantClient> {
        ClientBuilder {
            environment: self.environment,
            authentication,
            client_type: self.client_type,
        }
    }
}

impl<E: Environment, C> ClientBuilder<E, MerchantAuthentication, C> {
    pub fn build(self) -> Result<Client<E, MerchantAuthentication>> {
        let client_builder = reqwest::ClientBuilder::new();
        Ok(Client {
            environment: self.environment,
            client: client_builder.build().map_err(|err| {
                errors::Error::ClientBuilderError(
                    errors::ClientBuilderError::CannotInstantiateClient(format!("{:?}", err)),
                )
            })?,
            authentication: self.authentication,
            access_token: RefCell::new(None),
            access_token_expires_at: RefCell::new(None),
        })
    }
}

impl<E: Environment> Client<E, MerchantAuthentication> {
    pub(crate) async fn request<'a, R: DeserializeOwned + Debug>(
        &self,
        method: HttpMethod<'a>,
        uri: &RevolutEndpoint,
    ) -> Result<R> {
        unimplemented!()
    }
}
