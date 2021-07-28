CREATE TABLE channel_direct_message_publicaion ( 
    id BIGSERIAL NOT NULL,
    channel_id BIGSERIAL NOT NULL,
    author_application_user_administrator_id BIGSERIAL NOT NULL,
    type SMALLINT NOT NULL,
    type_component TEXT NOT NULL,
    visible_from TIMESTAMPTZ NOT NULL,
    delete_on TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,    
    PRIMARY KEY (id)
);