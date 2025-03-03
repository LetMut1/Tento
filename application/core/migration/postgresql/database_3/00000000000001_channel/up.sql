CREATE TABLE public.channel (
    id BIGINT,
    owner BIGINT,
    name TEXT,
    linked_name TEXT,
    description TEXT,
    access_modifier SMALLINT,
    visability_modifier SMALLINT,
    orientation SMALLINT[],
    cover_image_path TEXT,
    background_image_path TEXT,
    subscribers_quantity BIGINT,
    marks_quantity BIGINT,
    viewing_quantity BIGINT,
    obfuscation_value BIGINT,
    created_at BIGINT
) WITH (oids = false, fillfactor = 85, autovacuum_enabled = true);

CREATE SEQUENCE public.channel_1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE
START WITH 1 CACHE 1 NO CYCLE OWNED BY public.channel.id;

CREATE UNIQUE INDEX channel_2 ON public.channel
USING btree (id ASC NULLS LAST) WITH (fillfactor = 90, deduplicate_items = on);

CREATE UNIQUE INDEX channel_3 ON public.channel
USING btree (name COLLATE "C" ASC NULLS LAST) WITH (fillfactor = 80, deduplicate_items = on);

CREATE INDEX channel_5 ON public.channel
USING btree (visability_modifier ASC NULLS LAST) WITH (fillfactor = 90, deduplicate_items = on);

ALTER TABLE ONLY public.channel
ALTER COLUMN id SET NOT NULL,
ALTER COLUMN owner SET NOT NULL,
ALTER COLUMN name SET DATA TYPE TEXT COLLATE "C",
ALTER COLUMN name SET NOT NULL,
ALTER COLUMN linked_name SET NOT NULL,
ALTER COLUMN access_modifier SET NOT NULL,
ALTER COLUMN visability_modifier SET NOT NULL,
ALTER COLUMN orientation SET NOT NULL,
ALTER COLUMN subscribers_quantity SET NOT NULL,
ALTER COLUMN marks_quantity SET NOT NULL,
ALTER COLUMN viewing_quantity SET NOT NULL,
ALTER COLUMN obfuscation_value SET NOT NULL,
ALTER COLUMN created_at SET NOT NULL,
ADD CONSTRAINT channel_8 PRIMARY KEY USING INDEX channel_2,
ADD CONSTRAINT channel_9 FOREIGN KEY (owner)
REFERENCES public.user_ (id) ON DELETE RESTRICT,
ADD CONSTRAINT channel_10 UNIQUE USING INDEX channel_3;
