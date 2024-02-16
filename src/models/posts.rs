use serde::{Serialize, Deserialize};
use chrono::NaiveDate;




#[derive(Clone, Debug)]
pub struct Post {
    pub id: Option<usize>,
    pub title: String,
    pub date: NaiveDate,
    pub body: String,
    pub author_id: usize, 
}

#[derive(Debug, Clone)]
pub struct Posts {
    pub posts: Vec<Post>,
}

impl Post {
    pub fn new(title: String, date: NaiveDate, body: String, author_id: usize) -> Self {
        Post {
            id: None,
            title,
            date,
            body,
            author_id,
        }
    }
}

// May not be necessary
impl Posts {
    pub fn new(posts: Vec<Post>) -> Self {
        Posts { posts }
    }
}
