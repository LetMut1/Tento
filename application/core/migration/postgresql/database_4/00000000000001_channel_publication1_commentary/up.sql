CREATE TABLE public.channel_publication1_commentary (
    user__id BIGINT,
    channel_publication1__id BIGINT,
    text_ TEXT,
    marks_quantity BIGINT,
    created_at BIGINT
) WITH (oids = false, fillfactor = 75, autovacuum_enabled = true);

CREATE UNIQUE INDEX channel_publication1_commentary_1 ON public.channel_publication1_commentary
USING btree (channel_publication1__id, created_at ASC NULLS LAST) WITH (fillfactor = 90, deduplicate_items = on);

ALTER TABLE ONLY public.channel_publication1_commentary
ALTER COLUMN user__id SET NOT NULL,
ALTER COLUMN channel_publication1__id SET NOT NULL,
ALTER COLUMN text_ SET NOT NULL,
ALTER COLUMN marks_quantity SET NOT NULL,
ALTER COLUMN created_at SET NOT NULL,
ADD CONSTRAINT channel_publication1_commentary_2 UNIQUE USING INDEX channel_publication1_commentary_1;
