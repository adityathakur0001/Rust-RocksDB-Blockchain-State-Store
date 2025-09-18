use rocksdb::{Options, DB};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct BlockchainDB {
    inner: Arc<Mutex<DB>>,
}

impl BlockchainDB {
    pub fn new(path: &str) -> Self {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        let db = DB::open(&opts, path).unwrap();
        BlockchainDB { inner: Arc::new(Mutex::new(db)) }
    }

    pub async fn put(&self, key: &str, value: &[u8]) {
        let db = self.inner.clone();
        tokio::task::spawn_blocking(move || {
            let db = db.blocking_lock();
            db.put(key, value).unwrap();
        })
        .await
        .unwrap();
    }

    pub async fn get(&self, key: &str) -> Option<Vec<u8>> {
        let db = self.inner.clone();
        tokio::task::spawn_blocking(move || {
            let db = db.blocking_lock();
            db.get(key).unwrap()
        })
        .await
        .unwrap()
    }
}
