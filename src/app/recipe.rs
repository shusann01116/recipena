use validator::Validate;

use crate::{domain::recipe::Recipe, prelude::*};
use std::sync::Arc;

use crate::infra::repository::recipe::RecipeRepository;

pub struct RecipeService {
    recipe_repository: Arc<dyn RecipeRepository>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Validate)]
pub struct InsertRecipeRequest {
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(url)]
    pub recipe_url: String,
}

impl TryFrom<InsertRecipeRequest> for Recipe {
    type Error = Error;
    fn try_from(value: InsertRecipeRequest) -> Result<Self> {
        Ok(Self {
            id: ulid::Ulid::new(),
            name: value.name,
            recipe_url: url::Url::parse(&value.recipe_url)?,
        })
    }
}

impl RecipeService {
    pub fn new(recipe_repository: Arc<dyn RecipeRepository>) -> Self {
        Self { recipe_repository }
    }

    pub async fn insert_recipe(&self, recipe: InsertRecipeRequest) -> Result<()> {
        recipe.validate()?;

        self.recipe_repository
            .insert_recipe(recipe.try_into()?)
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::infra::repository::recipe::MockRecipeRepository;

    use super::*;

    #[tokio::test]
    async fn test_insert_recipe() {
        let mut recipe_repository = MockRecipeRepository::new();
        recipe_repository
            .expect_insert_recipe()
            .times(1)
            .returning(|_| Ok(()));

        let recipe_service = RecipeService::new(Arc::new(recipe_repository));

        let request = InsertRecipeRequest {
            name: "test".to_string(),
            recipe_url: "https://example.com".to_string(),
        };

        let result = recipe_service.insert_recipe(request).await;
        assert!(result.is_ok());
    }
}
