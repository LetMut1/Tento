CREATE TABLE public.system_registry (
    message TEXT,
    level SMALLINT,
    created_at TIMESTAMP(6) WITH TIME ZONE
) WITH (oids = false, fillfactor = 95, autovacuum_enabled = true);

CREATE INDEX system_registry1 ON public.system_registry
USING btree (level ASC NULLS LAST) WITH (fillfactor = 95, deduplicate_items = on)
WHERE level IN (3, 4, 5);

ALTER TABLE ONLY public.system_registry
ALTER COLUMN message SET NOT NULL,
ALTER COLUMN level SET NOT NULL,
ALTER COLUMN created_at SET NOT NULL;


-- Application system registry level gradation:
-- TRACE -> 0
-- DEBUG -> 1
-- INFO	 -> 2
-- WARN	 -> 3
-- ERROR ->	4
-- FATAL ->	5