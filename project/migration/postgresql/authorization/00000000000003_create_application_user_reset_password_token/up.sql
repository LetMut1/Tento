CREATE TABLE public.application_user_reset_password_token (
    application_user_id BIGINT,
    value TEXT,
    wrong_enter_tries_quantity SMALLINT,
    is_approved BOOLEAN,
    expires_at TIMESTAMP(6) WITH TIME ZONE
) WITH (oids = false, fillfactor = 95, autovacuum_enabled = true);

CREATE UNIQUE INDEX application_user_reset_password_token1 ON public.application_user_reset_password_token
USING btree (application_user_id ASC NULLS LAST) WITH (fillfactor = 90, deduplicate_items = on);

CREATE INDEX application_user_reset_password_token2 ON public.application_user_reset_password_token
USING btree (expires_at ASC NULLS LAST) WITH (fillfactor = 90, deduplicate_items = on);

ALTER TABLE ONLY public.application_user_reset_password_token
ALTER COLUMN application_user_id SET NOT NULL,
ALTER COLUMN value SET NOT NULL,
ALTER COLUMN wrong_enter_tries_quantity SET NOT NULL,
ALTER COLUMN is_approved SET NOT NULL,
ALTER COLUMN expires_at SET NOT NULL,
ADD CONSTRAINT application_user_reset_password_token3 UNIQUE USING INDEX application_user_reset_password_token1;