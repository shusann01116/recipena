use crate::prelude::*;
use config::Config;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct AppConfig {
    pub debug: bool,
    pub line_channel_secret: String,
    pub notion_integration_token: String,
    pub port: u16,
}

const CONFIG_FILE_NAME: &str = ".recipena";

pub fn load_config() -> Result<AppConfig> {
    Ok(Config::builder()
        .add_source(config::File::with_name(CONFIG_FILE_NAME).required(false))
        .add_source(config::Environment::with_prefix("RECIPENA"))
        .add_source(config::Environment::default())
        .build()?
        .try_deserialize::<AppConfig>()?)
}
