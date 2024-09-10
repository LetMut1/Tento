SELECT 'This is a stub.';

-- CREATE TABLE public.channel_feed_publication_mark (
--     channel_feed_publication_id BIGINT,
--     application_user__id BIGINT,
--     type SMALLINT
-- ) WITH (oids = false, fillfactor = 90, autovacuum_enabled = true);

-- -- CREATE INDEX channel_feed_publication_mark3 ON public.channel_feed_publication_mark
-- -- USING btree (channel_feed_publication_id ASC NULLS LAST) WITH (fillfactor = 70);

-- -- CREATE INDEX channel_feed_publication_mark4 ON public.channel_feed_publication_mark
-- -- USING btree (application_user__id ASC NULLS LAST) WITH (fillfactor = 70);

-- ALTER TABLE ONLY public.channel_feed_publication_mark
-- ALTER COLUMN channel_feed_publication_id SET NOT NULL,
-- ALTER COLUMN application_user__id SET NOT NULL,
-- ALTER COLUMN type SET NOT NULL,
-- ADD CONSTRAINT channel_feed_publication_mark6 FOREIGN KEY (channel_feed_publication_id)
-- REFERENCES public.channel_feed_publication (id) ON DELETE CASCADE,
-- ADD CONSTRAINT channel_feed_publication_mark7 FOREIGN KEY (application_user__id)
-- REFERENCES public.application_user (id) ON DELETE CASCADE;