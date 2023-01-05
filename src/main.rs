pub mod database;
pub mod http_server;
pub mod tree;

use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
};

use database::bruno_db::BrunoDb;
use flexi_logger::Logger;
use http_server::{axum_http_server::AxumHttpServer, HttpServer};

#[tokio::main]
async fn main() {
    Logger::try_with_env_or_str("bruno_db")
        .unwrap()
        .log_to_stdout()
        .start()
        .unwrap();

    let db = Arc::new(BrunoDb::default());
    let server = Box::new(AxumHttpServer::default());
    server
        .listen(
            &SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
            db,
        )
        .await;
}
