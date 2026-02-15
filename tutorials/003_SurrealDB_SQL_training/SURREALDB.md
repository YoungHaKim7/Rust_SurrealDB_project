# SurrealDB Cheat Sheet

## Quick Start

Magic words:
```bash
# Start in-memory database
surreal start --log trace --user root --pass root memory

# Start with file storage
surreal start --user root --pass root file://./data.db

# Connect to SQL shell
surreal sql --endpoint http://localhost:8000 --user root --pass root --namespace test --database test
```

---

## CLI Commands

### Main Command Structure
```bash
surreal [OPTIONS] <COMMAND>
```

### Global Options
| Flag | Description |
|------|-------------|
| `--log, -l <LEVEL>` | Set logging level (trace, debug, info, warn, error) |
| `--log-format <FORMAT>` | Log output format (text, json) |
| `--no-banner` | Hide startup banner |

### Subcommands

#### Start Server
```bash
surreal start [OPTIONS] [PATH]
```
| Flag | Description |
|------|-------------|
| `--path, -p <PATH>` | Database path (default: memory) |
| `--username, -u <USER>` | Initial root username |
| `--password, -P <PASS>` | Initial root password |
| `--bind, -b <ADDR:PORT>` | Bind address (default: 127.0.0.1:8000) |
| `--key <KEY>` | Encryption key |

#### SQL Shell
```bash
surreal sql [OPTIONS]
```
| Flag | Description |
|------|-------------|
| `--endpoint <URL>` | Database connection endpoint |
| `--username, -u <USER>` | Username for authentication |
| `--password, -p <PASS>` | Password for authentication |
| `--namespace, -n <NS>` | Namespace to use |
| `--database, -d <DB>` | Database to use |
| `--pretty` | Pretty print results |
| `--json` | Output in JSON format |
| `--multi` | Multi-line mode (backslash continuation) |
| `--hide-welcome` | Hide welcome message |

#### Import/Export
```bash
surreal import [OPTIONS] <FILE>
surreal export [OPTIONS] <FILE>
```

#### Other Commands
```bash
surreal version     # Show version
surreal upgrade     # Check for upgrades
surreal help        # Show help
```

---

## Configuration

### Environment Variables
```bash
export SURREAL_PATH="file://./data.db"
export SURREAL_USER="root"
export SURREAL_PASS="root"
export SURREAL_KEY="encryption_key"
export SURREAL_LOG="trace"
export SURREAL_BIND="0.0.0.0:8000"
```

### Configuration File (`surreal.toml`)
```toml
[server]
bind = "0.0.0.0:8000"
no_identification_headers = false

[database]
path = "file://./data.db"

[engine]
node_membership_refresh_interval = "3s"
changefeed_gc_interval = "30s"

[auth]
root_user = "root"
root_password = "password"
```

---

## SurrealQL Basics

### Data Types
| Type | Description | Example |
|------|-------------|---------|
| `bool` | Boolean | `true`, `false` |
| `int` | Integer | `42`, `-7` |
| `float` | Floating point | `3.14`, `-0.5` |
| `string` | Text | `"hello"` |
| `null` | Null value | `null` |
| `none` | None value | `NONE` |
| `array` | List | `[1, 2, 3]` |
| `object` | Key-value | `{a: 1, b: 2}` |
| `record` | Record reference | `user:tobie` |
| `datetime` | Date/time | `time::now()` |
| `duration` | Time duration | `1d`, `2h30m` |
| `uuid` | UUID | `uuid::v4()` |
| `geometry` | Geo shape | `geo::point(0, 0)` |

### Record ID Format
```
namespace:database:table:id    # Full path
namespace:table:id             # With namespace
table:id                       # Simple (uses current ns/db)
```

---

## CRUD Operations

### CREATE
```sql
-- Simple create
CREATE person SET name = "Tobie", age = 30;

-- Create with specific ID
CREATE person:tobie SET name = "Tobie", age = 30;

-- Create with relation
CREATE user:tobie->friend->user:john SET since = "2024-01-01";

-- Batch create
CREATE person:tobie, person:john SET name = "Tobie", age = 30;
```

### SELECT
```sql
-- Select all
SELECT * FROM person;

-- Select specific fields
SELECT name, age FROM person;

-- With WHERE clause
SELECT * FROM person WHERE age > 25;

-- With ONLY (exclude metadata)
SELECT * FROM person ONLY name, age;

-- With ORDER BY
SELECT * FROM person ORDER BY age DESC;

-- With LIMIT/OFFSET
SELECT * FROM person LIMIT 10 OFFSET 20;

-- Aggregation
SELECT count() FROM person;
SELECT array::avg(age) FROM person;
```

### UPDATE
```sql
-- Update specific record
UPDATE person:tobie SET age = 31;

-- Conditional update
UPDATE person SET age += 1 WHERE age > 25;

-- Patch update
UPDATE person:tobie PATCH {name: "New Name", age: 31};

-- Merge update
UPDATE person:tobie MERGE {city: "London"};
```

### UPSERT (Create or Update)
```sql
UPSERT person:tobie SET name = "Tobie", age = 31;
```

### DELETE
```sql
-- Delete specific record
DELETE FROM person:tobie;

-- Conditional delete
DELETE FROM person WHERE age < 18;

-- Delete all
DELETE FROM person;
```

---

## Schema Definition

### Namespace & Database
```sql
-- Define namespace
DEFINE NAMESPACE myapp;

-- Use namespace
USE NAMESPACE myapp;

-- Define database
DEFINE DATABASE mydb;

-- Use database
USE DATABASE mydb;
```

### Tables
```sql
-- Schemaful (strict schema)
DEFINE TABLE user SCHEMAFUL;

-- Schemaless (flexible schema)
DEFINE TABLE post SCHEMALESS;

-- Schemaless with specific permission
DEFINE TABLE log SCHEMALESS PERMISSIONS FULL;
```

### Fields
```sql
-- Define field
DEFINE FIELD name ON user TYPE string;
DEFINE FIELD email ON user TYPE string UNIQUE;
DEFINE FIELD age ON user TYPE int;
DEFINE FIELD created_at ON user TYPE datetime DEFAULT time::now();

-- Computed field
DEFINE FIELD full_name ON user TYPE string VALUE first_name + " " + last_name;

-- Flexible field (any type)
DEFINE FIELD metadata ON user FLEXIBLE;
```

### Indexes
```sql
-- Basic index
DEFINE INDEX email_idx ON user FIELDS email;

-- Unique index
DEFINE INDEX email_unique ON user FIELDS email UNIQUE;

-- Multi-field index
DEFINE INDEX name_age_idx ON user FIELDS name, age;

-- Full-text search index
DEFINE INDEX search_idx ON post FIELDS content FULLTEXT
    ANALYZER stem
    SCORING bm25;

-- Vector/HNSW index (for KNN search)
DEFINE INDEX vector_idx ON product
    FIELDS embedding
    HNSW
    DIMENSION 768
    DISTANCE cosine
    VECTOR_TYPE float32
    M 16
    EF_CONSTRUCTION 40;
```

---

## Authentication & Authorization

### Authentication Levels
| Level | Scope |
|-------|-------|
| `root` | Full system access |
| `namespace` | Access to specific namespace |
| `database` | Access to specific database |
| `record` | Access to specific records (record-based auth) |

### Authentication Commands
```sql
-- Sign in with credentials
SIGNIN USER root PASSWORD root;

-- Sign in with token
AUTHENTICATE TOKEN "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...";

-- Sign out
SIGNOUT;

-- Check current auth
INFO AUTH;
```

### User Management
```sql
-- Define user at namespace level
DEFINE USER admin
    PASSWORD 'password'
    NAMESPACE myapp;

-- Define user at database level
DEFINE USER app_user
    PASSWORD 'password'
    NAMESPACE myapp
    DATABASE mydb;

-- Define user with role
DEFINE USER reader
    PASSWORD 'password'
    ROLE reader;

-- List users
SHOW USERS;
```

### Permissions
```sql
-- Define table-level permissions
DEFINE ACCESS ON person FOR select, create;

-- Define custom permission
DEFINE ACCESS ON person
    FOR
        SELECT WHERE age >= 18,
        UPDATE WHERE owner = $auth.id,
        DELETE WHERE role = "admin";

-- Grant access
GRANT SELECT ON person TO user:john;
```

---

## Built-in Functions

### String Functions
```sql
string::len("hello")              -- 5
string::upper("hello")            -- "HELLO"
string::lower("HELLO")            -- "hello"
string::trim("  hello  ")         -- "hello"
string::contains("hello", "ell")  -- true
string::replace("hello", "l", "x")-- "hexxo"
string::split("a,b,c", ",")       -- ["a", "b", "c"]
string::join(["a", "b"], ",")     -- "a,b"
string::slug("Hello World")       -- "hello-world"
string::repeat("ab", 3)           -- "ababab"
string::reverse("hello")          -- "olleh"
```

### Math Functions
```sql
math::add(5, 3)        -- 8
math::sub(5, 3)        -- 2
math::mul(5, 3)        -- 15
math::div(5, 3)        -- 1.666...
math::mod(5, 3)        -- 2
math::pow(2, 3)        -- 8
math::sqrt(16)         -- 4
math::abs(-5)          -- 5
math::round(3.7)       -- 4
math::floor(3.7)       -- 3
math::ceil(3.2)        -- 4
math::min(5, 3)        -- 3
math::max(5, 3)        -- 5
```

### Array Functions
```sql
array::len([1, 2, 3])           -- 3
array::first([1, 2, 3])         -- 1
array::last([1, 2, 3])          -- 3
array::push([1, 2], 3)          -- [1, 2, 3]
array::pop([1, 2, 3])           -- [1, 2]
array::concat([1, 2], [3, 4])   -- [1, 2, 3, 4]
array::reverse([1, 2, 3])       -- [3, 2, 1]
array::sort_asc([3, 1, 2])      -- [1, 2, 3]
array::sort_desc([3, 1, 2])     -- [3, 2, 1]
array::unique([1, 2, 2, 3])     -- [1, 2, 3]
array::flatten([[1, 2], [3]])   -- [1, 2, 3]
array::sum([1, 2, 3])           -- 6
array::avg([1, 2, 3])           -- 2
array::min([1, 2, 3])           -- 1
array::max([1, 2, 3])           -- 3
array::median([1, 2, 3])        -- 2
```

### Object Functions
```sql
object::keys({a: 1, b: 2})      -- ["a", "b"]
object::values({a: 1, b: 2})    -- [1, 2]
object::merge({a: 1}, {b: 2})   -- {a: 1, b: 2}
object::len({a: 1, b: 2})       -- 2
```

### Time Functions
```sql
time::now()                              -- Current datetime
time::datetime("2024-01-01")             -- Parse datetime
time::duration("1d")                     -- 1 day duration
time::diff("2024-01-02", "2024-01-01")   -- 1 day
time::format("2024-01-01", "%Y-%m-%d")   -- "2024-01-01"
time::group("2024-01-01", "y")           -- Group by year
time::floor(time::now(), "d")            -- Start of day
time::ceil(time::now(), "d")             -- End of day
```

### Crypto Functions
```sql
crypto::md5("hello")        -- MD5 hash
crypto::sha1("hello")       -- SHA1 hash
crypto::sha256("hello")     -- SHA256 hash
crypto::sha512("hello")     -- SHA512 hash
crypto::blake3("hello")     -- Blake3 hash
crypto::joaat("hello")      -- JOAAT hash
```

### UUID Functions
```sql
uuid::v4()                  -- Random UUID
uuid::v7()                  -- Time-sorted UUID
uuid::rand()                -- Random UUID (alias)
```

### Geo Functions
```sql
geo::point(40.7128, -74.0060)           -- Create point
geo::distance(p1, p2)                    -- Distance between points
geo::within(point, circle)               -- Check if within area
geo::intersects(shape1, shape2)          -- Check intersection
```

---

## Graph Operations

### RELATE (Create Edges)
```sql
-- Simple relation
RELATE user:tobie->friend->user:john;

-- Relation with properties
RELATE user:tobie->friend->user:john
    SET since = "2024-01-01", strength = 0.8;

-- Multiple relations
RELATE user:tobie->friend->user:john,
       user:tobie->friend->user:jane;
```

### Query Graph
```sql
-- Direct relation
SELECT * FROM user:tobie->friend->user;

-- Traverse multiple levels
SELECT * FROM user->friend->friend->user;

-- With relation properties
SELECT *, friend.since FROM user:tobie->friend->user;

-- Recursive query
SELECT * FROM user WHERE ->friend->(user WHERE id = user:john);
```

---

## Time Series

### Versioning
```sql
-- Automatic versioning on every update
CREATE document SET content = "Version 1";
UPDATE document SET content = "Version 2";

-- Query current version
SELECT * FROM document;

-- Query specific version
SELECT * FROM document VERSION 1;

-- Query historical range
SELECT * FROM document SINCE "2024-01-01";
SELECT * FROM document BETWEEN "2024-01-01" AND "2024-12-31";
```

---

## Full-Text Search

### Create Full-Text Index
```sql
DEFINE INDEX search_idx ON articles
    FIELDS content
    FULLTEXT
    ANALYZER stem
    SCORING bm25;
```

### Search
```sql
-- Simple search
SELECT * FROM articles WHERE content SEARCH "database";

-- With scoring
SELECT *, score() FROM articles WHERE content SEARCH "database" ORDER BY score() DESC;

-- Multiple fields
DEFINE INDEX multi_search ON articles
    FIELDS title, content
    FULLTEXT
    ANALYZER stem
    SCORING bm25;
```

---

## Vector Search (KNN)

### Create Vector Index
```sql
DEFINE INDEX vector_idx ON products
    FIELDS embedding
    HNSW
    DIMENSION 768
    DISTANCE cosine
    VECTOR_TYPE float32
    M 16
    EF_CONSTRUCTION 40;
```

### KNN Search
```sql
-- KNN query
SELECT * FROM products
    WHERE embedding KNN $query_vector
    LIMIT 10
    DISTANCE cosine;

-- With distance result
SELECT *, distance(embedding, $query_vector, "cosine") as dist
    FROM products
    WHERE embedding KNN $query_vector
    ORDER BY dist ASC
    LIMIT 10;
```

---

## Geospatial Queries

### Store Geometry
```sql
-- Point
CREATE location SET point = geo::point(40.7128, -74.0060);

-- Line
CREATE route SET line = geo::line([
    geo::point(40.7128, -74.0060),
    geo::point(40.7306, -73.9352)
]);

-- Polygon
CREATE area SET polygon = geo::polygon([
    geo::point(40.7128, -74.0060),
    geo::point(40.7306, -73.9352),
    geo::point(40.7000, -73.9000)
]);
```

### Spatial Queries
```sql
-- Distance query
SELECT * FROM location
    WHERE geo::distance(point, geo::point(40, -74)) < 10000;

-- Within area
SELECT * FROM location
    WHERE point WITHIN geo::circle([40.7128, -74.0060], 1000);

-- Intersects
SELECT * FROM area
    WHERE polygon INTERSECTS geo::polygon([...]);
```

---

## Transactions

```sql
-- Transaction block
BEGIN TRANSACTION;

CREATE user SET name = "Tobie";
CREATE user SET name = "John";

COMMIT TRANSACTION;

-- Or rollback
-- CANCEL TRANSACTION;
```

---

## Live Queries

```sql
-- Real-time updates
LIVE SELECT * FROM person WHERE age > 25;

-- Live with specific fields
LIVE SELECT name, email FROM person;

-- Live aggregations
LIVE SELECT count() FROM person;
```

---

## Informational Commands

```sql
-- Show databases
SHOW DATABASES;

-- Show tables
SHOW TABLES;

-- Show indexes
SHOW INDEXES ON table_name;

-- Show users
SHOW USERS;

-- Show access
SHOW ACCESS;

-- Show analytics
INFO STATISTICS;

-- Explain query
EXPLAIN SELECT * FROM person WHERE age > 30;

-- Explain with analysis
EXPLAIN ANALYZE SELECT * FROM person WHERE age > 30;
```

---

## Connection Strings

| Storage | Connection String |
|---------|-------------------|
| Memory | `memory` |
| File (SurrealKV) | `file://./data.db` |
| RocksDB | `rocksdb://./data.db` |
| SQLite | `sqlite://./data.db` |
| TiKV | `tikv://127.0.0.1:2379` |
| Redis | `redis://127.0.0.1:6379` |
| PostgreSQL | `postgres://user:pass@host:port/db` |
| MySQL | `mysql://user:pass@host:port/db` |

---

## Common Patterns

### Conditional Logic
```sql
UPDATE person SET status =
    IF age < 18 THEN "minor"
    ELSE IF age < 65 THEN "adult"
    ELSE "senior"
    END
END;
```

### Subqueries
```sql
-- IN clause
SELECT * FROM person WHERE id IN (SELECT friend FROM person:friends);

-- EXISTS
SELECT * FROM person WHERE EXISTS (SELECT * FROM post WHERE author = person.id);
```

### Joins
```sql
-- Implicit join
SELECT * FROM user, post WHERE user.id = post.author;

-- Explicit JOIN
SELECT * FROM user JOIN post ON user.id = post.author;
```

### Pattern Matching
```sql
-- CONTAINS
SELECT * FROM user WHERE email CONTAINS "@gmail.com";

-- MATCHES (regex)
SELECT * FROM product WHERE name MATCHES "^Prod";

-- STARTS WITH
SELECT * FROM file WHERE name STARTS WITH "IMG";

-- ENDS WITH
SELECT * FROM file WHERE name ENDS WITH ".jpg";
```

---

## HTTP/WebSocket API

### Connect via WebSocket
```javascript
const ws = new WebSocket('ws://localhost:8000');

ws.send('USE NS test DB test;');
ws.send('SIGNIN USER root PASSWORD root;');
ws.send('SELECT * FROM person;');
```

### HTTP Request
```bash
# Query
curl -X POST http://localhost:8000/sql \
  -H "Authorization: Basic root:root" \
  -d "SELECT * FROM person"

# With JSON output
curl -X POST http://localhost:8000/sql \
  -H "Accept: application/json" \
  -H "Authorization: Basic root:root" \
  -d "SELECT * FROM person"
```

---

## Backup & Restore

```sql
-- Backup
BACKUP TO 'backup.surql';

-- Restore
RESTORE FROM 'backup.surql';

-- Export
surreal export --endpoint http://localhost:8000 backup.surql

-- Import
surreal import --endpoint http://localhost:8000 backup.surql
```

---

## Keyboard Shortcuts (SQL Shell)

| Key | Action |
|-----|--------|
| `Ctrl + C` | Cancel current query or exit |
| `Ctrl + D` | Exit shell |
| `Tab` | Auto-complete |
| `↑/↓` | Navigate history |

---

## Resources & Documentation

- **Official Docs**: https://surrealdb.com/docs
- **SurrealQL Reference**: https://surrealdb.com/docs/surrealql
- **Integration Guides**: https://surrealdb.com/docs/integration
- **Community**: https://surrealdb.com/community
- **GitHub**: https://github.com/surrealdb/surrealdb

---

## Tips & Tricks

1. **Use `ONLY` keyword** to exclude metadata (id, created_at) from results
2. **Use `SLEEP`** for testing: `SLEEP 1s`
3. **Use `RAND()`** for random values: `SELECT * FROM user ORDER BY RAND() LIMIT 5`
4. **Use `HTTP::*` functions** for external API calls
5. **Use `GROUP BY`** with aggregations: `SELECT category, count() FROM post GROUP BY category`
