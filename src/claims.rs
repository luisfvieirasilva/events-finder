use lazy_static::lazy_static;
use std::future::{ready, Ready};

use actix_web::dev::Payload;
use actix_web::{error::ErrorUnauthorized, FromRequest};
use actix_web::{Error, HttpRequest};
use jsonwebtoken::{self, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

use crate::server_config::ServerConfig;

lazy_static! {
    static ref DECODING_KEY: DecodingKey = generate_decoding_key();
}

pub fn initialize() {
    lazy_static::initialize(&DECODING_KEY);
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

fn generate_decoding_key() -> DecodingKey {
    let server_config =
        ServerConfig::load(crate::CONFIG_FILE).expect("Failed to load server configuration");
    DecodingKey::from_rsa_pem(server_config.keycloak_jwt_public_key.as_bytes())
        .expect("Failed to read JWT public key")
}

pub fn decode_jwt(token: &str) -> Result<Claims, Error> {
    let mut validation = Validation::new(jsonwebtoken::Algorithm::RS256);
    validation.set_audience(&["account".to_string()]);
    jsonwebtoken::decode::<Claims>(token, &DECODING_KEY, &validation)
        .map(|data| data.claims)
        .map_err(|e| ErrorUnauthorized(e.to_string()))
}
