use std::sync::Arc;

use anyhow::Context;
use axum::{
    Router,
    response::{IntoResponse, Response},
};
use http::StatusCode;
use line_api_webhook::models::{CallbackRequest, Event};
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
                handle_event(&e).await;
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

async fn handle_event(event: &Event) -> Result<()> {
    match event {
        Event::AccountLinkEvent {
            source,
            timestamp,
            mode,
            webhook_event_id,
            delivery_context,
        } => todo!(),
        Event::ActivatedEvent {
            source,
            timestamp,
            mode,
            webhook_event_id,
            delivery_context,
        } => todo!(),
        Event::BeaconEvent {
            source,
            timestamp,
            mode,
            webhook_event_id,
            delivery_context,
        } => todo!(),
        Event::BotResumedEvent {
            source,
            timestamp,
            mode,
            webhook_event_id,
            delivery_context,
        } => todo!(),
        Event::BotSuspendedEvent {
            source,
            timestamp,
            mode,
            webhook_event_id,
            delivery_context,
        } => todo!(),
        Event::DeactivatedEvent {
            source,
            timestamp,
            mode,
            webhook_event_id,
            delivery_context,
        } => todo!(),
        Event::PnpDeliveryCompletionEvent {
            source,
            timestamp,
            mode,
            webhook_event_id,
            delivery_context,
        } => todo!(),
        Event::FollowEvent {
            source,
            timestamp,
            mode,
            webhook_event_id,
            delivery_context,
        } => todo!(),
        Event::JoinEvent {
            source,
            timestamp,
            mode,
            webhook_event_id,
            delivery_context,
        } => todo!(),
        Event::LeaveEvent {
            source,
            timestamp,
            mode,
            webhook_event_id,
            delivery_context,
        } => todo!(),
        Event::MemberJoinedEvent {
            source,
            timestamp,
            mode,
            webhook_event_id,
            delivery_context,
        } => todo!(),
        Event::MemberLeftEvent {
            source,
            timestamp,
            mode,
            webhook_event_id,
            delivery_context,
        } => todo!(),
        Event::MembershipEvent {
            source,
            timestamp,
            mode,
            webhook_event_id,
            delivery_context,
        } => todo!(),
        Event::MessageEvent {
            source,
            timestamp,
            mode,
            webhook_event_id,
            delivery_context,
        } => todo!(),
        Event::ModuleEvent {
            source,
            timestamp,
            mode,
            webhook_event_id,
            delivery_context,
        } => todo!(),
        Event::PostbackEvent {
            source,
            timestamp,
            mode,
            webhook_event_id,
            delivery_context,
        } => todo!(),
        Event::ThingsEvent {
            source,
            timestamp,
            mode,
            webhook_event_id,
            delivery_context,
        } => todo!(),
        Event::UnfollowEvent {
            source,
            timestamp,
            mode,
            webhook_event_id,
            delivery_context,
        } => todo!(),
        Event::UnsendEvent {
            source,
            timestamp,
            mode,
            webhook_event_id,
            delivery_context,
        } => todo!(),
        Event::VideoPlayCompleteEvent {
            source,
            timestamp,
            mode,
            webhook_event_id,
            delivery_context,
        } => todo!(),
    }
}
