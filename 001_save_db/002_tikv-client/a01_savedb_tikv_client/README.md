# Result

```bash
❯ cargo r --release
    Finished `release` profile [optimized] target(s) in 0.47s
     Running `target/release/a01_savedb_tikv_client`
=== SurrealDB + TiKV Client Demo ===

1. Connecting to SurrealDB...
  SurrealDB error: There was an error processing a remote WS request: WebSocket protocol error: SubProtocol error: Server sent no subprotocol (make sure SurrealDB is running)

2. Direct TiKV access...
✓ TiKV direct access got value: Some([109, 121, 95, 118, 97, 108, 117, 101])

```

# fix2

- The code now compiles successfully. The fix was to handle the `Option` type returned by the `create()` method:

```rust
// Before: ❌ Using ? directly on Option
let _: Vec<surrealdb::sql::Value> = client
    .create("person")
    .content(serde_json::json!({ ... }))
    .await?;

// After: ✅ Convert Option to Result first
let result: Option<Vec<surrealdb::sql::Value>> = client
    .create("person")
    .content(serde_json::json!({ ... }))
    .await?;

let _created: Vec<surrealdb::sql::Value> = result.ok_or("Failed to create record")?;
```

The `.ok_or()` method converts the `Option` to a `Result`, allowing the `?` operator to work properly.

# fix

- Fixed! The issue was that the code was using `Client::new::<Ws>()` which doesn't exist. The correct way to initialize a SurrealDB client is to use `Surreal::new::<Ws>()`.

**Changes made:**
- Changed `use surrealdb::engine::remote::ws::{Client, Ws};` → `use surrealdb::engine::remote::ws::Ws;`
- Added `use surrealdb::Surreal;`
- Changed `Client::new::<Ws>("127.0.0.1:8000")` → `Surreal::new::<Ws>("127.0.0.1:8000")`

The code should now compile correctly.


## Complete Setup Summary

### Quick Start Commands

```bash
# 1. Start TiKV (required by SurrealDB as storage backend)
docker run -d --name pd -p 2379:2379 pingcap/pd:latest --name=pd --data-dir=/pd --client-urls=http://0.0.0.0:2379

docker run -d --name tikv -p 20160:20160 pingcap/tikv:latest --addr 0.0.0.0:20160 --advertise-addr 127.0.0.1:20160 --pd 127.0.0.1:2379

# 2. Start SurrealDB with TiKV backend
surreal start tikv://127.0.0.1:2379 --user root --pass root --namespace test --database test

# 3. In another terminal, run your Rust app
cargo run
```

### What this architecture gives you:

| Component | Purpose |
|-----------|---------|
| **SurrealDB (via `surrealdb` crate)** | SQL-like queries, relationships, auth, graph queries |
| **TiKV (via `tikv-client` crate)** | Distributed key-value storage, transactions |

You typically use **one** approach:
- **Use SurrealDB** for application data (recommended)
- **Use TiKV directly** only for special low-level key-value needs
