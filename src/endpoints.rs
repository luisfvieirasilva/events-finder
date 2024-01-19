use actix_web::{get, post, web, HttpResponse, Responder, Result};

use crate::{claims::Claims, keycloak, server_error::ServerError, WebServerState};

#[get("/health")]
pub async fn health() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[derive(serde::Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(serde::Serialize)]
struct LoginResponse<'a> {
    token: &'a str,
}

#[post("/login")]
pub async fn login(
    state: web::Data<WebServerState>,
    body: web::Json<LoginRequest>,
) -> Result<HttpResponse, ServerError> {
    let keycloak_client = keycloak::KeycloakClient::new(
        &state.config.keycloak_realm,
        &state.config.keycloak_client_id,
        &state.config.keycloak_client_secret,
        &state.config.keycloak_base_url,
    );

    let token = keycloak_client
        .get_token(&body.username, &body.password)
        .await?;

    let decoded_token = crate::claims::decode_jwt(&token);
    if let Err(e) = &decoded_token {
        return Err(ServerError::unable_to_decode_token(&e.to_string()));
    }

    Ok(HttpResponse::Ok().json(LoginResponse { token: &token }))
}
