CREATE TABLE public.pre_confirmed_application_user (
    id BIGINT,
    email CHARACTER VARYING(320),
    created_at TIMESTAMPTZ
);

CREATE SEQUENCE public.pre_confirmed_application_user__id__sequence INCREMENT BY 1 NO MINVALUE NO MAXVALUE
START WITH 1 CACHE 1 NO CYCLE OWNED BY public.pre_confirmed_application_user.id;

ALTER TABLE ONLY public.pre_confirmed_application_user
    ALTER COLUMN id SET NOT NULL DEFAULT nextval('public.pre_confirmed_application_user__id_sequence'),
    ALTER COLUMN email SET NOT NULL,
    ALTER COLUMN created_at SET NOT NULL,
    ADD CONSTRAINT pre_confirmed_application_user__id__primary_key PRIMARY KEY (id);

-- // TODO // TODO // TODO // TODO  Индексы изучить CREATE INDEX CONCURRENTLY !!!!!!!1.

-- https://www.postgresql.org/docs/8.3/sql-createindex.html


-- Чувствительны ли индексы к Регистру. (Нужно ли делать так для Емэйла или Никнейма)
-- To create an index on the expression lower(title), allowing efficient case-insensitive searches:
-- CREATE INDEX lower_title_idx ON films ((lower(title)));


--  Пройтись по Филфактору для B-tree в контексте постоянно обновляющихся данных.


-- TODO https://postgrespro.ru/docs/postgresql/9.6/datatype-numeric#datatype-serial    !!!! УДАЛИТь ВЕзДе SERIAL, заменить на рукописное написпние Secu


    -- // email value - уникальное
        -- // TODO удалять висящие кортежи (написать функцию либо через крон по бинарнику)
            -- // TODO Каскадное уделаение


-- // TODO On delete cascade (при удалении юзра должны удалятьсявсе зависимые таблицы)
-- // TODO Create Constraints (Внешние ключи обязательно ставить все. Если уникальное поле из 3 значений, то ставить внешний ключ на каждое, если эо поле по факту внешний ключ)
-- // TODO Сделать индексы на внешние ключи ( посмотреть, где это нужно)
-- // TODO все, где есть дата експирации, удалять по крону бинарнику (или из-под бд сразу)
-- // TODO проврить down