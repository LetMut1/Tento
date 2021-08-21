CREATE TABLE application_user_direct_message ( 
    id BIGINT,
    members_application_user_id JSONB,
    messages_quantity INTEGER,
    created_at TIMESTAMPTZ
) WITH (oids = false, fillfactor = 85, autovacuum_enabled = true);

CREATE SEQUENCE public.application_user_direct_message1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE
START WITH 1 CACHE 1 NO CYCLE OWNED BY public.application_user_direct_message.id;

CREATE UNIQUE INDEX application_user_direct_message2 ON public.application_user_direct_message
USING btree (id ASC NULLS LAST) WITH (fillfactor = 90, deduplicate_items = on);

CREATE INDEX application_user_direct_message3 ON public.application_user_direct_message
USING gin (members_application_user_id jsonb_path_ops) WITH (fastupdate = on);

CREATE FUNCTION public.check_application_user_direct_message_members_for_uniqueness(IN members_application_user_id JSONB) RETURNS BOOLEAN AS $$
DECLARE
  all_values TEXT;
  all_uniquess_values TEXT;
BEGIN
    SELECT COUNT(*) INTO all_values FROM jsonb_array_elements_text(members_application_user_id);

    SELECT COUNT(*) INTO all_uniquess_values FROM (SELECT DISTINCT * FROM jsonb_array_elements_text(members_application_user_id)) as result;

    IF all_values = all_uniquess_values THEN
      RETURN TRUE;
   ELSE
      RETURN FALSE;
   END IF;
END
$$
LANGUAGE plpgsql;

ALTER TABLE ONLY public.application_user_direct_message
ALTER COLUMN id SET NOT NULL,
ALTER COLUMN id SET DEFAULT nextval('public.application_user_direct_message1'),
ALTER COLUMN members_application_user_id SET NOT NULL,
ALTER COLUMN messages_quantity SET NOT NULL,
ALTER COLUMN created_at SET NOT NULL,
ADD CONSTRAINT application_user_direct_message6 PRIMARY KEY USING INDEX application_user_direct_message2,
ADD CONSTRAINT application_user_direct_message7 CHECK (public.check_application_user_direct_message_members_for_uniqueness(members_application_user_id));