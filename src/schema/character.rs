use crate::database::models::{Character as CharacterDatabaseModel, Race as RaceDatabaseModel};
use crate::database::repository::Repository;
use crate::schema::starship::*;
use async_graphql::*;
use serde_derive::{Deserialize, Serialize};

#[derive(Enum, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Race {
    Human,
    Droid,
    Wookie,
}

#[derive(Serialize, Deserialize)]
pub struct Character {
    pub id: i64,
    pub name: String,
    pub race: Race,
    pub starship_id: i64,
}

#[Object]
impl Character {
    async fn id(&self) -> ID {
        ID(self.id.to_string())
    }

    async fn name(&self) -> &str {
        self.name.as_str()
    }

    async fn race(&self) -> Race {
        self.race
    }

    async fn starship(&self, ctx: &Context<'_>) -> Option<Starship> {
        let repo = ctx.data::<Repository>().expect("error getting pool");

        let starship = match repo
            .get_starship(self.starship_id)
            .await {
                Ok(starship) => starship,
                Err(error) => {
                    tracing::error!("error fetching starship: {:?}", error);
                    return None;
                }
            };

        Some(Starship::from(starship))
    }

    async fn friends(&self, ctx: &Context<'_>) -> Option<Vec<Character>> {
        let repo = ctx.data::<Repository>().expect("error getting pool");

        let friends = match repo
            .list_characters_friends(self.id)
            .await {
                Ok(friends) => friends,
                Err(error) => {
                    tracing::error!("error fetching friends: {:?}", error);
                    return None;
                }
            };

        match friends.is_empty() {
            true => None,
            false => {
                let mut result = Vec::<Character>::with_capacity(friends.capacity());

                for char in friends {
                    result.push(Character::from(char))
                }

                Some(result)
            }
        }
    }
}

impl From<CharacterDatabaseModel> for Character {
    fn from(char: CharacterDatabaseModel) -> Self {
        Character {
            id: char.id,
            name: char.name,
            race: match char.race {
                RaceDatabaseModel::Wookie => Race::Wookie,
                RaceDatabaseModel::Droid => Race::Droid,
                RaceDatabaseModel::Human => Race::Human,
            },
            starship_id: char.starship_id,
        }
    }
}
