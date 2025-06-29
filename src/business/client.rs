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
use serde::de::DeserializeOwned;
use std::{cell::RefCell, string::ToString};

pub use crate::{
    client::{
        self, Client, ClientBuilder, Environment, HttpMethod, MissingClientAuthentication,
        MissingEnvironment, ProductionEnvironment, RevolutEndpoint, SandboxEnvironment,
    },
    errors::{self, ClientBuilderError, Error, Result},
    BusinessClient, MerchantClient, OpenBankingClient,
};

use std::{collections::HashMap, fmt::Debug, marker::PhantomData};

const CLIENT_ASSERTION_TYPE: &str = "urn:ietf:params:oauth:client-assertion-type:jwt-bearer";

pub fn business_client(
) -> ClientBuilder<MissingEnvironment, MissingClientAuthentication, BusinessClient> {
    ClientBuilder {
        environment: MissingEnvironment,
        authentication: MissingClientAuthentication,
        client_type: PhantomData,
    }
}

pub mod v10 {
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    pub struct ClientAuthenticationResponse {
        pub access_token: String,
        pub token_type: String,
        pub expires_in: i64,
    }

    #[derive(Debug, Deserialize)]
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
    pub fn with_environment_inherited_client_assertion(
        self,
        client_assertion_environment_variable: &str,
    ) -> Result<BusinessAuthenticationBuilder<String, C, R>> {
        let client_assertion =
            std::env::var(client_assertion_environment_variable).map_err(|_| {
                Error::ClientBuilderError(ClientBuilderError::MissingEnvironmentVariable(
                    client_assertion_environment_variable.into(),
                ))
            })?;
        Ok(BusinessAuthenticationBuilder {
            client_assertion,
            authorization_code: self.authorization_code,
            refresh_token: self.refresh_token,
        })
    }

    pub fn with_client_assertion(
        self,
        client_assertion: impl ToString,
    ) -> BusinessAuthenticationBuilder<String, C, R> {
        BusinessAuthenticationBuilder {
            client_assertion: client_assertion.to_string(),
            authorization_code: self.authorization_code,
            refresh_token: self.refresh_token,
        }
    }
}

impl<A, R> BusinessAuthenticationBuilder<A, MissingAuthorizationCode, R> {
    pub fn with_environment_inherited_authorization_code(
        self,
        authorization_code_environment_variable: &str,
    ) -> Result<BusinessAuthenticationBuilder<A, String, R>> {
        let authorization_code =
            std::env::var(authorization_code_environment_variable).map_err(|_| {
                Error::ClientBuilderError(ClientBuilderError::MissingEnvironmentVariable(
                    authorization_code_environment_variable.into(),
                ))
            })?;
        Ok(BusinessAuthenticationBuilder {
            client_assertion: self.client_assertion,
            authorization_code,
            refresh_token: self.refresh_token,
        })
    }

    pub fn with_authorization_code(
        self,
        authorization_code: impl ToString,
    ) -> BusinessAuthenticationBuilder<A, String, R> {
        BusinessAuthenticationBuilder {
            client_assertion: self.client_assertion,
            authorization_code: authorization_code.to_string(),
            refresh_token: self.refresh_token,
        }
    }
}

impl<A, C> BusinessAuthenticationBuilder<A, C, MissingRefreshToken> {
    pub fn with_environment_inherited_refresh_token(
        self,
        refresh_token_environment_variable: &str,
    ) -> Result<BusinessAuthenticationBuilder<A, C, String>> {
        let refresh_token = std::env::var(refresh_token_environment_variable).map_err(|_| {
            Error::ClientBuilderError(ClientBuilderError::MissingEnvironmentVariable(
                refresh_token_environment_variable.into(),
            ))
        })?;
        Ok(BusinessAuthenticationBuilder {
            client_assertion: self.client_assertion,
            authorization_code: self.authorization_code,
            refresh_token,
        })
    }

    pub fn with_refresh_token(
        self,
        refresh_token: impl ToString,
    ) -> BusinessAuthenticationBuilder<A, C, String> {
        BusinessAuthenticationBuilder {
            client_assertion: self.client_assertion,
            authorization_code: self.authorization_code,
            refresh_token: refresh_token.to_string(),
        }
    }
}

impl<C: MissingAuthorizationCodeT> BusinessAuthenticationBuilder<String, C, String> {
    pub fn build(self) -> BusinessAuthentication {
        BusinessAuthentication {
            client_assertion: self.client_assertion,
            authorization_code: None,
            refresh_token: Some(self.refresh_token),
        }
    }
}

impl<R: MissingRefreshTokenT> BusinessAuthenticationBuilder<String, String, R> {
    pub fn build(self) -> BusinessAuthentication {
        BusinessAuthentication {
            client_assertion: self.client_assertion,
            authorization_code: Some(self.authorization_code),
            refresh_token: None,
        }
    }
}

#[derive(Debug)]
pub struct BusinessAuthentication {
    pub client_assertion: String,
    pub authorization_code: Option<String>,
    pub refresh_token: Option<String>,
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
    ) -> Result<R> {
        self.client
            .post(&self.environment.uri("1.0", "/auth/token").0)
            .form(&params)
            .send()
            .await
            .map_err(|err| {
                errors::Error::ClientError(errors::ClientError::CannotLogIn(format!("{:?}", err)))
            })?
            .json()
            .await
            .map_err(|err| {
                errors::Error::ClientError(errors::ClientError::CannotLogIn(format!("{:?}", err)))
            })
    }

    async fn ensure_logged_in(&self) -> Result<()> {
        if let Some(access_token_expires_at) = &*self.access_token_expires_at.borrow() {
            if access_token_expires_at.to_utc() > Utc::now() {
                return Ok(());
            }
        }
        self.login().await
    }

    pub(crate) async fn request_raw<'a>(
        &self,
        method: HttpMethod<'a>,
        uri: &RevolutEndpoint,
    ) -> Result<Vec<u8>> {
        self.ensure_logged_in().await?;

        let Some(access_token) = (*self.access_token.borrow()).clone() else {
            return Err(errors::Error::ClientError(
                errors::ClientError::CannotLogIn("could not retrieve access token".to_string()),
            ));
        };

        let request = match method {
            HttpMethod::Get | HttpMethod::Delete => {
                if method == HttpMethod::Get {
                    self.client.get(Into::<&str>::into(uri))
                } else {
                    self.client.delete(Into::<&str>::into(uri))
                }
            }
            HttpMethod::Post { body } | HttpMethod::Patch { body } | HttpMethod::Put { body } => {
                if method == (HttpMethod::Post { body }) {
                    self.client.post(Into::<&str>::into(uri))
                } else if method == (HttpMethod::Patch { body }) {
                    self.client.patch(Into::<&str>::into(uri))
                } else {
                    self.client.put(Into::<&str>::into(uri))
                }
                .body(body.to_string())
            }
        };

        Ok(request
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await
            .map_err(|err| {
                errors::Error::ClientError(errors::ClientError::RequestError(format!("{:?}", err)))
            })?
            .bytes()
            .await
            .map_err(|err| {
                errors::Error::ClientError(errors::ClientError::RequestError(format!("{:?}", err)))
            })?
            .to_vec())
    }

    pub(crate) async fn request<'a, R: DeserializeOwned + Debug>(
        &self,
        method: HttpMethod<'a>,
        uri: &RevolutEndpoint,
    ) -> Result<R> {
        self.ensure_logged_in().await?;

        let Some(access_token) = (*self.access_token.borrow()).clone() else {
            return Err(errors::Error::ClientError(
                errors::ClientError::CannotLogIn("could not retrieve access token".to_string()),
            ));
        };

        let request = match method {
            HttpMethod::Get | HttpMethod::Delete => {
                if method == HttpMethod::Get {
                    self.client.get(Into::<&str>::into(uri))
                } else {
                    self.client.delete(Into::<&str>::into(uri))
                }
            }
            HttpMethod::Post { body } | HttpMethod::Patch { body } | HttpMethod::Put { body } => {
                if method == (HttpMethod::Post { body }) {
                    self.client.post(Into::<&str>::into(uri))
                } else if method == (HttpMethod::Patch { body }) {
                    self.client.patch(Into::<&str>::into(uri))
                } else {
                    self.client.put(Into::<&str>::into(uri))
                }
                .body(body.to_string())
            }
        };

        let response = request
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await
            .map_err(|err| {
                errors::Error::ClientError(errors::ClientError::RequestError(format!("{:?}", err)))
            })?;

        let response_ = format!("{:?}", response);

        response.json().await.map_err(|err| {
            errors::Error::ClientError(errors::ClientError::RequestError(format!(
                "{:?}: {}",
                err, response_,
            )))
        })
    }

    pub async fn login_with_authorization_code(
        &self,
    ) -> Result<v10::ClientAuthenticationWithRefreshTokenResponse> {
        let BusinessAuthentication {
            ref client_assertion,
            authorization_code: Some(ref authorization_code),
            ..
        } = self.authentication
        else {
            return Err(errors::Error::ClientError(
                errors::ClientError::CannotLogIn(String::from("missing authorization code")),
            ));
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

    pub async fn login_with_refresh_token(&self) -> Result<v10::ClientAuthenticationResponse> {
        let BusinessAuthentication {
            ref client_assertion,
            refresh_token: Some(ref refresh_token),
            ..
        } = self.authentication
        else {
            return Err(errors::Error::ClientError(
                errors::ClientError::CannotLogIn(String::from("missing refresh token")),
            ));
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

    async fn login(&self) -> Result<()> {
        let authentication = self.login_with_refresh_token().await?;

        *self.access_token.borrow_mut() = Some(authentication.access_token);
        *self.access_token_expires_at.borrow_mut() =
            Some(Utc::now() + Duration::seconds(authentication.expires_in));

        Ok(())
    }
}

impl<E, C> ClientBuilder<E, BusinessAuthentication, C> {
    pub fn build(self) -> Result<Client<E, BusinessAuthentication>> {
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
