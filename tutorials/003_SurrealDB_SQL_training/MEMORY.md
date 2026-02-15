# SurrealDB Project Memory

## Project Context

SurrealDB is a multi-model database (document, graph, relational, time-series, geospatial, KV) built in Rust.

## Key Commands

```bash
# Start dev server (in-memory)
cargo run --no-default-features --features storage-mem,http,scripting -- start --log trace --user root --pass root memory

# Format code (REQUIRED before commits)
cargo make fmt

# Run language tests
cd language-tests && cargo run run

# Run specific language test (paths relative to language-tests/tests/)
cd language-tests && cargo run run -- --test path/to/test.surql

# Auto-generate test results (test file must be empty)
cd language-tests && cargo run run -- --results accept path/to/test.surql
```

## Cheat Sheet

A comprehensive SurrealDB cheat sheet is available at `SURREALDB.md` in the project root, covering CLI commands, SurrealQL syntax, functions, and all major features.
