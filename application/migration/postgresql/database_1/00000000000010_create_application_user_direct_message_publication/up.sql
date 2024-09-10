SELECT 'This is a stub.';

-- CREATE TABLE public.application_user_direct_message_publication (
--     id BIGINT,
--     application_user_direct_message_id BIGINT,
--     application_user__id BIGINT,
--     channel_feed_publication_id BIGINT,
--     channel_feed_publication_reaction_id BIGINT,
--     created_at TIMESTAMP(6) WITH TIME ZONE
-- ) WITH (oids = false, fillfactor = 90, autovacuum_enabled = true);

-- CREATE SEQUENCE public.application_user_direct_message_publication1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE
-- START WITH 1 CACHE 1 NO CYCLE OWNED BY public.application_user_direct_message_publication.id;

-- CREATE UNIQUE INDEX application_user_direct_message_publication2 ON public.application_user_direct_message_publication
-- USING btree (id ASC NULLS LAST) WITH (fillfactor = 90, deduplicate_items = on);

-- CREATE INDEX application_user_direct_message_publication3 ON public.application_user_direct_message_publication
-- USING btree (application_user_direct_message_id, created_at ASC NULLS LAST) WITH (fillfactor = 70, deduplicate_items = on);

-- ALTER TABLE ONLY public.application_user_direct_message_publication
-- ALTER COLUMN id SET NOT NULL,
-- ALTER COLUMN application_user_direct_message_id SET NOT NULL,
-- ALTER COLUMN application_user__id SET NOT NULL,
-- ALTER COLUMN created_at SET NOT NULL,
-- ADD CONSTRAINT application_user_direct_message_publication4 PRIMARY KEY USING INDEX application_user_direct_message_publication2,
-- ADD CONSTRAINT application_user_direct_message_publication5 FOREIGN KEY (application_user_direct_message_id)
-- REFERENCES public.application_user_direct_message (id) ON DELETE CASCADE,
-- ADD CONSTRAINT application_user_direct_message_publication6 FOREIGN KEY (application_user__id)
-- REFERENCES public.application_user (id) ON DELETE CASCADE,
-- ADD CONSTRAINT application_user_direct_message_publication7 FOREIGN KEY (channel_feed_publication_id)
-- REFERENCES public.channel_feed_publication (id) ON DELETE CASCADE,
-- ADD CONSTRAINT application_user_direct_message_publication8 FOREIGN KEY (channel_feed_publication_reaction_id)
-- REFERENCES public.channel_feed_publication_reaction (id) ON DELETE CASCADE,
-- ADD CONSTRAINT application_user_direct_message_publication9 CHECK (
--     (channel_feed_publication_id IS NULL AND channel_feed_publication_reaction_id IS NOT NULL) OR
--     (channel_feed_publication_id IS NOT NULL AND channel_feed_publication_reaction_id IS NULL)
-- );