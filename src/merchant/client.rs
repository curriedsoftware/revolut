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

use std::{cell::RefCell, marker::PhantomData};

use crate::{
    client::{
        Client, ClientBuilder, MerchantClient, MissingClientAuthentication, MissingEnvironment,
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

impl<E, C> ClientBuilder<E, MerchantAuthentication, C> {
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
