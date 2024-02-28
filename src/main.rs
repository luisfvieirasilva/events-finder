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

static CONFIG_FILE: &str = "config.yml";

struct WebServerState {
    config: Arc<ServerConfig>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server_config =
        Arc::new(ServerConfig::load(CONFIG_FILE).expect("Failed to load server configuration"));

    claims::initialize();

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

async fn grants_extractor(req: &mut ServiceRequest) -> Result<HashSet<String>, ActixError> {
    let claims = req.extract::<claims::Claims>().await?;

    Ok(HashSet::from_iter(
        claims.realm_access.roles.clone().into_iter(),
    ))
}

fn create_app_config(cfg: &mut web::ServiceConfig) {
    cfg.service(endpoints::health);
    cfg.service(endpoints::login);
    cfg.service(endpoints::users_register);
    cfg.service(
        web::scope("")
            .wrap(GrantsMiddleware::with_extractor(grants_extractor))
            .service(endpoints::whoami),
    );
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
