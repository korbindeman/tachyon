mod config;
mod models;
mod routes;
mod services;
mod state;
mod utils;

use crate::{config::Config, state::AppState};
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
use tracing_subscriber::{EnvFilter, fmt};

async fn init_database(database_url: &str) -> Result<SqlitePool, sqlx::Error> {
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

fn init_tracing() {
    fmt()
        .json()
        .with_env_filter(EnvFilter::from_default_env())
        .with_current_span(true)
        .with_span_list(false)
        .init();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let _guard = init_tracing();

    let config = Config::from_env();

    let pool = init_database(&config.database_url)
        .await
        .expect("Database init failed");

    info!("Starting HTTP server on {}:{}", config.host, config.port);

    let state = AppState {
        db: pool.clone(),
        config: config.clone(),
    };

    HttpServer::new(move || {
        let rate_limiter = RateLimiter::builder(
            InMemoryBackend::builder().build(),
            SimpleInputFunctionBuilder::new(Duration::from_secs(1), config.rate_limit_rps)
                .real_ip_key()
                .build(),
        )
        .add_headers()
        .build();

        App::new()
            .app_data(MultipartFormConfig::default().total_limit(config.payload_limit))
            .app_data(web::Data::new(state.clone()))
            .wrap(Logger::default())
            .wrap(rate_limiter)
            .service(upload)
            .service(download)
            .service(download_info)
    })
    .bind((config.host.as_str(), config.port))?
    .run()
    .await
}
