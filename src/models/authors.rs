use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Author {
    pub id: Option<i64>,
    pub name: String,
    pub deleted: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Authors {
    pub authors: Vec<Author>,
}

impl Author {
    pub fn new(name: String) -> Self {
        Author { id: None, name, deleted: None }
    }

}

impl Authors {
    pub fn new(authors: Vec<Author>) -> Self {
        Authors { authors }
    }
}