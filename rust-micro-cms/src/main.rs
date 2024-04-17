use axum::*;

mod database;
mod models;

#[macro_use]
extern crate simple_log;

use dotenv::dotenv;

use simple_log::LogConfigBuilder;
use sqlx::SqlitePool;
use std::{env, error::Error};

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
    let pool = SqlitePool::connect(&db_path).await;
    database::create_author_table(&pool.unwrap());
    info!("Rust Micro CMS started");
}
