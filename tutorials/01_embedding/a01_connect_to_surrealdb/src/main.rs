use surrealdb::{engine::remote::ws::Ws, Surreal};

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    // Create database connection using RocksDB
    let db = Surreal::new::<Ws>("localhost:8000").await?;
    Ok(())
}
