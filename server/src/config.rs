use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct PackageConfig {
    pub name: String,
    pub version: String,
    pub author: String,
    pub post_script: Option<String>
}