CREATE TABLE channel_feed_publication_mark ( 
    id BIGINT,
    channel_feed_publication_id BIGINT,
    application_user_id BIGINT,
    type SMALLINT
) WITH (oids = false, fillfactor = 100, autovacuum_enabled = true);

CREATE SEQUENCE public.channel_feed_publication_mark1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE
START WITH 1 CACHE 1 NO CYCLE OWNED BY public.channel_feed_publication_mark.id;

CREATE UNIQUE INDEX channel_feed_publication_mark2 ON public.channel_feed_publication_mark
USING btree (id ASC NULLS LAST) WITH (fillfactor = 90, deduplicate_items = on);

CREATE INDEX channel_feed_publication_mark3 ON public.channel_feed_publication_mark
USING btree (channel_feed_publication_id ASC NULLS LAST) WITH (fillfactor = 70);

CREATE INDEX channel_feed_publication_mark4 ON public.channel_feed_publication_mark
USING btree (application_user_id ASC NULLS LAST) WITH (fillfactor = 70);

ALTER TABLE ONLY public.channel_feed_publication_mark
ALTER COLUMN id SET NOT NULL,
ALTER COLUMN id SET DEFAULT nextval('public.channel_feed_publication_mark1'),
ALTER COLUMN channel_feed_publication_id SET NOT NULL,
ALTER COLUMN application_user_id SET NOT NULL,
ALTER COLUMN type SET NOT NULL,
ADD CONSTRAINT channel_feed_publication_mark5 PRIMARY KEY USING INDEX channel_feed_publication_mark2,
ADD CONSTRAINT channel_feed_publication_mark6 FOREIGN KEY (channel_feed_publication_id)
REFERENCES public.channel_feed_publication(id) ON DELETE CASCADE,
ADD CONSTRAINT channel_feed_publication_mark7 FOREIGN KEY (application_user_id)
REFERENCES public.application_user(id) ON DELETE CASCADE;