mod server_config;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use server_config::ServerConfig;

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server_config =
        ServerConfig::load("config.yml").expect("Failed to load server configuration");

    let server = HttpServer::new(|| App::new().configure(create_app_config))
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
