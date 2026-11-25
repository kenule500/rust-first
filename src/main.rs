#![allow(unused)]
use std::{fmt::format, net::SocketAddr};
use axum::{Router, extract::{Path, Query}, middleware, response::{Html, IntoResponse, Response}, routing::{get, get_service}};
use serde::Deserialize;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
mod error;
mod web;
mod model;

pub use self::error::{Error, Result};




#[tokio::main]
async fn main() {
    let route_all = Router::new()
        .merge(web::routes_login::routes())
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());
    
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    print!("->> LISENING on {addr}\n");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, route_all).await.unwrap();
}

async fn main_response_mapper(res: Response) -> Response {
    print!("->> {:<12} -main response mapper", "RES_MAPPER");

    res
}

fn routes_static() -> ServeDir {
    ServeDir::new("./")
}




// #[tokio::main]
// async fn main() {
//     let routes_all = Router::new()
//     .merge(routes_hello());

//     let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
//     println!("-->> Listening on {addr}");
    
//     let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
//     axum::serve(listener, routes_all).await.unwrap();
// }



// fn routes_hello() -> Router {
//     Router::new()
//         .route("/hello", get(handler_hello))
//         .route("/hello/{name}", get(handler_hello2))
// }

// #[derive(Debug, Deserialize)]
// struct HelloParams {
//     name: Option<String>
// }

// async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
//    println!("->> {:<12} handler_hello - {params:?}", "HANDLER");

//    let name: &str = params.name.as_deref().unwrap_or("Ken");
//    Html(format!("Hello <strong>{name}!!!</strong>"))
// }

// async  fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
//     println!("->> {:<12} handler_hello2 - {name:?}", "HANDLER");

//     Html(format!("Hello <strong>{name}!!!</strong>"))
// }