use axum::{*, routing::get, routing::post};
use std::net::TcpListener;
use utils::youtube::*; 
use routes::download::{*};
use tower_http::cors::{CorsLayer, AllowOrigin};


mod utils;
mod routes;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/api/v1/download", post(download_video))
        .layer(CorsLayer::very_permissive());

        
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    let _ = download_youtube("https://www.youtube.com/watch?v=7Q6ae1TEn3k").await;

    axum::Server::from_tcp(listener)
        .unwrap()
        .serve(app.into_make_service())
        .await
        .unwrap();
    
}

async fn root() -> &'static str {
    "Hello, World!"
}


