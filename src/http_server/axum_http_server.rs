use std::net::{SocketAddr, SocketAddrV4};

use super::HttpServer;
use async_trait::async_trait;
use axum::{routing::get, Router};

#[derive(Default)]
pub struct AxumHttpServer {}

#[async_trait]
impl HttpServer for AxumHttpServer {
    async fn listen(&self, addr: &SocketAddr) {
        // build our application with a single route
        let app = Router::new().route("/", get(|| async { "Hello, World!" }));

        // run it with hyper on localhost:3000
        axum::Server::bind(addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
    }
}
