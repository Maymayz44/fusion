CREATE OR REPLACE FUNCTION json_is_hashmap(json_input JSON)
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

CREATE OR REPLACE FUNCTION json_is_keyvalue(json_input JSON)
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

ALTER TABLE sources
ADD COLUMN body_form STRING_HASHMAP NULL,
ADD COLUMN body_multi STRING_HASHMAP NULL;

ALTER TABLE sources
DROP CONSTRAINT ck_body;

ALTER TABLE sources
ADD CONSTRAINT ck_body CHECK (
  (body_type = 'text') = (body_text IS NOT NULL)
  AND (body_type = 'json') = (body_json IS NOT NULL)
  AND (body_type = 'form') = (body_form IS NOT NULL)
  AND (body_type = 'multi') = (body_multi IS NOT NULL)
);