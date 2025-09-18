mod db;
mod state;
mod benchmark;

use db::BlockchainDB;
use tokio::task;

#[tokio::main]
async fn main() {
    let db = BlockchainDB::new("./db");

    // Simulate concurrent writes
    let handles: Vec<_> = (1..=5)
        .map(|i| {
            let db = db.clone();
            task::spawn(async move {
                let key = format!("block_{}", i);
                let value = format!(r#"{{"state": "data_{}"}}"#, i);
                db.put(&key, value.as_bytes()).await;
            })
        })
        .collect();

    for handle in handles {
        handle.await.unwrap();
    }

    // Simulate read
    let value = db.get("block_1").await.unwrap();
    println!("Fetched state: {:?}", String::from_utf8(value).unwrap());
}
