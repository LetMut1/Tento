CREATE TABLE public.user_ (
    id BIGINT,
    email TEXT,
    nickname TEXT,
    password_hash TEXT,
    created_at BIGINT
) WITH (oids = false, fillfactor = 95, autovacuum_enabled = true);

CREATE SEQUENCE public.user_1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE
START WITH 1 CACHE 1 NO CYCLE OWNED BY public.user_.id;

CREATE UNIQUE INDEX user_2 ON public.user_
USING btree (id ASC NULLS LAST) WITH (fillfactor = 90, deduplicate_items = on);

CREATE UNIQUE INDEX user_3 ON public.user_
USING btree (email ASC NULLS LAST) WITH (fillfactor = 90, deduplicate_items = on);

CREATE UNIQUE INDEX user_4 ON public.user_
USING btree (nickname ASC NULLS LAST) WITH (fillfactor = 90, deduplicate_items = on);

ALTER TABLE ONLY public.user_
ALTER COLUMN id SET NOT NULL,
ALTER COLUMN email SET NOT NULL,
ALTER COLUMN nickname SET NOT NULL,
ALTER COLUMN password_hash SET NOT NULL,
ALTER COLUMN created_at SET NOT NULL,
ADD CONSTRAINT user_5 PRIMARY KEY USING INDEX user_2,
ADD CONSTRAINT user_6 UNIQUE USING INDEX user_3,
ADD CONSTRAINT user_7 UNIQUE USING INDEX user_4;