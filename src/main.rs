mod endpoints;
mod keycloak;
mod server_config;
mod server_error;

use std::sync::Arc;

use actix_web::{web, App, HttpServer};
use server_config::ServerConfig;

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
    cfg.service(endpoints::health);
    cfg.service(endpoints::login);
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
