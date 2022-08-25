CREATE TABLE public.application_user_registration_confirmation_token (
    application_user_email CHARACTER VARYING(320),
    value CHARACTER VARYING(6),
    wrong_enter_tries_quantity SMALLINT,
    is_approved BOOLEAN,
    created_at TIMESTAMP(6) WITH TIME ZONE
) WITH (oids = false, fillfactor = 95, autovacuum_enabled = true);

CREATE UNIQUE INDEX application_user_registration_confirmation_token1 ON public.application_user_registration_confirmation_token
USING btree (application_user_email ASC NULLS LAST) WITH (fillfactor = 90, deduplicate_items = on);

CREATE INDEX application_user_registration_confirmation_token2 ON public.application_user_registration_confirmation_token
USING btree (created_at ASC NULLS LAST) WITH (fillfactor = 90, deduplicate_items = on);

ALTER TABLE ONLY public.application_user_registration_confirmation_token
ALTER COLUMN application_user_email SET NOT NULL,
ALTER COLUMN value SET NOT NULL,
ALTER COLUMN wrong_enter_tries_quantity SET NOT NULL,
ALTER COLUMN is_approved SET NOT NULL,
ALTER COLUMN created_at SET NOT NULL,
ALTER COLUMN created_at SET DEFAULT current_timestamp(6),
ADD CONSTRAINT application_user_registration_confirmation_token3 UNIQUE USING INDEX application_user_registration_confirmation_token1;