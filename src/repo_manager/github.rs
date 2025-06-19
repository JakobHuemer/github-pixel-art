use std::fmt::Display;

use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::json;

const GITHUB_CLIENT_ID: &str = "Ov23liPmBme7kuQMJu3Q";
const GITHUB_SCOPES: &str = "repo";

#[derive(Debug, Serialize, Deserialize)]
pub enum GitHubAuthMethod {
    DeviceAuth { access_token: String },
    PersonalAccessToken { token: String },
    Unauthenticated,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitHub {
    username: String,
    auth: GitHubAuthMethod,
}

impl GitHub {
    pub fn new(username: String, auth: GitHubAuthMethod) -> GitHub {
        GitHub { username, auth }
    }
}

#[derive(Debug)]
pub struct GitHubAuthBuilder {}

#[derive(Debug)]
pub enum GitHubAuthError {
    DeviceCodeExpired,
    DeviceCodeInvalid,
    PATInvalid,
    Unknown(u8),
}

impl GitHubAuthBuilder {
    pub fn new() -> GitHubAuthBuilder {
        GitHubAuthBuilder {}
    }

    pub async fn with_pat(&self, token: String) -> Result<GitHubAuthMethod, GitHubAuthError> {
        let client = Client::new();

        let res = client
            .get("https://api.github.com/user")
            .header("Authorization", format!("Bearer {}", &token))
            .header("User-Agent", "rust-client")
            .send()
            .await
            .map_err(|_e| GitHubAuthError::PATInvalid)?;

        match res.status() == StatusCode::OK {
            true => Ok(GitHubAuthMethod::PersonalAccessToken { token: token }),
            false => Err(GitHubAuthError::Unknown(192)),
        }
    }

    pub async fn with_device(&self) -> Result<GitHubDeviceCodeBuilder, GitHubAuthError> {
        Ok(GitHubDeviceCodeBuilder::new().await?)
    }
}

// Device Code Builder

#[derive(Serialize, Deserialize, Debug)]
pub struct GitHubDeviceCodeBuilder {
    pub device_code: String,
    pub user_code: String,
    pub verification_uri: String,
    pub expires_in: u32,
    pub interval: u32,
    pub expiration_date: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct PollResponse {
    pub access_token: String,
    pub token_type: String,
    pub scope: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PollErrorResponse {
    pub error: GitHubDeviceCodePollerror,
    pub error_description: Option<String>,
    pub error_uri: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum GitHubDeviceCodePollResponse {
    Success(PollResponse),
    Error(PollErrorResponse),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum GitHubDeviceCodePollerror {
    #[serde(alias = "authorization_pending")]
    AuthorizationPending,
    #[serde(alias = "slow_down")]
    SlowDown,
    #[serde(alias = "expired_token")]
    ExpiredToken,
    #[serde(alias = "invalid_grant")]
    UnsupportedGrantType,
    #[serde(alias = "invalid_client")]
    IncorrectClientCredentials,
    #[serde(alias = "invalid_request")]
    IncorrectDeviceCode,
    #[serde(alias = "access_denied")]
    AccessDenied,
    #[serde(alias = "device_flow_disabled")]
    DeviceFlowDisabled,
}

impl GitHubDeviceCodeBuilder {
    pub async fn new() -> Result<GitHubDeviceCodeBuilder, GitHubAuthError> {
        let client = Client::new();

        let res = client
            .post("https://github.com/login/device/code")
            .header("User-Agent", "rust-client")
            .header("Accept", "application/json")
            .json(&json!({
                "client_id": GITHUB_CLIENT_ID,
                "scope": GITHUB_SCOPES
            }))
            .send()
            .await
            .map_err(|_e| GitHubAuthError::Unknown(101))?;

        if res.status() != StatusCode::OK {
            return Err(GitHubAuthError::Unknown(102));
        }

        let mut code_response = res
            .json::<GitHubDeviceCodeBuilder>()
            .await
            .map_err(|_e| GitHubAuthError::Unknown(103))?;

        code_response.expiration_date =
            Some(chrono::Utc::now() + chrono::Duration::seconds(code_response.expires_in as i64));

        Ok(code_response)
    }

    pub async fn wait_for_user(&mut self) -> Result<GitHubAuthMethod, GitHubAuthError> {
        let client = Client::new();

        while chrono::Utc::now() < self.expiration_date.unwrap() {
            let res = client
                .post("https://github.com/login/oauth/access_token")
                .header("User-Agent", "rust-client")
                .header("Accept", "application/json")
                .json(&json!({
                    "client_id": GITHUB_CLIENT_ID,
                    "device_code": self.device_code,
                    "grant_type": "urn:ietf:params:oauth:grant-type:device_code"
                }))
                .send()
                .await
                .map_err(|_e| GitHubAuthError::Unknown(104))?;

            if res.status() == StatusCode::OK {
                let data: GitHubDeviceCodePollResponse = res
                    .json()
                    .await
                    .map_err(|_x| GitHubAuthError::Unknown(105))?;

                match data {
                    GitHubDeviceCodePollResponse::Success(res) => {
                        println!("Access Token: {}", res.access_token);
                        return Ok(GitHubAuthMethod::DeviceAuth {
                            access_token: res.access_token,
                        });
                    }
                    GitHubDeviceCodePollResponse::Error(e) => {
                        match e.error {
                            GitHubDeviceCodePollerror::AuthorizationPending => {}
                            _ => {
                                return Err(GitHubAuthError::Unknown(109));
                            }
                        };
                    }
                };

                tokio::time::sleep(std::time::Duration::from_secs(self.interval as u64)).await;

                continue;
            }
            // TODO: handle this properly through a nice error
            panic!("The poll response of github responded with an error");
        }

        Err(GitHubAuthError::DeviceCodeExpired)
    }
}
