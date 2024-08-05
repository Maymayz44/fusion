CREATE FUNCTION json_is_hashmap(json_input JSON)
  RETURNS BOOLEAN
  LANGUAGE plpgsql
AS $$
DECLARE
  is_hashmap BOOLEAN;
BEGIN
  SELECT EVERY(json_typeof(pairs.value) = 'string')
  INTO is_hashmap
  FROM json_each(json_input) AS pairs;

  RETURN is_hashmap;
END; $$;

CREATE DOMAIN STRING_HASHMAP AS JSON
CHECK (json_is_hashmap(value));

CREATE TABLE destinations (
  id SERIAL PRIMARY KEY,
  path VARCHAR NOT NULL UNIQUE,
  headers STRING_HASHMAP NOT NULL DEFAULT '{}',
  filter TEXT NULL
);

CREATE TABLE sources (
  id SERIAL PRIMARY KEY,
  url VARCHAR NOT NULL,
  headers STRING_HASHMAP NOT NULL DEFAULT '{}',
  body TEXT NULL,
  params STRING_HASHMAP NOT NULL DEFAULT '{}'
);

CREATE TABLE destinations__sources (
  id SERIAL PRIMARY KEY,
  destination_id INT NOT NULL REFERENCES destinations(id),
  source_id INT NOT NULL REFERENCES sources(id)
);

CREATE UNIQUE INDEX ix_destinations__sources 
ON destinations__sources(destination_id, source_id);