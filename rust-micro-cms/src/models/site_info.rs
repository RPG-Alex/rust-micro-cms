pub struct SiteName {
    site_name: String,
}

pub struct AuthorName {
    author_name: String,
}

pub struct Description {
    description: String,
}


pub struct SiteInfo {
    pub id: i64,
    pub name: SiteName,
    pub author_name: AuthorName,
    pub description: Description,
}