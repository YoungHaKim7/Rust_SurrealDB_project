-- ------------------------------
-- OPTION
-- ------------------------------

OPTION IMPORT;

-- ------------------------------
-- TABLE: person
-- ------------------------------

DEFINE TABLE person TYPE ANY SCHEMALESS PERMISSIONS NONE;




-- ------------------------------
-- TABLE DATA: person
-- ------------------------------

INSERT [ { id: person:aeon } ];

-- ------------------------------
-- TABLE: town
-- ------------------------------

DEFINE TABLE town TYPE ANY SCHEMALESS PERMISSIONS NONE;




-- ------------------------------
-- TABLE DATA: town
-- ------------------------------

INSERT [ { data: { geography: 'Coastal town', location: 'Northwest of Toria' }, id: town:the_naimo, name: 'The Naimo', population: 7490 } ];


