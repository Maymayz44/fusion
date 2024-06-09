CREATE FUNCTION json_is_hashmap(json_input JSON) RETURNS BOOLEAN
RETURN json_typeof((json_each(json_input)).value) = 'string';

CREATE DOMAIN STRING_HASHMAP AS JSON
DEFAULT '{}'::JSON
CHECK (
  json_is_hashmap(value) AND json_typeof(value) = 'object'
);

CREATE TABLE destinations (
  id SERIAL PRIMARY KEY,
  path VARCHAR NOT NULL UNIQUE,
  headers STRING_HASHMAP NOT NULL,
  filter TEXT NULL
);

CREATE TABLE sources (
  id SERIAL PRIMARY KEY,
  url VARCHAR NOT NULL,
  headers STRING_HASHMAP NOT NULL,
  body TEXT NULL,
  params STRING_HASHMAP NOT NULL
);

CREATE TABLE destinations_sources (
  id SERIAL PRIMARY KEY,
  destination_id INT NOT NULL REFERENCES destinations(id),
  source_id INT NOT NULL REFERENCES sources(id)
);

CREATE UNIQUE INDEX ix_destinations_sources 
ON destinations_sources(destination_id, source_id);