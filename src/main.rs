use axum::{
    routing::{delete, get, post}, Router
};

use dotenv::dotenv;
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use std::env;
use anyhow::Result;
mod db;
mod errors;
mod models;
mod services;
mod utils;


#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = &env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    if !Sqlite::database_exists(database_url).await.unwrap_or(false) {
        println!("Creating database {}", database_url);
        match Sqlite::create_database(database_url).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }
}
