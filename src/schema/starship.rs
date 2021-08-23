use serde::{Deserialize, Serialize};
use async_graphql::*;

#[derive(Serialize, Deserialize)]
pub struct Starship {
    pub id: i64,
    pub name: String, 
    pub class: String
}

#[Object]
impl Starship {
    async fn id(&self) -> String {
        self.id.to_string()
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn class(&self) -> &str {
        &self.class
    }
}
