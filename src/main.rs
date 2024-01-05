mod keycloak;
mod server_config;
mod server_error;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use serde_json::json;
use server_config::ServerConfig;
use std::sync::Arc;

use crate::server_error::ServerError;

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[derive(serde::Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[post("/login")]
async fn login(
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

    Ok(HttpResponse::Ok().json(json!({ "token": token })))
}

struct WebServerState {
    config: Arc<ServerConfig>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server_config =
        Arc::new(ServerConfig::load("config.yml").expect("Failed to load server configuration"));

    let app_data = web::Data::new(WebServerState {
        config: server_config.clone(),
    });

    let server = HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .configure(create_app_config)
    })
    .bind((server_config.address.as_str(), server_config.port))?
    .run();

    println!(
        "Server running at {}:{}",
        server_config.address, server_config.port
    );

    server.await
}

fn create_app_config(cfg: &mut web::ServiceConfig) {
    cfg.service(health);
    cfg.service(login);
}

#[cfg(test)]
mod tests {
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_get_health() {
        let app = test::init_service(App::new().configure(super::create_app_config)).await;
        let req = test::TestRequest::get().uri("/health").to_request();

        let resp = test::call_and_read_body(&app, req).await;

        assert_eq!(resp, "OK");
    }
}
