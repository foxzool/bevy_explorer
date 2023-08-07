use std::net::{Ipv4Addr, SocketAddr};
use axum::{Router, Server};
use axum::routing::get;



pub async fn api_server() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" })) ;

    let address = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 8765));
    Server::bind(&address)
        .serve(app.into_make_service())
        .await.unwrap();


}

// #[tokio::main]
// pub async fn api_server() {
//     run().await.expect("API Server error");
// }