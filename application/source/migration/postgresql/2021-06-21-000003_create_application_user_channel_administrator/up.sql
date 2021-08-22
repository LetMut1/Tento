




-- TODO ПОКА ЧТО ЭТО ПРОСТО СТАБ РАДИ СУЩЕСТВОВАНИЯ СУЩНОСТИ, чтобы можно сделать FK на id.

CREATE TABLE application_user_channel_administrator ( 
    id BIGINT
    -- email CHARACTER VARYING(320),
    -- nickname CHARACTER VARYING(55),
    -- password_hash TEXT,
    -- created_at TIMESTAMPTZ
) WITH (oids = false, fillfactor = 100, autovacuum_enabled = true);

CREATE SEQUENCE public.application_user_channel_administrator1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE
START WITH 1 CACHE 1 NO CYCLE OWNED BY public.application_user_channel_administrator.id;

CREATE UNIQUE INDEX application_user_channel_administrator2 ON public.application_user_channel_administrator
USING btree (id ASC NULLS LAST) WITH (fillfactor = 90, deduplicate_items = on);


ALTER TABLE ONLY public.application_user_channel_administrator
ALTER COLUMN id SET NOT NULL,
ALTER COLUMN id SET DEFAULT nextval('public.application_user_channel_administrator1'),
-- ALTER COLUMN email SET NOT NULL,
-- ALTER COLUMN nickname SET NOT NULL,
-- ALTER COLUMN password_hash SET NOT NULL,
-- ALTER COLUMN created_at SET NOT NULL,
ADD CONSTRAINT application_user_channel_administrator3 PRIMARY KEY USING INDEX application_user_channel_administrator2;
-- ADD CONSTRAINT application_user_channel_administrator__email__unique_constraint UNIQUE USING INDEX application_user_channel_administrator__email__unique_index,
-- ADD CONSTRAINT application_user_channel_administrator__nickname__unique_constraint UNIQUE USING INDEX application_user_channel_administrator__nickname__unique_index;