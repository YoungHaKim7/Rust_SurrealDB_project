# Result

```bash

```


```sql
SELECT * FROM person;
SELECT * FROM town;
```

```bash
-------- Query 1 (110us) --------

[
	{
		id: person:9bd54nd27hb06ope1epr
	},
	{
		id: person:aeon
	},
	{
		id: person:nxjcdesv30xfme0dj5um
	}
]

-------- Query 2 (49us) --------

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

```
