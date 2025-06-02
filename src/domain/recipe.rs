#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Recipe {
    pub id: ulid::Ulid,
    pub name: String,
    pub recipe_url: url::Url,
}

impl Recipe {
    pub fn new(name: String, recipe_url: url::Url) -> Self {
        Self {
            id: ulid::Ulid::new(),
            name,
            recipe_url,
        }
    }
}
