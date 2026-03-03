# Result

```bash
-------- Query 1 (<100us) --------

'Database record `user:martin` already exists'

-------- Query 2 (<100us) --------

'Database record `user:ignacio` already exists'

-------- Query 3 (1ms) --------

'Database record `user:tobie` already exists'

-------- Query 4 (<100us) --------

user:jaime

-------- Query 5 (<100us) --------

[
	{
		id: user:ignacio,
		name: 'ignacio'
	},
	{
		id: user:martin,
		lastname: 'schaer',
		name: 'martin',
		team: team:green
	},
	{
		id: user:tobie
	}
]

-------- Query 6 (<100us) --------

[
	{
		id: user:martin,
		lastname: 'schaer',
		name: 'martin',
		team: team:green
	}
]

-------- Query 7 (<100us) --------

[]

-------- Query 8 (<100us) --------

'Database record `team:green` already exists'

-------- Query 9 (<100us) --------

'Database record `team:red` already exists'

-------- Query 10 (<100us) --------

[
	{
		id: user:martin,
		lastname: 'schaer',
		name: 'martin',
		team: team:green
	}
]

-------- Query 11 (<100us) --------

[
	{
		id: user:ignacio,
		name: 'ignacio',
		team: {
			name: NONE
		}
	},
	{
		id: user:martin,
		name: 'martin',
		team: {
			name: 'Green'
		}
	},
	{
		id: user:tobie,
		name: NONE,
		team: {
			name: NONE
		}
	}
]

-------- Query 12 (<100us) --------

[
	{
		id: user:ignacio,
		name: 'ignacio',
		team: [
			NONE
		]
	},
	{
		id: user:martin,
		name: 'martin',
		team: {
			id: team:green,
			name: 'Green'
		}
	},
	{
		id: user:tobie,
		name: NONE,
		team: [
			NONE
		]
	}
]

-------- Query 13 (<100us) --------

[
	{
		id: plays_for:a1hmfye0g5ajf6hn00f7,
		in: user:martin,
		out: team:green,
		since: d'2025-10-23T00:00:00Z'
	}
]

-------- Query 14 (<100us) --------

[
	{
		id: plays_for:0p4dmt89mtyy9nmexe08,
		in: user:ignacio,
		out: team:green
	}
]

-------- Query 15 (1ms) --------

[
	{
		id: plays_for:3olylzv07etzcagdzsn2,
		in: user:tobie,
		out: team:red
	}
]

-------- Query 16 (<100us) --------

[
	{
		id: user:ignacio,
		name: 'ignacio',
		team: [
			team:green,
			team:green
		]
	},
	{
		id: user:martin,
		lastname: 'schaer',
		name: 'martin',
		team: [
			team:green,
			team:green
		]
	},
	{
		id: user:tobie,
		team: [
			team:red,
			team:red
		]
	}
]
```
