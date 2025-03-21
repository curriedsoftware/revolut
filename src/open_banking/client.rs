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

use std::marker::PhantomData;

pub use crate::client::{
    ClientBuilder, MissingClientAuthentication, MissingEnvironment, OpenBankingAuthentication,
    OpenBankingClient,
};

pub fn openbanking_client(
) -> ClientBuilder<MissingEnvironment, MissingClientAuthentication, OpenBankingClient> {
    ClientBuilder {
        environment: MissingEnvironment,
        authentication: MissingClientAuthentication,
        client_type: PhantomData,
    }
}

impl<E> ClientBuilder<E, MissingClientAuthentication, OpenBankingClient> {
    pub fn with_authentication(
        self,
        authentication: OpenBankingAuthentication,
    ) -> ClientBuilder<E, OpenBankingAuthentication, OpenBankingClient> {
        ClientBuilder {
            environment: self.environment,
            authentication,
            client_type: self.client_type,
        }
    }
}
