ALTER TABLE spells
ADD schools_id INTEGER REFERENCES schools(id) NOT NULL;
