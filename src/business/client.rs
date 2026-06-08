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

use chrono::{Duration, Utc};
use reqwest::StatusCode;
use reqwest_retry::{RetryTransientMiddleware, policies::ExponentialBackoff};
use serde::{Serialize, de::DeserializeOwned};
use std::sync::RwLock;
use std::{clone::Clone, string::ToString};

pub use crate::{
    BusinessClient, MerchantClient, OpenBankingClient,
    business::Error,
    client::{
        self, Body, Client, ClientBuilder, Environment, HttpMethod, MissingClientAuthentication,
        MissingEnvironment, ProductionEnvironment, RevolutEndpoint, SandboxEnvironment,
    },
    errors::{self, ClientBuilderError, ClientError},
};

use std::{collections::HashMap, fmt::Debug, marker::PhantomData};

const CLIENT_ASSERTION_TYPE: &str = "urn:ietf:params:oauth:client-assertion-type:jwt-bearer";

pub fn business_client()
-> ClientBuilder<MissingEnvironment, MissingClientAuthentication, BusinessClient> {
    ClientBuilder {
        environment: MissingEnvironment,
        authentication: MissingClientAuthentication,
        client_type: PhantomData,
    }
}

impl Environment for SandboxEnvironment<BusinessClient> {
    fn uri(&self, version: &str, path: &str) -> RevolutEndpoint {
        RevolutEndpoint(format!(
            "https://sandbox-b2b.revolut.com/api/{version}{path}",
        ))
    }

    fn unversioned_uri(&self, _path: &str) -> RevolutEndpoint {
        // The Business API is always versioned.
        unreachable!()
    }
}

impl Environment for ProductionEnvironment<BusinessClient> {
    fn uri(&self, version: &str, path: &str) -> RevolutEndpoint {
        RevolutEndpoint(format!("https://b2b.revolut.com/api/{version}{path}",))
    }

    fn unversioned_uri(&self, _path: &str) -> RevolutEndpoint {
        // The Business API is always versioned.
        unreachable!()
    }
}

pub mod v10 {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize)]
    pub struct ClientAuthenticationResponse {
        pub access_token: String,
        pub token_type: String,
        pub expires_in: u64,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct ClientAuthenticationWithRefreshTokenResponse {
        pub access_token: String,
        pub token_type: String,
        pub expires_in: u64,
        pub refresh_token: String,
    }
}

pub struct MissingClientAssertion;
pub struct MissingAuthorizationCode;
pub struct MissingRefreshToken;

pub trait MissingAuthorizationCodeT {}
impl MissingAuthorizationCodeT for MissingAuthorizationCode {}
pub trait MissingRefreshTokenT {}
impl MissingRefreshTokenT for MissingRefreshToken {}

pub struct BusinessAuthenticationBuilder<A, C, R> {
    client_assertion: A,
    authorization_code: C,
    refresh_token: R,
}

impl Default
    for BusinessAuthenticationBuilder<
        MissingClientAssertion,
        MissingAuthorizationCode,
        MissingRefreshToken,
    >
{
    fn default() -> Self {
        BusinessAuthenticationBuilder {
            client_assertion: MissingClientAssertion,
            authorization_code: MissingAuthorizationCode,
            refresh_token: MissingRefreshToken,
        }
    }
}

impl<C, R> BusinessAuthenticationBuilder<MissingClientAssertion, C, R> {
    #[cfg(test)]
    pub fn with_dummy_client_assertion(self) -> BusinessAuthenticationBuilder<(), C, R> {
        BusinessAuthenticationBuilder {
            client_assertion: (),
            authorization_code: self.authorization_code,
            refresh_token: self.refresh_token,
        }
    }

    pub fn with_environment_inherited_client_assertion(
        self,
        client_assertion_environment_variable: &str,
    ) -> Result<BusinessAuthenticationBuilder<String, C, R>, ClientBuilderError> {
        let client_assertion =
            std::env::var(client_assertion_environment_variable).map_err(|_| {
                ClientBuilderError::MissingEnvironmentVariable(
                    client_assertion_environment_variable.into(),
                )
            })?;
        if client_assertion.is_empty() {
            return Err(ClientBuilderError::InvalidSecret);
        }
        Ok(BusinessAuthenticationBuilder {
            client_assertion,
            authorization_code: self.authorization_code,
            refresh_token: self.refresh_token,
        })
    }

    pub fn with_client_assertion(
        self,
        client_assertion: impl ToString,
    ) -> Result<BusinessAuthenticationBuilder<String, C, R>, ClientBuilderError> {
        let client_assertion = client_assertion.to_string();
        if client_assertion.is_empty() {
            return Err(ClientBuilderError::InvalidSecret);
        }
        Ok(BusinessAuthenticationBuilder {
            client_assertion,
            authorization_code: self.authorization_code,
            refresh_token: self.refresh_token,
        })
    }
}

impl<A, R> BusinessAuthenticationBuilder<A, MissingAuthorizationCode, R> {
    #[cfg(test)]
    pub fn with_dummy_authorization_code(self) -> BusinessAuthenticationBuilder<A, (), R> {
        BusinessAuthenticationBuilder {
            client_assertion: self.client_assertion,
            authorization_code: (),
            refresh_token: self.refresh_token,
        }
    }

    pub fn with_environment_inherited_authorization_code(
        self,
        authorization_code_environment_variable: &str,
    ) -> Result<BusinessAuthenticationBuilder<A, String, R>, ClientBuilderError> {
        let authorization_code =
            std::env::var(authorization_code_environment_variable).map_err(|_| {
                ClientBuilderError::MissingEnvironmentVariable(
                    authorization_code_environment_variable.into(),
                )
            })?;
        if authorization_code.is_empty() {
            return Err(ClientBuilderError::InvalidSecret);
        }
        Ok(BusinessAuthenticationBuilder {
            client_assertion: self.client_assertion,
            authorization_code,
            refresh_token: self.refresh_token,
        })
    }

    pub fn with_authorization_code(
        self,
        authorization_code: impl ToString,
    ) -> Result<BusinessAuthenticationBuilder<A, String, R>, ClientBuilderError> {
        let authorization_code = authorization_code.to_string();
        if authorization_code.is_empty() {
            return Err(ClientBuilderError::InvalidSecret);
        }
        Ok(BusinessAuthenticationBuilder {
            client_assertion: self.client_assertion,
            authorization_code: authorization_code.to_string(),
            refresh_token: self.refresh_token,
        })
    }
}

impl<A, C> BusinessAuthenticationBuilder<A, C, MissingRefreshToken> {
    #[cfg(test)]
    pub fn with_dummy_refresh_token(self) -> BusinessAuthenticationBuilder<A, C, ()> {
        BusinessAuthenticationBuilder {
            client_assertion: self.client_assertion,
            authorization_code: self.authorization_code,
            refresh_token: (),
        }
    }

    pub fn with_environment_inherited_refresh_token(
        self,
        refresh_token_environment_variable: &str,
    ) -> Result<BusinessAuthenticationBuilder<A, C, String>, ClientBuilderError> {
        let refresh_token = std::env::var(refresh_token_environment_variable).map_err(|_| {
            ClientBuilderError::MissingEnvironmentVariable(
                refresh_token_environment_variable.into(),
            )
        })?;
        if refresh_token.is_empty() {
            return Err(ClientBuilderError::InvalidSecret);
        }
        Ok(BusinessAuthenticationBuilder {
            client_assertion: self.client_assertion,
            authorization_code: self.authorization_code,
            refresh_token,
        })
    }

    pub fn with_refresh_token(
        self,
        refresh_token: impl ToString,
    ) -> Result<BusinessAuthenticationBuilder<A, C, String>, ClientBuilderError> {
        let refresh_token = refresh_token.to_string();
        if refresh_token.is_empty() {
            return Err(ClientBuilderError::InvalidSecret);
        }
        Ok(BusinessAuthenticationBuilder {
            client_assertion: self.client_assertion,
            authorization_code: self.authorization_code,
            refresh_token: refresh_token.to_string(),
        })
    }
}

impl<C: MissingAuthorizationCodeT> BusinessAuthenticationBuilder<String, C, String> {
    pub fn build(self) -> BusinessAuthentication {
        BusinessAuthentication {
            client_assertion: self.client_assertion,
            authorization_code: None,
            refresh_token: Some(self.refresh_token),
            access_token_expires_at: RwLock::new(None),
            access_token: RwLock::new(None),
        }
    }
}

impl<R: MissingRefreshTokenT> BusinessAuthenticationBuilder<String, String, R> {
    pub fn build(self) -> BusinessAuthentication {
        BusinessAuthentication {
            client_assertion: self.client_assertion,
            authorization_code: Some(self.authorization_code),
            refresh_token: None,
            access_token_expires_at: RwLock::new(None),
            access_token: RwLock::new(None),
        }
    }
}

#[cfg(test)]
impl<C: MissingAuthorizationCodeT> BusinessAuthenticationBuilder<(), C, ()> {
    pub fn build(self) -> BusinessAuthentication {
        BusinessAuthentication {
            client_assertion: String::new(),
            authorization_code: None,
            refresh_token: Some(String::new()),
            access_token_expires_at: RwLock::new(None),
            access_token: RwLock::new(None),
        }
    }
}

#[cfg(test)]
impl<R: MissingRefreshTokenT> BusinessAuthenticationBuilder<(), (), R> {
    pub fn build(self) -> BusinessAuthentication {
        BusinessAuthentication {
            client_assertion: String::new(),
            authorization_code: Some(String::new()),
            refresh_token: None,
            access_token_expires_at: RwLock::new(None),
            access_token: RwLock::new(None),
        }
    }
}

#[derive(Debug)]
pub struct BusinessAuthentication {
    pub client_assertion: String,
    pub authorization_code: Option<String>,
    pub refresh_token: Option<String>,
    pub access_token_expires_at: RwLock<Option<chrono::DateTime<chrono::Utc>>>,
    pub access_token: RwLock<Option<String>>,
}

impl<E> ClientBuilder<E, MissingClientAuthentication, BusinessClient> {
    pub fn with_authentication(
        self,
        authentication: BusinessAuthentication,
    ) -> ClientBuilder<E, BusinessAuthentication, BusinessClient> {
        ClientBuilder {
            environment: self.environment,
            authentication,
            client_type: self.client_type,
        }
    }
}

impl<E: Environment> Client<E, BusinessAuthentication> {
    async fn token_with_params<R: DeserializeOwned + Debug>(
        &self,
        params: HashMap<String, String>,
    ) -> Result<R, Error> {
        let res = self
            .client
            .post(&self.environment.uri("1.0", "/auth/token").0)
            .form(&params)
            .send()
            .await
            .map_err(|err| {
                Error::ClientError(Box::new(errors::ClientError::CannotLogIn(format!(
                    "{err:?}"
                ))))
            })?;

        if res.status().is_client_error() {
            return Err(Error::ClientError(Box::new(ClientError::HttpStatus(
                res.status().as_u16(),
            ))));
        }

        res.json().await.map_err(|err| {
            Error::ClientError(Box::new(errors::ClientError::CannotLogIn(format!(
                "{err:?}"
            ))))
        })
    }

    async fn ensure_logged_in(&self) -> Result<(), Error> {
        if let Some(access_token_expires_at) = &*self
            .authentication
            .access_token_expires_at
            .read()
            .map_err(|err| {
                Error::ClientError(Box::new(errors::ClientError::CannotLogIn(format!(
                    "{err:?}"
                ))))
            })?
            && access_token_expires_at.to_utc() > Utc::now()
        {
            return Ok(());
        }
        self.login().await
    }

    async fn request_raw_<T: Serialize + Clone>(
        &self,
        method: HttpMethod<'_, T>,
        uri: &RevolutEndpoint,
    ) -> Result<Vec<u8>, Error> {
        self.ensure_logged_in().await?;

        let Some(access_token) = (*self.authentication.access_token.read().map_err(|err| {
            Error::ClientError(Box::new(errors::ClientError::CannotLogIn(format!(
                "{err:?}"
            ))))
        })?)
        .clone() else {
            return Err(Error::ClientError(Box::new(
                errors::ClientError::CannotLogIn("could not retrieve access token".to_string()),
            )));
        };

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

        let res = request
            .header("Authorization", format!("Bearer {access_token}"))
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|err| {
                Error::ClientError(Box::new(errors::ClientError::RequestError(format!(
                    "{err:?}"
                ))))
            })?;

        if res.status().is_client_error() {
            return Err(Error::ClientError(Box::new(ClientError::HttpStatus(
                res.status().as_u16(),
            ))));
        }

        Ok(res
            .bytes()
            .await
            .map_err(|err| {
                Error::ClientError(Box::new(errors::ClientError::RequestError(format!(
                    "{err:?}"
                ))))
            })?
            .to_vec())
    }

    #[cfg(not(test))]
    pub(crate) async fn request_raw<T: Serialize + Clone>(
        &self,
        method: HttpMethod<'_, T>,
        uri: &RevolutEndpoint,
    ) -> Result<Vec<u8>, Error> {
        self.request_raw_(method, uri).await
    }

    #[cfg(test)]
    pub(crate) async fn request_raw<T: Serialize + Clone + std::default::Default>(
        &self,
        method: HttpMethod<'_, T>,
        uri: &RevolutEndpoint,
    ) -> Result<Vec<u8>, Error> {
        if self.authentication.client_assertion.is_empty() {
            return Ok(Default::default());
        }

        self.request_raw_(method, uri).await
    }

    async fn request_<R: DeserializeOwned + Debug, T: Serialize + Clone>(
        &self,
        method: HttpMethod<'_, T>,
        uri: &RevolutEndpoint,
    ) -> Result<R, Error> {
        self.ensure_logged_in().await?;

        let Some(access_token) = (*self.authentication.access_token.read().map_err(|err| {
            Error::ClientError(Box::new(errors::ClientError::CannotLogIn(format!(
                "{err:?}"
            ))))
        })?)
        .clone() else {
            return Err(Error::ClientError(Box::new(
                errors::ClientError::CannotLogIn("could not retrieve access token".to_string()),
            )));
        };

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
            .header("Authorization", format!("Bearer {access_token}"))
            .header("Accept", "application/json")
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
    pub(crate) async fn request<R: DeserializeOwned + Debug, T: Serialize + Clone>(
        &self,
        method: HttpMethod<'_, T>,
        uri: &RevolutEndpoint,
    ) -> Result<R, Error> {
        self.request_(method, uri).await
    }

    #[cfg(test)]
    pub(crate) async fn request<
        R: DeserializeOwned + Debug + std::default::Default,
        T: Serialize + Clone,
    >(
        &self,
        method: HttpMethod<'_, T>,
        uri: &RevolutEndpoint,
    ) -> Result<R, Error> {
        if self.authentication.client_assertion.is_empty() {
            return Ok(Default::default());
        }

        self.request_(method, uri).await
    }

    pub async fn login_with_authorization_code(
        &self,
    ) -> Result<v10::ClientAuthenticationWithRefreshTokenResponse, Error> {
        let BusinessAuthentication {
            ref client_assertion,
            authorization_code: Some(ref authorization_code),
            ..
        } = self.authentication
        else {
            return Err(Error::ClientError(Box::new(
                errors::ClientError::CannotLogIn(String::from("missing authorization code")),
            )));
        };

        let mut params = HashMap::new();
        params.insert("grant_type".to_string(), "authorization_code".to_string());
        params.insert("code".to_string(), authorization_code.to_string());
        params.insert(
            "client_assertion_type".to_string(),
            CLIENT_ASSERTION_TYPE.to_string(),
        );
        params.insert("client_assertion".to_string(), client_assertion.to_string());

        self.token_with_params(params).await
    }

    pub async fn login_with_refresh_token(
        &self,
    ) -> Result<v10::ClientAuthenticationResponse, Error> {
        let BusinessAuthentication {
            ref client_assertion,
            refresh_token: Some(ref refresh_token),
            ..
        } = self.authentication
        else {
            return Err(Error::ClientError(Box::new(
                errors::ClientError::CannotLogIn(String::from("missing refresh token")),
            )));
        };

        let mut params = HashMap::new();
        params.insert("grant_type".to_string(), "refresh_token".to_string());
        params.insert("refresh_token".to_string(), refresh_token.to_string());
        params.insert(
            "client_assertion_type".to_string(),
            CLIENT_ASSERTION_TYPE.to_string(),
        );
        params.insert("client_assertion".to_string(), client_assertion.to_string());

        self.token_with_params(params).await
    }

    async fn login(&self) -> Result<(), Error> {
        let authentication = self.login_with_refresh_token().await?;

        *self.authentication.access_token.write().map_err(|err| {
            Error::ClientError(Box::new(errors::ClientError::CannotLogIn(format!(
                "{err:?}"
            ))))
        })? = Some(authentication.access_token);
        *self
            .authentication
            .access_token_expires_at
            .write()
            .map_err(|err| {
                Error::ClientError(Box::new(errors::ClientError::CannotLogIn(format!(
                    "{err:?}"
                ))))
            })? = Some(
            Utc::now()
                + Duration::seconds(authentication.expires_in.try_into().map_err(|err| {
                    Error::ClientError(Box::new(errors::ClientError::CannotLogIn(format!(
                        "{err:?}"
                    ))))
                })?),
        );

        Ok(())
    }
}

impl<E: Environment, C> ClientBuilder<E, BusinessAuthentication, C> {
    pub fn build(self) -> Result<Client<E, BusinessAuthentication>, ClientBuilderError> {
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
