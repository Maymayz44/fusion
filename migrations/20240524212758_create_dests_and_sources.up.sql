CREATE TABLE destinations (
  id SERIAL PRIMARY KEY,
  path VARCHAR NOT NULL UNIQUE,
  protocol VARCHAR NOT NULL,
  headers JSON NULL,
  filter TEXT NULL
);

CREATE TABLE sources (
  id SERIAL PRIMARY KEY,
  url VARCHAR NOT NULL,
  headers JSON NULL,
  body TEXT,
  params JSON NULL
);

CREATE TABLE destinations_sources (
  id SERIAL PRIMARY KEY,
  destination_id INT REFERENCES destinations(id),
  source_id INT REFERENCES sources(id)
)