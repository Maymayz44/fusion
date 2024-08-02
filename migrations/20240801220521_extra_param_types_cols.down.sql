ALTER TABLE sources
DROP CONSTRAINT ck_body;

ALTER TABLE sources
ADD CONSTRAINT ck_body CHECK (
  (body_type = 'text') = (body_text IS NOT NULL)
  AND (body_type = 'json') = (body_json IS NOT NULL)
);

ALTER TABLE sources
DROP COLUMN body_form,
DROP COLUMN body_multi;

CREATE OR REPLACE FUNCTION json_is_hashmap(json_input JSON)
  RETURNS BOOLEAN
  LANGUAGE plpgsql
AS $$
DECLARE
  is_hashmap BOOLEAN;
BEGIN
  SELECT EVERY(json_typeof(pairs.value) = 'string')
  INTO is_hashmap
  FROM json_each(json_input) AS pairs;

  RETURN is_hashmap;
END; $$;

CREATE OR REPLACE FUNCTION json_is_keyvalue(json_input JSON)
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