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

#[cfg(test)]
mod tests {
    use crate::infra::line::MockLineClient;

    use super::*;

    #[tokio::test]
    async fn test_echo() {
        let mut line_client = MockLineClient::new();
        line_client
            .expect_reply_messages()
            .once()
            .returning(|_, _| Ok(()));

        let echo_service = EchoService::new(Arc::new(line_client));

        let request = EchoRequest {
            reply_token: "reply_token".to_string(),
            message: "message".to_string(),
        };

        let result = echo_service.echo(request).await;
        assert!(result.is_ok());
    }
}
