use async_trait::async_trait;

use crate::prelude::*;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait HtmlClient {
    async fn get_title(&self, url: &str) -> Result<String>;
}
