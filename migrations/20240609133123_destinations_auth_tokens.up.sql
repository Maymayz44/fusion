ALTER TABLE destinations
ADD COLUMN is_auth BOOLEAN DEFAULT FALSE;

CREATE TABLE auth_tokens (
  id SERIAL PRIMARY KEY,
  value TEXT NOT NULL UNIQUE,
  expiration TIMESTAMP WITH TIME ZONE NULL
);

CREATE TABLE destinations__auth_tokens (
  id SERIAL PRIMARY KEY,
  destination_id INT NOT NULL REFERENCES destinations(id),
  auth_token_id INT NOT NULL REFERENCES auth_tokens(id)
);

CREATE UNIQUE INDEX ix_destinations__auth_tokens
ON destinations__auth_tokens(destination_id, auth_token_id);
