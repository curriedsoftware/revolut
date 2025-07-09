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

//! [Business team members API](https://developer.revolut.com/docs/business/team-members).
//!
//! [^note]: This feature is **not** available in the sandbox
//! environment. Trying to use such a feature using a sandbox client
//! will result in an error at compile time.

use crate::{
    business::client::{self, BusinessAuthentication, Environment, HttpMethod},
    client::{Body, Client, ProductionEnvironment},
    errors::ApiResult,
};

pub mod v10 {
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Default)]
    pub struct ListParams {
        pub created_before: Option<String>,
        pub limit: Option<u64>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct TeamMember {
        pub id: String,
        pub email: String,
        pub first_name: Option<String>,
        pub last_name: Option<String>,
        pub state: TeamMemberState,
        pub role_id: String,
        pub created_at: String,
        pub updated_at: String,
    }

    #[derive(Clone, Debug, Deserialize, strum::Display, Serialize)]
    #[serde(rename_all = "snake_case")]
    #[strum(serialize_all = "snake_case")]
    pub enum TeamMemberState {
        #[serde(alias = "CREATED")]
        Created,
        #[serde(alias = "CONFIRMED")]
        Confirmed,
        #[serde(alias = "WAITING")]
        Waiting,
        #[serde(alias = "ACTIVE")]
        Active,
        #[serde(alias = "LOCKED")]
        Locked,
        #[serde(alias = "DISABLED")]
        Disabled,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct TeamMemberInviteRequest {
        pub email: String,
        pub role_id: String,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct TeamMemberInvite {
        pub email: String,
        pub id: String,
        pub role_id: String,
        pub created_at: String,
        pub updated_at: String,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct TeamRole {
        pub id: String,
        pub name: String,
        pub created_at: String,
        pub updated_at: String,
    }
}

impl std::fmt::Display for v10::ListParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let query = [
            ("created_before", &self.created_before),
            ("limit", &self.limit.map(|limit| limit.to_string())),
        ]
        .iter()
        .fold(String::new(), |acc, (key, value)| {
            if let Some(value) = value {
                let value = urlencoding::encode(value);
                if acc.is_empty() {
                    format!("{acc}?{key}={}", value)
                } else {
                    format!("{acc}&{key}={}", value)
                }
            } else {
                acc
            }
        });
        write!(f, "{query}")
    }
}

pub async fn list(
    client: &Client<ProductionEnvironment<client::BusinessClient>, BusinessAuthentication>,
    list_params: v10::ListParams,
) -> ApiResult<Vec<v10::TeamMember>> {
    client
        .request(
            HttpMethod::<()>::Get,
            &client
                .environment
                .uri("1.0", &format!("/team-members{}", list_params)),
        )
        .await
}

pub async fn invite_new_member(
    client: &Client<ProductionEnvironment<client::BusinessClient>, BusinessAuthentication>,
    member_invite: &v10::TeamMemberInviteRequest,
) -> ApiResult<Vec<v10::TeamMemberInvite>> {
    client
        .request(
            HttpMethod::Post {
                body: Some(Body::Json(member_invite)),
            },
            &client.environment.uri("1.0", "/team-members"),
        )
        .await
}

pub async fn list_team_roles(
    client: &Client<ProductionEnvironment<client::BusinessClient>, BusinessAuthentication>,
    list_params: v10::ListParams,
) -> ApiResult<Vec<v10::TeamRole>> {
    client
        .request(
            HttpMethod::<()>::Get,
            &client.environment.uri("1.0", "/roles"),
        )
        .await
}
