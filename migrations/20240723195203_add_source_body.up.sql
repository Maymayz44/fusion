ALTER TABLE IF EXISTS sources
ALTER COLUMN auth_type SET NOT NULL;

CREATE TYPE BODY AS ENUM('none', 'text', 'json');

ALTER TABLE sources
ADD COLUMN body_type BODY NOT NULL DEFAULT 'none',
ADD COLUMN body_json JSON NULL;

ALTER TABLE sources
RENAME COLUMN body TO body_text;

ALTER TABLE sources
ADD CONSTRAINT ck_body CHECK (
  (body_type = 'text') = (body_text IS NOT NULL)
  AND (body_type = 'json') = (body_json IS NOT NULL)
) NOT VALID;