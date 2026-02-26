- https://surrealdb.com/learn/book/chapter-03

# Result

```bash

```

- db에 잘 저장된다.


```sql
SELECT * FROM town;
SELECT * FROM person;
```

```bash
-------- Query 1 (119us) --------
[
	{
		data: {
			geography: 'Coastal town',
			location: 'Northwest of Toria'
		},
		id: town:the_naimo,
		name: 'The Naimo',
		population: 7490
	}
]

-------- Query 2 (27us) --------

[
	{
		id: person:aeon
	}
]
```

