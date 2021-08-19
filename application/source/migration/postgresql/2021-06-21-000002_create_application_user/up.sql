CREATE TABLE application_user ( 
    id BIGINT,
    email CHARACTER VARYING(320),
    nickname CHARACTER VARYING(55),
    password_hash TEXT,
    created_at TIMESTAMPTZ
);

CREATE SEQUENCE public.application_user__id__sequence INCREMENT BY 1 NO MINVALUE NO MAXVALUE
START WITH 1 CACHE 1 NO CYCLE OWNED BY public.application_user.id;

CREATE UNIQUE INDEX application_user__id__unique_index ON public.application_user
USING btree (id ASC NULLS LAST) WITH (FILLFACTOR = 90);

CREATE UNIQUE INDEX application_user__email__unique_index ON public.application_user
USING btree (email ASC NULLS LAST) WITH (FILLFACTOR = 90);

CREATE UNIQUE INDEX application_user__nickname__unique_index ON public.application_user
USING btree (nickname ASC NULLS LAST) WITH (FILLFACTOR = 90);

ALTER TABLE ONLY public.application_user
ALTER COLUMN id SET NOT NULL,
ALTER COLUMN id SET DEFAULT nextval('public.application_user__id__sequence'),
ALTER COLUMN email SET NOT NULL,
ALTER COLUMN nickname SET NOT NULL,
ALTER COLUMN password_hash SET NOT NULL,
ALTER COLUMN created_at SET NOT NULL,
ADD CONSTRAINT application_user__id__primary_key PRIMARY KEY USING INDEX application_user__id__unique_index,
ADD CONSTRAINT application_user__email__unique_constraint UNIQUE USING INDEX application_user__email__unique_index,
ADD CONSTRAINT application_user__nickname__unique_constraint UNIQUE USING INDEX application_user__nickname__unique_index;
    
-- // TODO нужно ли ДатаПоследнегоВхода-Выхода. По идее, да. Но это нагруза на Бд. С другой стороны, видимость пользтвателелй и активноти. Подумать, что нужно еще