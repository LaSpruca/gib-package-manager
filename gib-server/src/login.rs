use actix_web::{
    get,
    web::{Query},
    client::{Client},
    HttpResponse,
};
use dotenv::dotenv;
use serde::Deserialize;

use crate::db::models::User as UserInfo;
use crate::db::{establish_connection, get_user_by_id, create_user, create_token, get_token, delete_token};

#[derive(Deserialize)]
pub struct AuthRequest {
    code: String,
}

#[derive(Deserialize)]
pub struct LogOutRequest {
    auth_token: i32,
}

#[derive(Deserialize, Debug)]
struct AuthToken {
    access_token: String,
    scope: String,
    token_type: String,
}

#[get("/logout")]
pub fn logout(request: Query<LogOutRequest>) -> HttpResponse {

    let db_connection = match establish_connection() {
        Ok(a) => a,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .header("Content-Type", "application/json")
                .body(format!(
                    r#"{{"status": 3, error: "Database connection error {}" "#,
                    e
                ))
        }
    };

    let tokens = get_token(&db_connection, request.auth_token).unwrap();

    if tokens.len() > 0 {
        delete_token(&db_connection, tokens.get(0).unwrap().to_owned().id);
    }

    return HttpResponse::Ok().body(r#"<!doctype html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport"
          content="width=device-width, user-scalable=no, initial-scale=1.0, maximum-scale=1.0, minimum-scale=1.0">
    <meta http-equiv="X-UA-Compatible" content="ie=edge">
    <meta http-equiv="refresh" content="0; url=/">
    <title>Please Wait</title>
</head>
<body>
    <script>
        sessionStorage["authToken"] = "";
        sessionStorage["loggedIn"] = "false";
    </script>
</body>
</html>"#);
}

#[get("/login/oauth")]
pub async fn login(request: Query<AuthRequest>) -> HttpResponse {
    dotenv().ok();

    let client = Client::default();

    let secret = match std::env::var("CLIENT_SECRET") {
        Ok(a) => a,
        Err(_) => return HttpResponse::InternalServerError().body(r#"{"status": 8, "error": "Error getting client secret for github authentication, please set CLIENT_SECRET env var"}"#)
    };

    let mut token = match client
        .post(format!("https://github.com/login/oauth/access_token?client_id=3858b07a17ad5a97dd40&client_secret={}&code={}", secret, request.code))
        .header("User-Agent", "gib-pm")
        .header("Accept", "application/json")
        .send()
        .await {
        Err(e) => {
            return HttpResponse::InternalServerError()
                .header("Content-Type", "application/json")
                .body(format!(
                    r#"{{"status": 8, "error": "Error with GitHub: "{:?}""}}"#,
                    e
                ));
        }
        Ok(e) => e,
    };

    let bytes = token.body().await.unwrap().to_vec();
    let decoded = String::from_utf8(bytes).unwrap();
    let token_response = match serde_json::from_str::<AuthToken>(&decoded) {
        Err(e) => {
            return HttpResponse::InternalServerError()
                .header("Content-Type", "application/json")
                .body(format!(
                    r#"{{"status": 8, "error": 'Error with GitHub: {:?}'}}"#,
                    e
                ));
        }
        Ok(e) => e,
    };

    println!("token {}", token_response.access_token);

    let mut e = match client.get("https://api.github.com/user")
        .header("Authorization", format!("token {}", token_response.access_token))
        .header("User-Agent", "gib-pm")
        .send().await {
        Err(e) => {
            return HttpResponse::InternalServerError()
                .header("Content-Type", "application/json")
                .body(format!(
                    r#"{{"status": 8, "error": 'Error with GitHub: {:?}'}}"#,
                    e
                ));
        }
        Ok(e) => e,
    };

    let bytes = e.body().await.unwrap().to_vec();
    let decoded = String::from_utf8(bytes).unwrap();
    println!("{}", decoded);
    let user_info = serde_json::from_str::<UserInfo>(&decoded).unwrap();

    let db_connection = match establish_connection() {
        Ok(a) => a,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .header("Content-Type", "application/json")
                .body(format!(
                    r#"{{"status": 3, error: "Database connection error {}" "#,
                    e
                ))
        }
    };

    let mut user = match get_user_by_id(&db_connection, user_info.clone().id) {
        Ok(a) => a,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .header("Content-Type", "application/json")
                .body(format!(
                    r#"{{"status": 6, error: "Database error {}" "#,
                    e
                ))
        }
    };

    if user.len() < 1 {
        user.push(create_user(&db_connection, user_info).unwrap());
    }

    let user_token = create_token(&db_connection, user[0].id);

    return HttpResponse::Ok().body(format!(r#"<!doctype html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport"
          content="width=device-width, user-scalable=no, initial-scale=1.0, maximum-scale=1.0, minimum-scale=1.0">
    <meta http-equiv="X-UA-Compatible" content="ie=edge">
    <meta http-equiv="refresh" content="0; url=/">
    <title>Please Wait</title>
</head>
<body>
    <script>
        sessionStorage["authToken"] = "{}";
        sessionStorage["loggedIn"] = "true";
    </script>
</body>
</html>"#, user_token.unwrap().id));
}
