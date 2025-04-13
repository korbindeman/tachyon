mod routes;
mod services;
mod utils;

use actix_multipart::form::MultipartFormConfig;
use actix_web::{App, HttpServer, middleware::Logger, web};
use routes::{download::download, upload::upload};
use sqlx::{Executor, Sqlite, SqlitePool, migrate::MigrateDatabase};
use tracing::info;
use tracing_subscriber::fmt;
use utils::GIGABYTE;

const PAYLOAD_LIMIT: usize = GIGABYTE * 5;

const DATABASE_URL: &str = "sqlite://sqlite.db";

async fn init_database() -> Result<SqlitePool, sqlx::Error> {
    // Initialize the database connection
    if !Sqlite::database_exists(DATABASE_URL).await.unwrap_or(false) {
        println!("Creating database {}", DATABASE_URL);
        match Sqlite::create_database(DATABASE_URL).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }

    SqlitePool::connect(DATABASE_URL).await
}

struct AppState {
    db: SqlitePool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    fmt::init();

    info!("Starting HTTP server on 127.0.0.1:8080");

    let pool = init_database().await.unwrap();

    // initialize the database
    pool.execute(include_str!("./schema.sql"))
        .await
        .expect("Failed to initialize DB");

    HttpServer::new(move || {
        App::new()
            .app_data(MultipartFormConfig::default().total_limit(PAYLOAD_LIMIT))
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .wrap(Logger::default())
            .service(upload)
            .service(download)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
