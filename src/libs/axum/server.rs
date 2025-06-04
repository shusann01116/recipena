use std::sync::Arc;

use anyhow::Context;
use async_trait::async_trait;
use axum::{
    Router,
    extract::State,
    response::{IntoResponse, Response},
};
use http::StatusCode;
use line_bot_sdk_rust::line_webhook::{self, models::CallbackRequest};
use tokio::{
    net::TcpListener,
    task::{self, JoinHandle},
};

use crate::{
    Server,
    app::{
        echo::{EchoRequest, EchoService},
        recipe::{InsertRecipeRequest, RecipeService},
    },
    config::AppConfig,
    libs::{
        line::client::LineClientImpl,
        notion::{client::NotionClient, recipe::RecipeRepositoryImpl},
        reqwest::ReqwestClient,
    },
    prelude::*,
};

use super::line::middleware::verify_line_signature;

pub struct HttpServer {
    app_state: Arc<AppState>,
}

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    echo_service: EchoService,
    recipe_service: RecipeService,
}

impl HttpServer {
    pub fn new(config: AppConfig) -> impl Server {
        let line_client = LineClientImpl::new(config.line_channel_access_token.clone());
        let notion_client =
            NotionClient::from_api_key(config.notion_integration_token.clone()).unwrap();
        let recipe_repository =
            RecipeRepositoryImpl::new(Arc::new(notion_client), config.notion_database_id.clone());

        let app_state = Arc::new(AppState {
            config,
            echo_service: EchoService::new(Arc::new(line_client)),
            recipe_service: RecipeService::new(
                Arc::new(recipe_repository),
                Arc::new(ReqwestClient::default()),
            ),
        });
        Self { app_state }
    }

    async fn post_callback(
        State(state): State<Arc<AppState>>,
        request: axum::Json<CallbackRequest>,
    ) -> std::result::Result<Response, Error> {
        let destination = &request.destination;
        let events = &request.events;
        tracing::info!(?destination, ?events, "getting request");

        for e in request.events.iter() {
            let e = e.clone();

            let echo_service = state.echo_service.clone();
            let recipe_service = state.recipe_service.clone();

            let handle: JoinHandle<Result<()>> = task::spawn(async move {
                match e {
                    line_webhook::models::Event::MessageEvent(message_event) => {
                        let (reply_token, message) = extract_message(&message_event)
                            .ok_or(anyhow::anyhow!("failed to extract message"))?;
                        let echo_request = EchoRequest {
                            reply_token,
                            message: message.clone(),
                        };
                        echo_service.echo(echo_request).await?;

                        let recipe_request = InsertRecipeRequest {
                            recipe_url: message.clone(),
                        };
                        recipe_service.insert_recipe(recipe_request).await?;

                        Ok(())
                    }
                    _ => Ok(()),
                }
            });
            handle.await.with_context(|| "failed to handle event")??;
        }

        Ok(().into_response())
    }
}

fn extract_message(message_event: &line_webhook::models::MessageEvent) -> Option<(String, String)> {
    let reply_token = message_event.reply_token.clone()?;
    let message = match message_event.message.as_ref() {
        line_webhook::models::MessageContent::TextMessageContent(text_message_content) => {
            Some(text_message_content.text.clone())
        }
        _ => None,
    }?;
    Some((reply_token, message))
}

#[async_trait]
impl crate::infra::server::Server for HttpServer {
    async fn run(&self) -> Result<()> {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", self.app_state.config.port))
            .await
            .with_context(|| format!("failed to bind to port {}", self.app_state.config.port))?;
        tracing::info!(
            "listening on {}",
            listener
                .local_addr()
                .with_context(|| "failed to bind local address")?
        );

        let app = Router::new()
            .layer(axum::middleware::from_fn_with_state(
                self.app_state.clone(),
                verify_line_signature,
            ))
            .layer(TraceLayer::new_for_http())
            .route("/", axum::routing::post(Self::post_callback))
            .with_state(self.app_state.clone());

        axum::serve(listener, app).await?;
        Ok(())
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        tracing::error!(%self, "failed to handle request");
        (StatusCode::INTERNAL_SERVER_ERROR, "something went wrong").into_response()
    }
}
