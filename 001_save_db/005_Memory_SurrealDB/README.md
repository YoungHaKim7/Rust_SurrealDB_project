# memory

```bash
surreal start --user root --pass root memory

```

- `info for ns;` & `info for db;`

```spl
INFO FOR NS;
-- Query 1 (execution time: 223.916µs)
{
	accesses: {  },
	databases: {
		database: 'DEFINE DATABASE database'
	},
	users: {  }
}

namespace/database> INFO FOR DB;
-- Query 1 (execution time: 429.042µs)
{
	accesses: {
		user_access: "DEFINE ACCESS user_access ON DATABASE TYPE RECORD SIGNUP (CREATE user SET email = $email, password = crypto::argon2::generate($password)) SIGNIN (SELECT * FROM user WHERE email = $email AND crypto::argon2::compare(password, $password)) WITH JWT ALGORITHM HS512 KEY '[REDACTED]' WITH ISSUER KEY '[REDACTED]' DURATION FOR TOKEN 1h, FOR SESSION NONE"
	},
	analyzers: {  },
	apis: {  },
	buckets: {  },
	configs: {  },
	functions: {  },
	models: {  },
	modules: {  },
	params: {  },
	sequences: {  },
	tables: {  },
	users: {  }
}
```

# Authentication
- https://surrealdb.com/docs/surrealdb/security/authentication
