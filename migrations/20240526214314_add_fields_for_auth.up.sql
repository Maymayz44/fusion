CREATE TYPE AUTH AS ENUM('none', 'basic', 'bearer');

ALTER TABLE sources
ADD COLUMN auth_type AUTH DEFAULT 'none',
ADD COLUMN auth_username VARCHAR NULL,
ADD COLUMN auth_password VARCHAR NULL,
ADD COLUMN auth_token VARCHAR NULL,
ADD CONSTRAINT ck_auth CHECK (
  CASE
    WHEN auth_type = 'none' THEN auth_username IS NULL AND auth_password IS NULL AND auth_token IS NULL
    WHEN auth_type = 'basic' THEN auth_username IS NOT NULL AND auth_password IS NOT NULL AND auth_token IS NULL
    WHEN auth_type = 'bearer' THEN auth_username IS NULL AND auth_password IS NULL AND auth_token IS NOT NULL
  END
);