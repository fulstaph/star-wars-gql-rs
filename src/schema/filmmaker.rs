use crate::database::models::{
    Filmmaker as FilmmakerDatabaseModel, Profession as ProfessionDatabaseModel,
};
use crate::schema::filmmaker::Profession::{Cinematographer, Director, Producer, Scriptwriter};
use async_graphql::{Enum, Object, ID};
use serde::{Deserialize, Serialize};

#[derive(Enum, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Profession {
    Director,
    Scriptwriter,
    Producer,
    Cinematographer,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Filmmaker {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub profession: Profession,
}

#[Object]
impl Filmmaker {
    async fn id(&self) -> ID {
        ID(self.id.to_string())
    }

    async fn first_name(&self) -> &str {
        self.first_name.as_str()
    }

    async fn last_name(&self) -> &str {
        self.last_name.as_str()
    }

    async fn profession(&self) -> Profession {
        self.profession
    }
}

impl From<FilmmakerDatabaseModel> for Filmmaker {
    fn from(filmmaker: FilmmakerDatabaseModel) -> Self {
        Filmmaker {
            id: filmmaker.id,
            first_name: filmmaker.first_name,
            last_name: filmmaker.last_name,
            profession: match filmmaker.profession {
                ProfessionDatabaseModel::Director => Director,
                ProfessionDatabaseModel::Cinematographer => Cinematographer,
                ProfessionDatabaseModel::Producer => Producer,
                ProfessionDatabaseModel::Scriptwriter => Scriptwriter,
            },
        }
    }
}

impl From<&FilmmakerDatabaseModel> for Filmmaker {
    fn from(filmmaker: &FilmmakerDatabaseModel) -> Self {
        Filmmaker {
            id: filmmaker.id,
            first_name: filmmaker.first_name.clone(),
            last_name: filmmaker.last_name.clone(),
            profession: match filmmaker.profession {
                ProfessionDatabaseModel::Director => Director,
                ProfessionDatabaseModel::Cinematographer => Cinematographer,
                ProfessionDatabaseModel::Producer => Producer,
                ProfessionDatabaseModel::Scriptwriter => Scriptwriter,
            },
        }
    }
}
