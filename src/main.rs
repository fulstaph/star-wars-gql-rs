//! Actix web async-graphql example
//!
use std::io;
use actix_web::{web, App, HttpResponse, HttpServer};

use routes::configure_service;
use sqlx::postgres::PgPoolOptions;
use tracing_actix_web::TracingLogger;

pub mod config;
mod database;
pub mod routes;
mod schema;
pub mod observability;

use crate::routes::create_schema_with_repository;
use crate::observability::*;
use database::repository::*;

#[actix_web::main]
async fn main() -> io::Result<()> {
    let subscriber = get_subscriber(
        "wookiepedia".into(),
        std::env::var("LOG_LEVEL").unwrap_or_else(|_| "debug".into()),
        std::io::stdout
    );
    init_subscriber(subscriber);
    
    let cfg = config::get_config().expect("failed to read config");

    let connection_pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(20))
        .connect(&cfg.database.url)
        .await
        .expect("failed to get postgres conn");

    let repo = Repository::new(connection_pool);

    let schema = create_schema_with_repository(repo);

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
