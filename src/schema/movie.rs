use crate::database::models::Movie as MovieDatabaseModel;
use crate::database::repository::Repository;
use crate::schema::filmmaker::Filmmaker;
use async_graphql::{Context, Object, ID};
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Movie {
    pub id: i64,
    pub title: String,
    pub director_id: i64,
    pub scriptwriter_id: i64,
    pub producer_id: i64,
    pub release_date: chrono::NaiveDate,
}

#[Object]
impl Movie {
    async fn id(&self) -> ID {
        ID(self.id.to_string())
    }

    async fn title(&self) -> &str {
        self.title.as_str()
    }

    async fn director(&self, ctx: &Context<'_>) -> Option<Filmmaker> {
        let repo = ctx.data::<Repository>().expect("error getting pool");

        let filmmaker = repo
            .get_filmmaker(self.director_id)
            .await
            .expect("error fetching director");

        Some(Filmmaker::from(filmmaker))
    }

    async fn scriptwriter(&self, ctx: &Context<'_>) -> Option<Filmmaker> {
        let repo = ctx.data::<Repository>().expect("error getting pool");

        let filmmaker = repo
            .get_filmmaker(self.scriptwriter_id)
            .await
            .expect("error fetching director");

        Some(Filmmaker::from(filmmaker))
    }

    async fn producer(&self, ctx: &Context<'_>) -> Option<Filmmaker> {
        let repo = ctx.data::<Repository>().expect("error getting pool");

        let filmmaker = repo
            .get_filmmaker(self.producer_id)
            .await
            .expect("error fetching director");

        Some(Filmmaker::from(filmmaker))
    }

    async fn release_date(&self) -> chrono::NaiveDate {
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
            release_date: chrono::NaiveDate::from(movie.release_date),
        }
    }
}
