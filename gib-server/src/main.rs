#[macro_use]
extern crate diesel;

use actix_web::middleware::Logger;
use actix_web::{get, App, HttpResponse, HttpServer, Scope, web};
use actix_web_middleware_redirect_scheme::RedirectSchemeBuilder;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::io::Result;
use handlebars::Handlebars;
use actix_session::{Session, CookieSession};
use crate::db::establish_connection;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;

#[macro_use]
extern crate serde_json;

mod db;
mod login;
mod pkg;
mod front_end;

#[get("/")]
fn index() -> HttpResponse {
    HttpResponse::Ok().body(r#"{"gib-server-version": "1.0.0"}"#)
}

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv();
    env_logger::init();

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    let address = format!(
        "0.0.0.0:{}",
        std::env::var("PORT").unwrap_or("5000".to_string())
    );

    println!("=> Starting on https://{}", address);

    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "./static/.templates")
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);

    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let manager = ConnectionManager::<PgConnection>::new(connspec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(RedirectSchemeBuilder::new().build())
            .wrap(CookieSession::signed(&[0; 32]) // <- create cookie based session middleware
                .secure(false)
            )
            .service(
                Scope::new("/api")
                    .service(pkg::create_scope())
                    .service(index),
            )
            .service(front_end::index::index)
            .service(login::login)
            .service(login::logout)
            .service(actix_files::Files::new("/", "static"))
            .app_data(handlebars_ref.clone())
            .data(pool.clone())
    })
    .bind_openssl(address.as_str(), builder)?
    .run()
    .await
}
