CREATE TABLE public.user_reset_password_token (
    user__id BIGINT,
    user_device__id TEXT,
    value TEXT,
    wrong_enter_tries_quantity SMALLINT,
    is_approved BOOLEAN,
    expires_at BIGINT,
    can_be_resent_from BIGINT
) WITH (oids = false, fillfactor = 85, autovacuum_enabled = true);

CREATE UNIQUE INDEX user_reset_password_token_1 ON public.user_reset_password_token
USING btree (user__id, user_device__id ASC NULLS LAST) WITH (fillfactor = 90, deduplicate_items = on);

ALTER TABLE ONLY public.user_reset_password_token
ALTER COLUMN user__id SET NOT NULL,
ALTER COLUMN user_device__id SET NOT NULL,
ALTER COLUMN value SET NOT NULL,
ALTER COLUMN wrong_enter_tries_quantity SET NOT NULL,
ALTER COLUMN is_approved SET NOT NULL,
ALTER COLUMN expires_at SET NOT NULL,
ALTER COLUMN can_be_resent_from SET NOT NULL,
ADD CONSTRAINT user_reset_password_token_2 UNIQUE USING INDEX user_reset_password_token_1;