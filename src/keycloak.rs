use actix_web::http::StatusCode;

use crate::{server_config::ServerConfig, server_error::ServerError};

pub struct KeycloakClient {
    realm: String,
    user_client_id: String,
    user_client_secret: String,
    admin_client_id: String,
    admin_client_secret: String,
    base_url: String,
    client: awc::Client,
}

impl KeycloakClient {
    pub fn new(
        realm: &str,
        user_client_id: &str,
        user_client_secret: &str,
        admin_client_id: &str,
        admin_client_secret: &str,
        base_url: &str,
    ) -> Self {
        KeycloakClient {
            realm: realm.to_string(),
            user_client_id: user_client_id.to_string(),
            user_client_secret: user_client_secret.to_string(),
            admin_client_id: admin_client_id.to_string(),
            admin_client_secret: admin_client_secret.to_string(),
            base_url: base_url.to_string(),
            client: awc::Client::default(),
        }
    }

    pub fn from_server_config(config: &ServerConfig) -> Self {
        KeycloakClient::new(
            config.keycloak_realm.as_str(),
            config.keycloak_user_client_id.as_str(),
            config.keycloak_user_client_secret.as_str(),
            config.keycloak_admin_client_id.as_str(),
            config.keycloak_admin_client_secret.as_str(),
            config.keycloak_base_url.as_str(),
        )
    }

    pub async fn get_user_token(
        &self,
        username: &str,
        password: &str,
    ) -> Result<String, ServerError> {
        let url = self.get_full_url("protocol/openid-connect/token");
        let params = [
            ("grant_type", "password"),
            ("client_id", self.user_client_id.as_str()),
            ("client_secret", self.user_client_secret.as_str()),
            ("username", username),
            ("password", password),
        ];
        let mut resp = self
            .client
            .post(url)
            .send_form(&params)
            .await
            .map_err(|err| ServerError::fail_to_communicate_with_keycloak(&err.to_string()))?;

        if resp.status().eq(&StatusCode::UNAUTHORIZED) {
            return Err(ServerError::invalid_user_credentials());
        } else if !resp.status().is_success() {
            return Err(ServerError::fail_to_communicate_with_keycloak(
                format!("Status code: {}", resp.status().as_u16()).as_str(),
            ));
        }

        let token_response = resp
            .json::<GetTokenResponse>()
            .await
            .map_err(|err| ServerError::unable_to_parse_response(&err.to_string()))?;

        Ok(token_response.access_token)
    }

    pub async fn create_user(&self, username: &str, password: &str) -> Result<(), ServerError> {
        let admin_token = self.get_admin_token().await?;
        let url = self.get_full_admin_url("users");

        let body = CreateUserRequest {
            username,
            enabled: true,
            credentials: vec![CreateUserRequestCredentials {
                type_: "password".to_string(),
                value: password,
                temporary: false,
            }],
        };

        let resp = self
            .client
            .post(url)
            .bearer_auth(admin_token)
            .send_json(&body)
            .await
            .map_err(|err| ServerError::fail_to_communicate_with_keycloak(&err.to_string()))?;

        if resp.status().eq(&StatusCode::CONFLICT) {
            return Err(ServerError::user_already_exists());
        } else if !resp.status().is_success() {
            return Err(ServerError::fail_to_communicate_with_keycloak(
                format!("Status code: {}", resp.status().as_u16()).as_str(),
            ));
        }

        Ok(())
    }

    // TODO: We can implement cache and refresh token logic here
    async fn get_admin_token(&self) -> Result<String, ServerError> {
        let url = self.get_full_url("protocol/openid-connect/token");
        let params = [
            ("grant_type", "client_credentials"),
            ("client_id", self.admin_client_id.as_str()),
            ("client_secret", self.admin_client_secret.as_str()),
        ];
        println!("Getting admin token. URL: {}, params: {:?}", url, params);
        let mut resp = self
            .client
            .post(url)
            .send_form(&params)
            .await
            .map_err(|err| ServerError::fail_to_communicate_with_keycloak(&err.to_string()))?;

        if !resp.status().is_success() {
            return Err(ServerError::fail_to_communicate_with_keycloak(
                format!("Status code: {}", resp.status().as_u16()).as_str(),
            ));
        }

        let token_response = resp
            .json::<GetTokenResponse>()
            .await
            .map_err(|err| ServerError::unable_to_parse_response(&err.to_string()))?;

        Ok(token_response.access_token)
    }

    pub fn get_full_url(&self, path: &str) -> String {
        format!("{}/realms/{}/{}", self.base_url, self.realm, path)
    }

    pub fn get_full_admin_url(&self, path: &str) -> String {
        format!("{}/admin/realms/{}/{}", self.base_url, self.realm, path)
    }
}

#[derive(serde::Serialize)]
struct CreateUserRequest<'a> {
    username: &'a str,
    enabled: bool,
    credentials: Vec<CreateUserRequestCredentials<'a>>,
}

#[derive(serde::Serialize)]
struct CreateUserRequestCredentials<'a> {
    #[serde(rename = "type")]
    type_: String,
    value: &'a str,
    temporary: bool,
}

#[allow(dead_code)]
#[derive(serde::Deserialize)]
struct GetTokenResponse {
    access_token: String,
    expires_in: u64,
    refresh_expires_in: u64,
    refresh_token: Option<String>,
    token_type: String,
    #[serde(rename = "not-before-policy")]
    not_before_policy: u64,
    session_state: Option<String>,
    scope: String,
}
