mod routes;
mod services;
mod utils;

use actix_extensible_rate_limit::{
    RateLimiter,
    backend::{SimpleInputFunctionBuilder, memory::InMemoryBackend},
};
use actix_multipart::form::MultipartFormConfig;
use actix_web::{App, HttpServer, middleware::Logger, web};
use dotenv::dotenv;
use routes::{
    download::{download, download_info},
    upload::upload,
};
use sqlx::{Executor, Sqlite, SqlitePool, migrate::MigrateDatabase};
use std::time::Duration;
use tracing::info;
use tracing_subscriber::fmt;
use utils::{GIGABYTE, get_database_url, get_port};

const PAYLOAD_LIMIT: usize = GIGABYTE * 5;

async fn init_database() -> Result<SqlitePool, sqlx::Error> {
    let database_url = &get_database_url();
    if !Sqlite::database_exists(database_url).await.unwrap_or(false) {
        println!("Creating database {}", database_url);
        match Sqlite::create_database(database_url).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }

    SqlitePool::connect(database_url).await
}

struct AppState {
    db: SqlitePool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    fmt::init();

    info!("Starting HTTP server on 127.0.0.1:8080");

    let pool = init_database().await.unwrap();

    // initialize the database table
    pool.execute(include_str!("./schema.sql"))
        .await
        .expect("Failed to initialize DB");

    let backend = InMemoryBackend::builder().build();

    HttpServer::new(move || {
        let input = SimpleInputFunctionBuilder::new(Duration::from_secs(1), 5)
            .real_ip_key()
            .build();
        let rate_limiter = RateLimiter::builder(backend.clone(), input)
            .add_headers()
            .build();

        App::new()
            .app_data(MultipartFormConfig::default().total_limit(PAYLOAD_LIMIT))
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .wrap(Logger::default())
            .wrap(rate_limiter)
            .service(upload)
            .service(download)
            .service(download_info)
    })
    .bind(("127.0.0.1", get_port()))?
    .run()
    .await
}
