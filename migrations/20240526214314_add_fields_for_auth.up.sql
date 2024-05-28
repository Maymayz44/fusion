CREATE TYPE AUTH_TYPE AS ENUM('none', 'basic', 'bearer');

ALTER TABLE sources
ADD COLUMN auth_type AUTH_TYPE DEFAULT 'none',
ADD COLUMN auth_username VARCHAR NULL,
ADD COLUMN auth_password VARCHAR NULL,
ADD COLUMN auth_token VARCHAR NULL;