use serde_json::json;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::{Error, Surreal};

// Install at https://surrealdb.com/install
// and use `surreal start --user root --pass root`
// to start a working database to take the following queries

// See the results via `surreal sql --ns namespace --db database --pretty`
// or https://surrealist.app/
// followed by the query `SELECT * FROM person;`

#[tokio::main]
async fn main() -> Result<(), Error> {
    let db = Surreal::new::<Ws>("localhost:8000").await?;

    // Signin as a namespace, database, or root user
    db.signin(Root {
        username: "root".to_string(),
        password: "root".to_string(),
    })
    .await?;

    // Select a specific namespace / database
    db.use_ns("namespace").use_db("database").await?;

    // Create a new person with a random ID
    let _created: Option<serde_json::Value> = db
        .create("person")
        .content(json!({
            "title": "Founder & CEO",
            "name": {
                "first": "Tobie",
                "last": "Morgan Hitchcock"
            },
            "marketing": true
        }))
        .await?;

    // Create a new person with a specific ID
    let _created: Option<serde_json::Value> = db
        .create(("person", "jaime"))
        .content(json!({
            "title": "Founder & COO",
            "name": {
                "first": "Jaime",
                "last": "Morgan Hitchcock"
            },
            "marketing": false
        }))
        .await?;

    // Update a person record with a specific ID
    let _updated: Option<serde_json::Value> = db
        .update(("person", "jaime"))
        .merge(json!({"marketing": true}))
        .await?;

    // Select all people records
    let _people: Vec<serde_json::Value> = db.select("person").await?;

    // Perform a custom advanced query
    let query = r#"
        SELECT marketing, count()
        FROM type::table($table)
        GROUP BY marketing
    "#;

    let _groups = db.query(query).bind(("table", "person")).await?;

    Ok(())
}
