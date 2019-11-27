CREATE TABLE spells (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL UNIQUE,
  cast_duration VARCHAR NOT NULL,
  options TEXT[] NOT NULL,
  range VARCHAR NOT NULL,
  difficulty VARCHAR NOT NULL,
  typus VARCHAR NOT NULL,
  enforced VARCHAR NOT NULL,
  effect TEXT NOT NULL,
  duration_of_effect VARCHAR NOT NULL,
  cost VARCHAR NOT NULL
);
