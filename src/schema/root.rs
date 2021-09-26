use crate::schema::character::Character;
use crate::schema::movie::Movie;
use crate::schema::planet::Planet;
use crate::schema::starship::Starship;
use crate::SharedWookiepediaRepository;
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

        let repo = match ctx.data::<SharedWookiepediaRepository>() {
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

        let repo = match ctx.data::<SharedWookiepediaRepository>() {
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

        let repo = match ctx.data::<SharedWookiepediaRepository>() {
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

        let repo = match ctx.data::<SharedWookiepediaRepository>() {
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

#[cfg(test)]
mod tests {
    use crate::{
        database::models::{Planet, Starship},
        database::repository::MockWookiepediaRepository,
        routes::create_schema_with_repository,
    };
    use async_graphql::Request;
    use std::sync::Arc;

    #[async_std::test]
    async fn test_fetching_starship() {
        let mut repo = MockWookiepediaRepository::new();
        repo.expect_get_starship()
            .with(mockall::predicate::eq(1))
            .times(1)
            .returning(|_| {
                Ok(Starship {
                    id: 1,
                    name: "test".to_string(),
                    class: "test".to_string(),
                })
            });

        let schema = create_schema_with_repository(Arc::new(repo));

        let response = schema
            .execute(Request::new("{ starship(id: \"1\") { id name class}}"))
            .await;

        assert_eq!(
            response.data.to_string(),
            r#"{starship: {class: "test",id: "1",name: "test"}}"#.to_string()
        );
    }

    #[async_std::test]
    async fn test_fetching_planet() {
        let mut repo = MockWookiepediaRepository::new();
        repo.expect_get_planet()
            .with(mockall::predicate::eq(1))
            .times(1)
            .returning(|_| {
                Ok(Planet {
                    id: 1,
                    name: "test".to_string(),
                })
            });

        let schema = create_schema_with_repository(Arc::new(repo));

        let response = schema
            .execute(Request::new("{ planet(id: \"1\") { id name }}"))
            .await;

        assert_eq!(
            response.data.to_string(),
            r#"{planet: {id: "1",name: "test"}}"#.to_string()
        );
    }
}
