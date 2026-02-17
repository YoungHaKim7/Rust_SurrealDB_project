- https://surrealdb.com/learn/book/chapter-03

# SQL start할떄 pretty해야 보기 좋다.

```bash
surreal sql --user root --pass root --pretty
```

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

## import (파일을 surrealDB에 넣어주는건 import로 바꿔주면됨.)

```bash
$ surreal import --user root --password root --namespace test_name --database test_db export.surql
2026-02-17T06:55:43.556903Z ERROR surrealdb_server::cli::import: Surreal import failed, import might only be partially completed or have failed entirely.
2026-02-17T06:55:43.556965Z ERROR surrealdb_server::cli: Thrown error: Database record `person:aeon` already exists
```

