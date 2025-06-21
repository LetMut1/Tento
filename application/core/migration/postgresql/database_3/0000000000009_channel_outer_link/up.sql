CREATE TABLE public.channel_outer_link (
    from_ BIGINT,
    alias TEXT,
    address TEXT,
    created_at BIGINT
) WITH (oids = false, fillfactor = 85, autovacuum_enabled = true);

CREATE INDEX channel_outer_link_1 ON public.channel_outer_link
USING btree (from_ ASC NULLS LAST) WITH (fillfactor = 90, deduplicate_items = on);

ALTER TABLE ONLY public.channel_outer_link
ALTER COLUMN from_ SET NOT NULL,
ALTER COLUMN alias SET NOT NULL,
ALTER COLUMN address SET NOT NULL,
ALTER COLUMN created_at SET NOT NULL;