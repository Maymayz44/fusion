ALTER TABLE sources
DROP CONSTRAINT ck_auth,
DROP COLUMN auth_type,
DROP COLUMN auth_username,
DROP COLUMN auth_password,
DROP COLUMN auth_token;

DROP TYPE AUTH;