# Result


```bash

$ cargo r --release

[src/main.rs:41:5] surql = "\n-- ------------------------------\n-- OPTION\n-- ------------------------------\n\nOPTION IMPORT;\n\n-- ------------------------------\n-- TABLE: person\n-- ------------------------------\n\nDEFINE TABLE person TYPE ANY SCHEMALESS PERMISSIONS NONE;\n\n\n\n\n-- ------------------------------\n-- TABLE DATA: person\n-- ------------------------------\n\nINSERT [ { id: person:aeon } ];\n"
successful!

```

