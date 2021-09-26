use crate::database::models;
use async_graphql::types::ID;
use async_graphql::*;
use serde::{Deserialize, Serialize};

type StarshipDatabaseModel = models::Starship;

#[derive(Serialize, Deserialize)]
pub struct Starship {
    pub id: i64,
    pub name: String,
    pub class: String,
}

#[Object]
impl Starship {
    async fn id(&self) -> ID {
        ID(self.id.to_string())
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn class(&self) -> &str {
        &self.class
    }
}

impl From<StarshipDatabaseModel> for Starship {
    fn from(starship: StarshipDatabaseModel) -> Self {
        Starship {
            id: starship.id,
            name: starship.name,
            class: starship.class,
        }
    }
}
