DROP TABLE config_versions;

DROP INDEX ix_destinations__auth_tokens;
DROP TABLE destinations__auth_tokens;

DROP TABLE auth_tokens;

DROP INDEX ix_destinations__sources;
DROP TABLE destinations__sources;

DROP TABLE sources;

DROP INDEX ix_destinations;
DROP TABLE destinations;

DROP TYPE BODY;
DROP TYPE AUTH;

DROP DOMAIN STRING_KEYVALUE;
DROP DOMAIN STRING_HASHMAP;

DROP FUNCTION json_is_keyvalue;
DROP FUNCTION json_is_hashmap;