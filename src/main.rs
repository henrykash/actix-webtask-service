use std::net::SocketAddr;
use axum::routing::get;
use axum::{Router, response::Html};

#[tokio:: main]

async fn main() {

     //create a route for /hello and return a string "Hello, world!"
    let routes_hello = Router::new().route(
        "/hello",
        get(|| async {Html("Hello, <strong> world!! </strong>" )}),
    );

    // Create a socket address to listen on 
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Listening on  {addr}\n");

    // Run server
    axum::Server::bind(&addr)
        .serve(routes_hello.into_make_service())
        .await
        .unwrap();
    
}