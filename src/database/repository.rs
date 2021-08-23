use crate::database::models;
use sqlx::{Acquire, PgConnection, PgPool};

pub struct Repository<'a> {
    conn_pool: &'a PgPool,
}

impl Repository {
    pub fn new(pool: &PgPool) -> Repository {
        Repository { conn_pool: pool }
    }

    fn conn(&self) -> &PgPool {
        self.conn_pool
    }

    pub fn get_starship(&self, id: i64) -> Result<models::Starship, sqlx::Error> {
        let starship = sqlx::query_as!(
            models::Starship,
            r#"
                SELECT id, name, class FROM starships WHERE id = $1
                "#,
            id
        )
        .fetch_one(self.conn())
        .await?;

        Ok(starship)
    }
}
