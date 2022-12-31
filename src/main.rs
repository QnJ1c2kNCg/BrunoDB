pub mod http_server;

use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4};

use http_server::{axum_http_server::AxumHttpServer, HttpServer};

#[tokio::main]
async fn main() {
    let server = Box::new(AxumHttpServer::default());
    server
        .listen(&SocketAddr::new(
            IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            8080,
        ))
        .await;
}
