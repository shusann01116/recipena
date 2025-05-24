use std::sync::Arc;

use anyhow::Ok;
use axum::Router;
use tokio::net::TcpListener;

use crate::{config::AppConfig, prelude::*};

#[derive(Debug)]
pub struct HttpServer {
    config: Arc<AppConfig>,
}

impl HttpServer {
    pub fn new(config: AppConfig) -> Self {
        Self {
            config: Arc::new(config),
        }
    }

    pub async fn run(&self) -> Result<()> {
        let app = self.app();
        let listener = TcpListener::bind(format!("0.0.0.0:{}", self.config.port)).await?;
        tracing::info!("Listening on {}", listener.local_addr()?);
        axum::serve(listener, app).await?;
        Ok(())
    }

    fn app(&self) -> axum::Router {
        Router::new()
            .route("/", axum::routing::post(|| async { "Hello world!" }))
            .layer(axum::middleware::from_fn_with_state(
                self.config.clone(),
                crate::line::middleware::verify_line_signature,
            ))
    }
}
