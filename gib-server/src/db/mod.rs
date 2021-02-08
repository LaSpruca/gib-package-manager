use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use crate::db::models::{Package, NewPackage, NewPackageArchive, PackageArchive, User, CreateToken, UserToken};

pub mod models;
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

pub fn get_package_by_name(conn: &PgConnection, pkg: String) -> QueryResult<Vec<Package>> {
    use schema::gib_pm::packages::dsl::*;

    packages.filter(package_name.like(pkg))
        .load::<Package>(conn)
}

pub fn get_package_archive(conn: &PgConnection, pkg_id: i32, ver: String) -> QueryResult<Vec<PackageArchive>> {
    use schema::gib_pm::package_archives::dsl::*;

    package_archives
        .filter(package_id.eq(pkg_id))
        .filter(version.like(ver))
        .load::<PackageArchive>(conn)
}

pub fn get_user_by_id(conn: &PgConnection, usr_id: i32) -> QueryResult<Vec<User>> {
    use schema::gib_pm::users::dsl::*;

    users.filter(id.eq(usr_id))
        .load::<User>(conn)
}

pub fn create_user(conn: &PgConnection, info: User) -> QueryResult<User> {
    use schema::gib_pm::users;

    diesel::insert_into(users::table)
        .values(info)
        .get_result(conn)
}

pub fn create_token(conn: &PgConnection, user_id: i32) -> QueryResult<UserToken> {
    use schema::gib_pm::user_tokens;

    diesel::insert_into(user_tokens::table)
        .values(CreateToken { user_id })
        .get_result(conn)
}

pub fn delete_token(conn: &PgConnection, token_id: i32) {
    use schema::gib_pm::user_tokens::dsl::*;

    diesel::delete(user_tokens.filter(id.eq(token_id))).execute(conn);
}

pub fn get_token(conn: &PgConnection, token_id: i32) -> QueryResult<Vec<UserToken>> {
    use schema::gib_pm::user_tokens::dsl::*;

    user_tokens.filter(id.eq(token_id))
        .load::<UserToken>(conn)
}
