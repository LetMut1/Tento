CREATE TABLE public.action_round_register (
    route TEXT,
    status_code SMALLINT,
    context TEXT,
    created_at TIMESTAMP(6) WITH TIME ZONE
) WITH (oids = false, fillfactor = 95, autovacuum_enabled = true);

CREATE INDEX action_round_register1 ON public.action_round_register
USING btree (status_code ASC NULLS LAST) WITH (fillfactor = 95, deduplicate_items = on)
WHERE status_code >= 400;

ALTER TABLE ONLY public.action_round_register
ALTER COLUMN route SET NOT NULL,
ALTER COLUMN status_code SET NOT NULL,
ALTER COLUMN created_at SET NOT NULL;