use chrono::{NaiveDate, Utc};
use crate::{db::DBConnection, models::Post, models::Posts};
use anyhow::{Result, Error};

#[derive(Clone)]
pub struct PostService {
    pub db: DBConnection,
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
    pub async fn get_all_posts_for_author(&self, author_id: i32) -> Result<Posts, Error> {
        self.db.fetch_all_posts_for_author(author_id).await
    }
    pub async fn change_post(&self, post: &Post) -> Result<()> {
        self.db.update_post(post).await
    }
    pub async fn remove_post(&self, post_id: i32) -> Result<()> {
        self.db.delete_post(post_id).await
    }
    pub async fn post_draft(&self, post_id: i32) -> Result<()> {
        self.db.toggle_post_draft(post_id).await
    }
    pub async fn post_active(&self, post_id: i32) -> Result<()> {
        self.db.toggle_post_active(post_id).await
    }
}