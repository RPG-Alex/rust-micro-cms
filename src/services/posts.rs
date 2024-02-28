use std::sync::Arc;
use chrono::{NaiveDate, Utc};
use crate::{db::DBConnection, models::Post, models::Posts};
use serde::{Deserialize, Serialize};
use anyhow::{Result, Error};

#[derive(Clone)]
pub struct PostService {
    pub db: Arc<DBConnection>,
}

impl PostService {
    pub async fn create_post(&self, payload: &Post) -> Result<Post, Error> {
        self.db.insert_new_post(payload).await
    }
    pub async fn get_all_posts(&self) -> Result<Posts, Error> {
        self.db.fetch_all_posts().await
    }
    pub async fn get_post_by_id(&self, post_id: i32) -> Result<Option<Post>> {
        self.db.fetch_post_by_id(post_id).await
    }
}