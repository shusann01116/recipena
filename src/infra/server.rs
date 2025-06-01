use crate::prelude::*;
use async_trait::async_trait;

#[async_trait]
pub trait Server {
    async fn run(&self) -> Result<()>;
}
