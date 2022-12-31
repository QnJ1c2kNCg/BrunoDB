pub mod axum_http_server;

use std::net::{SocketAddr, SocketAddrV4};

use async_trait::async_trait;

#[async_trait]
pub trait HttpServer {
    async fn listen(&self, addr: &SocketAddr);
}
