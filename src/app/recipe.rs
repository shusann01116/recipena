use validator::Validate;

use crate::{domain::recipe::Recipe, infra::html::HtmlClient, prelude::*};
use std::sync::Arc;

use crate::infra::repository::recipe::RecipeRepository;

#[derive(Clone)]
pub struct RecipeService {
    recipe_repository: Arc<dyn RecipeRepository + Send + Sync>,
    html_client: Arc<dyn HtmlClient + Send + Sync>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Validate)]
pub struct InsertRecipeRequest {
    #[validate(url)]
    pub recipe_url: String,
}

impl RecipeService {
    pub fn new(
        recipe_repository: Arc<dyn RecipeRepository + Send + Sync>,
        html_client: Arc<dyn HtmlClient + Send + Sync>,
    ) -> Self {
        Self {
            recipe_repository,
            html_client,
        }
    }

    pub async fn insert_recipe(&self, recipe: InsertRecipeRequest) -> Result<()> {
        recipe.validate()?;

        let name = self.html_client.get_title(&recipe.recipe_url).await?;
        let recipe = Recipe::new(name, url::Url::parse(&recipe.recipe_url)?);
        self.recipe_repository.insert_recipe(recipe).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::infra::{html::MockHtmlClient, repository::recipe::MockRecipeRepository};

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

        let recipe_service = RecipeService::new(Arc::new(recipe_repository), Arc::new(html_client));

        let request = InsertRecipeRequest {
            recipe_url: "https://example.com".to_string(),
        };

        let result = recipe_service.insert_recipe(request).await;
        assert!(result.is_ok());
    }
}
