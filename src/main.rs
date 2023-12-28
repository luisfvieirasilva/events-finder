mod server_config;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use server_config::ServerConfig;

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server_config =
        ServerConfig::load("config.yml").expect("Failed to load server configuration");

    let server = HttpServer::new(|| App::new().service(health))
        .bind((server_config.address.as_str(), server_config.port))?
        .run();

    println!(
        "Server running at {}:{}",
        server_config.address, server_config.port
    );

    server.await
}
