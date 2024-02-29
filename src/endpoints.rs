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
    let keycloak_client = keycloak::KeycloakClient::from_server_config(&state.config);

    let token = keycloak_client
        .get_user_token(&body.username, &body.password)
        .await?;

    let decoded_token = crate::claims::decode_jwt(&token);
    if let Err(e) = &decoded_token {
        return Err(ServerError::unable_to_decode_token(&e.to_string()));
    }

    Ok(HttpResponse::Ok().json(LoginResponse { token: &token }))
}

#[derive(serde::Deserialize)]
struct UsersRegisterRequest {
    username: String,
    password: String,
}

#[post("/users/register")]
pub async fn users_register(
    state: web::Data<WebServerState>,
    body: web::Json<UsersRegisterRequest>,
) -> Result<HttpResponse, ServerError> {
    let keycloak_client = keycloak::KeycloakClient::from_server_config(&state.config);

    keycloak_client
        .create_user(&body.username, &body.password)
        .await?;

    Ok(HttpResponse::Ok().finish())
}

#[derive(serde::Serialize)]
struct WhoamiResponse<'a> {
    username: &'a str,
}

#[get("/whoami")]
pub async fn whoami(claims: Claims) -> impl Responder {
    HttpResponse::Ok().json(WhoamiResponse {
        username: &claims.username,
    })
}
