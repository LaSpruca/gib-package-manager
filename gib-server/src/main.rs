#[macro_use]
extern crate diesel;

use std::io::Result;
use actix_web::{App, get, HttpResponse, HttpServer, Scope};
use actix_web::middleware::Logger;
use actix_web_middleware_redirect_scheme::RedirectSchemeBuilder;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

mod pkg;
mod db;

#[get("/")]
fn index() -> HttpResponse {
    HttpResponse::Ok().body(r#"{"gib-server-version": "1.0.0"}"#)
}

#[actix_web::main]
async fn main() -> Result<()>{
    env_logger::init();

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder.set_private_key_file("key.pem", SslFiletype::PEM).unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    let address = format!("0.0.0.0:{}", std::env::var("PORT").unwrap_or("5000".to_string()));

    println!("=> Starting on https://{}", address);

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(RedirectSchemeBuilder::new().build())
            .service(Scope::new("/api").service(pkg::create_scope()).service(index))
            .service(actix_files::Files::new("/", "static"))
    })
        .bind_openssl(address.as_str(), builder)?
        .run()
        .await
}
