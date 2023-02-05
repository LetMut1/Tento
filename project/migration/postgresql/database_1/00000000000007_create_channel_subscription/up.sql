CREATE TABLE public.channel_subscription (
    id BIGINT,
    channel_id BIGINT,
    application_user_id BIGINT,
    created_at TIMESTAMP(6) WITH TIME ZONE
) WITH (oids = false, fillfactor = 90, autovacuum_enabled = true);

CREATE SEQUENCE public.channel_subscription1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE
START WITH 1 CACHE 1 NO CYCLE OWNED BY public.channel_subscription.id;

CREATE UNIQUE INDEX channel_subscription2 ON public.channel_subscription
USING btree (id ASC NULLS LAST) WITH (fillfactor = 90, deduplicate_items = on);

CREATE INDEX channel_subscription3 ON public.channel_subscription
USING btree (application_user_id ASC NULLS LAST) WITH (fillfactor = 70);

CREATE INDEX channel_subscription4 ON public.channel_subscription
USING btree (channel_id ASC NULLS LAST) WITH (fillfactor = 70);

CREATE INDEX channel_subscription5 ON public.channel_subscription
USING btree (created_at ASC NULLS LAST) WITH (fillfactor = 90, deduplicate_items = on);

ALTER TABLE ONLY public.channel_subscription
ALTER COLUMN id SET NOT NULL,
ALTER COLUMN id SET DEFAULT nextval('public.channel_subscription1'),
ALTER COLUMN channel_id SET NOT NULL,
ALTER COLUMN application_user_id SET NOT NULL,
ALTER COLUMN created_at SET NOT NULL,
ALTER COLUMN created_at SET DEFAULT current_timestamp(6),
ADD CONSTRAINT channel_subscription6 PRIMARY KEY USING INDEX channel_subscription2,
ADD CONSTRAINT channel_subscription7 FOREIGN KEY (channel_id)
REFERENCES public.channel (id) ON DELETE CASCADE,
ADD CONSTRAINT channel_subscription8 FOREIGN KEY (application_user_id)
REFERENCES public.application_user (id) ON DELETE CASCADE;