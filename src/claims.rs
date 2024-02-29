use std::future::{ready, Ready};
use std::sync::OnceLock;

use actix_web::dev::Payload;
use actix_web::{error::ErrorUnauthorized, FromRequest};
use actix_web::{Error, HttpRequest};
use jsonwebtoken::{self, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

use crate::server_config::ServerConfig;

static DECODING_KEY: OnceLock<DecodingKey> = OnceLock::new();

pub fn initialize(config_file: &str) -> std::io::Result<()> {
    DECODING_KEY
        .set(generate_decoding_key(config_file))
        .map_err(|_| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                "Decoding key already initialized",
            )
        })
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    #[serde(rename = "preferred_username")]
    pub username: String,
    pub exp: i64,
    pub realm_access: ClaimsRealmAccess,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClaimsRealmAccess {
    pub roles: Vec<String>,
}

impl FromRequest for Claims {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        ready(
            req.headers()
                .get(actix_web::http::header::AUTHORIZATION)
                .ok_or_else(|| ErrorUnauthorized("Missing authorization header"))
                .and_then(|h| h.to_str().map_err(|e| ErrorUnauthorized(e.to_string())))
                .and_then(|h| {
                    h.strip_prefix("Bearer ")
                        .ok_or_else(|| ErrorUnauthorized("Invalid authorization header"))
                })
                .and_then(|token| decode_jwt(token).map_err(|e| ErrorUnauthorized(e.to_string()))),
        )
    }
}

fn generate_decoding_key(config_file: &str) -> DecodingKey {
    let server_config =
        ServerConfig::load(config_file).expect("Failed to load server configuration");
    DecodingKey::from_rsa_pem(server_config.keycloak_jwt_public_key.as_bytes())
        .expect("Failed to read JWT public key")
}

pub fn decode_jwt(token: &str) -> Result<Claims, Error> {
    let mut validation = Validation::new(jsonwebtoken::Algorithm::RS256);
    validation.set_audience(&["account".to_string()]);

    let decoding_key = DECODING_KEY
        .get()
        .ok_or_else(|| ErrorUnauthorized("Decoding key not initialized"))?;

    jsonwebtoken::decode::<Claims>(token, decoding_key, &validation)
        .map(|data| data.claims)
        .map_err(|e| ErrorUnauthorized(e.to_string()))
}
