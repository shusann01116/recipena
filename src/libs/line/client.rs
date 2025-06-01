use std::sync::Arc;

use crate::{infra::line::LineMessage, prelude::*};
use async_trait::async_trait;
use line_bot_sdk_rust::{
    client::LINE,
    line_messaging_api::{apis::MessagingApiApi, models::ReplyMessageRequest},
};

use crate::infra::line::LineClient;

#[derive(Clone)]
pub struct LineClientImpl {
    client: Arc<LINE>,
}

impl LineClientImpl {
    pub fn new(channel_access_token: String) -> Self {
        let client = LINE::new(channel_access_token);
        Self {
            client: Arc::new(client),
        }
    }
}

#[async_trait]
impl LineClient for LineClientImpl {
    async fn reply_messages(&self, token: &str, message: Vec<LineMessage>) -> Result<()> {
        let request = ReplyMessageRequest {
            reply_token: token.to_string(),
            messages: message.into_iter().map(|m| m.into()).collect(),
            notification_disabled: None,
        };
        self.client
            .messaging_api_client
            .reply_message(request)
            .await
            .map_err(Into::<crate::libs::line::error::LineClientError>::into)?;
        Ok(())
    }
}
