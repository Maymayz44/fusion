CREATE FUNCTION json_is_keyvalue(json_input JSON)
  RETURNS BOOLEAN
  LANGUAGE plpgsql
AS $$
DECLARE
  is_keyvalue BOOLEAN;
BEGIN
  SELECT COUNT(*) = 1
  INTO is_keyvalue
  FROM json_each(json_input);

  RETURN is_keyvalue;
END; $$;

CREATE DOMAIN STRING_KEYVALUE AS JSON
CHECK (json_is_keyvalue(value));

ALTER TABLE sources
ADD COLUMN auth_param STRING_KEYVALUE NULL;

ALTER TABLE sources
DROP CONSTRAINT ck_auth;

ALTER TABLE sources
ADD CONSTRAINT ck_auth CHECK (
  (auth_type = 'basic') = (auth_username IS NOT NULL AND auth_password IS NOT NULL)
  AND (auth_type = 'bearer') = (auth_token IS NOT NULL)
  AND (auth_type = 'param') = (auth_param IS NOT NULL)
);