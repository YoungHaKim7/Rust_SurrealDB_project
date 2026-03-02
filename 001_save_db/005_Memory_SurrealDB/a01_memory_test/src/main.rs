use surrealdb::opt::auth::Record;

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let db = surrealdb::Surreal::new::<surrealdb::engine::local::Mem>(()).await?;

    // Select the namespace/database to use
    // Note: For in-memory databases, no authentication is required
    db.use_ns("namespace").use_db("database").await?;

    // Define the user record access
    let surql = r#"
    DEFINE TABLE user SCHEMAFULL
    	PERMISSIONS
    		FOR select, update, delete WHERE id = $auth.id;

    DEFINE FIELD name ON user TYPE string;
    DEFINE FIELD email ON user TYPE string ASSERT string::is_email($value);
    DEFINE FIELD password ON user TYPE string;

    DEFINE INDEX email ON user FIELDS email UNIQUE;

    DEFINE ACCESS user ON DATABASE TYPE RECORD
    	SIGNIN (
    		SELECT * FROM user WHERE email = $email AND crypto::argon2::compare(password, $password)
    	)
    	SIGNUP (
    		CREATE user CONTENT {
    			name: $name,
    			email: $email,
    			password: crypto::argon2::generate($password)
    		}
    	);
"#;
    db.query(surql).await?.check()?;

    // Sign a user up using named parameters
    db.signup(Record {
        namespace: "namespace".to_string(),
        database: "database".to_string(),
        access: "user".to_string(),
        params: serde_json::json!({
            "name": "John Doe",
            "email": "john.doe@example.com",
            "password": "1234"
        }),
    })
    .await?;

    println!("User signup successful!");

    Ok(())
}
