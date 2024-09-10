SELECT 'This is a stub.';

-- CREATE TABLE public.application_user_subscription (
--     publisher_application_user__id BIGINT,
--     subscriber_application_user__id BIGINT
-- ) WITH (oids = false, fillfactor = 90, autovacuum_enabled = true);

-- -- CREATE INDEX application_user_subscription3 ON public.application_user_subscription
-- -- USING btree (publisher_application_user__id ASC NULLS LAST) WITH (fillfactor = 70);

-- -- CREATE INDEX application_user_subscription4 ON public.application_user_subscription
-- -- USING btree (subscriber_application_user__id ASC NULLS LAST) WITH (fillfactor = 70);

-- ALTER TABLE ONLY public.application_user_subscription
-- ALTER COLUMN publisher_application_user__id SET NOT NULL,
-- ALTER COLUMN subscriber_application_user__id SET NOT NULL,
-- ADD CONSTRAINT application_user_subscription6 FOREIGN KEY (publisher_application_user__id)
-- REFERENCES public.application_user (id) ON DELETE CASCADE,
-- ADD CONSTRAINT application_user_subscription7 FOREIGN KEY (subscriber_application_user__id)
-- REFERENCES public.application_user (id) ON DELETE CASCADE,
-- ADD CONSTRAINT application_user_subscription8 CHECK (publisher_application_user__id != subscriber_application_user__id);