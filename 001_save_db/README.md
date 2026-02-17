# 저장되는 폴더

- [Where is the database stored in surrealDB?](https://stackoverflow.com/questions/73869506/where-is-the-database-stored-in-surrealdb)

- backup & data export
  - https://surrealdb.com/docs/cloud/operate-and-manage/data-export-and-backup

# SurrealDB자체 저장하기
- https://surrealdb.com/learn/book/chapter-03

```bash
surreal start --user root --pass root surrealkv://mydb
```

## export(저장된 파일을 뽑아내고 싶다면 id , pw 입력해 주면됨.

```bash
$ surreal export --user root --password root --namespace test_name --database test_db export.surql
2026-02-17T06:53:31.775350Z  INFO surrealdb_server::cli::export: The SurrealQL file was exported successfully
```

# A low-level, versioned, embedded, ACID-compliant, key-value database for Rust 
- https://github.com/surrealdb/surrealkv
