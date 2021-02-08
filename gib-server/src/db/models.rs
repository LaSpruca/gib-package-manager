use serde::{Serialize, Deserialize};

use super::schema::gib_pm::{packages, package_archives, users, user_tokens};

#[derive(Queryable, Serialize, Clone, Insertable, Debug, Deserialize)]
#[table_name="users"]
pub struct User {
    pub id: i32,
    #[serde(rename="login")]
    pub username: String,
    pub email: String,
    pub avatar_url: String,
}

#[derive(Queryable, Serialize, Clone, Debug)]
pub struct Package {
    pub id: i32,
    pub package_name: String,
    pub publisher: i32,
    pub configuration: String,
    pub current_version: String,
}

#[derive(Queryable, Serialize, Clone, Debug, Deserialize)]
pub struct UserToken {
    pub id: i32 ,
    pub user_id: i32
}

#[derive(Queryable, Serialize, Clone, Debug)]
pub struct PackageArchive {
    pub id: i32,
    pub package_id: i32,
    pub version: String,
    pub archive: Vec<u8>,
}

#[derive(Insertable)]
#[table_name="packages"]
pub struct NewPackage<'a> {
    pub package_name: &'a str,
    pub publisher: i32,
    pub configuration: &'a str,
    pub current_version: &'a str
}

#[derive(Insertable)]
#[table_name="package_archives"]
pub struct NewPackageArchive {
    pub package_id: i32,
    pub version: String,
    pub archive: Vec<u8>,
}

#[derive(Insertable)]
#[table_name="user_tokens"]
pub struct CreateToken {
    pub user_id: i32
}
