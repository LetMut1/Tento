CREATE TABLE public.channel_publication1 (
    id BIGINT,
    channel__id BIGINT,
    images_pathes TEXT[],
    text_ TEXT,
    commentaries_quantity BIGINT,
    marks_quantity BIGINT,
    view_quantity BIGINT,
    obfuscation_value BIGINT,
    created_at BIGINT,
    is_predeleted BOOLEAN,
    can_be_deleted_from BIGINT
) WITH (oids = false, fillfactor = 85, autovacuum_enabled = true);

CREATE SEQUENCE public.channel_publication1_1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE
START WITH 1 CACHE 1 NO CYCLE OWNED BY public.channel_publication1.id;

CREATE UNIQUE INDEX channel_publication1_2 ON public.channel_publication1
USING btree (id ASC NULLS LAST) WITH (fillfactor = 90, deduplicate_items = on);

CREATE INDEX channel_publication1_3 ON public.channel_publication1
USING btree (channel__id, is_predeleted, created_at ASC NULLS LAST) WITH (fillfactor = 80, deduplicate_items = on);

ALTER TABLE ONLY public.channel_publication1
ALTER COLUMN id SET NOT NULL,
ALTER COLUMN channel__id SET NOT NULL,
ALTER COLUMN images_pathes SET NOT NULL,
ALTER COLUMN commentaries_quantity SET NOT NULL,
ALTER COLUMN marks_quantity SET NOT NULL,
ALTER COLUMN view_quantity SET NOT NULL,
ALTER COLUMN obfuscation_value SET NOT NULL,
ALTER COLUMN created_at SET NOT NULL,
ALTER COLUMN is_predeleted SET NOT NULL,
ALTER COLUMN can_be_deleted_from SET NOT NULL,
ADD CONSTRAINT channel_publication1_4 UNIQUE USING INDEX channel_publication1_2;
