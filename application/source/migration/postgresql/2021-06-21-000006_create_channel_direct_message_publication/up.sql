CREATE TABLE channel_direct_message_publication ( 
    id BIGINT,
    channel_id BIGINT,
    author_application_user_channel_administrator_id BIGINT,
    content_type SMALLINT,
    content_type_component TEXT,
    viewing_quantity BIGINT,
    status SMALLINT,
    visible_from TIMESTAMPTZ,
    delete_on TIMESTAMPTZ,
    created_at TIMESTAMPTZ
);

CREATE SEQUENCE public.channel_direct_message_publication__id__sequence INCREMENT BY 1 NO MINVALUE NO MAXVALUE
START WITH 1 CACHE 1 NO CYCLE OWNED BY public.channel_direct_message_publication.id;

CREATE UNIQUE INDEX channel_direct_message_publication__id__unique_index ON public.channel_direct_message_publication
USING btree (id ASC NULLS LAST) WITH (FILLFACTOR = 90);

CREATE UNIQUE INDEX channel_direct_message_publication__channel_id__visible_from__unique_index ON public.channel_direct_message_publication
USING btree (channel_id, visible_from ASC NULLS LAST) WITH (FILLFACTOR = 65);

CREATE INDEX channel_direct_message_publication__delete_on__index ON public.channel_direct_message_publication
USING btree (delete_on ASC NULLS LAST) WITH (FILLFACTOR = 65);

ALTER TABLE ONLY public.channel_direct_message_publication
ALTER COLUMN id SET NOT NULL,
ALTER COLUMN id SET DEFAULT nextval('public.channel_direct_message_publication__id__sequence'),
ALTER COLUMN channel_id SET NOT NULL,
ALTER COLUMN author_application_user_channel_administrator_id SET NOT NULL,
ALTER COLUMN content_type SET NOT NULL,
ALTER COLUMN content_type_component SET NOT NULL,
ALTER COLUMN viewing_quantity SET NOT NULL,
ALTER COLUMN status SET NOT NULL,
ALTER COLUMN visible_from SET NOT NULL,
ALTER COLUMN delete_on SET NOT NULL,
ALTER COLUMN created_at SET NOT NULL,
ADD CONSTRAINT channel_direct_message_publication__id__primary_key PRIMARY KEY USING INDEX channel_direct_message_publication__id__unique_index,
ADD CONSTRAINT channel_direct_message_publication__channel_id__foreign_key FOREIGN KEY (channel_id)
REFERENCES public.channel(id) ON DELETE RESTRICT,
ADD CONSTRAINT channel_direct_message_publication__author_application_user_channel_administrator_id__foreign_key FOREIGN KEY (author_application_user_channel_administrator_id)
REFERENCES public.application_user_channel_administrator(id) ON DELETE RESTRICT;