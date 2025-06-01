use crate::infra::line::{LineClient, LineMessage};
use crate::prelude::*;
use std::sync::Arc;
use validator::Validate;

#[derive(Clone)]
pub struct EchoService {
    line_client: Arc<dyn LineClient + Send + Sync>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Validate)]
pub struct EchoRequest {
    #[validate(length(min = 1))]
    pub reply_token: String,
    pub message: String,
}

impl EchoService {
    pub fn new(line_client: Arc<dyn LineClient + Send + Sync>) -> Self {
        Self { line_client }
    }

    pub async fn echo(&self, request: EchoRequest) -> Result<()> {
        request.validate()?;
        self.line_client
            .reply_messages(
                &request.reply_token,
                vec![LineMessage::Text(request.message)],
            )
            .await?;
        Ok(())
    }
}
