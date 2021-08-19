CREATE TABLE channel_feed_publication_mark ( 
    id BIGINT,
    channel_feed_publication_id BIGINT,
    application_user_id BIGINT,
    type SMALLINT
);

CREATE SEQUENCE public.channel_feed_publication_mark__id__sequence INCREMENT BY 1 NO MINVALUE NO MAXVALUE
START WITH 1 CACHE 1 NO CYCLE OWNED BY public.channel_feed_publication_mark.id;

CREATE UNIQUE INDEX channel_feed_publication_mark__id__unique_index ON public.channel_feed_publication_mark
USING btree (id ASC NULLS LAST) WITH (FILLFACTOR = 90);

CREATE INDEX channel_feed_publication_mark__channel_feed_publication_id__index ON public.channel_feed_publication_mark
USING btree (channel_feed_publication_id ASC NULLS LAST) WITH (FILLFACTOR = 65);

CREATE INDEX channel_feed_publication_mark__application_user_id__index ON public.channel_feed_publication_mark
USING btree (application_user_id ASC NULLS LAST) WITH (FILLFACTOR = 65);

ALTER TABLE ONLY public.channel_feed_publication_mark
ALTER COLUMN id SET NOT NULL,
ALTER COLUMN id SET DEFAULT nextval('public.channel_feed_publication_mark__id__sequence'),
ALTER COLUMN channel_feed_publication_id SET NOT NULL,
ALTER COLUMN application_user_id SET NOT NULL,
ALTER COLUMN type SET NOT NULL,
ADD CONSTRAINT channel_feed_publication_mark__id__primary_key PRIMARY KEY USING INDEX channel_feed_publication_mark__id__unique_index,
ADD CONSTRAINT channel_feed_publication_mark__channel_feed_publication_id__foreign_key FOREIGN KEY (channel_feed_publication_id)
REFERENCES public.channel_feed_publication(id) ON DELETE CASCADE,
ADD CONSTRAINT channel_feed_publication_mark__application_user_id__foreign_key FOREIGN KEY (application_user_id)
REFERENCES public.application_user(id) ON DELETE CASCADE;