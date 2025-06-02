use crate::prelude::*;

pub struct NotionClient(pub notion_client::endpoints::Client);

impl NotionClient {
    pub fn from_api_key(api_key: String) -> Result<Self> {
        let client = notion_client::endpoints::Client::new(api_key, None)?;
        Ok(Self(client))
    }
}
