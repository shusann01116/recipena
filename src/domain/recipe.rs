#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Recipe {
    pub id: ulid::Ulid,
    pub name: String,
    pub recipe_url: url::Url,
}
