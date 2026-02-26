use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    // Connect to the server
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;

    // Signin as a namespace, database, or root user
    db.signin(Root {
        username: "root".to_string(),
        password: "secret".to_string(),
    })
    .await?;

    // Select a specific namespace / database
    db.use_ns("test").use_db("test").await?;

    let sql_content = std::fs::read_to_string("./assets/input_sql_file/input_db_data.surql")
        .expect("input SQL file, error");
    let some_queries = db.query(sql_content).await?;
    dbg!(some_queries);
    Ok(())
}
