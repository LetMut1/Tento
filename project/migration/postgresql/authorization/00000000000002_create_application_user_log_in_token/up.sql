CREATE TABLE public.application_user_log_in_token (
    application_user_id BIGINT,
    device_id TEXT,
    value CHARACTER VARYING(6),
    wrong_enter_tries_quantity SMALLINT,
    expires_at TIMESTAMP(6) WITH TIME ZONE
) WITH (oids = false, fillfactor = 95, autovacuum_enabled = true);

CREATE UNIQUE INDEX application_user_log_in_token1 ON public.application_user_log_in_token
USING btree (application_user_id, device_id ASC NULLS LAST) WITH (fillfactor = 90, deduplicate_items = on);

CREATE INDEX application_user_log_in_token2 ON public.application_user_log_in_token
USING btree (application_user_id ASC NULLS LAST) WITH (fillfactor = 90, deduplicate_items = on);

CREATE INDEX application_user_log_in_token3 ON public.application_user_log_in_token
USING btree (expires_at ASC NULLS LAST) WITH (fillfactor = 90, deduplicate_items = on);

ALTER TABLE ONLY public.application_user_log_in_token
ALTER COLUMN application_user_id SET NOT NULL,
ALTER COLUMN device_id SET NOT NULL,
ALTER COLUMN value SET NOT NULL,
ALTER COLUMN wrong_enter_tries_quantity SET NOT NULL,
ALTER COLUMN expires_at SET NOT NULL,
ADD CONSTRAINT application_user_log_in_token4 UNIQUE USING INDEX application_user_log_in_token1;

-- TODO  device_id TEXT - какой СHARACTER VARYING(?)