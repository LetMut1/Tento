CREATE TABLE public.application_user_access_refresh_token (
    application_user_id BIGINT,
    application_user_log_in_token_device_id TEXT,
    application_user_access_token_id TEXT,
    obfuscation_value TEXT,
    expires_at BIGINT,
    updated_at TIMESTAMP(6) WITH TIME ZONE
) WITH (oids = false, fillfactor = 95, autovacuum_enabled = true);

CREATE UNIQUE INDEX application_user_access_refresh_token1 ON public.application_user_access_refresh_token
USING btree (application_user_id, application_user_log_in_token_device_id ASC NULLS LAST) WITH (fillfactor = 90, deduplicate_items = on);

-- The index is not put on the field `expires_at` on purpose, because otherwise there will be a high load,
-- because this field will be constantly updated.

ALTER TABLE ONLY public.application_user_access_refresh_token
ALTER COLUMN application_user_id SET NOT NULL,
ALTER COLUMN application_user_log_in_token_device_id SET NOT NULL,
ALTER COLUMN application_user_access_token_id SET NOT NULL,
ALTER COLUMN obfuscation_value SET NOT NULL,
ALTER COLUMN expires_at SET NOT NULL,
ALTER COLUMN updated_at SET NOT NULL,
ADD CONSTRAINT application_user_access_refresh_token2 UNIQUE USING INDEX application_user_access_refresh_token1;