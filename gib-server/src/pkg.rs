use actix_web::{Scope, HttpResponse, get, post, web, HttpRequest};
use actix_multipart::Multipart;
use futures::{StreamExt, TryStreamExt};
use std::io::{Write, Read};
use std::fs::File;
use flate2::read::GzDecoder;
use tar::Archive;
use regex::Regex;
use crate::db::{establish_connection, create_pacakge, upload_package_archive, get_package_by_name, get_package_archive};
use actix_web::web::Bytes;
use gib_common::config::PackageConfig;

pub fn create_scope() -> Scope {
    Scope::new("/pkg")
        .service(create_pkg)
        .service(create_pkg_get)
        .service(get_package_latest)
        .service(get_package_version)
        .service(get_package)
}

#[post("/new")]
async fn create_pkg(mut playload: Multipart) -> HttpResponse {
    while let Ok(Some(mut field)) = playload.try_next().await {
        let content_type = field.content_disposition().unwrap();
        let filename = content_type.get_filename().unwrap();
        let filepath = format!("./tmp/{}", sanitize_filename::sanitize(filename));
        let filepath2 = filepath.clone();
        let mut f = web::block(|| std::fs::File::create(filepath2))
            .await
            .unwrap();

        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            f = web::block(move || f.write_all(&data).map(|_| f)).await.unwrap();
        }


        let tar_gz = match File::open(filepath.as_str()) {
            Err(e) => {
                return HttpResponse::InternalServerError()
                    .header("Content-Type", "application/json")
                    .body(format!(r#"{{"status": 1, "error": "Error processing archive: {}"}}"#, e));
            }
            Ok(e) => e
        };

        let tar = GzDecoder::new(tar_gz);
        let mut archive = Archive::new(tar);

        let mut config_str = String::new();

        for x in archive.entries().unwrap().filter(|x| x.as_ref().unwrap().path().unwrap().ends_with("package.toml")) {
            x.unwrap().read_to_string(&mut config_str).unwrap();
            break;
        }

        let mut data = match toml::from_str::<PackageConfig>(config_str.as_str()) {
            Ok(a) => a,
            Err(e) => {
                return HttpResponse::BadRequest()
                    .header("Content-Type", "application/json")
                    .body(format!(r#"{{"status": 2, "error": "Bad config: {}"}}"#, e));
            }
        };

        data.name = data.name.to_ascii_lowercase();
        data.version = data.version.to_ascii_lowercase();

        let re = Regex::new(r"^[a-z0-9-_.]+$").unwrap();

        if !re.is_match(data.name.as_str()) {
            return HttpResponse::BadRequest()
                .header("Content-Type", "application/json")
                .body(format!(r#"{{"status": 2, "error": "Invalid name in package.toml, provided: {}"}}"#, data.name.clone()));
        }

        if !re.is_match(data.version.as_str()) {
            return HttpResponse::BadRequest()
                .header("Content-Type", "application/json")
                .body(format!(r#"{{"status": 2, "error": "Invalid name in package.toml, provided: {}"}}"#, data.name.clone()));
        }

        let db_connection = match establish_connection() {
            Ok(a) => a,
            Err(e) => return HttpResponse::InternalServerError()
                .header("Content-Type", "application/json")
                .body(format!(r#"{{"status": 3, error: "Database connection error {}" "#, e))
        };

        let package = match create_pacakge(&db_connection, data.name.as_str(), 0, serde_json::to_string(&data).unwrap().as_str(), data.version.as_str()) {
            Ok(a) => a,
            Err(e) => return HttpResponse::InternalServerError()
                .header("Content-Type", "application/json")
                .body(format!(r#"{{"status": 4, error: "Error creating package in database: {}" "#, e)),
        };

        let mut f = File::open(&filepath).expect("no file found");
        let metadata = std::fs::metadata(&filepath).expect("unable to read metadata");
        let mut buffer = vec![0; metadata.len() as usize];
        f.read(&mut buffer).expect("buffer overflow");

        match upload_package_archive(&db_connection, package.id.clone(), data.version.clone(), buffer) {
            Ok(_) => {},
            Err(e) => return HttpResponse::InternalServerError().header("Content-Type", "application/json")
                .body(format!(r#"{{"status": 4, error: "Error uploading archive to database: {}" "#, e)),
        };

        return HttpResponse::Ok()
            .header("Content-Type", "application/json")
            .body(format!(r#"{{"status": "0", "id": {}, "package": {}}}"#, package.id, serde_json::to_string(&data).unwrap()));
    }

    return HttpResponse::BadRequest()
        .header("Content-Type", "application/json")
        .body(format!(r#"{{"status: 5", "error": "No files uploaded"}}"#));
}

#[get("/new")]
fn create_pkg_get() -> HttpResponse {
    HttpResponse::Ok().body(r#"<html>
        <head><title>Upload Test</title></head>
        <body>
            <form target="/pkg/new" method="post" enctype="multipart/form-data">
                <input type="file" multiple name="file" accept=".tar.gz"/>
                <button type="submit">Submit</button>
            </form>
        </body>
    </html>"#)
}

#[get("/get/{package}")]
fn get_package(request: HttpRequest) -> HttpResponse {
    let package = match request.match_info().get("package") {
        None => { return HttpResponse::BadRequest()
            .header("Content-Type", "application/json")
            .body(r#"{"status": 6, "error": "Please provide a package name"}"#); }
        Some(a) => a
    };

    let db_connection = match establish_connection() {
        Ok(a) => a,
        Err(e) => return HttpResponse::InternalServerError().header("Content-Type", "application/json")
            .body(format!(r#"{{"status": 3, error: "{}" "#, e))
    };

    let packages = match get_package_by_name(&db_connection, package.to_string()) {
        Ok(a) => a,
        Err(e) => return HttpResponse::InternalServerError()
            .header("Content-Type", "application/json")
            .body(format!(r#"{{"status": 6, error: "{}" "#, e)),
    };

    if !packages.is_empty() {
        return HttpResponse::Ok()
            .header("Content-Type", "application/json")
            .body(format!(r#"{{"status": "0", "package": {}}}"#, serde_json::to_string(packages.get(0).unwrap()).unwrap()));
    }

    return HttpResponse::BadRequest()
        .header("Content-Type", "application/json")
        .body(format!(r#"{{"status": "7", "error": "no package found called: {}"}}"#, package));
}

#[get("/get/{package}@latest")]
async fn get_package_latest(request: HttpRequest) -> HttpResponse {
    let package = request.match_info().get("package").unwrap();

    let db_connection = match establish_connection() {
        Ok(a) => a,
        Err(e) => return HttpResponse::InternalServerError().header("Content-Type", "application/json")
            .body(format!(r#"{{"status": 3, error: "{}" "#, e))
    };

    let packages = match get_package_by_name(&db_connection, package.to_string()) {
        Ok(a) => a,
        Err(e) => return HttpResponse::InternalServerError().header("Content-Type", "application/json")
            .body(format!(r#"{{"status": 6, error: "{}" "#, e)),
    };

    if packages.is_empty() {
        return HttpResponse::BadRequest()
            .header("Content-Type", "application/json")
            .body(format!(r#"{{"status": "7", "error": "no package found called: {}"}}"#, package));
    }

    let package = packages.get(0).unwrap();

    let data = serde_json::from_str::<PackageConfig>(package.configuration.as_str()).unwrap();

    let archives = match get_package_archive(&db_connection, package.id, data.version) {
        Ok(a) => a,
        Err(e) => return HttpResponse::InternalServerError().header("Content-Type", "application/json")
            .body(format!(r#"{{"status": 6, error: "{}" "#, e)),
    };

    if archives.is_empty() {
        return HttpResponse::BadRequest()
            .header("Content-Type", "application/json")
            .body(format!(r#"{{"status": "7", "error": "no archive forund for: {}@{}"}}"#, package.package_name, package.current_version));
    }

    let archive = archives.get(0).unwrap().archive.as_slice();
    let slice = archive.to_owned();

    return HttpResponse::Ok()
        .header("Content-Type", "application/x-gzip")
        .body(Bytes::from(slice))
}

#[get("/get/{package}@{version}")]
async fn get_package_version(request: HttpRequest) -> HttpResponse {
    let package = request.match_info().get("package").unwrap();
    let version = request.match_info().get("version").unwrap();

    let db_connection = match establish_connection() {
        Ok(a) => a,
        Err(e) => return HttpResponse::InternalServerError().header("Content-Type", "application/json")
            .body(format!(r#"{{"status": 3, error: "{}" "#, e))
    };

    let packages = match get_package_by_name(&db_connection, package.to_string()) {
        Ok(a) => a,
        Err(e) => return HttpResponse::InternalServerError().header("Content-Type", "application/json")
            .body(format!(r#"{{"status": 6, error: "{}" "#, e)),
    };

    if packages.is_empty() {
        return HttpResponse::BadRequest()
            .header("Content-Type", "application/json")
            .body(format!(r#"{{"status": "7", "error": "no package found called: {}"}}"#, package));
    }

    let package = packages.get(0).unwrap();

    let archives = match get_package_archive(&db_connection, package.id, version.to_string()) {
        Ok(a) => a,
        Err(e) => return HttpResponse::InternalServerError().header("Content-Type", "application/json")
            .body(format!(r#"{{"status": 6, error: "{}" "#, e)),
    };

    if archives.is_empty() {
        return HttpResponse::BadRequest()
            .header("Content-Type", "application/json")
            .body(format!(r#"{{"status": "7", "error": "no archive forund for: {}@{}"}}"#, package.package_name, version));
    }

    let archive = archives.get(0).unwrap().archive.as_slice();
    let slice = archive.to_owned();

    return HttpResponse::Ok()
        .header("Content-Type", "application/x-gzip")
        .body(Bytes::from(slice))
}
