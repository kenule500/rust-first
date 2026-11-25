use axum::{Router, routing::post, response::Json};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tower_cookies::{Cookies, Cookie};
use crate::{error::{Error, Result}, web};

pub fn routes() -> Router {
    Router::new()
        .route("/api/login", post(handler_login))
}

async fn handler_login(cookie: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    let username = &payload.username; // username is a &String
    let password = &payload.password; // password is a &String

    if username == "admin" && password == "password" {
        return Err(Error::LoginFailed);
    } 
    cookie.add(Cookie::new(web::AUTH_TOKEN, "user-auth-token"));

    let response = Json(json!({
            "status": "success",
            "message": "Login successful"
    }));

    Ok(response)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}