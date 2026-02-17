-- ------------------------------
-- OPTION
-- ------------------------------

OPTION IMPORT;

-- ------------------------------
-- TABLE: airplane
-- ------------------------------

DEFINE TABLE airplane TYPE ANY SCHEMALESS PERMISSIONS NONE;




-- ------------------------------
-- TABLE DATA: airplane
-- ------------------------------

INSERT [ { id: airplane:31sxu1oh3xzdsqx0f0f5, name: "Aeon's plane", seats: 20 }, { id: airplane:aeons_plane, name: "Aeon's plane", seats: 20 }, { id: airplane:fqpc66n36jequrm615hb, name: "Aeon's plane", seats: 20 } ];

-- ------------------------------
-- TABLE: book_event
-- ------------------------------

DEFINE TABLE book_event TYPE ANY SCHEMALESS PERMISSIONS NONE;




-- ------------------------------
-- TABLE DATA: book_event
-- ------------------------------

INSERT [ { content: 'Aeon hears about some books', id: book_event:['Aeon', 0] }, { content: 'Aeon discovers the tunnel', id: book_event:['Aeon', 1] }, { content: 'Aeon sees a computer for the first time', id: book_event:['Aeon', 2] }, { content: 'Eats some chicken', id: book_event:["Aeon's cat", 0] }, { content: 'Sleeps', id: book_event:["Aeon's cat", 1] }, { content: 'Tired from all the sleeping so gets some more sleep', id: book_event:["Aeon's cat", 2] } ];

-- ------------------------------
-- TABLE: person
-- ------------------------------

DEFINE TABLE person TYPE ANY SCHEMALESS PERMISSIONS NONE;




-- ------------------------------
-- TABLE DATA: person
-- ------------------------------

INSERT [ { id: person:aeon } ];

-- ------------------------------
-- TABLE: resident
-- ------------------------------

DEFINE TABLE resident TYPE ANY SCHEMALESS PERMISSIONS NONE;




-- ------------------------------
-- TABLE DATA: resident
-- ------------------------------

INSERT [ { id: resident:02n7zgkj5o8hjq83h4pc, name: 'Mother of Aeon', of: 'Toria' }, { id: resident:kw6lteigdpu2txr5w7va, name: 'Aeon', of: 'Toria' } ];

-- ------------------------------
-- TABLE: resident_of
-- ------------------------------

DEFINE TABLE resident_of TYPE ANY SCHEMALESS PERMISSIONS NONE;




-- ------------------------------
-- TABLE DATA: resident_of
-- ------------------------------

INSERT [ { id: resident_of:['Toria', 'Aeon'] }, { id: resident_of:['Toria', 'Mother of Aeon'] } ];

-- ------------------------------
-- TABLE: town
-- ------------------------------

DEFINE TABLE town TYPE ANY SCHEMALESS PERMISSIONS NONE;




-- ------------------------------
-- TABLE DATA: town
-- ------------------------------

INSERT [ { id: town:6xybotiylezofi01tfug, name: 'Sukh', population: 2955 }, { id: town:sg2dsww8w4atlxgkraqv, name: 'Sukh', population: 2955 }, { id: town:sukh, name: 'Sukh', population: 2955 }, { data: { geography: 'Coastal town', location: 'Northwest of Toria' }, id: town:the_naimo, name: 'The Naimo', population: 2955 } ];

