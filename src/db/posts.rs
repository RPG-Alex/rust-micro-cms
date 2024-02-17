use chrono::{NaiveDate, Utc};
use crate::models::{Author, Authors, Post, Posts};
use serde::{Deserialize, Serialize};
use sqlx::{migrate::MigrateDatabase, Pool, Sqlite, sqlite::SqlitePool};
use std::path::Path;
use std::fs;

#[derive(Clone, Debug, Serialize)]
pub struct NewPost {
    pub title: String,
    pub date: NaiveDate,
    pub body: String,
    pub author_id: usize,
}

impl NewPost {
    pub fn new(title: String, body: String, author_id: usize) -> Self {
        NewPost {
            title,
            date: Utc::now().naive_local().date(),
            body,
            author_id,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct UpdatePost {
    pub post_id: usize,
    pub title: Option<String>,
    pub date: NaiveDate,
    pub body: Option<String>,
    pub author_id: Option<usize>,
}

impl UpdatePost {
    pub fn new(post_id: usize, title: Option<String>, body: Option<String>, author_id: Option<usize>) -> Self {
        UpdatePost {
            post_id,
            title,
            date: Utc::now().naive_local().date(),
            body,
            author_id,
        }
    }
}