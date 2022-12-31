pub mod bruno_db;

use async_trait::async_trait;

#[async_trait]
pub trait Database: Send + Sync {
    async fn put_record(&self, key: u64, record: String);
}
