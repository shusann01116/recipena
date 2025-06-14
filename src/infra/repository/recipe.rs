use crate::{domain::recipe::Recipe, prelude::*};
use async_trait::async_trait;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait RecipeRepository {
    async fn insert_recipe(&self, recipe: Recipe) -> Result<()>;
    async fn get_random_recipe(&self) -> Result<Option<Recipe>>;
}
