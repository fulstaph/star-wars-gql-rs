use crate::database::models;
use log::{error, info, log, warn};
use sqlx::PgPool;
use std::sync::Arc;

pub struct Repository {
    conn_pool: Arc<PgPool>,
}

impl Repository {
    pub fn new(pool: PgPool) -> Repository {
        Repository {
            conn_pool: Arc::new(pool),
        }
    }

    pub async fn get_starship(&self, id: i64) -> Result<models::Starship, sqlx::Error> {
        let starship = sqlx::query_as!(
            models::Starship,
            r#"
                SELECT id, name, class FROM starships WHERE id = $1
                "#,
            id
        )
        .fetch_one(&*self.conn_pool)
        .await?;

        Ok(starship)
    }

    pub async fn get_movie(&self, id: i64) -> Result<models::Movie, sqlx::Error> {
        let movie = sqlx::query_as!(
            models::Movie,
            r#"
                SELECT id, title, director_id, scriptwriter_id, producer_id, release_date
                FROM movies
                WHERE id = $1
                "#,
            id
        )
        .fetch_one(&*self.conn_pool)
        .await?;

        Ok(movie)
    }

    pub async fn get_planet(&self, id: i64) -> Result<models::Planet, sqlx::Error> {
        let planet = sqlx::query_as!(
            models::Planet,
            r#"
                SELECT id, name
                FROM planets
                WHERE id = $1
            "#,
            id
        )
        .fetch_one(&*self.conn_pool)
        .await?;

        Ok(planet)
    }

    pub async fn get_character(&self, id: i64) -> Result<models::Character, sqlx::Error> {
        let character = sqlx::query_as!(
            models::Character,
            r#"
                SELECT id as "id!", name as "name!", race as "race: _", starship_id as "starship_id!"
                FROM characters
                WHERE id = $1
            "#,
            id
        )
        .fetch_one(&*self.conn_pool)
        .await?;

        Ok(character)
    }

    pub async fn list_characters_friends(
        &self,
        character_id: i64,
    ) -> Result<Vec<models::Character>, sqlx::Error> {
        let friends = sqlx::query_as!(
            models::Character,
            r#"
                SELECT id as "id!", name as "name!", race as "race: _", starship_id as "starship_id!"
                FROM characters
                WHERE id in (
                    SELECT friend_character_id
                    FROM friends
                    WHERE character_id = $1
                )
            "#,
            character_id
        )
        .fetch_all(&*self.conn_pool)
        .await?;

        Ok(friends)
    }

    pub async fn list_characters(
        &self,
        character_ids: &Vec<i64>,
    ) -> Result<Vec<models::Character>, sqlx::Error> {
        let characters = sqlx::query_as!(
            models::Character,
            r#"
                SELECT id as "id!", name as "name!", race as "race: _", starship_id as "starship_id!"
                FROM characters
                WHERE id = ANY($1)
            "#,
            character_ids
        )
        .fetch_all(&*self.conn_pool)
        .await?;

        Ok(characters)
    }

    pub async fn get_filmmaker(&self, id: i64) -> Result<models::Filmmaker, sqlx::Error> {
        let filmmaker = sqlx::query_as!(
            models::Filmmaker,
            r#"
                SELECT id as "id!",
                    first_name as "first_name!",
                    last_name as "last_name!",
                    profession as "profession: _"
                FROM filmmakers
                WHERE id = $1
            "#,
            id
        )
        .fetch_one(&*self.conn_pool)
        .await?;

        Ok(filmmaker)
    }
}
