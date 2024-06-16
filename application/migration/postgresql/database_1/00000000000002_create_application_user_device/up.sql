CREATE TABLE public.application_user_device (
    id TEXT,
    application_user__id BIGINT
) WITH (oids = false, fillfactor = 95, autovacuum_enabled = true);

CREATE UNIQUE INDEX application_user_device1 ON public.application_user_device
USING btree (application_user__id, id ASC NULLS LAST) WITH (fillfactor = 90, deduplicate_items = on);

ALTER TABLE ONLY public.application_user_device
ALTER COLUMN id SET NOT NULL,
ALTER COLUMN application_user__id SET NOT NULL,
ADD CONSTRAINT application_user_device2 UNIQUE USING INDEX application_user_device1,
ADD CONSTRAINT application_user_device3 FOREIGN KEY (application_user__id)
REFERENCES public.application_user (id) ON DELETE CASCADE;