-- CREATE TABLE application_user_direct_message ( 
--     id BIGINT,
--     publisher_application_user_id BIGINT,
--     subscriber_application_user_id BIGINT
-- );






-- CREATE SEQUENCE public.application_user_subscription__id__sequence INCREMENT BY 1 NO MINVALUE NO MAXVALUE
-- START WITH 1 CACHE 1 NO CYCLE OWNED BY public.application_user_subscription.id;

-- CREATE UNIQUE INDEX application_user_subscription__id__unique_index ON public.application_user_subscription
-- USING btree (id ASC NULLS LAST) WITH (FILLFACTOR = 90);

-- CREATE INDEX application_user_subscription__publisher_application_user_id__index ON public.application_user_subscription
-- USING btree (publisher_application_user_id ASC NULLS LAST) WITH (FILLFACTOR = 65);

-- CREATE INDEX application_user_subscription__subscriber_application_user_id__index ON public.application_user_subscription
-- USING btree (subscriber_application_user_id ASC NULLS LAST) WITH (FILLFACTOR = 65);

-- ALTER TABLE ONLY public.application_user_subscription
-- ALTER COLUMN id SET NOT NULL,
-- ALTER COLUMN id SET DEFAULT nextval('public.application_user_subscription__id__sequence'),
-- ALTER COLUMN publisher_application_user_id SET NOT NULL,
-- ALTER COLUMN subscriber_application_user_id SET NOT NULL,
-- ADD CONSTRAINT application_user_subscription__id__primary_key PRIMARY KEY USING INDEX application_user_subscription__id__unique_index,
-- ADD CONSTRAINT application_user_subscription__publisher_application_user_id__foreign_key FOREIGN KEY (publisher_application_user_id)
-- REFERENCES public.application_user(id) ON DELETE CASCADE,
-- ADD CONSTRAINT application_user_subscription__subscriber_application_user_id__foreign_key FOREIGN KEY (subscriber_application_user_id)
-- REFERENCES public.application_user(id) ON DELETE CASCADE,
-- ADD CONSTRAINT application_user_subscription____check CHECK (publisher_application_user_id != subscriber_application_user_id);