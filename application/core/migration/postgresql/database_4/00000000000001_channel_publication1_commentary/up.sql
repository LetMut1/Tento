CREATE TABLE public.channel_publication1_commentary (
    id BIGINT,
    author BIGINT,
    channel_publication1__id BIGINT,
    text_ TEXT,
    marks_quantity BIGINT,
    created_at BIGINT,
    is_predeleted BOOLEAN,
    can_be_deleted_from BIGINT
) WITH (oids = false, fillfactor = 85, autovacuum_enabled = true);

CREATE SEQUENCE public.channel_publication1_commentary_1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE
START WITH 1 CACHE 1 NO CYCLE OWNED BY public.channel_publication1_commentary.id;

CREATE UNIQUE INDEX channel_publication1_commentary_2 ON public.channel_publication1_commentary
USING btree (id ASC NULLS LAST) WITH (fillfactor = 90, deduplicate_items = on);

CREATE INDEX channel_publication1_commentary_3 ON public.channel_publication1_commentary
USING btree (channel_publication1__id, created_at ASC NULLS LAST) WITH (fillfactor = 75, deduplicate_items = on);

ALTER TABLE ONLY public.channel_publication1_commentary
ALTER COLUMN author SET NOT NULL,
ALTER COLUMN channel_publication1__id SET NOT NULL,
ALTER COLUMN text_ SET NOT NULL,
ALTER COLUMN marks_quantity SET NOT NULL,
ALTER COLUMN created_at SET NOT NULL,
ALTER COLUMN is_predeleted SET NOT NULL,
ALTER COLUMN can_be_deleted_from SET NOT NULL,
ADD CONSTRAINT channel_publication1_commentary_4 UNIQUE USING INDEX channel_publication1_commentary_2;
