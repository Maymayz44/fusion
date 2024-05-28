ALTER TABLE sources
DROP COLUMN auth_type,
DROP COLUMN auth_username,
DROP COLUMN auth_password,
DROP COLUMN auth_token;

DROP TYPE AUTH_TYPE;