use async_trait::async_trait;
use log::trace;

use super::Database;

#[derive(Default)]
pub struct BrunoDb {}

#[async_trait]
impl Database for BrunoDb {
    async fn put_record(&self, key: u64, record: String) {
        trace!("PUT {}, {}", key, record);
        todo!()
    }
}
