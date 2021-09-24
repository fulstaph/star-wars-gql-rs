use crate::database::models::Movie as MovieDatabaseModel;
use crate::database::repository::Repository;
use crate::schema::filmmaker::Filmmaker;
use async_graphql::{Context, ID, Object};
use serde::{Deserialize, Serialize};
use log::error;

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

    async fn director(&self, ctx: &Context<'_>) -> Option<Filmmaker> {
        let repo = ctx.data::<Repository>().expect("error getting pool");

        let filmmaker = match repo
            .get_filmmaker(self.director_id)
            .await {
                Ok(filmmaker) => filmmaker,
                Err(error) => {
                    error!("error fetching director: {:?}", error);
                    return None;
                }
            };

        Some(Filmmaker::from(filmmaker))
    }

    async fn scriptwriter(&self, ctx: &Context<'_>) -> Option<Filmmaker> {
        let repo = ctx.data::<Repository>().expect("error getting pool");

        let filmmaker = match repo
            .get_filmmaker(self.scriptwriter_id)
            .await {
                Ok(filmmaker) => filmmaker,
                Err(error) => {
                    error!("error fetching scriptwriter: {:?}", error);
                    return None;
                }
            };

        Some(Filmmaker::from(filmmaker))
    }

    async fn producer(&self, ctx: &Context<'_>) -> Option<Filmmaker> {
        let repo = ctx.data::<Repository>().expect("error getting pool");

        let filmmaker = match repo
            .get_filmmaker(self.producer_id)
            .await {
                Ok(filmmaker) => filmmaker,
                Err(error) => {
                    error!("error fetching producer: {:?}", error);
                    return None;
                }
            };

        Some(Filmmaker::from(filmmaker))
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
