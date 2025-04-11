CREATE TABLE public.channel_delayed_deletion (
    channel__id BIGINT,
    can_be_deleted_from BIGINT,
    created_at BIGINT
) WITH (oids = false, fillfactor = 90, autovacuum_enabled = true);

CREATE INDEX channel_delayed_deletion_1 ON public.channel_delayed_deletion
USING btree (can_be_deleted_from ASC NULLS LAST) WITH (fillfactor = 90, deduplicate_items = on);

ALTER TABLE ONLY public.channel_delayed_deletion
ALTER COLUMN channel_publication1__id SET NOT NULL,
ALTER COLUMN can_be_deleted_from SET NOT NULL,
ALTER COLUMN created_at SET NOT NULL;
