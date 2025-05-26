#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("generic error: {0}")]
    Generic(String),
    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("hmac error: {0}")]
    HmacError(#[from] hmac::digest::InvalidLength),
    #[error("config error: {0}")]
    ConfigError(#[from] config::ConfigError),
    #[error("anyhow error: {0}")]
    AnyhowError(#[from] anyhow::Error),
}
