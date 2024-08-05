ALTER TYPE BODY RENAME TO BODY_DEL;

CREATE TYPE BODY AS ENUM('none', 'text', 'json');

ALTER TABLE sources
ADD COLUMN body_type_torename BODY NOT NULL DEFAULT 'none';

UPDATE sources
SET body_type_torename = CASE WHEN body_type = 'form' OR body_type = 'multi' THEN 'none' ELSE body_type::TEXT::BODY END;

ALTER TABLE sources
DROP CONSTRAINT ck_body;

ALTER TABLE sources
DROP COLUMN body_type;

ALTER TABLE sources
RENAME COLUMN body_type_torename TO body_type;

ALTER TABLE sources
ADD CONSTRAINT ck_body CHECK (
  (body_type = 'text') = (body_text IS NOT NULL)
  AND (body_type = 'json') = (body_json IS NOT NULL)
);

DROP TYPE BODY_DEL;