use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Author {
    pub id: Option<i64>,
    pub first_name: String,
    pub last_name: String,
    pub deleted: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Authors {
    pub authors: Vec<Author>,
}

impl Author {
    pub fn new(first_name: String, last_name: String) -> Self {
        Author { id: None, first_name, last_name, deleted: None }
    }

}

impl Authors {
    pub fn new(authors: Vec<Author>) -> Self {
        Authors { authors }
    }
}