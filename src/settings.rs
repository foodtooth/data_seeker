use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::env;

#[derive(Deserialize, Debug)]
pub struct Server {
    pub data_seeker: DataSeeker,
}

#[derive(Deserialize, Debug)]
pub struct DataSeeker {
    pub port: u16,
    pub host: String,
}

#[derive(Deserialize, Debug)]
pub struct Log {
    pub level: String,
}

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub server: Server,
    pub log: Log,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let run_env = env::var("RUN_ENV").unwrap_or_else(|_| "development".into());

        Config::builder()
            // add "default" config
            .add_source(File::with_name("config/default"))
            // add current env config file, default to "development" env
            .add_source(File::with_name(&format!("config/{}", run_env)).required(false))
            // add local config file. This file shouldn't be checked in to git
            .add_source(File::with_name("config/local").required(false))
            // add in settings from environment (with a prefix of APP)
            // e.g. `APP_DEBUG=1 ./target/app` would set the `debug` key
            // `APP_SERVER__PORT` will set `server.port`
            .add_source(Environment::with_prefix("app").separator("__"))
            // override settings programmatically
            // .set_override("server.postgres", "postgres://")?
            .build()?
            .try_deserialize()
    }
}
