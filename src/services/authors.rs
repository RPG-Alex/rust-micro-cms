use crate::{db::DBConnection, models::{Author, Authors}};
use std::sync::Arc;
use anyhow::{Result, Error};

#[derive(Clone)]
pub struct AuthorService {
    pub db: Arc<DBConnection>,
}

impl AuthorService {
    pub async fn create_author(&self, payload: &Author) -> Result<Author, Error> {
        self.db.insert_new_author(payload).await
    }

    pub async fn get_all_authors(&self) -> Result<Authors, Error> {
        self.db.fetch_all_authors().await
    }

    pub async fn get_author_by_id(&self, author_id: i32) -> Result<Option<Author>, Error> {
        self.db.fetch_author_by_id(author_id).await
    }
    pub async fn change_author(&self, author_id: i32, new_first_name: &str, new_last_name: &str) -> Result<(), Error> {
        self.db.update_author(author_id, new_first_name, new_last_name).await
    }

    pub async fn deactivate_author(&self, author_id: i32) -> Result<(), Error> {
        self.db.soft_delete_author(author_id).await
    }

    pub async fn restore_author(&self, author_id: i32) -> Result<(), Error> {
        self.db.reactivate_author(author_id).await
    }

    pub async fn remove_author(&self, author_id: i32) -> Result<(), Error> {
        self.db.delete_author(author_id).await
    }
}
