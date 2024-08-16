CREATE FUNCTION json_is_hashmap(json_input JSON)
  RETURNS BOOLEAN
  LANGUAGE plpgsql
AS $$
DECLARE
  is_hashmap BOOLEAN;
BEGIN
  IF json_typeof(json_input) != 'object' THEN
    RETURN FALSE;
  END IF;

  SELECT coalesce(EVERY(json_typeof(pairs.value) = 'string'), TRUE) OR json_input IS NULL
  INTO is_hashmap
  FROM json_each(json_input) AS pairs;

  RETURN is_hashmap;
END; $$;

CREATE FUNCTION json_is_keyvalue(json_input JSON)
  RETURNS BOOLEAN
  LANGUAGE plpgsql
AS $$
DECLARE
  is_keyvalue BOOLEAN;
BEGIN
  IF json_typeof(json_input) != 'object' THEN
    RETURN FALSE;
  END IF;

  SELECT coalesce(COUNT(*) = 1, FALSE) OR json_input IS NULL
  INTO is_keyvalue
  FROM json_each(json_input);

  RETURN is_keyvalue;
END; $$;

CREATE DOMAIN STRING_HASHMAP AS JSON
CHECK (json_is_hashmap(value));

CREATE DOMAIN STRING_KEYVALUE AS JSON
CHECK (json_is_keyvalue(value));

CREATE TYPE AUTH AS ENUM('none', 'basic', 'bearer', 'param');

CREATE TYPE BODY AS ENUM('none', 'text', 'json', 'form', 'multi');

CREATE TABLE destinations (
  id SERIAL PRIMARY KEY,
  code VARCHAR NOT NULL UNIQUE,
  path VARCHAR NOT NULL,
  is_active BOOLEAN NOT NULL DEFAULT FALSE,
  headers STRING_HASHMAP NOT NULL DEFAULT '{}',
  filter TEXT NULL,
  is_auth BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE UNIQUE INDEX ix_destinations
ON destinations(path, is_active);

CREATE TABLE sources (
  id SERIAL PRIMARY KEY,
  code VARCHAR NOT NULL UNIQUE,
  url VARCHAR NOT NULL,
  headers STRING_HASHMAP NOT NULL DEFAULT '{}',
  body_type BODY NOT NULL DEFAULT 'none',
  body_text TEXT NULL,
  body_json JSON NULL,
  body_form STRING_HASHMAP NULL,
  body_multi STRING_HASHMAP NULL,
  params STRING_HASHMAP NOT NULL DEFAULT '{}',
  timeout INTERVAL NULL,
  fallback JSON NULL,
  auth_type AUTH NOT NULL DEFAULT 'none',
  auth_username VARCHAR NULL,
  auth_password VARCHAR NULL,
  auth_token VARCHAR NULL,
  auth_param STRING_KEYVALUE NULL,
  CONSTRAINT ck_auth CHECK (
    (auth_type = 'basic') = (auth_username IS NOT NULL AND auth_password IS NOT NULL)
    AND (auth_type = 'bearer') = (auth_token IS NOT NULL)
    AND (auth_type = 'param') = (auth_param IS NOT NULL)
  ),
  CONSTRAINT ck_body CHECK (
    (body_type = 'text') = (body_text IS NOT NULL)
    AND (body_type = 'json') = (body_json IS NOT NULL)
    AND (body_type = 'form') = (body_form IS NOT NULL)
    AND (body_type = 'multi') = (body_multi IS NOT NULL)
  )
);

CREATE TABLE destinations__sources (
  id SERIAL PRIMARY KEY,
  destination_id INT NOT NULL REFERENCES destinations(id),
  source_id INT NOT NULL REFERENCES sources(id)
);

CREATE UNIQUE INDEX ix_destinations__sources 
ON destinations__sources(destination_id, source_id);

CREATE TABLE auth_tokens (
  id SERIAL PRIMARY KEY,
  value BYTEA NOT NULL UNIQUE,
  expiration TIMESTAMP WITH TIME ZONE NULL
);

CREATE TABLE destinations__auth_tokens (
  id SERIAL PRIMARY KEY,
  destination_id INT NOT NULL REFERENCES destinations(id),
  auth_token_id INT NOT NULL REFERENCES auth_tokens(id)
);

CREATE UNIQUE INDEX ix_destinations__auth_tokens
ON destinations__auth_tokens(destination_id, auth_token_id);

CREATE TABLE config_versions (
  id UUID PRIMARY KEY DEFAULT GEN_RANDOM_UUID(),
  updated_on TIMESTAMP WITH TIME ZONE NOT NULL,
  hash BYTEA NOT NULL
);
