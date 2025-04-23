CREATE TABLE public.quantity_limiter (
    user__id BIGINT,
    owned_channels_quantity SMALLINT,
    created_at BIGINT
) WITH (oids = false, fillfactor = 90, autovacuum_enabled = true);

CREATE UNIQUE INDEX quantity_limiter_1 ON public.quantity_limiter
USING btree (user__id ASC NULLS LAST) WITH (fillfactor = 90, deduplicate_items = on);

ALTER TABLE ONLY public.quantity_limiter
ALTER COLUMN user__id SET NOT NULL,
ALTER COLUMN owned_channels_quantity SET NOT NULL,
ALTER COLUMN created_at SET NOT NULL,
ADD CONSTRAINT quantity_limiter_2 UNIQUE USING INDEX quantity_limiter_1;
