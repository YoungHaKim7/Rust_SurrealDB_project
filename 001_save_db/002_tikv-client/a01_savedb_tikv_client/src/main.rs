use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use tikv_client::TransactionClient;

// Connect to SurrealDB (which uses TiKV as storage)
async fn connect_surrealdb() -> Result<(), Box<dyn std::error::Error>> {
    let client = Surreal::new::<Ws>("127.0.0.1:8000").await?;

    // Sign in as root
    client
        .signin(Root {
            username: "root",
            password: "root",
        })
        .await?;

    // Select namespace and database
    client.use_ns("test").use_db("test").await?;

    // Create a record
    let result: Option<Vec<surrealdb::sql::Value>> = client
        .create("person")
        .content(serde_json::json!({
            "name": "John Doe",
            "age": 30
        }))
        .await?;

    let _created: Vec<surrealdb::sql::Value> = result.ok_or("Failed to create record")?;

    println!("✓ SurrealDB operation completed");

    Ok(())
}

// Direct TiKV client access (for raw key-value operations)
async fn direct_tikv_access() -> Result<(), Box<dyn std::error::Error>> {
    let txn_client = TransactionClient::new(vec!["127.0.0.1:2379"]).await?;
    let mut txn = txn_client.begin_optimistic().await?;

    // Direct TiKV operations
    txn.put("my_key".to_owned(), "my_value".to_owned()).await?;
    let value = txn.get("my_key".to_owned()).await?;
    println!("✓ TiKV direct access got value: {:?}", value);

    txn.commit().await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== SurrealDB + TiKV Client Demo ===\n");

    // Option 1: Use SurrealDB (recommended - SQL-like queries)
    println!("1. Connecting to SurrealDB...");
    if let Err(e) = connect_surrealdb().await {
        println!("  SurrealDB error: {} (make sure SurrealDB is running)", e);
    }

    // Option 2: Direct TiKV access (raw key-value)
    println!("\n2. Direct TiKV access...");
    if let Err(e) = direct_tikv_access().await {
        println!("  TiKV error: {} (make sure TiKV is running)", e);
    }

    Ok(())
}
