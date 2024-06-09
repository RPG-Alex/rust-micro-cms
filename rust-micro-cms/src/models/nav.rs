use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::fmt;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NavItemType {
    ThumbnailUrl,
    BlogSummary,
    SocialLink,
}

impl FromStr for NavItemType {
    type Err = ();

    fn from_str(input: &str) -> Result<NavItemType, Self::Err> {
        match input.to_lowercase().as_str() {
            "thumbnailurl" => Ok(NavItemType::ThumbnailUrl),
            "blogsummary" => Ok(NavItemType::BlogSummary),
            "sociallink" => Ok(NavItemType::SocialLink),
            _ => Err(()),
        }
    }
}


impl fmt::Display for NavItemType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NavItemType::ThumbnailUrl => write!(f, "thumbnailurl"),
            NavItemType::BlogSummary => write!(f, "blogsummary"),
            NavItemType::SocialLink => write!(f, "sociallink"),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NavItem {
    pub id: i64,
    pub item_type: NavItemType,
    pub content: String,
    pub url: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewNavItem {
    pub item_type: NavItemType,
    pub content: String,
    pub url: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Nav {
    pub items: Vec<NavItem>,
}
