ALTER TABLE sources
DROP COLUMN body_json,
DROP COLUMN body_type;

ALTER TABLE sources
RENAME COLUMN body_text TO body;

DROP TYPE BODY;