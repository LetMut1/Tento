CREATE TABLE public.channel_inner_link (
    from_ BIGINT,
    to_ BIGINT,
    created_at TIMESTAMP(6) WITH TIME ZONE
) WITH (oids = false, fillfactor = 85, autovacuum_enabled = true);

CREATE UNIQUE INDEX channel_inner_link1 ON public.channel_inner_link
USING btree (from_, to_ ASC NULLS LAST) WITH (fillfactor = 90, deduplicate_items = on);

CREATE INDEX channel_inner_link2 ON public.channel_inner_link
USING btree (to_ ASC NULLS LAST) WITH (fillfactor = 85, deduplicate_items = on);

ALTER TABLE ONLY public.channel_inner_link
ALTER COLUMN from_ SET NOT NULL,
ALTER COLUMN to_ SET NOT NULL,
ALTER COLUMN created_at SET NOT NULL,
ADD CONSTRAINT channel_inner_link3 FOREIGN KEY (from_)
REFERENCES public.channel (id) ON DELETE CASCADE,
ADD CONSTRAINT channel_inner_link4 FOREIGN KEY (to_)
REFERENCES public.channel (id) ON DELETE CASCADE,
ADD CONSTRAINT channel_inner_link5 UNIQUE USING INDEX channel_inner_link1;

COMMENT ON COLUMN public.channel_inner_link.from_ IS 'public.channel.id';
COMMENT ON COLUMN public.channel_inner_link.to_ IS 'public.channel.id';