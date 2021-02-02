use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use crate::db::models::{Package, NewPackage, NewPackageArchive, PackageArchive};

mod models;
mod schema;

pub fn establish_connection() -> Result<PgConnection, Box<dyn std::error::Error>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")?;
    Ok(PgConnection::establish(&database_url)?)
}

pub fn create_pacakge<'a>(conn: &PgConnection, package_name: &'a str, publisher: i32, configuration: &'a str, current_version: &'a str) -> QueryResult<Package> {
    use crate::db::schema::gib_pm::packages;

    let new_package = NewPackage {
        package_name,
        publisher,
        configuration,
        current_version,
    };

    diesel::insert_into(packages::table)
        .values(&new_package)
        .get_result(conn)
}

pub fn upload_package_archive<'a>(conn: &PgConnection, package_id: i32, version: String, archive: Vec<u8>) -> QueryResult<PackageArchive> {
    use crate::db::schema::gib_pm::package_archives;

    let new_package = NewPackageArchive {
        package_id,
        version,
        archive
    };

    diesel::insert_into(package_archives::table)
        .values(&new_package)
        .get_result(conn)
}