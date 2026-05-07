-- DROP SCHEMA public;

CREATE SCHEMA public AUTHORIZATION pg_database_owner;

-- public.active_card definition

CREATE TABLE public.active_card (
	id uuid DEFAULT gen_random_uuid() NOT NULL,
	game_id uuid NOT NULL,
	card_id varchar NOT NULL,
	duration_ms int4 NOT NULL,
	expected_turns int4 NOT NULL,
	actual_turns int4 NOT NULL,
	created_at timestamptz DEFAULT now() NOT NULL,
	CONSTRAINT active_card_pkey PRIMARY KEY (id)
);


-- public.card definition

CREATE TABLE public.card (
	id uuid DEFAULT gen_random_uuid() NOT NULL,
	game_id uuid NOT NULL,
	card_id varchar NOT NULL,
	duration_ms int4 NOT NULL,
	created_at timestamptz DEFAULT now() NOT NULL,
	CONSTRAINT card_pkey PRIMARY KEY (id)
);


-- public.game definition

CREATE TABLE public.game (
	id uuid NOT NULL,
	join_code varchar NOT NULL,
	addons _text NOT NULL,
	"version" varchar NOT NULL,
	initiated_at timestamptz NOT NULL,
	started_at timestamptz NOT NULL,
	ended_at timestamptz NOT NULL,
	card_play_count int4 NOT NULL,
	card_avg_duration_ms int4 NOT NULL,
	card_median_duration_ms int4 NOT NULL,
	players_initial int4 NOT NULL,
	players_loaded int4 NOT NULL,
	players_added int4 NOT NULL,
	players_renamed int4 NOT NULL,
	players_removed int4 NOT NULL,
	used_avatars _text NOT NULL,
	exclusion_reasons _text NOT NULL,
	game_end_reason varchar NOT NULL,
	created_at timestamptz DEFAULT now() NOT NULL,
	CONSTRAINT telemetry_game_pkey PRIMARY KEY (id)
);


-- public.client definition

CREATE TABLE public.client (
	id uuid NOT NULL,
	game_id uuid NOT NULL,
	is_host bool NOT NULL,
	connected_at timestamptz NOT NULL,
	disconnected_at timestamptz NULL,
	reconnect_count int4 DEFAULT 0 NOT NULL,
	total_connected_ms int4 NOT NULL,
	user_agent text NULL,
	country_code bpchar(2) NULL,
	theme text NULL,
	load_images bool NULL,
	allow_nsfw bool NULL,
	created_at timestamptz DEFAULT now() NOT NULL,
	CONSTRAINT client_pkey PRIMARY KEY (id),
	CONSTRAINT client_game_id_fkey FOREIGN KEY (game_id) REFERENCES public.game(id) ON DELETE CASCADE ON UPDATE CASCADE
);


-- public.player definition

CREATE TABLE public.player (
	id uuid DEFAULT gen_random_uuid() NOT NULL,
	game_id uuid NOT NULL,
	"name" text NOT NULL,
	avatar text NOT NULL,
	is_hand_picked bool NOT NULL,
	was_loaded bool NOT NULL,
	created_at timestamptz DEFAULT now() NOT NULL,
	CONSTRAINT player_pkey PRIMARY KEY (id),
	CONSTRAINT player_game_id_fkey FOREIGN KEY (game_id) REFERENCES public.game(id) ON DELETE CASCADE ON UPDATE CASCADE
);
