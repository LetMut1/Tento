CREATE TABLE public.channel_publication1 (
    id BIGINT,
    channel__id BIGINT,
    images_pathes TEXT[],
    text_ TEXT,
    created_at BIGINT
) WITH (oids = false, fillfactor = 85, autovacuum_enabled = true);

CREATE SEQUENCE public.channel_publication1_1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE
START WITH 1 CACHE 1 NO CYCLE OWNED BY public.channel_publication1.id;

CREATE UNIQUE INDEX channel_publication1_2 ON public.channel_publication1
USING btree (id ASC NULLS LAST) WITH (fillfactor = 90, deduplicate_items = on);

CREATE UNIQUE INDEX channel_publication1_3 ON public.channel_publication1
USING btree (channel__id, created_at ASC NULLS LAST) WITH (fillfactor = 90, deduplicate_items = on);

ALTER TABLE ONLY public.channel_publication1
ALTER COLUMN id SET NOT NULL,
ALTER COLUMN channel__id SET NOT NULL,
ALTER COLUMN images_pathes SET NOT NULL,
ALTER COLUMN created_at SET NOT NULL,
ADD CONSTRAINT channel_publication1_4 PRIMARY KEY USING INDEX channel_publication1_2,
ADD CONSTRAINT channel_publication1_5 UNIQUE USING INDEX channel_publication1_3;
