//! Actix web async-graphql example
//!
use std::io;
use std::sync::Arc;

use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};

use routes::{configure_service, create_schema_with_context};
use sqlx::postgres::PgPoolOptions;

mod schema;
pub mod config;
pub mod routes;
mod database;

#[actix_web::main]
async fn main() -> io::Result<()> {
    let cfg = config::get_config().expect("failed to read config");

    std::env::set_var("RUST_LOG", "actix_web=debug,info");
    env_logger::init();

    let connection_pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(20))
        .connect(&cfg.database.connection_string())
        .await
        .expect("failed to get postgres conn");

    let schema = create_schema_with_context(connection_pool);    

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .configure(configure_service).data(schema.clone())
            .default_service(web::to(|| async { HttpResponse::NotFound() }))
    })
        .bind(cfg.addr())?
        .run()
        .await
}