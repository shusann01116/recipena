use std::sync::Arc;

use anyhow::Context;
use async_trait::async_trait;
use axum::{
    Router,
    extract::State,
    response::{IntoResponse, Response},
};
use http::StatusCode;
use line_bot_sdk_rust::line_webhook::models::CallbackRequest;
use tokio::{
    net::TcpListener,
    task::{self, JoinHandle},
};
use tower_http::trace::TraceLayer;

use crate::{
    app::{echo::EchoService, random_recipe::RandomRecipeService, recipe::RecipeService},
    config::AppConfig,
    infra::handler::handle_event,
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
    pub echo_service: EchoService,
    pub recipe_service: RecipeService,
    pub random_recipe_service: std::sync::Arc<dyn RandomRecipeService + Send + Sync>,
}

impl HttpServer {
    pub fn new(config: AppConfig) -> Self {
        let line_client = LineClientImpl::new(config.line_channel_access_token.clone());
        let notion_client =
            NotionClient::from_api_key(config.notion_integration_token.clone()).unwrap();
        let recipe_repository =
            RecipeRepositoryImpl::new(Arc::new(notion_client), config.notion_database_id.clone());

        let app_state = Arc::new(AppState {
            config,
            echo_service: EchoService::new(Arc::new(line_client.clone())),
            recipe_service: RecipeService::new(
                Arc::new(recipe_repository.clone()),
                Arc::new(line_client.clone()),
                Arc::new(ReqwestClient::default()),
            ),
            random_recipe_service: Arc::new(
                crate::app::random_recipe::RandomRecipeServiceImpl::new(
                    recipe_repository.clone(),
                    Arc::new(line_client) as Arc<dyn crate::infra::line::LineClient + Send + Sync>,
                ),
            ),
        });
        Self { app_state }
    }

    async fn post_callback(
        State(state): State<Arc<AppState>>,
        request: axum::Json<CallbackRequest>,
    ) -> std::result::Result<Response, Error> {
        let mut handles = Vec::new();
        for e in request.events.iter() {
            let e = e.clone();

            let state = state.clone();
            let handle: JoinHandle<Result<()>> =
                task::spawn(async move { handle_event(state, e).await });
            handles.push(handle);
        }

        for handle in handles {
            handle.await.with_context(|| "failed to handle event")??;
        }

        Ok(().into_response())
    }
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
