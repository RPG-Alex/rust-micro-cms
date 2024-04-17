use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct NewPostForm {
    pub title: String,
    pub body: String,
    pub draft: bool, 
    pub author_id: i64,
}
