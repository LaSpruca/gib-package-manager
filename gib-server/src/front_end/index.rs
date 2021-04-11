use actix_web::{get, web, HttpResponse, HttpRequest};
use handlebars::Handlebars;
use crate::db::{get_token, DbPool};
use actix_session::{Session};
use diesel::PgConnection;

#[get("/")]
pub fn index(handlebars: web::Data<Handlebars<'_>>, session: Session, pool: web::Data<DbPool>) -> HttpResponse {
    let conn = pool.get().expect("Could not get data from pool");
    let session_key = session.get::<String>("authToken").unwrap();

    session.set("a", "b");

    println!("Session key: {:?}", session_key);

    let mut token = if session_key.is_some() {
        Some (get_token(&conn, session_key.unwrap().parse().unwrap()).unwrap())
    } else {
        None
    };

    let data = json!("{}");

    HttpResponse::Ok().body(handlebars.render("index", &data).unwrap())
}