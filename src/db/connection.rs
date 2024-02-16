use sqlx::{migrate::MigrateDatabase, Pool, Sqlite, sqlite::SqlitePool};
use crate::models::{Author, Authors, Post, Posts};
use anyhow::{Result, Context};
use std::path::Path;
use std::fs;


pub struct DBConnection {
    pool: Pool<Sqlite>,
}

impl DBConnection {
    // this may
    pub async fn new(db_path: &str) -> Result<Self> {
        if !Sqlite::database_exists(db_path).await.unwrap_or(false) {
            println!("Creating database {}", db_path);
            match Sqlite::create_database(db_path).await {
                Ok(_) => {
                    println!("Created db successfully");
                },
                Err(error) => panic!("error: {}", error),
            }
        } else {
            println!("Database already exists");
        }
        let pool = SqlitePool::connect(&format!("sqlite:{}", db_path)).await.with_context(|| format!("Could not connect to database at '{}'", db_path))?;
        Ok(DBConnection { pool })
    }

}

