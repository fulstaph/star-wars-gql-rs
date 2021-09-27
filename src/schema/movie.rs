use crate::database::models::Movie as MovieDatabaseModel;
use crate::loaders::*;
use crate::schema::filmmaker::Filmmaker;
use async_graphql::dataloader::DataLoader;
use async_graphql::{Context, Object, Result, ID};
use serde::{Deserialize, Serialize};

type Date = chrono::NaiveDate;

#[derive(Serialize, Deserialize)]
pub struct Movie {
    pub id: i64,
    pub title: String,
    pub director_id: i64,
    pub scriptwriter_id: i64,
    pub producer_id: i64,
    pub release_date: Date,
}

#[Object]
impl Movie {
    async fn id(&self) -> ID {
        ID(self.id.to_string())
    }

    async fn title(&self) -> &str {
        self.title.as_str()
    }

    async fn director(&self, ctx: &Context<'_>) -> Result<Filmmaker> {
        let loader = ctx
            .data::<DataLoader<FilmmakerLoader>>()
            .expect("error getting dataloader");

        let filmmaker = loader.load_one(self.director_id).await?;

        filmmaker.ok_or_else(|| "Not found".into())
    }

    async fn scriptwriter(&self, ctx: &Context<'_>) -> Result<Filmmaker> {
        let loader = ctx
            .data::<DataLoader<FilmmakerLoader>>()
            .expect("error getting dataloader");

        let filmmaker = loader.load_one(self.director_id).await?;

        filmmaker.ok_or_else(|| "Not found".into())
    }

    async fn producer(&self, ctx: &Context<'_>) -> Result<Filmmaker> {
        let loader = ctx
            .data::<DataLoader<FilmmakerLoader>>()
            .expect("error getting dataloader");

        let filmmaker = loader.load_one(self.director_id).await?;

        filmmaker.ok_or_else(|| "Not found".into())
    }

    async fn release_date(&self) -> Date {
        self.release_date
    }
}

impl From<MovieDatabaseModel> for Movie {
    fn from(movie: MovieDatabaseModel) -> Self {
        Movie {
            id: movie.id,
            title: movie.title,
            director_id: movie.director_id,
            scriptwriter_id: movie.scriptwriter_id,
            producer_id: movie.producer_id,
            release_date: movie.release_date,
        }
    }
}
