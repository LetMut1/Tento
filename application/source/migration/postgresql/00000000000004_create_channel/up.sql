CREATE TABLE public.channel ( 
    id BIGINT,
    application_user_channel_administrator_id BIGINT,
    name CHARACTER VARYING(75),
    personalization_image_path TEXT,
    description CHARACTER VARYING(500),
    is_private BOOLEAN,
    subscribers_quantity BIGINT,
    public_marks_quantity BIGINT,
    hidden_marks_quantity BIGINT,
    reactions_quantity BIGINT,
    viewing_quantity BIGINT,
    entertaining_seeable_only_content_quantity BIGINT,
    entertaining_seeable_and_hearable_content_quantity BIGINT,
    non_entertaining_seeable_only_content_quantity BIGINT,
    non_entertaining_seeable_and_hearable_content_quantity BIGINT,
    created_at TIMESTAMPTZ
) WITH (oids = false, fillfactor = 85, autovacuum_enabled = true);

CREATE SEQUENCE public.channel1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE
START WITH 1 CACHE 1 NO CYCLE OWNED BY public.channel.id;

CREATE UNIQUE INDEX channel2 ON public.channel
USING btree (id ASC NULLS LAST) WITH (fillfactor = 90, deduplicate_items = on);

CREATE UNIQUE INDEX channel3 ON public.channel
USING btree (name ASC NULLS LAST) WITH (fillfactor = 70, deduplicate_items = on);

CREATE UNIQUE INDEX channel4 ON public.channel
USING btree (is_private ASC NULLS LAST) WITH (fillfactor = 85, deduplicate_items = on);

CREATE INDEX channel5 ON public.channel
USING btree (subscribers_quantity ASC NULLS LAST) WITH (fillfactor = 70, deduplicate_items = on);

CREATE INDEX channel6 ON public.channel
USING btree (created_at ASC NULLS LAST) WITH (fillfactor = 90, deduplicate_items = on);

ALTER TABLE ONLY public.channel
ALTER COLUMN id SET NOT NULL,
ALTER COLUMN id SET DEFAULT nextval('public.channel1'),
ALTER COLUMN application_user_channel_administrator_id SET NOT NULL,
ALTER COLUMN name SET NOT NULL,
ALTER COLUMN personalization_image_path SET NOT NULL,
ALTER COLUMN is_private SET NOT NULL,
ALTER COLUMN subscribers_quantity SET NOT NULL,
ALTER COLUMN public_marks_quantity SET NOT NULL,
ALTER COLUMN hidden_marks_quantity SET NOT NULL,
ALTER COLUMN reactions_quantity SET NOT NULL,
ALTER COLUMN viewing_quantity SET NOT NULL,
ALTER COLUMN entertaining_seeable_only_content_quantity SET NOT NULL,
ALTER COLUMN entertaining_seeable_and_hearable_content_quantity SET NOT NULL,
ALTER COLUMN non_entertaining_seeable_only_content_quantity SET NOT NULL,
ALTER COLUMN non_entertaining_seeable_and_hearable_content_quantity SET NOT NULL,
ALTER COLUMN created_at SET NOT NULL,
ADD CONSTRAINT channel7 PRIMARY KEY USING INDEX channel2,
ADD CONSTRAINT channel8 FOREIGN KEY (application_user_channel_administrator_id)
REFERENCES public.application_user_channel_administrator(id) ON DELETE RESTRICT,
ADD CONSTRAINT channel9 UNIQUE USING INDEX channel3;

--  // TODO Аккаунт public.application_user_channel_administrator(id) не удалять, пока есть channel__owner_application_user_channel_administrator_id__foreign_key.
--  либо заставлять сначала удалить все паблики вручную. 
  -- // TODO Оффет делаем как (where id < ... ORDER BY DESC) !! (Удалить данную запись, как только использую данный метод)