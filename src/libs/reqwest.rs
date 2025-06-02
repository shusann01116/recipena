use crate::prelude::*;
use anyhow::Context;
use async_trait::async_trait;

use crate::infra::html::HtmlClient;

pub struct ReqwestClient(reqwest::Client);

impl Default for ReqwestClient {
    fn default() -> Self {
        Self::new(reqwest::Client::new())
    }
}

impl ReqwestClient {
    pub fn new(client: reqwest::Client) -> Self {
        Self(client)
    }

    async fn get(&self, url: &str) -> Result<String> {
        Ok(self.0.get(url).send().await?.text().await?)
    }

    fn query_node<'a>(dom: &'a tl::VDom<'_>, selector: &str) -> Result<&'a tl::Node<'a>> {
        let handle = dom
            .query_selector(selector)
            .and_then(|mut m| m.next())
            .with_context(|| format!("no node found for selector: {}", selector))?;
        Ok(handle
            .get(&dom.parser())
            .with_context(|| format!("no node found for selector: {}", selector))?)
    }
}

#[async_trait]
impl HtmlClient for ReqwestClient {
    async fn get_title(&self, url: &str) -> Result<String> {
        let html = self.get(url).await?;

        let dom = tl::parse(&html, tl::ParserOptions::default())?;
        let node = Self::query_node(&dom, "title")?;

        Ok(node.inner_text(&dom.parser()).to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_node() {
        let body = r#""
        <html>
            <head>
                <title>Google</title>
            </head>
        </html>
        "#;
        let dom = tl::parse(body, tl::ParserOptions::default()).unwrap();
        let node = ReqwestClient::query_node(&dom, "title").unwrap();
        assert_eq!(node.inner_text(&dom.parser()), "Google");
    }
}
