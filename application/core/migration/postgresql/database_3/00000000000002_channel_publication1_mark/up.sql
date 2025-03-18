CREATE TABLE public.channel_publication1_mark (
    user__id BIGINT,
    channel_publication1__id BIGINT,
    created_at BIGINT
) WITH (oids = false, fillfactor = 85, autovacuum_enabled = true);

CREATE UNIQUE INDEX channel_publication1_mark_1 ON public.channel_publication1_mark
USING btree (user__id, channel_publication1__id ASC NULLS LAST) WITH (fillfactor = 90, deduplicate_items = on);

ALTER TABLE ONLY public.channel_publication1_mark
ALTER COLUMN user__id SET NOT NULL,
ALTER COLUMN channel_publication1__id SET NOT NULL,
ALTER COLUMN created_at SET NOT NULL,
ADD CONSTRAINT channel_publication1_mark_2 UNIQUE USING INDEX channel_publication1_mark_1;
