- https://surrealdb.com/docs/surrealql/statements/info

# local server 구동할때 아이디 비번 다 설정하고 들어가야함

```bash
$ surreal start --user root --pass 1

 .d8888b.                                             888 8888888b.  888888b.
d88P  Y88b                                            888 888  'Y88b 888  '88b
Y88b.                                                 888 888    888 888  .88P
 'Y888b.   888  888 888d888 888d888  .d88b.   8888b.  888 888    888 8888888K.
    'Y88b. 888  888 888P'   888P'   d8P  Y8b     '88b 888 888    888 888  'Y88b
      '888 888  888 888     888     88888888 .d888888 888 888    888 888    888
Y88b  d88P Y88b 888 888     888     Y8b.     888  888 888 888  .d88P 888   d88P
 'Y8888P'   'Y88888 888     888      'Y8888  'Y888888 888 8888888P'  8888888P'


2026-02-15T15:27:56.572847Z  INFO surreal::env: Running 2.6.1 for macos on aarch64
2026-02-15T15:27:56.572998Z  INFO surrealdb::core::kvs::ds: Starting kvs store in memory
2026-02-15T15:27:56.573061Z  INFO surrealdb::core::kvs::ds: Started kvs store in memory
2026-02-15T15:27:56.574485Z  INFO surreal::dbs: Initialising credentials user=root
2026-02-15T15:27:56.574580Z  INFO surrealdb::core::kvs::ds: Credentials were provided, and no root users were found. The root user 'root' will be created
2026-02-15T15:27:56.621087Z  INFO surrealdb::net: Started web server on 127.0.0.1:8000
2026-02-15T15:27:56.621099Z  INFO surrealdb::net: Listening for a system shutdown signal.

```


```bash
$ surreal sql --user root --pass 1

#
#  Welcome to the SurrealDB SQL shell
#
#  How to use this shell:
#    - Different statements within a query should be separated by a (;) semicolon.
#    - To create a multi-line query, end your lines with a (\) backslash, and press
#    - To exit, send a SIGTERM or press CTRL+C
#
#  Consult https://surrealdb.com/docs/cli/sql for further instructions
#
#  SurrealDB version: 2.6.1
#

> DEFINE USER root ON ROOT PASSWORD '1';
["The root user 'root' already exists"]

> USE NS test DB test;
[NONE]

test/test> CREATE category SET name = 'Technology', created_at = time::now();
[[{ created_at: d'2026-02-15T15:29:12.291572Z', id: category:846lh141sao1i93bt217, name: 'Technology' }]]

test/test> INFO FOR DB;                                                             [{ accesses: {  }, analyzers: {  }, apis: {  }, configs: {  }, functions: {  }, models: {  }, params: {  }, tables: { category: 'DEFINE TABLE category TYPE ANY SCHEMALESS PERMISSIONS NONE' }, users: {  } }]

test/test> INFO FOR NS;
[{ accesses: {  }, databases: { test: 'DEFINE DATABASE test' }, users: {  } }]

test/test> INFO FOR ROOT;
[{ accesses: {  }, namespaces: { test: 'DEFINE NAMESPACE test' }, nodes: { "bdec37bb-6ee6-4bb0-80a1-14e22bd5db79": 'NODE bdec37bb-6ee6-4bb0-80a1-14e22bd5db79 SEEN 1771169453639 ACTIVE' }, system: { available_parallelism: 8, cpu_usage: 0.08978643268346786f, load_average: [3.98681640625f, 2.89013671875f, 2.5029296875f], memory_allocated: 13226633, memory_usage: 32882688, physical_cores: 8, threads: 20 }, users: { root: "DEFINE USER root ON ROOT PASSHASH '$argon2id$v=19$m=19456,t=2,p=1$kmY2cdU12IAtgkQr/1Vd6A$HwB6LphDmRROXFmTJf9nQ/gMCwLdtuZ/EywCjJJHB4k' ROLES OWNER DURATION FOR TOKEN 1h, FOR SESSION NONE" } }]

test/test>

```
