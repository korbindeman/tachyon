mod models;
mod routes;
mod services;
mod state;
mod utils;

use crate::state::AppState;
use actix_extensible_rate_limit::{
    RateLimiter,
    backend::{SimpleInputFunctionBuilder, memory::InMemoryBackend},
};
use actix_multipart::form::MultipartFormConfig;
use actix_web::{App, HttpServer, middleware::Logger, web};
use dotenv::dotenv;
use routes::*;
use sqlx::{Executor, Sqlite, SqlitePool, migrate::MigrateDatabase};
use std::time::Duration;
use tracing::info;
use tracing_subscriber::fmt;
use utils::env::{get_database_url, get_host, get_payload_limit, get_port, get_rate_limit_rps};

async fn init_database() -> Result<SqlitePool, sqlx::Error> {
    let database_url = &get_database_url();
    if !Sqlite::database_exists(database_url).await.unwrap_or(false) {
        println!("Creating database {}", database_url);
        Sqlite::create_database(database_url)
            .await
            .expect("Failed to create database");
    }

    let pool = SqlitePool::connect(database_url).await?;
    pool.execute(include_str!("./schema.sql"))
        .await
        .expect("Failed to initialize DB schema");
    Ok(pool)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    fmt::init();

    let host = get_host();
    let port = get_port();

    info!("Starting HTTP server on {}:{}", host, port);

    let pool = init_database().await.expect("Database init failed");

    HttpServer::new(move || {
        let backend = InMemoryBackend::builder().build();
        let input = SimpleInputFunctionBuilder::new(Duration::from_secs(1), get_rate_limit_rps())
            .real_ip_key()
            .build();
        let rate_limiter = RateLimiter::builder(backend, input).add_headers().build();

        App::new()
            .app_data(MultipartFormConfig::default().total_limit(get_payload_limit()))
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .wrap(Logger::default())
            .wrap(rate_limiter)
            .service(upload)
            .service(download)
            .service(download_info)
    })
    .bind((host, port))?
    .run()
    .await
}
