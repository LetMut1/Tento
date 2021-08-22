CREATE TABLE public.pre_confirmed_application_user (
    id BIGINT,
    email CHARACTER VARYING(320),
    created_at TIMESTAMPTZ
) WITH (oids = false, fillfactor = 100, autovacuum_enabled = true);

CREATE SEQUENCE public.pre_confirmed_application_user1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE
START WITH 1 CACHE 1 NO CYCLE OWNED BY public.pre_confirmed_application_user.id;

CREATE UNIQUE INDEX pre_confirmed_application_user2 ON public.pre_confirmed_application_user
USING btree (id ASC NULLS LAST) WITH (fillfactor = 90, deduplicate_items = on);

CREATE UNIQUE INDEX pre_confirmed_application_user3 ON public.pre_confirmed_application_user
USING btree (email ASC NULLS LAST) WITH (fillfactor = 90, deduplicate_items = on);

ALTER TABLE ONLY public.pre_confirmed_application_user
ALTER COLUMN id SET NOT NULL,
ALTER COLUMN id SET DEFAULT nextval('public.pre_confirmed_application_user1'),
ALTER COLUMN email SET NOT NULL,
ALTER COLUMN created_at SET NOT NULL,
ADD CONSTRAINT pre_confirmed_application_user4 PRIMARY KEY USING INDEX pre_confirmed_application_user2,
ADD CONSTRAINT pre_confirmed_application_user5 UNIQUE USING INDEX pre_confirmed_application_user3;

-- // TODO удалять висящие кортежи (написать функцию либо через крон по бинарнику)
-- // TODO все, где есть дата експирации, удалять по крону бинарнику (или из-под бд сразу)
-- // TODO проврить down