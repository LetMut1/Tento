CREATE TABLE public.channel_publication1_marked_view (
    user__id BIGINT,
    channel_publication1__id BIGINT,
    marked_at BIGINT
    created_at BIGINT
) WITH (oids = false, fillfactor = 85, autovacuum_enabled = true);

CREATE UNIQUE INDEX channel_publication1_marked_view_1 ON public.channel_publication1_marked_view
USING btree (user__id, channel_publication1__id ASC NULLS LAST) WITH (fillfactor = 75, deduplicate_items = on);

ALTER TABLE ONLY public.channel_publication1_marked_view
ALTER COLUMN user__id SET NOT NULL,
ALTER COLUMN channel_publication1__id SET NOT NULL,
ALTER COLUMN marked_at SET NOT NULL,
ALTER COLUMN created_at SET NOT NULL,
ADD CONSTRAINT channel_publication1_marked_view_2 UNIQUE USING INDEX channel_publication1_marked_view_1;
