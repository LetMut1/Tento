SELECT 'This is a stub.';

-- CREATE TABLE public.application_user_direct_message (
--     id BIGINT,
--     list_of_members JSONB
-- ) WITH (oids = false, fillfactor = 90, autovacuum_enabled = true);

-- CREATE SEQUENCE public.application_user_direct_message1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE
-- START WITH 1 CACHE 1 NO CYCLE OWNED BY public.application_user_direct_message.id;

-- CREATE UNIQUE INDEX application_user_direct_message2 ON public.application_user_direct_message
-- USING btree (id ASC NULLS LAST) WITH (fillfactor = 90, deduplicate_items = on);

-- CREATE INDEX application_user_direct_message3 ON public.application_user_direct_message
-- USING gin (list_of_members jsonb_path_ops) WITH (fastupdate = on);

-- ALTER TABLE ONLY public.application_user_direct_message
-- ALTER COLUMN id SET NOT NULL,
-- ALTER COLUMN list_of_members SET NOT NULL,
-- ADD CONSTRAINT application_user_direct_message4 PRIMARY KEY USING INDEX application_user_direct_message2;

-- --  // TODO Проверка в коде: Каждое значение из list_of_members есть как первичный ключ.
-- -- // TODO Проверка через Валидатор на количесество Мнмнбров в джсоне