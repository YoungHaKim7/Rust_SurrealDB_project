# `Cargo.toml` 패턴

```toml
[dependencies]
surrealdb = "3.0.1"
tokio = { version = "1.49.0", features = ["macros", "rt-multi-thread"] }

[profile.release]
lto = true
strip = true
opt-level = 3
panic = 'abort'
codegen-units = 1
```


- axum
  - The serde crate will also need a feature flag for its Serialize and Deserialize macros. Your cargo.toml dependencies should look like this:
- https://surrealdb.com/docs/sdk/rust/frameworks/axum

```toml
axum = "0.8.8"
fake = "4.4.0"
serde = { version = "1.0.228", features = ["derive"] }
surrealdb = "3.0.0-beta.2"
thiserror = "2.0.18"
tokio = "1.49.0"
```

# SurrealDB자체 저장하기
- https://surrealdb.com/learn/book/chapter-03
- https://github.com/surrealdb/surrealdb/tree/main/surrealdb

- SDK
  - https://surrealdb.com/docs/sdk/rust

```bash
surreal start --user root --pass root
```

## export(저장된 파일을 뽑아내고 싶다면 id , pw 입력해 주면됨.

```bash
$ surreal export --user root --password root --namespace test_name --database test_db export.surql
2026-02-17T06:53:31.775350Z  INFO surrealdb_server::cli::export: The SurrealQL file was exported successfully
```

## import (파일을 surrealDB에 넣어주는건 import로 바꿔주면됨.)

```bash
$ surreal import --user root --password root --namespace test_name --database test_db export.surql
2026-02-17T06:55:43.556903Z ERROR surrealdb_server::cli::import: Surreal import failed, import might only be partially completed or have failed entirely.
2026-02-17T06:55:43.556965Z ERROR surrealdb_server::cli: Thrown error: Database record `person:aeon` already exists
```
