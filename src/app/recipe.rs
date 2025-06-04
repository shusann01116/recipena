use anyhow::Context;
use validator::Validate;

use crate::{
    domain::recipe::Recipe,
    infra::{
        html::HtmlClient,
        line::{LineClient, LineMessage},
    },
    prelude::*,
};
use std::sync::Arc;

use crate::infra::repository::recipe::RecipeRepository;

const INSERT_RECIPE_MESSAGE: &str = "レシピを登録したよ✨";

#[derive(Clone)]
pub struct RecipeService {
    recipe_repository: Arc<dyn RecipeRepository + Send + Sync>,
    line_client: Arc<dyn LineClient + Send + Sync>,
    html_client: Arc<dyn HtmlClient + Send + Sync>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Validate)]
pub struct InsertRecipeRequest {
    #[validate(url)]
    pub recipe_url: String,
    #[validate(length(min = 1))]
    pub reply_token: String,
}

impl RecipeService {
    pub fn new(
        recipe_repository: Arc<dyn RecipeRepository + Send + Sync>,
        line_client: Arc<dyn LineClient + Send + Sync>,
        html_client: Arc<dyn HtmlClient + Send + Sync>,
    ) -> Self {
        Self {
            recipe_repository,
            line_client,
            html_client,
        }
    }

    pub async fn insert_recipe(&self, insert_recipe_request: InsertRecipeRequest) -> Result<()> {
        insert_recipe_request.validate()?;

        let name = self
            .html_client
            .get_title(&insert_recipe_request.recipe_url)
            .await?;
        let recipe = Recipe::new(name, url::Url::parse(&insert_recipe_request.recipe_url)?);
        self.recipe_repository.insert_recipe(recipe).await?;

        self.line_client
            .reply_messages(
                &insert_recipe_request.reply_token,
                vec![LineMessage::Text(INSERT_RECIPE_MESSAGE.to_string())],
            )
            .await
            .with_context(|| "failed to reply message")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::infra::{
        html::MockHtmlClient, line::MockLineClient, repository::recipe::MockRecipeRepository,
    };

    use super::*;

    #[tokio::test]
    async fn test_insert_recipe() {
        let mut html_client = MockHtmlClient::new();
        html_client
            .expect_get_title()
            .times(1)
            .returning(|_| Ok("test".to_string()));

        let mut recipe_repository = MockRecipeRepository::new();
        recipe_repository
            .expect_insert_recipe()
            .times(1)
            .returning(|_| Ok(()));

        let mut line_client = MockLineClient::new();
        line_client
            .expect_reply_messages()
            .times(1)
            .returning(|_, _| Ok(()));

        let recipe_service = RecipeService::new(
            Arc::new(recipe_repository),
            Arc::new(line_client),
            Arc::new(html_client),
        );

        let request = InsertRecipeRequest {
            recipe_url: "https://example.com".to_string(),
            reply_token: "reply_token".to_string(),
        };

        let result = recipe_service.insert_recipe(request).await;
        assert!(result.is_ok());
    }
}
