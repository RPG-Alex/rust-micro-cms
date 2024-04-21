use axum::{
    extract::Extension,
    handler::{Handler, HandlerWithoutStateExt},
    http::{header, StatusCode},
    routing::{delete, get, post, put},
    Json, Router,
};

mod database;
mod handlers;
mod models;
mod state;

#[macro_use]
extern crate simple_log;

use dotenv::dotenv;
use simple_log::LogConfigBuilder;
use sqlx::SqlitePool;
use std::{env, error::Error};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let config = LogConfigBuilder::builder()
        .path("./log/rust_micro_cms.log")
        .size(1 * 1024 * 1024) // 1 MB
        .roll_count(5)
        .level("info")
        .output_file()
        .output_console()
        .build();

    simple_log::new(config);

    dotenv().ok();
    let db_path = &env::var("DATABASE_URL").expect("DATABASE_URL Must be set in .env file");
    let pool = SqlitePool::connect(&db_path).await.expect("FAILED to load database");

    info!("Rust Micro CMS started");
    let app = Router::new()
        //.route("/authors", get(handlers::authors::get_authors).post(handlers::authors::add_author))
        .route("/authors/:id", get(handlers::authors::get_author))
        .layer(Extension(pool));
    let listener = TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind");
    println!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
