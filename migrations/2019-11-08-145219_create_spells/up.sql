CREATE TABLE spells (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL UNIQUE,
  spell_duration VARCHAR NOT NULL,
  options TEXT[] NOT NULL,
  reach VARCHAR NOT NULL,
  difficulty VARCHAR NOT NULL,
  typus VARCHAR NOT NULL,
  enforced VARCHAR NOT NULL,
  effect TEXT NOT NULL,
  cast_duration VARCHAR NOT NULL,
  cost VARCHAR NOT NULL
);
