use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq,Serialize, Deserialize)]
pub struct SiteInfo {
    pub site_name: String,
}
