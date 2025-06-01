pub mod app;
pub mod config;
pub mod domain;
pub mod infra;
pub mod libs;
pub mod logger;
pub mod prelude;

pub use infra::server::Server;
pub use libs::axum::server::HttpServer;

pub(crate) mod error;
