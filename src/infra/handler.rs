use std::sync::Arc;

use crate::{
    app::{echo::EchoRequest, recipe::InsertRecipeRequest},
    libs::axum::server::AppState,
    prelude::*,
};
use line_bot_sdk_rust::line_webhook;
use validator::Validate;

pub async fn handle_event(state: Arc<AppState>, e: line_webhook::models::Event) -> Result<()> {
    match e {
        line_webhook::models::Event::MessageEvent(message_event) => {
            let (reply_token, message) = extract_message(&message_event)
                .ok_or(anyhow::anyhow!("failed to extract message"))?;

            let recipe_request = InsertRecipeRequest {
                recipe_url: message.clone(),
                reply_token: reply_token.clone(),
            };

            match recipe_request.validate() {
                Ok(_) => {
                    state.recipe_service.insert_recipe(recipe_request).await?;
                }
                Err(_) => {
                    let echo_request = EchoRequest {
                        reply_token,
                        message: message.clone(),
                    };
                    state.echo_service.echo(echo_request).await?;
                }
            }

            Ok(())
        }
        _ => Ok(()),
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
