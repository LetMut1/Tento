CREATE TABLE public.user_registration_token (
    user__email TEXT,
    user_device__id TEXT,
    value TEXT,
    wrong_enter_tries_quantity SMALLINT,
    is_approved BOOLEAN,
    expires_at BIGINT,
    can_be_resent_from BIGINT
) WITH (oids = false, fillfactor = 85, autovacuum_enabled = true);

CREATE UNIQUE INDEX user_registration_token1 ON public.user_registration_token
USING btree (user__email, user_device__id ASC NULLS LAST) WITH (fillfactor = 90, deduplicate_items = on);

-- The index is not put on the field `expires_at` on purpose, because otherwise there will be a high load.

ALTER TABLE ONLY public.user_registration_token
ALTER COLUMN user__email SET NOT NULL,
ALTER COLUMN user_device__id SET NOT NULL,
ALTER COLUMN value SET NOT NULL,
ALTER COLUMN wrong_enter_tries_quantity SET NOT NULL,
ALTER COLUMN is_approved SET NOT NULL,
ALTER COLUMN expires_at SET NOT NULL,
ALTER COLUMN can_be_resent_from SET NOT NULL,
ADD CONSTRAINT user_registration_token2 UNIQUE USING INDEX user_registration_token1;