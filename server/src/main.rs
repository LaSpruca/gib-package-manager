use actix_web::{App, HttpServer, get, HttpResponse};
use std::io::Result;
use actix_web::middleware::Logger;
use openssl::ssl::{SslAcceptor, SslMethod, SslFiletype};
use actix_web_middleware_redirect_scheme::RedirectSchemeBuilder;

mod pkg;
mod config;

#[get("/")]
fn index() -> HttpResponse {
    HttpResponse::Ok().body(r#"{"server-version": "1.0.0"}"#)
}

#[actix_web::main]
async fn main() -> Result<()>{
    env_logger::init();

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder.set_private_key_file("key.pem", SslFiletype::PEM).unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    HttpServer::new(|| {
        App::new()
            .service(index)
            .wrap(Logger::default())
            .wrap(RedirectSchemeBuilder::new().build())
            .service(pkg::create_scope())
    })
        .bind_openssl("0.0.0.0:5000", builder)?
        .run()
        .await
}
