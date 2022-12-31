pub mod axum_http_server;

use std::{net::SocketAddr, sync::Arc};

use async_trait::async_trait;

use crate::database::Database;

#[async_trait]
pub trait HttpServer {
    async fn listen(&self, addr: &SocketAddr, db: Arc<dyn Database>);
}
