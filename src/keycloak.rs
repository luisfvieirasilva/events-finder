use actix_web::http::StatusCode;

use crate::server_error::ServerError;

pub struct KeycloakClient {
    realm: String,
    client_id: String,
    client_secret: String,
    base_url: String,
    client: awc::Client,
}

impl KeycloakClient {
    pub fn new(realm: &str, client_id: &str, client_secret: &str, base_url: &str) -> Self {
        KeycloakClient {
            realm: realm.to_string(),
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            base_url: base_url.to_string(),
            client: awc::Client::default(),
        }
    }

    pub async fn get_token(&self, username: &str, password: &str) -> Result<String, ServerError> {
        let url = self.get_full_url("protocol/openid-connect/token");
        let params = [
            ("grant_type", "password"),
            ("client_id", self.client_id.as_str()),
            ("client_secret", self.client_secret.as_str()),
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

    fn get_full_url(&self, path: &str) -> String {
        format!("{}/realms/{}/{}", self.base_url, self.realm, path)
    }
}

#[allow(dead_code)]
#[derive(serde::Deserialize)]
struct GetTokenResponse {
    access_token: String,
    expires_in: u64,
    refresh_expires_in: u64,
    refresh_token: String,
    token_type: String,
    #[serde(rename = "not-before-policy")]
    not_before_policy: u64,
    session_state: String,
    scope: String,
}
