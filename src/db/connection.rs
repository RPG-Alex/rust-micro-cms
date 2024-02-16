use sqlx::{Sqlite, sqlite::SqlitePool, Pool};
use crate::models::{Author, Authors, Post, Posts};
use anyhow::{Result, Context};
use std::path::Path;
use std::fs;


pub struct DBConnection {
    pool: Pool<Sqlite>,
}

impl DBConnection {
    pub async fn new(db_path: &str) -> Result<Self> {

        if let Some(parent) = Path::new(db_path).parent() {
            fs::create_dir_all(parent).with_context(|| format!("Could not create directory for database file at '{}'", parent.display()))?;
        }

        if !Path::new(db_path).exists() {
            fs::File::create(db_path).with_context(|| format!("Could not create database file at '{}'", db_path))?;
        }

        let pool = SqlitePool::connect(&format!("sqlite:{}", db_path)).await.with_context(|| format!("Could not connect to database at '{}'", db_path))?;
        Ok(DBConnection { pool })
    }

}

