use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    domain::recipe::Recipe,
    infra::line::{LineClient, LineMessage},
    prelude::*,
};

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct GetRandomRecipeRequest {
    pub reply_token: String,
}

#[async_trait::async_trait]
pub trait RandomRecipeService {
    async fn get_random_recipe(&self, request: GetRandomRecipeRequest) -> Result<()>;
}

pub struct RandomRecipeServiceImpl<R> {
    recipe_repository: R,
    line_client: std::sync::Arc<dyn LineClient + Send + Sync>,
}

impl<R> RandomRecipeServiceImpl<R> {
    pub fn new(
        recipe_repository: R,
        line_client: std::sync::Arc<dyn LineClient + Send + Sync>,
    ) -> Self {
        Self {
            recipe_repository,
            line_client,
        }
    }
}

#[async_trait::async_trait]
impl<R> RandomRecipeService for RandomRecipeServiceImpl<R>
where
    R: crate::infra::repository::recipe::RecipeRepository + Send + Sync,
{
    async fn get_random_recipe(&self, request: GetRandomRecipeRequest) -> Result<()> {
        let recipe = self.recipe_repository.get_random_recipe().await?;

        let message = match recipe {
            Some(Recipe {
                id: _,
                name,
                recipe_url,
            }) => {
                format!("今日のおすすめレシピ:\n{}\n{}", name, recipe_url)
            }
            None => "レシピが見つかりませんでした。".to_string(),
        };

        self.line_client
            .reply_messages(&request.reply_token, vec![LineMessage::Text(message)])
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        domain::recipe::Recipe,
        infra::{line::MockLineClient, repository::recipe::MockRecipeRepository},
    };
    use mockall::predicate::*;
    use std::sync::Arc;
    use url::Url;

    #[tokio::test]
    async fn test_get_random_recipe_success() {
        let mut mock_repository = MockRecipeRepository::new();
        let mut mock_line_client = MockLineClient::new();

        let recipe = Recipe::new(
            "テストレシピ".to_string(),
            Url::parse("https://example.com/recipe").unwrap(),
        );

        mock_repository
            .expect_get_random_recipe()
            .times(1)
            .returning(move || Ok(Some(recipe.clone())));

        mock_line_client
            .expect_reply_messages()
            .with(eq("reply_token_123"), always())
            .times(1)
            .returning(|_, _| Ok(()));

        let service = RandomRecipeServiceImpl::new(mock_repository, Arc::new(mock_line_client));

        let request = GetRandomRecipeRequest {
            reply_token: "reply_token_123".to_string(),
        };

        let result = service.get_random_recipe(request).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_random_recipe_no_recipe_found() {
        let mut mock_repository = MockRecipeRepository::new();
        let mut mock_line_client = MockLineClient::new();

        mock_repository
            .expect_get_random_recipe()
            .times(1)
            .returning(|| Ok(None));

        mock_line_client
            .expect_reply_messages()
            .with(eq("reply_token_123"), always())
            .times(1)
            .returning(|_, _| Ok(()));

        let service = RandomRecipeServiceImpl::new(mock_repository, Arc::new(mock_line_client));

        let request = GetRandomRecipeRequest {
            reply_token: "reply_token_123".to_string(),
        };

        let result = service.get_random_recipe(request).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_random_recipe_repository_error() {
        let mut mock_repository = MockRecipeRepository::new();
        let mock_line_client = MockLineClient::new();

        mock_repository
            .expect_get_random_recipe()
            .times(1)
            .returning(|| Err(crate::error::Error::Generic("Database error".to_string())));

        let service = RandomRecipeServiceImpl::new(mock_repository, Arc::new(mock_line_client));

        let request = GetRandomRecipeRequest {
            reply_token: "reply_token_123".to_string(),
        };

        let result = service.get_random_recipe(request).await;
        assert!(result.is_err());
    }
}
