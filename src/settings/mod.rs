mod app_env;
mod server;

use app_env::ApplicationEnvironment;
use config::{Config, ConfigError, Environment, File};
use dotenv::dotenv;
use serde::Deserialize;
use server::Server;

#[derive(Debug, Deserialize)]
pub struct Settings {
    server: Server,
}

impl Settings {
    pub fn load() -> Result<Self, ConfigError> {
        let environment = match std::env::var("RUST_ENV") {
            Ok(env) => match env.as_str() {
                "development" => ApplicationEnvironment::Development,
                "production" => ApplicationEnvironment::Production,
                _ => ApplicationEnvironment::Development,
            },
            Err(_) => ApplicationEnvironment::Development,
        };

        dotenv().ok();

        let mut s = Config::default();

        // Start off by merging in the "default" configuration file
        s.merge(File::with_name("config/default.toml"))?;

        // Add in the current environment file
        // Default to 'development' env
        // Note that this file is _optional_
        s.merge(File::with_name(format!("config/{}.toml", environment).as_str()).required(false))?;

        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        s.merge(Environment::with_prefix("app"))?;

        // You may also programmatically change settings
        // s.set("database.url", "postgres://")?;

        // You can deserialize (and thus freeze) the entire configuration as
        s.try_into()
    }
}
