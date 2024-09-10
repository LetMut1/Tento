SELECT 'This is a stub.';

-- CREATE TABLE public.channel_feed_publication (
--     id BIGINT,
--     channel__id BIGINT,
--     application_user__id BIGINT,
--     content_type SMALLINT,
--     content_type_component TEXT,  -- // TODO small_description large_description путь до картинки, путь до музыки
--     content_type_component_preview TEXT,  -- // TODO Превью для обширных типов, чтобы первично передавать меньше информации. Если обычный мем с музыкой, то Нулл. В запросе делать условие на тип.
--     public_marks_quantity BIGINT,
--     hidden_marks_quantity BIGINT,
--     reactions_quantity BIGINT,
--     viewing_quantity BIGINT,
--     status SMALLINT,            -- // TODO  Создан, удален (добавятся, возможно, еще). СДелано, чтобы можно было удалять с S3 через команду на кроне
--     visible_from TIMESTAMP(6) WITH TIME ZONE,
--     delete_on TIMESTAMP(6) WITH TIME ZONE,      -- // TODO Написать команду для удаления
--     created_at TIMESTAMP(6) WITH TIME ZONE
-- ) WITH (oids = false, fillfactor = 85, autovacuum_enabled = true);

-- CREATE SEQUENCE public.channel_feed_publication1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE
-- START WITH 1 CACHE 1 NO CYCLE OWNED BY public.channel_feed_publication.id;

-- CREATE UNIQUE INDEX channel_feed_publication2 ON public.channel_feed_publication
-- USING btree (id ASC NULLS LAST) WITH (fillfactor = 90, deduplicate_items = on);

-- -- CREATE UNIQUE INDEX channel_feed_publication3 ON public.channel_feed_publication
-- -- USING btree (channel__id, visible_from ASC NULLS LAST) WITH (fillfactor = 70);

-- -- CREATE INDEX channel_feed_publication4 ON public.channel_feed_publication
-- -- USING btree (delete_on ASC NULLS LAST) WITH (fillfactor = 90, deduplicate_items = on)
-- -- WHERE delete_on IS NOT NULL;

-- ALTER TABLE ONLY public.channel_feed_publication
-- ALTER COLUMN id SET NOT NULL,
-- ALTER COLUMN channel__id SET NOT NULL,
-- ALTER COLUMN application_user__id SET NOT NULL,
-- ALTER COLUMN content_type SET NOT NULL,
-- ALTER COLUMN content_type_component SET NOT NULL,
-- ALTER COLUMN public_marks_quantity SET NOT NULL,
-- ALTER COLUMN hidden_marks_quantity SET NOT NULL,
-- ALTER COLUMN reactions_quantity SET NOT NULL,
-- ALTER COLUMN viewing_quantity SET NOT NULL,
-- ALTER COLUMN status SET NOT NULL,
-- ALTER COLUMN visible_from SET NOT NULL,
-- ALTER COLUMN created_at SET NOT NULL,
-- ADD CONSTRAINT channel_feed_publication5 PRIMARY KEY USING INDEX channel_feed_publication2,
-- ADD CONSTRAINT channel_feed_publication6 FOREIGN KEY (channel__id)
-- REFERENCES public.channel (id) ON DELETE RESTRICT,
-- ADD CONSTRAINT channel_feed_publication7 FOREIGN KEY (application_user__id)
-- REFERENCES public.application_user (id) ON DELETE RESTRICT;
-- -- ADD CONSTRAINT channel_feed_publication8 UNIQUE USING INDEX channel_feed_publication3;





-- -- // TODO Удаление публикации - это status (deleted). То есть, если удаяелтся паблик, все публикации должны перейти в статус (делетед). (как быть при удалении  channel? )
-- -- // TODO НАПИСАТЬ команду, которая на кроне проходит по Стутус=делетед и удаляет с S3 и из бд данные.

-- -- // TODO При Зпросе из приватных каналов, нужно проверять, подписан ли пользователь на этот канал. !!!!!!!!!!!!!!!!!
-- -- нужны ли Open/close каналы (Закрытый - не видишь контент, пока не подпишешься)

-- -- // TODO TODO TODO !!!! (channel__id,visible_from)- уникальное. visible_from всегда кратно N минутам. То есть, за час можно выложить 60/N постов. ПРОВЕРЯТЬ ЭТО НА БЭке
-- --  !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!! (строчка выше). НА Фронте у админов можно отображать календарь.
-- -- // TODO Оффет делаем как (where channel__id = ... and visible_from < ... ORDER BY DESC) !! (Удалить данную запись, как только использую данный метод).