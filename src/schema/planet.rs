use crate::database::models;
use async_graphql::Object;
use async_graphql::ID;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Planet {
    pub id: i64,
    pub name: String,
}

#[Object]
impl Planet {
    async fn id(&self) -> ID {
        ID(self.id.to_string())
    }

    async fn name(&self) -> &str {
        self.name.as_str()
    }
}

type PlanetDatabaseModel = models::Planet;

impl From<PlanetDatabaseModel> for Planet {
    fn from(planet: PlanetDatabaseModel) -> Self {
        Planet {
            id: planet.id,
            name: planet.name,
        }
    }
}
