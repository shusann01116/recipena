use std::{collections::BTreeMap, sync::Arc};

use anyhow::Context;
use async_trait::async_trait;
use notion_client::{
    endpoints::pages::create::request::CreateAPageRequestBuilder,
    objects::{
        page::PageProperty,
        parent::Parent,
        rich_text::{RichText, Text},
    },
};

use crate::{domain::recipe::Recipe, infra::repository::recipe::RecipeRepository, prelude::*};

use super::client::NotionClient;

#[derive(Clone)]
pub struct RecipeRepositoryImpl {
    notion_client: Arc<NotionClient>,
    db_id: String,
}

impl RecipeRepositoryImpl {
    pub fn new(notion_client: Arc<NotionClient>, db_id: String) -> Self {
        Self {
            notion_client,
            db_id,
        }
    }
}

#[async_trait]
impl RecipeRepository for RecipeRepositoryImpl {
    async fn insert_recipe(&self, recipe: Recipe) -> Result<()> {
        let mut properties = BTreeMap::new();
        properties.insert("Name".to_string(), title_property(recipe.name));
        properties.insert(
            "リンク".to_string(),
            link_property(recipe.recipe_url.to_string()),
        );

        let request = CreateAPageRequestBuilder::default()
            .parent(Parent::DatabaseId {
                database_id: self.db_id.clone(),
            })
            .properties(properties)
            .build()
            .with_context(|| "Failed to build CreateAPageRequestBuilder request")?;

        let _ = self.notion_client.0.pages.create_a_page(request).await?;
        Ok(())
    }

    async fn get_random_recipe(&self) -> Result<Option<Recipe>> {
        use notion_client::endpoints::databases::query::request::QueryDatabaseRequest;
        use rand::seq::SliceRandom;
        use url::Url;

        let request = QueryDatabaseRequest {
            filter: None,
            sorts: None,
            start_cursor: None,
            page_size: None,
        };

        let response = self
            .notion_client
            .0
            .databases
            .query_a_database(&self.db_id, request)
            .await?;

        if response.results.is_empty() {
            return Ok(None);
        }

        let mut rng = rand::thread_rng();
        let random_page = response.results.choose(&mut rng);

        if let Some(page) = random_page {
            let properties = &page.properties;
            let name =
                extract_title_property(properties, "Name").unwrap_or_else(|| "無題".to_string());
            let recipe_url = extract_url_property(properties, "リンク")
                .and_then(|url_str| Url::parse(&url_str).ok())
                .ok_or_else(|| anyhow::anyhow!("Invalid recipe URL"))?;

            return Ok(Some(Recipe::new(name, recipe_url)));
        }

        Ok(None)
    }
}

fn title_property(name: String) -> PageProperty {
    let rich_text = RichText::Text {
        text: Text {
            content: name,
            link: None,
        },
        annotations: None,
        plain_text: None,
        href: None,
    };
    PageProperty::Title {
        title: vec![rich_text],
        id: None,
    }
}

fn link_property(link: String) -> PageProperty {
    PageProperty::Url {
        url: Some(link),
        id: None,
    }
}

fn extract_title_property(
    properties: &std::collections::HashMap<String, PageProperty>,
    key: &str,
) -> Option<String> {
    if let Some(PageProperty::Title { title, .. }) = properties.get(key) {
        title.first().and_then(|rich_text| match rich_text {
            RichText::Text { text, .. } => Some(text.content.clone()),
            _ => None,
        })
    } else {
        None
    }
}

fn extract_url_property(
    properties: &std::collections::HashMap<String, PageProperty>,
    key: &str,
) -> Option<String> {
    if let Some(PageProperty::Url { url, .. }) = properties.get(key) {
        url.clone()
    } else {
        None
    }
}
