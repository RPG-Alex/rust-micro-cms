use axum::{
    routing::{delete, get, post}, Router
};

use dotenv::dotenv;
use sqlx::SqlitePool;
use std::env;
use anyhow::Result;
mod db;
mod errors;
mod models;
mod services;
mod utils;


#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    Ok(())
}
