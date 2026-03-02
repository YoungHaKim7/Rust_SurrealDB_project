use surrealdb::{Surreal, engine::remote::ws::Ws, opt::auth::Root};

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;

    db.signin(Root {
        username: "root".to_string(),
        password: "root".to_string(),
    })
    .await?;
    // Select the namespace/database to use
    // Note: For in-memory databases, no authentication is required
    db.use_ns("namespace").use_db("database").await?;

    // Define the user record access
    let surql = r#"
-- ------------------------------
-- OPTION
-- ------------------------------

OPTION IMPORT;

-- ------------------------------
-- TABLE: person
-- ------------------------------

DEFINE TABLE person TYPE ANY SCHEMALESS PERMISSIONS NONE;




-- ------------------------------
-- TABLE DATA: person
-- ------------------------------

INSERT [ { id: person:aeon } ];
"#;
    db.query(surql).await?.check()?;

    dbg!(surql);

    // Sign a user up using named parameters

    println!("successful!");

    Ok(())
}
