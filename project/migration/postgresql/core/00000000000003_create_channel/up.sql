CREATE FUNCTION public.limit_channel_subscribers_quantity(IN channel_subscribers_quantity BIGINT) RETURNS BIGINT AS
$$
  BEGIN
    RETURN channel_subscribers_quantity / 50;
  END
$$
LANGUAGE plpgsql
IMMUTABLE;

CREATE TABLE public.channel (
    id BIGINT,
    application_user_channel_administrator_id BIGINT,
    name TEXT,
    personalization_image_path TEXT,
    description TEXT,
    is_private BOOLEAN,
    subscribers_quantity BIGINT,
    public_marks_quantity BIGINT,
    hidden_marks_quantity BIGINT,
    reactions_quantity BIGINT,
    viewing_quantity BIGINT,
    created_at TIMESTAMP(6) WITH TIME ZONE
) WITH (oids = false, fillfactor = 85, autovacuum_enabled = true);

CREATE SEQUENCE public.channel1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE
START WITH 1 CACHE 1 NO CYCLE OWNED BY public.channel.id;

CREATE UNIQUE INDEX channel2 ON public.channel
USING btree (id ASC NULLS LAST) WITH (fillfactor = 90, deduplicate_items = on);

CREATE UNIQUE INDEX channel3 ON public.channel
USING btree (name COLLATE "C" ASC NULLS LAST) WITH (fillfactor = 70, deduplicate_items = on);

CREATE INDEX channel4 ON public.channel
USING btree (is_private ASC NULLS LAST) WITH (fillfactor = 85, deduplicate_items = on);

CREATE INDEX channel5 ON public.channel
USING btree (public.limit_channel_subscribers_quantity(subscribers_quantity) ASC NULLS LAST) WITH (fillfactor = 70, deduplicate_items = on);

CREATE UNIQUE INDEX channel6 ON public.channel
USING btree (created_at ASC NULLS LAST) WITH (fillfactor = 90, deduplicate_items = on);

ALTER TABLE ONLY public.channel
ALTER COLUMN id SET NOT NULL,
ALTER COLUMN id SET DEFAULT nextval('public.channel1'),
ALTER COLUMN application_user_channel_administrator_id SET NOT NULL,
ALTER COLUMN name SET DATA TYPE TEXT COLLATE "C",
ALTER COLUMN name SET NOT NULL,
ALTER COLUMN personalization_image_path SET NOT NULL,
ALTER COLUMN is_private SET NOT NULL,
ALTER COLUMN subscribers_quantity SET NOT NULL,
ALTER COLUMN public_marks_quantity SET NOT NULL,
ALTER COLUMN hidden_marks_quantity SET NOT NULL,
ALTER COLUMN reactions_quantity SET NOT NULL,
ALTER COLUMN viewing_quantity SET NOT NULL,
ALTER COLUMN created_at SET NOT NULL,
ALTER COLUMN created_at SET DEFAULT current_timestamp(6),
ADD CONSTRAINT channel7 PRIMARY KEY USING INDEX channel2,
ADD CONSTRAINT channel8 FOREIGN KEY (application_user_channel_administrator_id)
REFERENCES public.application_user_channel_administrator(id) ON DELETE RESTRICT,
ADD CONSTRAINT channel9 UNIQUE USING INDEX channel3,
ADD CONSTRAINT channel10 UNIQUE USING INDEX channel6;


--  // TODO Аккаунт public.application_user_channel_administrator(id) не удалять, пока есть channel__owner_application_user_channel_administrator_id__foreign_key.
--  либо заставлять сначала удалить все паблики вручную.
  -- // TODO Оффет делаем как (where id < ... ORDER BY DESC) !! (Удалить данную запись, как только использую данный метод)