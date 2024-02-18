use chrono::{NaiveDate, Utc};
use crate::models::Post;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize)]
pub struct NewPost {
    pub title: String,
    pub body: String,
    pub author_id: i32,
}

impl NewPost {
    pub fn into_post(self) -> Post {
        Post {
            id: None,
            title: self.title,
            date: Utc::now().naive_local().date(),
            body: self.body,
            author_id: self.author_id,
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct UpdatePostInput {
    pub title: Option<String>,
    pub body: Option<String>,
}

impl UpdatePostInput {
    pub fn update_post(&self, mut post: Post) -> Post {
        if let Some(title) = &self.title {
            post.title = title.clone();
        }
        if let Some(body) = &self.body {
            post.body = body.clone();
        }
        post.date = Utc::now().naive_local().date(); // Update date to current
        post
    }
}
