ALTER TABLE sources
DROP CONSTRAINT ck_auth;

ALTER TABLE sources
DROP COLUMN auth_param;

DROP DOMAIN STRING_KEYVALUE;

ALTER TYPE AUTH RENAME TO AUTH_DEL;

CREATE TYPE AUTH AS ENUM('none', 'basic', 'bearer');

ALTER TABLE sources
ADD COLUMN auth_type_torename AUTH DEFAULT 'none';

UPDATE sources
SET auth_type_torename = CASE WHEN auth_type = 'param' THEN 'none' ELSE auth_type::TEXT::AUTH END;

ALTER TABLE sources
DROP COLUMN auth_type;

ALTER TABLE sources
RENAME COLUMN auth_type_torename TO auth_type;

ALTER TABLE sources
ADD CONSTRAINT ck_auth CHECK (
  CASE
    WHEN auth_type = 'none' THEN auth_username IS NULL AND auth_password IS NULL AND auth_token IS NULL
    WHEN auth_type = 'basic' THEN auth_username IS NOT NULL AND auth_password IS NOT NULL AND auth_token IS NULL
    WHEN auth_type = 'bearer' THEN auth_username IS NULL AND auth_password IS NULL AND auth_token IS NOT NULL
  END
);

DROP TYPE AUTH_DEL;

DROP FUNCTION json_is_keyvalue;