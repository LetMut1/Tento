CREATE TABLE public.application_user_subscription (
    id BIGINT,
    publisher_application_user_id BIGINT,
    subscriber_application_user_id BIGINT
) WITH (oids = false, fillfactor = 90, autovacuum_enabled = true);

CREATE SEQUENCE public.application_user_subscription1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE
START WITH 1 CACHE 1 NO CYCLE OWNED BY public.application_user_subscription.id;

CREATE UNIQUE INDEX application_user_subscription2 ON public.application_user_subscription
USING btree (id ASC NULLS LAST) WITH (fillfactor = 90, deduplicate_items = on);

CREATE INDEX application_user_subscription3 ON public.application_user_subscription
USING btree (publisher_application_user_id ASC NULLS LAST) WITH (fillfactor = 70);

CREATE INDEX application_user_subscription4 ON public.application_user_subscription
USING btree (subscriber_application_user_id ASC NULLS LAST) WITH (fillfactor = 70);

ALTER TABLE ONLY public.application_user_subscription
ALTER COLUMN id SET NOT NULL,
ALTER COLUMN publisher_application_user_id SET NOT NULL,
ALTER COLUMN subscriber_application_user_id SET NOT NULL,
ADD CONSTRAINT application_user_subscription5 PRIMARY KEY USING INDEX application_user_subscription2,
ADD CONSTRAINT application_user_subscription6 FOREIGN KEY (publisher_application_user_id)
REFERENCES public.application_user (id) ON DELETE CASCADE,
ADD CONSTRAINT application_user_subscription7 FOREIGN KEY (subscriber_application_user_id)
REFERENCES public.application_user (id) ON DELETE CASCADE,
ADD CONSTRAINT application_user_subscription8 CHECK (publisher_application_user_id != subscriber_application_user_id);