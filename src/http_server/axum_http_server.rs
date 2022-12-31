use std::{net::SocketAddr, sync::Arc};

use crate::database::Database;

use super::HttpServer;
use async_trait::async_trait;
use axum::{extract::State, routing::put, Json, Router};

use log::info;
use serde::Deserialize;

#[derive(Default)]
pub struct AxumHttpServer {}

#[derive(Deserialize)]
struct PutRecord {
    key: u64,
    record: String,
}

async fn put_record(State(db): State<Arc<dyn Database>>, Json(put): Json<PutRecord>) {
    db.put_record(put.key, put.record).await
}

#[async_trait]
impl HttpServer for AxumHttpServer {
    async fn listen(&self, addr: &SocketAddr, db: Arc<dyn Database>) {
        let app = Router::new().route("/", put(put_record)).with_state(db);

        info!("Listening on {:?}", addr);
        axum::Server::bind(addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
    }
}
