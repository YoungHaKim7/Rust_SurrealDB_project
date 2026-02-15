# Result

```bash
$ cargo r

[src/main.rs:58:5] created = Some(
    Record {
        id: Thing {
            tb: "person",
            id: String(
                "bwuww5lo0ajbe8xxa0xz",
            ),
        },
    },
)
[src/main.rs:65:5] updated = None
[src/main.rs:69:5] people = [
    Record {
        id: Thing {
            tb: "person",
            id: String(
                "bwuww5lo0ajbe8xxa0xz",
            ),
        },
    },
]
[src/main.rs:76:5] groups = Response {
    results: {
        0: (
            Stats {
                execution_time: Some(
                    1.268ms,
                ),
            },
            Ok(
                Array(
                    Array(
                        [
                            Object(
                                Object(
                                    {
                                        "count": Number(
                                            Int(
                                                1,
                                            ),
                                        ),
                                        "marketing": Bool(
                                            true,
                                        ),
                                    },
                                ),
                            ),
                        ],
                    ),
                ),
            ),
        ),
    },
    live_queries: {},
}

```

