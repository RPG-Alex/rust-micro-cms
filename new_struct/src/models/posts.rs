use serde::{Serialize, Deserialize};
use chrono::NaiveDate;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Post {
    pub id: Option<usize>,
    pub title: String,
    pub date: NaiveDate,
    pub body: String,
    pub author_id: usize, 
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
