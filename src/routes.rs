use actix_web::web::Data;
use actix_web::{web, HttpRequest, HttpResponse};
use async_graphql::extensions::Tracing;
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_actix_web::{Request, Response};
use sqlx::PgPool;

use crate::database::repository::Repository;
use crate::schema::root::{AppSchema, Query};

pub fn configure_service(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/graphql")
            .service(web::resource("").to(index))
            .service(web::resource("/playground").to(index_playground)),
    );
}

async fn index(schema: web::Data<AppSchema>, _http_req: HttpRequest, req: Request) -> Response {
    let query = req.into_inner();

    schema.execute(query).await.into()
}

async fn index_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new("/")))
}

pub fn create_schema_with_context(pool: PgPool) -> Schema<Query, EmptyMutation, EmptySubscription> {
    // let details_data_loader =
    // DataLoader::new(DetailsLoader { pool: cloned_pool }).max_batch_size(10);

    Schema::build(Query, EmptyMutation, EmptySubscription)
        // limits are commented out, because otherwise introspection query won't work
        // .limit_depth(3)
        // .limit_complexity(15)
        .data(Data::new(pool))
        .extension(Tracing)
        .finish()
}

pub fn create_schema_with_repository(
    repository: Repository,
) -> Schema<Query, EmptyMutation, EmptySubscription> {
    // let details_data_loader =
    // DataLoader::new(DetailsLoader { pool: cloned_pool }).max_batch_size(10);

    Schema::build(Query, EmptyMutation, EmptySubscription)
        // limits are commented out, because otherwise introspection query won't work
        // .limit_depth(3)
        // .limit_complexity(15)
        .data(repository)
        .extension(Tracing)
        .finish()
}
