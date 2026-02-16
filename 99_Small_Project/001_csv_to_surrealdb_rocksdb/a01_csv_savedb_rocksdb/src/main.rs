use serde::{Deserialize, Serialize};
use surrealdb::Surreal;
use surrealdb::engine::local::RocksDb;
use std::error::Error;

// Define a struct to match the CSV structure
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
    // Specify the path for the RocksDB storage
    let db = Surreal::new::<RocksDb>("customer_db").await?;

    // Use a specific namespace and database
    db.use_ns("customer_ns").use_db("customer_db").await?;

    // Path to the CSV file
    let csv_path = "customers-1000.csv";

    // Read the CSV file
    let mut rdr = csv::Reader::from_path(csv_path)?;

    println!("Reading CSV file and importing to SurrealDB with RocksDB...");

    let mut count = 0;
    for result in rdr.deserialize() {
        let customer: Customer = result?;

        // Save each customer to SurrealDB
        let _: Option<Customer> = db
            .create("customers")
            .content(customer)
            .await?;

        count += 1;
        if count % 100 == 0 {
            println!("Imported {} customers...", count);
        }
    }

    println!("Successfully imported {} customers to SurrealDB!", count);

    // Verify by fetching a few records
    let customers: Vec<Customer> = db.select("customers").await?;
    println!("Total customers in database: {}", customers.len());

    // Display first 3 customers as a sample
    println!("\nSample customers:");
    for (i, customer) in customers.iter().take(3).enumerate() {
        println!("{}. {} {} - {} - {}",
            i + 1,
            customer.first_name,
            customer.last_name,
            customer.email,
            customer.company
        );
    }

    Ok(())
}
