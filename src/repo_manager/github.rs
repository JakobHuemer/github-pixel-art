use std::{error::Error, fmt::Display};

use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::json;

const GITHUB_CLIENT_ID: &str = "Ov23liPmBme7kuQMJu3Q";
const GITHUB_SCOPES: &str = "repo";

#[derive(Debug)]
pub enum GitHubAuthMethod {
    DeviceAuth {
        access_token: String,
        expires_at: chrono::DateTime<chrono::Utc>,
    },
    PersonalAccessToken {
        token: String,
    },
    Unauthenticated,
}

#[derive(Debug)]
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
pub struct GitHubAuthBuilder {
    username: String,
}

// errrors

#[derive(Debug)]
pub enum GitHubAuthError {
    DeviceCodeExpired,
    DeviceCodeInvalid,
    PATInvalid,
    Unknown(u8),
}

impl GitHubAuthBuilder {
    pub fn new(username: String) -> GitHubAuthBuilder {
        GitHubAuthBuilder { username: username }
    }

    pub async fn with_pat(&self, token: String) -> Result<GitHub, GitHubAuthError> {
        let client = Client::new();

        let res = client
            .get("https://api.github.com/user")
            .header("Authorization", format!("Bearer {}", &token))
            .header("User-Agent", "rust-client")
            .send()
            .await
            .map_err(|_e| GitHubAuthError::PATInvalid)?;

        match res.status() == StatusCode::OK {
            true => Ok(GitHub {
                username: self.username.clone(),
                auth: GitHubAuthMethod::PersonalAccessToken { token: token },
            }),
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
}

impl Display for GitHubDeviceCodeBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Device Code: {}, User Code: {}, Verification URI: {}",
            self.device_code, self.user_code, self.verification_uri
        )
    }
}

impl GitHubDeviceCodeBuilder {
    pub async fn new() -> Result<GitHubDeviceCodeBuilder, GitHubAuthError> {
        let client = Client::new();

        let params = [("client_id", GITHUB_CLIENT_ID), ("scope", "repo")];

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

        dbg!(&res);

        let code_response = res
            .json::<GitHubDeviceCodeBuilder>()
            .await
            .map_err(|_e| GitHubAuthError::Unknown(103))?;

        dbg!(&code_response);

        Ok(code_response)
    }
}
