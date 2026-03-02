use surrealdb::engine::local::SurrealKv;
use surrealdb::{Error, Surreal};

// Install at https://surrealdb.com/install
// and use `surreal start --user root --pass root surrealkv://mydb`
// to start a working database to take the following queries

// See the results via `surreal sql --ns namespace --db database --pretty`
// or https://surrealist.app/
// followed by the query `SELECT * FROM person;`

#[tokio::main]
async fn main() -> Result<(), Error> {
    let db = Surreal::new::<SurrealKv>("mydb").await?;

    // For embedded SurrealKv, authentication is optional
    // Skip signin for local embedded database

    // Select a specific namespace / database
    db.use_ns("namespace").use_db("database").await?;

    // Create a new person using SQL query
    db.query(r#"
        CREATE person SET
            title = 'Founder & CEO',
            first_name = 'Tobie',
            last_name = 'Morgan Hitchcock',
            marketing = true
    "#).await?;

    // Create a new person with a specific ID using SQL
    db.query(r#"
        CREATE person:jaime SET
            title = 'Founder & COO',
            first_name = 'Jaime',
            last_name = 'Morgan Hitchcock',
            marketing = false
    "#).await?;

    // Update a person record with a specific ID using SQL
    db.query(r#"
        UPDATE person:jaime SET marketing = true
    "#).await?;

    // Select all people records using SQL
    let _people = db.query("SELECT * FROM person").await?;

    // Perform a custom advanced query
    let _groups = db
        .query("SELECT * FROM person WHERE marketing = true")
        .await?;

    Ok(())
}
