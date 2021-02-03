use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageConfig {
    pub name: String,
    pub version: String,
    pub author: String,
    pub post_script: Option<String>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ClientConfig {
    pub repos: Vec<String>,
    pub installed: Vec<Package>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub repo: String
}
