use crate::database::repository::Repository;
use crate::schema::character::Character;
use crate::schema::movie::Movie;
use crate::schema::planet::Planet;
use crate::schema::starship::Starship;
use async_graphql::*;

pub type AppSchema = Schema<Query, EmptyMutation, EmptySubscription>;

pub struct Query;

#[Object]
impl Query {
    async fn starship(&self, ctx: &Context<'_>, id: ID) -> Option<Starship> {
        let id = match id.parse::<i64>() {
            Ok(id) => id,
            Err(error) => {
                tracing::warn!("id parsing error: {:?}", error);
                return None;
            }
        };

        let repo = match ctx.data::<Repository>() {
            Ok(repo) => repo,
            Err(error) => {
                tracing::error!("error getting pool: {:?}", error);
                return None;
            }
        };

        let starship = match repo.get_starship(id).await {
            Ok(starship) => starship,
            Err(error) => match error {
                sqlx::Error::RowNotFound => {
                    tracing::info!("starship not found");
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
                tracing::warn!("id parsing error: {:?}", error);
                return None;
            }
        };

        let repo = match ctx.data::<Repository>() {
            Ok(repo) => repo,
            Err(error) => {
                tracing::error!("error getting pool: {:?}", error);
                return None;
            }
        };

        let planet = match repo.get_planet(id).await {
            Ok(planet) => planet,
            Err(error) => {
                tracing::error!("error fetching planet: {:?}", error);
                return None;
            }
        };

        Some(Planet::from(planet))
    }

    async fn character(&self, ctx: &Context<'_>, id: ID) -> Option<Character> {
        let id = match id.parse::<i64>() {
            Ok(id) => id,
            Err(error) => {
                tracing::warn!("id parsing error: {:?}", error);
                return None;
            }
        };

        let repo = match ctx.data::<Repository>() {
            Ok(repo) => repo,
            Err(error) => {
                tracing::error!("error getting pool: {:?}", error);
                return None;
            }
        };

        let character = match repo.get_character(id).await {
            Ok(char) => char,
            Err(error) => match error {
                sqlx::Error::RowNotFound => {
                    tracing::info!("character not found");
                    return None;
                }
                _ => {
                    tracing::error!("error fetching character: {:?}", error);
                    return None;
                }
            },
        };

        Some(Character::from(character))
    }

    async fn movie(&self, ctx: &Context<'_>, id: ID) -> Option<Movie> {
        let id = match id.parse::<i64>() {
            Ok(id) => id,
            Err(error) => {
                tracing::warn!("id parsing error: {:?}", error);
                return None;
            }
        };

        let repo = match ctx.data::<Repository>() {
            Ok(repo) => repo,
            Err(error) => {
                tracing::error!("error getting pool: {:?}", error);
                return None;
            }
        };

        let movie = match repo.get_movie(id).await {
            Ok(movie) => movie,
            Err(error) => match error {
                sqlx::Error::RowNotFound => {
                    tracing::info!("movie not found");
                    return None;
                }
                _ => {
                    tracing::error!("error fetching movie: {:?}", error);
                    return None;
                }
            },
        };

        Some(Movie::from(movie))
    }
}
