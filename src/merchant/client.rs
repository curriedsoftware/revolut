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

use reqwest::StatusCode;
use reqwest_retry::{RetryTransientMiddleware, policies::ExponentialBackoff};
use serde::{Serialize, de::DeserializeOwned};
use std::{clone::Clone, fmt::Debug, marker::PhantomData};

pub use crate::{
    client::{
        self, Body, Client, ClientBuilder, Environment, HttpMethod, MerchantClient,
        MissingClientAuthentication, MissingEnvironment, ProductionEnvironment, RevolutEndpoint,
        SandboxEnvironment,
    },
    errors::{self, ClientBuilderError},
    merchant::{ApiResult, Error},
};

pub fn merchant_client()
-> ClientBuilder<MissingEnvironment, MissingClientAuthentication, MerchantClient> {
    ClientBuilder {
        environment: MissingEnvironment,
        authentication: MissingClientAuthentication,
        client_type: PhantomData,
    }
}

impl Environment for SandboxEnvironment<MerchantClient> {
    fn uri(&self, version: &str, path: &str) -> RevolutEndpoint {
        self.unversioned_uri(&format!("/{version}{path}"))
    }

    fn unversioned_uri(&self, path: &str) -> RevolutEndpoint {
        RevolutEndpoint(format!("https://sandbox-merchant.revolut.com/api{path}",))
    }
}

impl Environment for ProductionEnvironment<MerchantClient> {
    fn uri(&self, version: &str, path: &str) -> RevolutEndpoint {
        self.unversioned_uri(&format!("/{version}{path}"))
    }

    fn unversioned_uri(&self, path: &str) -> RevolutEndpoint {
        RevolutEndpoint(format!("https://merchant.revolut.com/api{path}"))
    }
}

pub struct MissingSecretKey;

pub trait MissingSecretKeyT {}
impl MissingSecretKeyT for MissingSecretKey {}

pub struct MerchantAuthenticationBuilder<S> {
    secret_key: S,
}

impl Default for MerchantAuthenticationBuilder<MissingSecretKey> {
    fn default() -> Self {
        MerchantAuthenticationBuilder {
            secret_key: MissingSecretKey,
        }
    }
}

impl MerchantAuthenticationBuilder<MissingSecretKey> {
    #[cfg(test)]
    pub fn with_dummy_secret_key(self) -> MerchantAuthenticationBuilder<()> {
        MerchantAuthenticationBuilder { secret_key: () }
    }

    pub fn with_environment_inherited_secret_key(
        self,
        secret_key_environment_variable: &str,
    ) -> Result<MerchantAuthenticationBuilder<String>, ClientBuilderError> {
        let secret_key = std::env::var(secret_key_environment_variable).map_err(|_| {
            ClientBuilderError::MissingEnvironmentVariable(secret_key_environment_variable.into())
        })?;
        if secret_key.is_empty() {
            return Err(ClientBuilderError::InvalidSecret);
        }
        Ok(MerchantAuthenticationBuilder { secret_key })
    }

    pub fn with_secret_key(
        self,
        secret_key: &str,
    ) -> Result<MerchantAuthenticationBuilder<String>, ClientBuilderError> {
        if secret_key.is_empty() {
            return Err(ClientBuilderError::InvalidSecret);
        }
        Ok(MerchantAuthenticationBuilder {
            secret_key: secret_key.to_string(),
        })
    }
}

impl MerchantAuthenticationBuilder<String> {
    pub fn build(self) -> MerchantAuthentication {
        MerchantAuthentication {
            secret_key: self.secret_key,
        }
    }
}

#[cfg(test)]
impl MerchantAuthenticationBuilder<()> {
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
    pub fn build(self) -> Result<Client<E, MerchantAuthentication>, ClientBuilderError> {
        let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
        let client_builder =
            reqwest_middleware::ClientBuilder::new(crate::client::reqwest_client())
                .with(RetryTransientMiddleware::new_with_policy(retry_policy));
        Ok(Client {
            environment: self.environment,
            client: client_builder.build(),
            authentication: self.authentication,
        })
    }
}

impl<E: Environment> Client<E, MerchantAuthentication> {
    pub(crate) async fn request_<R: DeserializeOwned + Debug, T: Clone + Debug + Serialize>(
        &self,
        method: HttpMethod<'_, T>,
        uri: &RevolutEndpoint,
    ) -> ApiResult<R> {
        let request = match method {
            HttpMethod::Get => self.client.get(Into::<&str>::into(uri)),
            HttpMethod::Delete => self.client.delete(Into::<&str>::into(uri)),
            HttpMethod::Post { ref body }
            | HttpMethod::Patch { ref body }
            | HttpMethod::Put { ref body } => {
                let client = self
                    .client
                    .request((&method).into(), Into::<&str>::into(uri));
                match body {
                    Some(Body::Json(body)) => client.json(body),
                    Some(Body::Raw(body)) => client.body(body.to_vec()),
                    Some(Body::Multipart(parts)) => {
                        let mut multipart_form = reqwest::multipart::Form::new();
                        for part in parts.iter() {
                            let multipart_part =
                                reqwest::multipart::Part::bytes(Vec::from(part.contents))
                                    .mime_str(part.mime_str);
                            multipart_form =
                                multipart_form.part(part.file_name.to_string(), multipart_part?);
                        }
                        client.multipart(multipart_form)
                    }
                    None => client.header("Content-Length", 0),
                }
            }
        };

        let response = request
            .header(
                "Authorization",
                format!("Bearer {}", self.authentication.secret_key),
            )
            .header("Accept", "application/json")
            .header("Revolut-Api-Version", "2025-12-04")
            .send()
            .await
            .map_err(|err| {
                Error::ClientError(Box::new(errors::ClientError::RequestError(format!(
                    "{err:?}"
                ))))
            })?;

        if response.status().is_success() {
            if response.status() == StatusCode::NO_CONTENT {
                return Ok(serde_json::from_value(serde_json::Value::Null).unwrap());
            }
            let response_ = format!("{response:?}");
            Ok(response.json().await.map_err(|err| {
                Error::ClientError(Box::new(errors::ClientError::RequestError(format!(
                    "{err:?}: {response_}",
                ))))
            })?)
        } else {
            Err(Error::BackendError(response.json().await?))
        }
    }

    #[cfg(not(test))]
    pub(crate) async fn request<R: DeserializeOwned + Debug, T: Clone + Debug + Serialize>(
        &self,
        method: HttpMethod<'_, T>,
        uri: &RevolutEndpoint,
    ) -> ApiResult<R> {
        self.request_(method, uri).await
    }

    #[cfg(test)]
    pub(crate) async fn request<
        R: DeserializeOwned + Debug + std::default::Default,
        T: Clone + Debug + Serialize,
    >(
        &self,
        method: HttpMethod<'_, T>,
        uri: &RevolutEndpoint,
    ) -> ApiResult<R> {
        if self.authentication.secret_key.is_empty() {
            return Ok(Default::default());
        }
        self.request_(method, uri).await
    }
}
