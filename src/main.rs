#![allow(unused)]

use std::net::SocketAddr;

use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::Router;

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route("/", get(handler_home));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!(">>> Listening on {addr}\n");
    axum::Server::bind(&addr)
        .serve(routes_hello.into_make_service())
        .await
        .unwrap();
}

async fn handler_home() -> impl IntoResponse {
    println!(">> {:<12} - handler_home", "HANDLER");
    Html("Hello from home")
}
