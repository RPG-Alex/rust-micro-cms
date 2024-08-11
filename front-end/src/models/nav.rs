use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum NavItemType {
    ThumbnailUrl,
    BlogSummary,
    SocialLink,
}

impl NavItemType {
    pub fn as_str(&self) -> &'static str {
        match self {
            NavItemType::ThumbnailUrl => "ThumbnailUrl",
            NavItemType::BlogSummary => "BlogSummary",
            NavItemType::SocialLink => "SocialLink",
        }
    }
    pub fn from_str(s: &str) -> NavItemType {
        match s.to_lowercase().as_str() {
            "ThumbnailUrl" => NavItemType::ThumbnailUrl,
            "BlogSummary" => NavItemType::BlogSummary,
            _ => NavItemType::SocialLink,
        }
    }
    pub fn iterator() -> impl Iterator<Item = NavItemType> {
        [NavItemType::ThumbnailUrl, NavItemType::BlogSummary, NavItemType::SocialLink].iter().cloned()
    }
}



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Nav {
    pub items: Vec<NavItem>,
}

impl Default for Nav {
    fn default() -> Self {
        Nav { items: Vec::new() }
    }
}
