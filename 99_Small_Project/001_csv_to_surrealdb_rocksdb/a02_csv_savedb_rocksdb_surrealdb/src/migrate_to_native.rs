use serde::{Deserialize, Serialize};
use surrealdb::Surreal;
use surrealdb::engine::local::{RocksDb, File};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
struct Customer {
    #[serde(rename = "Index")]
    index: u32,
    #[serde(rename = "Customer Id")]
    customer_id: String,
    #[serde(rename = "First Name")]
    first_name: String,
    #[serde(rename = "Last Name")]
    last_name: String,
    #[serde(rename = "Company")]
    company: String,
    #[serde(rename = "City")]
    city: String,
    #[serde(rename = "Country")]
    country: String,
    #[serde(rename = "Phone 1")]
    phone1: String,
    #[serde(rename = "Phone 2")]
    phone2: String,
    #[serde(rename = "Email")]
    email: String,
    #[serde(rename = "Subscription Date")]
    subscription_date: String,
    #[serde(rename = "Website")]
    website: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("=== Migrating from RocksDB to SurrealDB Native Storage ===\n");

    // Step 1: Connect to existing RocksDB database
    println!("Step 1: Connecting to RocksDB database...");
    let rocks_db = Surreal::new::<RocksDb>("customer_db").await?;
    rocks_db.use_ns("customer_ns").use_db("customer_db").await?;
    println!("✓ Connected to RocksDB\n");

    // Step 2: Read all customers from RocksDB
    println!("Step 2: Reading customers from RocksDB...");
    let customers: Vec<Customer> = rocks_db.select("customers").await?;
    println!("✓ Found {} customers\n", customers.len());

    // Step 3: Create new SurrealDB native storage
    println!("Step 3: Creating SurrealDB native storage database...");
    let native_db = Surreal::new::<File>("customer_native_db").await?;
    native_db.use_ns("customer_ns").use_db("customer_db").await?;
    println!("✓ Created native storage database\n");

    // Step 4: Migrate all customers to native storage
    println!("Step 4: Migrating customers to native storage...");
    let mut count = 0;

    for customer in customers {
        let _: Option<Customer> = native_db
            .create("customers")
            .content(customer)
            .await?;

        count += 1;
        if count % 100 == 0 {
            println!("  Migrated {} customers...", count);
        }
    }
    println!("✓ Migrated {} customers\n", count);

    // Step 5: Verify migration
    println!("Step 5: Verifying migration...");
    let native_customers: Vec<Customer> = native_db.select("customers").await?;
    println!("✓ Verified: {} customers in native storage\n", native_customers.len());

    // Display sample data
    println!("=== Sample customers from native storage ===");
    for (i, customer) in native_customers.iter().take(3).enumerate() {
        println!("{}. {} {} - {} - {}",
            i + 1,
            customer.first_name,
            customer.last_name,
            customer.email,
            customer.company
        );
    }

    println!("\n=== Migration Complete! ===");
    println!("Native database location: customer_native_db/");
    println!("You can now use the new database with:");
    println!("  let db = Surreal::new::<File>(\"customer_native_db\").await?;");

    Ok(())
}
