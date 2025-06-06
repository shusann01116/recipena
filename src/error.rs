use crate::libs::line::error::LineClientError;

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
    #[error("validator error: {0}")]
    ValidatorError(#[from] validator::ValidationErrors),
    #[error("url error: {0}")]
    UrlError(#[from] url::ParseError),
    #[error("line error: {0}")]
    LineError(#[from] LineClientError),
    #[error("notion error: {0}")]
    NotionError(#[from] notion_client::NotionClientError),
    #[error("reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("tl error: {0}")]
    TlError(#[from] tl::ParseError),
    #[error("anyhow error: {0}")]
    AnyhowError(#[from] anyhow::Error),
}
