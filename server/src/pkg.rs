use actix_web::{Scope, HttpResponse, get, post, web, HttpRequest};
use actix_multipart::Multipart;
use futures::{StreamExt, TryStreamExt};
use std::io::{Write, Read};
use std::fs::File;
use flate2::read::GzDecoder;
use tar::Archive;

pub fn create_scope() -> Scope {
    Scope::new("/pkg")
        .service(create_pkg)
        .service(create_pkg_get)
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
                eprintln!("{}", e);
                return HttpResponse::InternalServerError().body("Could not process archive");
            }
            Ok(e) => e
        };

        let tar = GzDecoder::new(tar_gz);
        let mut archive = Archive::new(tar);

        let mut config_str = String::new();

        for x in archive.entries().unwrap().filter(|x| x.as_ref().unwrap().path().unwrap().ends_with("package.toml")) {
            x.unwrap().read_to_string(&mut config_str);
            break;
        }

        let data = match toml::from_str::<super::config::Config>(config_str.as_str()) {
            Ok(a) => a,
            Err(e) => {
                return HttpResponse::BadRequest().header("Content-Type", "application/json").body(format!(r#"{{"status": -1, "error": "Invalid config file: {}"}}"#, e))
            }
        };



        return HttpResponse::Ok().header("Content-Type", "application/json").body(format!(r#"{{"status": "0", "package": {}}}"#, serde_json::to_string(&data).unwrap()));
    }

    return HttpResponse::BadRequest().header("Content-Type", "application/json").body(format!(r#"{{"status: 1", "error": "no files uploaded"}}"#));
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
fn get_package(request: HttpRequest) -> HttpResponse{
    let package = match request.match_info().get("package") {
        None => {return HttpResponse::BadRequest().header("Content-Type", "application/json").body(r#"{"status": 1, "error": "Please provide a package name"}"#)},
        Some(a) => a
    };
    println!("{}", package);
    return HttpResponse::NotImplemented().body("Fuck you this is not implemented yes");
}