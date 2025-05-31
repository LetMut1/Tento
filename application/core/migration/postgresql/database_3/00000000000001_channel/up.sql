CREATE TABLE public.channel (
    id BIGINT,
    owner BIGINT,
    name TEXT,
    linked_name TEXT,
    description TEXT,
    access_modifier SMALLINT,
    visability_modifier SMALLINT,
    cover_image_path TEXT,
    background_image_path TEXT,
    subscribers_quantity BIGINT,
    created_at BIGINT
) WITH (oids = false, fillfactor = 85, autovacuum_enabled = true);

COMMENT ON COLUMN public.channel.access_modifier IS '0 - Open, 1 - Close';
COMMENT ON COLUMN public.channel.visability_modifier IS '0 - Public, 1 - Private';

CREATE SEQUENCE public.channel_1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE
START WITH 1 CACHE 1 NO CYCLE OWNED BY public.channel.id;

CREATE UNIQUE INDEX channel_2 ON public.channel
USING btree (id ASC NULLS LAST) WITH (fillfactor = 90, deduplicate_items = on);

CREATE UNIQUE INDEX channel_3 ON public.channel
USING btree (name COLLATE "C" ASC NULLS LAST) WITH (fillfactor = 80, deduplicate_items = on);

CREATE INDEX channel_4 ON public.channel
USING btree (visability_modifier, name COLLATE "C" ASC NULLS LAST) WITH (fillfactor = 90, deduplicate_items = on) WHERE visability_modifier = 0;

CREATE INDEX channel_5 ON public.channel
USING btree (owner ASC NULLS LAST) WITH (fillfactor = 90, deduplicate_items = on);

ALTER TABLE ONLY public.channel
ALTER COLUMN id SET NOT NULL,
ALTER COLUMN owner SET NOT NULL,
ALTER COLUMN name SET DATA TYPE TEXT COLLATE "C",
ALTER COLUMN name SET NOT NULL,
ALTER COLUMN linked_name SET NOT NULL,
ALTER COLUMN access_modifier SET NOT NULL,
ALTER COLUMN visability_modifier SET NOT NULL,
ALTER COLUMN subscribers_quantity SET NOT NULL,
ALTER COLUMN created_at SET NOT NULL,
ADD CONSTRAINT channel_6 UNIQUE USING INDEX channel_2,
ADD CONSTRAINT channel_7 UNIQUE USING INDEX channel_3;
