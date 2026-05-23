-- Add FK to telemetry tables --
ALTER TABLE public.played_card ADD CONSTRAINT played_card_card_fk FOREIGN KEY (card_id) REFERENCES public.card(id) ON UPDATE CASCADE;
ALTER TABLE public.played_active_card ADD CONSTRAINT played_active_card_card_fk FOREIGN KEY (card_id) REFERENCES public.card(id) ON UPDATE CASCADE;
