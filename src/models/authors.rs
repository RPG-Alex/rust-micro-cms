use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Author {
    pub id: Option<usize>,
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Authors {
    pub authors: Vec<Author>,
}

impl Author {
    pub fn new(name: String) -> Self {
        Author { id: None, name }
    }

}

impl Authors {
    pub fn new(authors: Vec<Author>) -> Self {
        Authors { authors }
    }
}