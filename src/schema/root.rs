use crate::database::repository::Repository;
use crate::schema::character::Character;
use crate::schema::movie::Movie;
use crate::schema::planet::Planet;
use crate::schema::starship::Starship;
use async_graphql::*;
use log::{error, info, log, warn};

pub type AppSchema = Schema<Query, EmptyMutation, EmptySubscription>;

pub struct Query;

#[Object]
impl Query {
    async fn starship(&self, ctx: &Context<'_>, id: ID) -> Option<Starship> {
        let id = id.parse::<i64>().unwrap();

        let repo = ctx.data::<Repository>().expect("error getting pool");

        let starship = match repo.get_starship(id).await {
            Ok(starship) => starship,
            Err(error) => match error {
                sqlx::Error::RowNotFound => {
                    info!("starship not found");
                    return None;
                }
                _ => panic!("error fetching starship"),
            },
        };

        Some(Starship::from(starship))
    }

    async fn planet(&self, ctx: &Context<'_>, id: ID) -> Option<Planet> {
        let id = match id.parse::<i64>() {
            Ok(id) => id,
            Err(error) => {
                warn!("id parsing error: {:?}", error);
                return None;
            }
        };

        let repo = match ctx.data::<Repository>() {
            Ok(repo) => repo,
            Err(error) => {
                error!("error getting pool: {:?}", error);
                return None;
            }
        };

        let planet = match repo.get_planet(id).await {
            Ok(planet) => planet,
            Err(error) => {
                error!("error fetching planet: {:?}", error);
                return None;
            }
        };

        Some(Planet::from(planet))
    }

    async fn character(&self, ctx: &Context<'_>, id: ID) -> Option<Character> {
        let id = id.parse::<i64>().unwrap();

        let repo = ctx.data::<Repository>().expect("error getting pool");

        let character = repo
            .get_character(id)
            .await
            .expect("error fetching character");

        Some(Character::from(character))
    }

    async fn movie(&self, ctx: &Context<'_>, id: ID) -> Option<Movie> {
        let id = id.parse::<i64>().unwrap();

        let repo = ctx.data::<Repository>().expect("error getting pool");

        let movie = repo.get_movie(id).await.expect("error fetching movie");

        Some(Movie::from(movie))
    }
}
