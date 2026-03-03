# Result

```bash
-------- Query 1 (3ms) --------

[
	{
		id: stored_in:xda7d12ml5j63n4e09as,
		in: document:doc_2,
		out: container:parent_folder
	}
]

-------- Query 2 (<100us) --------

'Docs by category:'

-------- Query 3 (<100us) --------

[
	{
		docs: [
			document:doc_1
		],
		id: category:important
	}
]

-------- Query 4 (<100us) --------

'Docs by tag:'

-------- Query 5 (<100us) --------

[
	{
		docs: [
			document:doc_1
		],
		id: tag:blue
	},
	{
		docs: [
			document:doc_2,
			document:doc_1
		],
		id: tag:red
	}
]

-------- Query 6 (<100us) --------

'Container relations:'

-------- Query 7 (<100us) --------

[
	{
		id: container:folder_1,
		relations: [
			{
				id: stored_in:6je8nb7i19j0mx04esoe,
				in: document:doc_1,
				out: container:folder_1
			},
			{
				id: stored_in:fstgps9kozvv9356cgdx,
				in: container:folder_1,
				out: container:parent_folder
			}
		]
	},
	{
		id: container:parent_folder,
		relations: [
			{
				id: stored_in:fstgps9kozvv9356cgdx,
				in: container:folder_1,
				out: container:parent_folder
			},
			{
				id: stored_in:xda7d12ml5j63n4e09as,
				in: document:doc_2,
				out: container:parent_folder
			}
		]
	}
]

-------- Query 8 (<100us) --------

'Related nodes by document:'

-------- Query 9 (1ms) --------

[
	{
		id: document:doc_1,
		related: [
			tag:blue,
			tag:red,
			category:important,
			container:folder_1
		]
	},
	{
		id: document:doc_2,
		related: [
			tag:red,
			container:parent_folder
		]
	}
]

-------- Query 10 (<100us) --------

'Recursive document path (fixed levels):'

-------- Query 11 (<100us) --------

[
	{
		cont1: [
			container:folder_1
		],
		cont2: [
			container:parent_folder
		],
		cont3: [],
		id: document:doc_1
	},
	{
		cont1: [
			container:parent_folder
		],
		cont2: [],
		cont3: [],
		id: document:doc_2
	}
]

-------- Query 12 (<100us) --------

'Recursive document path:'

-------- Query 13 (1ms) --------

[
	{
		"": {
			container: [
				{
					container: [
						{
							container: [],
							id: container:parent_folder
						}
					],
					id: container:folder_1
				}
			],
			id: document:doc_1
		}
	},
	{
		"": {
			container: [
				{
					container: [],
					id: container:parent_folder
				}
			],
			id: document:doc_2
		}
	}
]

-------- Query 14 (<100us) --------

[
	{
		"->?": {
			"->?": [
				tag:blue,
				tag:red,
				category:important,
				container:folder_1
			]
		},
		id: document:doc_1
	},
	{
		"->?": {
			"->?": [
				tag:red,
				container:parent_folder
			]
		},
		id: document:doc_2
	}
]

-------- Query 15 (4ms) --------

NONE

-------- Query 16 (<100us) --------

'Recursive with filters:'

-------- Query 17 (1ms) --------

[
	{
		"": {
			id: node:1,
			next: []
		}
	},
	{
		"": {
			id: node:2,
			next: []
		}
	},
	{
		"": {
			id: node:3,
			next: [
				{
					id: node:4,
					next: []
				}
			]
		}
	},
	{
		"": {
			id: node:4,
			next: []
		}
	},
	{
		"": {
			id: node:5,
			next: [
				{
					id: node:6,
					next: [
						{
							id: node:7,
							next: []
						}
					]
				}
			]
		}
	},
	{
		"": {
			id: node:6,
			next: [
				{
					id: node:7,
					next: []
				}
			]
		}
	},
	{
		"": {
			id: node:7,
			next: []
		}
	},
	{
		"": {
			id: node:8,
			next: []
		}
	},
	{
		"": {
			id: node:9,
			next: []
		}
	}
]

-------- Query 18 (<100us) --------

"The table 'a' does not exist"
```
