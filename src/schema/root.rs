use async_graphql::*;
use async_graphql::guard::Guard;
use actix_web::web;
use sqlx::PgPool;
use crate::schema::starship::Starship;


pub type AppSchema = Schema<Query, EmptyMutation, EmptySubscription>;

pub struct Query;

#[Object] 
impl Query {
    async fn starship(&self, ctx: &Context<'_>, id: ID) -> Option<Starship> {
        let id = id.parse::<i64>().unwrap();

        let pool = ctx.data::<web::Data<PgPool>>().expect("error getting pool");

        let starship = sqlx::query_as!(
            Starship,
                r#"
                SELECT id, name, class FROM starships WHERE id = $1
                "#,
                id
        )
            .fetch_one(pool.get_ref())
            .await
            .expect("error fetching data");

        Some(starship)
    }
}
