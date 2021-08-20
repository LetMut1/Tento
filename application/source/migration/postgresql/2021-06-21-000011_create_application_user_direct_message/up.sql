CREATE TABLE application_user_direct_message ( 
    id BIGINT,
    members_application_user_id BIGINT ARRAY
);








-- CREATE INDEX application_user_direct_message__members_application_user_id__index ON public.application_user_direct_message
-- USING gin (members_application_user_id);




