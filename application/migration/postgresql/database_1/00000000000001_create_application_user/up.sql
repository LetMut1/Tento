CREATE TABLE public.application_user (
    id BIGINT,
    email TEXT,
    nickname TEXT,
    password_hash TEXT,
    created_at BIGINT
) WITH (oids = false, fillfactor = 95, autovacuum_enabled = true);

CREATE SEQUENCE public.application_user1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE
START WITH 1 CACHE 1 NO CYCLE OWNED BY public.application_user.id;

CREATE UNIQUE INDEX application_user2 ON public.application_user
USING btree (id ASC NULLS LAST) WITH (fillfactor = 90, deduplicate_items = on);

CREATE UNIQUE INDEX application_user3 ON public.application_user
USING btree (email ASC NULLS LAST) WITH (fillfactor = 90, deduplicate_items = on);

CREATE UNIQUE INDEX application_user4 ON public.application_user
USING btree (nickname ASC NULLS LAST) WITH (fillfactor = 90, deduplicate_items = on);

ALTER TABLE ONLY public.application_user
ALTER COLUMN id SET NOT NULL,
ALTER COLUMN email SET NOT NULL,
ALTER COLUMN nickname SET NOT NULL,
ALTER COLUMN password_hash SET NOT NULL,
ALTER COLUMN created_at SET NOT NULL,
ADD CONSTRAINT application_user5 PRIMARY KEY USING INDEX application_user2,
ADD CONSTRAINT application_user6 UNIQUE USING INDEX application_user3,
ADD CONSTRAINT application_user7 UNIQUE USING INDEX application_user4;