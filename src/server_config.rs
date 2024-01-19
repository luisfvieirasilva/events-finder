use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub address: String,
    pub port: u16,

    pub keycloak_realm: String,
    pub keycloak_client_id: String,
    pub keycloak_client_secret: String,
    pub keycloak_base_url: String,
    pub keycloak_jwt_public_key: String,
}

impl ServerConfig {
    pub fn load(config_file: &str) -> Result<ServerConfig, ConfigError> {
        let builder = Config::builder()
            .set_default("address", "127.0.0.1")?
            .set_default("port", 8080)?
            .add_source(File::with_name(config_file))
            .add_source(Environment::with_prefix("APP"));

        let settings = builder.build()?;

        // Deserialize the configuration into our struct
        settings.try_deserialize::<ServerConfig>()
    }
}
