use serde::{Deserialize, Serialize};
use surrealdb::Surreal;
use surrealdb::engine::local::RocksDb;

// Define a struct that can be serialized/deserialized
#[derive(Debug, Serialize, Deserialize)]
struct Person {
    title: String,
    name: String,
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    // Specify the path for the RocksDB storage. This creates a folder named "tempdb"
    // in the current directory to store the data on disk.
    let db = Surreal::new::<RocksDb>("tempdb").await?;

    // Use a specific namespace and database
    db.use_ns("test_namespace").use_db("test_db").await?;

    // Create a new person record
    let person = Person {
        title: "Mr.".to_string(),
        name: "John Doe".to_string(),
    };

    // Save the data to the database
    // The `create` method serializes the struct and stores it persistently
    let created: Option<Person> = db.create("person").content(person).await?;

    println!("Created person: {:?}", created);

    // Fetch the data back to verify persistence
    let fetched: Vec<Person> = db.select("person").await?;
    println!("Fetched people: {:?}", fetched);

    Ok(())
}
