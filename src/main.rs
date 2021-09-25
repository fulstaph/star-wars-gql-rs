//! Actix web async-graphql example
//!
use std::io;

use actix_web::{middleware, web, App, HttpResponse, HttpServer};

use routes::configure_service;
use sqlx::postgres::PgPoolOptions;

pub mod config;
mod database;
pub mod routes;
mod schema;

use crate::routes::create_schema_with_repository;
use database::repository::*;

#[actix_web::main]
async fn main() -> io::Result<()> {
    let cfg = config::get_config().expect("failed to read config");

    std::env::set_var("RUST_LOG", "actix_web=debug,info");
    env_logger::init();

    let connection_pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(20))
        .connect(&cfg.database.url)
        .await
        .expect("failed to get postgres conn");

    let repo = Repository::new(connection_pool);

    let schema = create_schema_with_repository(repo);

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .configure(configure_service)
            .data(schema.clone())
            .default_service(web::to(|| async { HttpResponse::NotFound().await }))
    })
    .bind(cfg.addr())?
    .run()
    .await
}
