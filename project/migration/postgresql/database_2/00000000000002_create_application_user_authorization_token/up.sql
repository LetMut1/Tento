CREATE TABLE public.application_user_authorization_token (
    application_user_id BIGINT,
    application_user_device_id TEXT,
    value TEXT,
    wrong_enter_tries_quantity SMALLINT,
    expires_at BIGINT
) WITH (oids = false, fillfactor = 85, autovacuum_enabled = true);

CREATE UNIQUE INDEX application_user_authorization_token1 ON public.application_user_authorization_token
USING btree (application_user_id, application_user_device_id ASC NULLS LAST) WITH (fillfactor = 90, deduplicate_items = on);

-- The index is not put on the field `expires_at` on purpose, because otherwise there will be a high load.

ALTER TABLE ONLY public.application_user_authorization_token
ALTER COLUMN application_user_id SET NOT NULL,
ALTER COLUMN application_user_device_id SET NOT NULL,
ALTER COLUMN value SET NOT NULL,
ALTER COLUMN wrong_enter_tries_quantity SET NOT NULL,
ALTER COLUMN expires_at SET NOT NULL,
ADD CONSTRAINT application_user_authorization_token2 UNIQUE USING INDEX application_user_authorization_token1;