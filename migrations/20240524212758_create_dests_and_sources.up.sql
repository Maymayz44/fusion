CREATE FUNCTION json_is_hashmap(json_input JSON) RETURNS BOOLEAN
RETURN (SELECT EVERY(json_typeof(value) = 'string') FROM json_each(json_input));

CREATE TABLE destinations (
  id SERIAL PRIMARY KEY,
  path VARCHAR NOT NULL UNIQUE,
  headers JSON NULL,
  filter TEXT NULL,
  CONSTRAINT ck_headers_hashmap CHECK (json_is_hashmap(headers))
);

CREATE TABLE sources (
  id SERIAL PRIMARY KEY,
  url VARCHAR NOT NULL,
  headers JSON NULL,
  body TEXT,
  params JSON NULL,
  CONSTRAINT ck_headers_hashmap CHECK (json_is_hashmap(headers)),
  CONSTRAINT ck_params_hashmap CHECK (json_is_hashmap(params))
);

CREATE TABLE destinations_sources (
  id SERIAL PRIMARY KEY,
  destination_id INT REFERENCES destinations(id),
  source_id INT REFERENCES sources(id)
);

CREATE UNIQUE INDEX ix_destinations_sources 
ON destinations_sources(destination_id, source_id);