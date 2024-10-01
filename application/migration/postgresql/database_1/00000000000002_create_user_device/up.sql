CREATE TABLE public.user_device (
    id TEXT,
    user__id BIGINT
) WITH (oids = false, fillfactor = 95, autovacuum_enabled = true);

CREATE UNIQUE INDEX user_device1 ON public.user_device
USING btree (user__id, id ASC NULLS LAST) WITH (fillfactor = 90, deduplicate_items = on);

ALTER TABLE ONLY public.user_device
ALTER COLUMN id SET NOT NULL,
ALTER COLUMN user__id SET NOT NULL,
ADD CONSTRAINT user_device2 UNIQUE USING INDEX user_device1,
ADD CONSTRAINT user_device3 FOREIGN KEY (user__id)
REFERENCES public.user_ (id) ON DELETE CASCADE;