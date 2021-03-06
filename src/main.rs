//! Actix web async-graphql example
//!
use actix_web::{web, App, HttpResponse, HttpServer};
use std::io;

use routes::configure_service;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tracing_actix_web::TracingLogger;

pub mod config;
mod database;
pub mod loaders;
pub mod observability;
pub mod routes;
mod schema;

use crate::observability::*;
use database::repository::*;

#[actix_web::main]
async fn main() -> io::Result<()> {
    let subscriber = get_subscriber(
        "wookiepedia".into(),
        std::env::var("LOG_LEVEL").unwrap_or_else(|_| "debug".into()),
        std::io::stdout,
    );
    init_subscriber(subscriber);

    let cfg = config::get_config().expect("failed to read config");

    let connection_pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(20))
        .connect(&cfg.database.url)
        .await
        .expect("failed to get postgres conn");

    let repo = Repository::new(connection_pool);

    let schema = routes::create_schema_with_repository(Arc::new(repo));

    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger)
            .configure(configure_service)
            .data(schema.clone())
            .default_service(web::to(|| async { HttpResponse::NotFound().await }))
    })
    .bind(cfg.addr())?
    .run()
    .await
}
