mod claims;
mod endpoints;
mod keycloak;
mod server_config;
mod server_error;

use std::collections::HashSet;
use std::sync::Arc;

use actix_web::Error as ActixError;
use actix_web::{dev::ServiceRequest, web, App, HttpServer};
use actix_web_grants::GrantsMiddleware;
use server_config::ServerConfig;

#[derive(Clone)]
pub struct WebServerState {
    config: Arc<ServerConfig>,
}

#[actix_web::main]
pub async fn run(config_file: &str) -> std::io::Result<()> {
    claims::initialize(config_file)?;

    let app_data = create_app_data(config_file);
    let server_config = app_data.config.clone();

    let server = HttpServer::new(move || {
        App::new().configure(|cfg| create_app_config(cfg, app_data.clone()))
    })
    .bind((server_config.address.as_str(), server_config.port))?
    .run();

    println!(
        "Server running at {}:{}",
        server_config.address, server_config.port
    );

    server.await
}

async fn grants_extractor(req: &mut ServiceRequest) -> Result<HashSet<String>, ActixError> {
    let claims = req.extract::<claims::Claims>().await;

    match claims {
        Ok(claims) => Ok(HashSet::from_iter(
            claims.realm_access.roles.clone().into_iter(),
        )),
        Err(_) => Ok(HashSet::new()),
    }
}

pub fn create_app_config(cfg: &mut web::ServiceConfig, app_data: web::Data<WebServerState>) {
    cfg.app_data(app_data.clone());
    cfg.service(endpoints::health);
    cfg.service(endpoints::login);
    cfg.service(endpoints::users_register);
    cfg.service(
        web::scope("")
            .wrap(GrantsMiddleware::with_extractor(grants_extractor))
            .service(endpoints::whoami),
    );
}

pub fn create_app_data(config_file: &str) -> web::Data<WebServerState> {
    let server_config =
        Arc::new(ServerConfig::load(config_file).expect("Failed to load server configuration"));

    web::Data::new(WebServerState {
        config: server_config.clone(),
    })
}
