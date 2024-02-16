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
    db::DBConnection::new(&database_url).await.unwrap();

    
}
