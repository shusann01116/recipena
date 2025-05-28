use std::sync::Arc;

use anyhow::Context;
use axum::{
    Router,
    response::{IntoResponse, Response},
};
use http::StatusCode;
use line_bot_sdk_rust::line_webhook::{self, models::CallbackRequest};
use tokio::{net::TcpListener, task};

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
        let listener = TcpListener::bind(format!("0.0.0.0:{}", self.config.port))
            .await
            .with_context(|| format!("Failed to bind to port {}", self.config.port))?;
        tracing::info!(
            "Listening on {}",
            listener
                .local_addr()
                .with_context(|| "failed to get local address")?
        );
        axum::serve(listener, app).await?;
        Ok(())
    }

    fn app(&self) -> axum::Router {
        let mut routes = self.setup_layer(Router::new());
        routes = self.setup_route(routes);
        routes
    }

    fn setup_layer(&self, mut router: Router) -> Router {
        if !self.config.debug {
            router = router.layer(axum::middleware::from_fn_with_state(
                self.config.clone(),
                crate::line::middleware::verify_line_signature,
            ));
        }
        router
    }

    fn setup_route(&self, mut router: Router) -> Router {
        router = router.route("/", axum::routing::post(Self::post_callback));
        router
    }

    async fn post_callback(
        request: axum::Json<CallbackRequest>,
    ) -> std::result::Result<Response, Error> {
        let destination = &request.destination;
        let events = &request.events;
        tracing::info!(?destination, ?events, "getting request");

        for e in request.events.iter() {
            let e = e.clone();
            task::spawn(async move {
                if let Err(e) = handle_event(&e).await {
                    tracing::error!(?e, "error occurred");
                }
            });
        }

        Ok(().into_response())
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        tracing::error!(%self, "error");
        (StatusCode::INTERNAL_SERVER_ERROR, "something went wrong").into_response()
    }
}

async fn handle_event(event: &line_webhook::models::Event) -> Result<()> {
    match event {
        line_webhook::models::Event::MessageEvent(event) => {
            if let line_webhook::models::MessageContent::TextMessageContent(text) = &*event.message
            {
                tracing::info!(?text, "message");
            }
            Ok(())
        }
        _ => return Ok(()),
    }
}
