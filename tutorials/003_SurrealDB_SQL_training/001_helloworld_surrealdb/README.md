# db store

```bash
$ surreal sql --user root --pass root

#
#  Welcome to the SurrealDB SQL shell
#
#  How to use this shell:
#    - Different statements within a query should be separated by a (;) semicolon.
#    - To create a multi-line query, end your lines with a (\) backslash, and press enter.
#    - To exit, send a SIGTERM or press CTRL+C
#
#  Consult https://surrealdb.com/docs/cli/sql for further instructions
#
#  SurrealDB version: 3.0.0-nightly+20260215.2d16ea0
#
		
main/main> USE NS test_db
[{ database: 'main', namespace: 'test_db' }]

test_db/main> USE DB test_store
[{ database: 'test_store', namespace: 'test_db' }]

test_db/test_store> CREATE users CONTENT {
  name: "Alice Smith",
  email: "alice@example.com",
  age: 29,
  addresses: [
      {
          type: "home",
          address_line: "123 Maple St",
          city: "Springfield",
          country: "USA"
      },
      {
          type: "work",
          address_line: "456 Oak Ave",
          city: "Metropolis",
          country: "USA"
      }
  ]
};

test_db/test_store> select * from users;
[[{ addresses: [{ address_line: '123 Maple St', city: 'Springfield', country: 'USA', type: 'home' }, { address_line: '456 Oak Ave', city: 'Metropolis', country: 'USA', type: 'work' }], age: 29, email: 'alice@example.com', id: users:bcf9wg1s7f1gox2ewboo, name: 'Alice Smith' }]]

test_db/test_store>
```
