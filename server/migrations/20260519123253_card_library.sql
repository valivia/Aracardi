-- Rename --
ALTER TABLE public.card        RENAME TO played_card;
ALTER TABLE public.active_card RENAME TO played_active_card;

-- Make new tables --
CREATE TABLE public.addon (
    id          varchar     NOT NULL,
    created_at  timestamptz DEFAULT now() NOT NULL,
    title       varchar     NOT NULL,
    description varchar     NOT NULL,
    file_name   varchar     NOT NULL,
    is_default  bool        DEFAULT false NOT NULL,
    CONSTRAINT addon_pk     PRIMARY KEY (id),
    CONSTRAINT addon_unique UNIQUE (file_name)
);

CREATE TABLE public.card (
    id           varchar   NOT NULL,
    addon_id     varchar   NOT NULL,
    created_at   timestamptz DEFAULT now() NOT NULL,
    title        varchar   NULL,
    "text"       varchar   NOT NULL,
    turns        integer   NULL,
    min_players  integer   NULL,
    max_players  integer   NULL,
    is_deprecated bool     DEFAULT false NOT NULL,
    has_image     bool     DEFAULT false NOT NULL,
    is_nsfw       bool     DEFAULT false NOT NULL,
    has_wheel     bool     DEFAULT false NOT NULL,
    overrides    varchar[] NULL,
    CONSTRAINT card_pk                   PRIMARY KEY (id),
    CONSTRAINT card_addon_fk             FOREIGN KEY (addon_id) REFERENCES public.addon(id) ON UPDATE CASCADE,
    CONSTRAINT card_turns_valid          CHECK (turns = -1 OR turns > 0),
    CONSTRAINT card_min_players_positive CHECK (min_players >= 2),
    CONSTRAINT card_max_players_gte_min  CHECK (max_players >= min_players)
);

-- Index --
CREATE INDEX idx_cards_addon_id ON public.card (addon_id);
