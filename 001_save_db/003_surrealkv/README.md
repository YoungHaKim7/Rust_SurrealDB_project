# suurealkv실행 안 해 주고 `cargo r --release` 해도 파일 생긴다

- 완전 신기

```bash
cargo r --release
```

- 이후 데이터 잘 들어갔는지 확인하면 된다.

```bash
$ surreal sql --user root --pass root --ns namespace --db database --pretty
```

# SuurealDB자체 DB

- https://surrealdb.com/learn/book/chapter-03
```bash
surreal start --user root --pass root surrealkv://mydb

```

# 예시 굿
- https://docs.rs/surrealdb/latest/surrealdb/engine/remote/index.html

