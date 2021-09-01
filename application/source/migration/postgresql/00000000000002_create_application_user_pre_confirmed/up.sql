CREATE TABLE public.application_user_pre_confirmed (
    id BIGINT,
    application_user_email CHARACTER VARYING(320),
    created_at TIMESTAMP(6) WITH TIME ZONE
) WITH (oids = false, fillfactor = 100, autovacuum_enabled = true);

CREATE SEQUENCE public.application_user_pre_confirmed1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE
START WITH 1 CACHE 1 NO CYCLE OWNED BY public.application_user_pre_confirmed.id;

CREATE UNIQUE INDEX application_user_pre_confirmed2 ON public.application_user_pre_confirmed
USING btree (id ASC NULLS LAST) WITH (fillfactor = 90, deduplicate_items = on);

CREATE UNIQUE INDEX application_user_pre_confirmed3 ON public.application_user_pre_confirmed
USING btree (application_user_email ASC NULLS LAST) WITH (fillfactor = 90, deduplicate_items = on);

ALTER TABLE ONLY public.application_user_pre_confirmed
ALTER COLUMN id SET NOT NULL,
ALTER COLUMN id SET DEFAULT nextval('public.application_user_pre_confirmed1'),
ALTER COLUMN application_user_email SET NOT NULL,
ALTER COLUMN created_at SET NOT NULL,
ADD CONSTRAINT application_user_pre_confirmed4 PRIMARY KEY USING INDEX application_user_pre_confirmed2,
ADD CONSTRAINT application_user_pre_confirmed5 UNIQUE USING INDEX application_user_pre_confirmed3;

-- // TODO удалять висящие кортежи (написать функцию либо через крон по бинарнику)
-- // TODO все, где есть дата експирации, удалять по крону бинарнику (или из-под бд сразу)