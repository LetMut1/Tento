SELECT 'This is a stub.';

-- CREATE TABLE public.channel_feed_publication_reaction (
--     id BIGINT,
--     channel_feed_publication_id BIGINT,
--     application_user__id BIGINT,
--     content_type SMALLINT,  -- // TODO  Просто текст (больше количества текста), Текст и имеющиаяся у человека публикация.
--     content_type_component TEXT,
--     public_marks_quantity BIGINT,
--     created_at TIMESTAMP(6) WITH TIME ZONE
-- ) WITH (oids = false, fillfactor = 85, autovacuum_enabled = true);

-- CREATE SEQUENCE public.channel_feed_publication_reaction1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE
-- START WITH 1 CACHE 1 NO CYCLE OWNED BY public.channel_feed_publication_reaction.id;

-- CREATE UNIQUE INDEX channel_feed_publication_reaction2 ON public.channel_feed_publication_reaction
-- USING btree (id ASC NULLS LAST) WITH (fillfactor = 90, deduplicate_items = on);

-- CREATE INDEX channel_feed_publication_reaction3 ON public.channel_feed_publication_reaction
-- USING btree (channel_feed_publication_id ASC NULLS LAST) WITH (fillfactor = 70);

-- CREATE INDEX channel_feed_publication_reaction4 ON public.channel_feed_publication_reaction
-- USING btree (application_user__id ASC NULLS LAST) WITH (fillfactor = 70);

-- -- CREATE INDEX channel_feed_publication_reaction5 ON public.channel_feed_publication_reaction
-- -- USING btree (created_at ASC NULLS LAST) WITH (fillfactor = 90, deduplicate_items = on);

-- ALTER TABLE ONLY public.channel_feed_publication_reaction
-- ALTER COLUMN id SET NOT NULL,
-- ALTER COLUMN channel_feed_publication_id SET NOT NULL,
-- ALTER COLUMN application_user__id SET NOT NULL,
-- ALTER COLUMN content_type SET NOT NULL,
-- ALTER COLUMN content_type_component SET NOT NULL,
-- ALTER COLUMN public_marks_quantity SET NOT NULL,
-- ALTER COLUMN created_at SET NOT NULL,
-- ADD CONSTRAINT channel_feed_publication_reaction6 PRIMARY KEY USING INDEX channel_feed_publication_reaction2,
-- ADD CONSTRAINT channel_feed_publication_reaction7 FOREIGN KEY (channel_feed_publication_id)
-- REFERENCES public.channel_feed_publication (id) ON DELETE CASCADE,
-- ADD CONSTRAINT channel_feed_publication_reaction8 FOREIGN KEY (application_user__id)
-- REFERENCES public.application_user (id) ON DELETE CASCADE;