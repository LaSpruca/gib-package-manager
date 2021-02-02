use serde::Serialize;

#[derive(Queryable, Serialize, Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Queryable, Serialize, Clone)]
pub struct Package {
    pub id: i32,
    pub package_name: String,
    pub publisher: i32,
    pub configuration: String,
    pub current_version: String,
}

#[derive(Queryable, Serialize, Clone)]
pub struct PackageArchive {
    pub id: i32,
    pub package_id: i32,
    pub version: String,
    pub archive: Vec<u8>,
}

use super::schema::gib_pm::{packages, package_archives};

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
