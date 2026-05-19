ALTER TABLE client ADD "version" varchar NULL;

UPDATE client
SET version = game.version
FROM game
WHERE client.game_id = game.id AND client.is_host = true;

UPDATE client
SET version = 'unknown'
WHERE version IS NULL;

ALTER TABLE public.client ALTER COLUMN "version" SET NOT NULL;

ALTER TABLE game DROP COLUMN "version";
