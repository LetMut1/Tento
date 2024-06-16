CREATE TABLE public.application_user_access_refresh_token (
    application_user__id BIGINT,
    application_user_device__id TEXT,
    application_user_access_token__id TEXT,
    obfuscation_value TEXT,
    expires_at BIGINT,
    updated_at BIGINT
) WITH (oids = false, fillfactor = 95, autovacuum_enabled = true);

CREATE UNIQUE INDEX application_user_access_refresh_token1 ON public.application_user_access_refresh_token
USING btree (application_user__id, application_user_device__id ASC NULLS LAST) WITH (fillfactor = 90, deduplicate_items = on);

-- The index is not put on the field `expires_at` on purpose, because otherwise there will be a high load,
-- because this field will be constantly updated. That is, we will have a constant rebuilding of the index.

ALTER TABLE ONLY public.application_user_access_refresh_token
ALTER COLUMN application_user__id SET NOT NULL,
ALTER COLUMN application_user_device__id SET NOT NULL,
ALTER COLUMN application_user_access_token__id SET NOT NULL,
ALTER COLUMN obfuscation_value SET NOT NULL,
ALTER COLUMN expires_at SET NOT NULL,
ALTER COLUMN updated_at SET NOT NULL,
ADD CONSTRAINT application_user_access_refresh_token2 UNIQUE USING INDEX application_user_access_refresh_token1;