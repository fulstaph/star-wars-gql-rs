use actix_web::web::Data;
use actix_web::{guard, web, Error, HttpResponse, HttpRequest};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{Context, EmptyMutation, EmptySubscription, Schema};
use async_graphql_actix_web::{Request, Response};
use sqlx::PgPool;
use std::sync::Arc;

use crate::schema::root::{Query, AppSchema};

pub fn configure_service(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/")
            .route(web::post().to(index))
            .route(web::get().to(index_playground)),
    );
}

async fn index(schema: web::Data<AppSchema>, http_req: HttpRequest, req: Request) -> Response {
    let query = req.into_inner();

    schema.execute(query).await.into()
}

async fn index_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/")
        ))
}

pub fn create_schema_with_context(pool: PgPool) -> Schema<Query, EmptyMutation, EmptySubscription> {
    // let details_data_loader =
    // DataLoader::new(DetailsLoader { pool: cloned_pool }).max_batch_size(10);

    Schema::build(Query, EmptyMutation, EmptySubscription)
        // limits are commented out, because otherwise introspection query won't work
        // .limit_depth(3)
        // .limit_complexity(15)
        .data(
            Data::new(pool).clone()
        )
        .finish()
}
