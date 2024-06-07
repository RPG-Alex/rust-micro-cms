use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Nav {
    pub summary: String,
    pub socials: Vec<Social>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Social {
    pub social: String,
    pub url: String,
}

pub enum NavItemType {
    summary,
    social,
}

pub struct NewNavItem {
    pub item: String,
    pub 
}