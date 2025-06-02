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
