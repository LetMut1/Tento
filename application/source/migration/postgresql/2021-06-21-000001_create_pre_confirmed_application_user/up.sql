CREATE TABLE public.pre_confirmed_application_user (
    id BIGINT,
    email CHARACTER VARYING(320),
    created_at TIMESTAMPTZ
);

CREATE SEQUENCE public.pre_confirmed_application_user__id__sequence INCREMENT BY 1 NO MINVALUE NO MAXVALUE
START WITH 1 CACHE 1 NO CYCLE OWNED BY public.pre_confirmed_application_user.id;

CREATE UNIQUE INDEX pre_confirmed_application_user__id__unique_index ON public.pre_confirmed_application_user
USING btree (id ASC NULLS LAST) WITH (FILLFACTOR = 90);

CREATE UNIQUE INDEX pre_confirmed_application_user__email__unique_index ON public.pre_confirmed_application_user
USING btree (email ASC NULLS LAST) WITH (FILLFACTOR = 90);

ALTER TABLE ONLY public.pre_confirmed_application_user
ALTER COLUMN id SET NOT NULL,
ALTER COLUMN id SET DEFAULT nextval('public.pre_confirmed_application_user__id__sequence'),
ALTER COLUMN email SET NOT NULL,
ALTER COLUMN created_at SET NOT NULL,
ADD CONSTRAINT pre_confirmed_application_user__id__primary_key PRIMARY KEY USING INDEX pre_confirmed_application_user__id__unique_index,
ADD CONSTRAINT pre_confirmed_application_user__email__unique_constraint UNIQUE USING INDEX pre_confirmed_application_user__email__unique_index;




-- TODO create an index on the expression lower(title), allowing efficient case-insensitive searches:
-- CREATE INDEX lower_title_idx ON films ((lower(title)));


-- // TODO On delete cascade (при удалении юзра должны удалятьсявсе зависимые таблицы)
-- // TODO удалять висящие кортежи (написать функцию либо через крон по бинарнику)
    -- // TODO Каскадное уделаение

-- // TODO Create Constraints (Внешние ключи обязательно ставить все. Если уникальное поле из 3 значений, то ставить внешний ключ на каждое, если эо поле по факту внешний ключ)
-- // TODO Сделать индексы на внешние ключи ( посмотреть, где это нужно)
-- // TODO все, где есть дата експирации, удалять по крону бинарнику (или из-под бд сразу)
-- // TODO проврить down