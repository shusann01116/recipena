use crate::prelude::*;
use async_trait::async_trait;

#[derive(Debug, Clone)]
pub enum LineMessage {
    Text(String),
}

#[async_trait]
pub trait LineClient {
    async fn reply_messages(&self, token: &str, message: Vec<LineMessage>) -> Result<()>;
}
