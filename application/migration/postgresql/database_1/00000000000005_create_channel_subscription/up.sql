CREATE TABLE public.channel_subscription (
    application_user__id BIGINT,
    channel__id BIGINT,
    created_at BIGINT
) WITH (oids = false, fillfactor = 90, autovacuum_enabled = true);

CREATE UNIQUE INDEX channel_subscription1 ON public.channel_subscription
USING btree (application_user__id, channel__id ASC NULLS LAST) WITH (fillfactor = 70);

ALTER TABLE ONLY public.channel_subscription
ALTER COLUMN application_user__id SET NOT NULL,
ALTER COLUMN channel__id SET NOT NULL,
ALTER COLUMN created_at SET NOT NULL,
ADD CONSTRAINT channel_subscription2 FOREIGN KEY (channel__id)
REFERENCES public.channel (id) ON DELETE CASCADE,
ADD CONSTRAINT channel_subscription3 FOREIGN KEY (application_user__id)
REFERENCES public.application_user (id) ON DELETE CASCADE;