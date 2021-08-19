CREATE TABLE channel_subscription ( 
    id BIGINT,
    channel_id BIGINT,
    application_user_id BIGINT,
    created_at TIMESTAMPTZ
);

CREATE SEQUENCE public.channel_subscription__id__sequence INCREMENT BY 1 NO MINVALUE NO MAXVALUE
START WITH 1 CACHE 1 NO CYCLE OWNED BY public.channel_subscription.id;

CREATE UNIQUE INDEX channel_subscription__id__unique_index ON public.channel_subscription
USING btree (id ASC NULLS LAST) WITH (FILLFACTOR = 90);

CREATE INDEX channel_subscription__application_user_id__index ON public.channel_subscription
USING btree (application_user_id ASC NULLS LAST) WITH (FILLFACTOR = 65);

CREATE INDEX channel_subscription__channel_id__index ON public.channel_subscription
USING btree (channel_id ASC NULLS LAST) WITH (FILLFACTOR = 65);

CREATE INDEX channel_subscription__created_at__index ON public.channel_subscription
USING btree (created_at ASC NULLS LAST) WITH (FILLFACTOR = 90);

ALTER TABLE ONLY public.channel_subscription
ALTER COLUMN id SET NOT NULL,
ALTER COLUMN id SET DEFAULT nextval('public.channel_subscription__id__sequence'),
ALTER COLUMN channel_id SET NOT NULL,
ALTER COLUMN application_user_id SET NOT NULL,
ALTER COLUMN created_at SET NOT NULL,
ADD CONSTRAINT channel_subscription__id__primary_key PRIMARY KEY USING INDEX channel_subscription__id__unique_index,
ADD CONSTRAINT channel_subscription__channel_id__foreign_key FOREIGN KEY (channel_id)
REFERENCES public.channel(id) ON DELETE CASCADE,
ADD CONSTRAINT channel_subscription__application_user_id__foreign_key FOREIGN KEY (application_user_id)
REFERENCES public.application_user(id) ON DELETE CASCADE;