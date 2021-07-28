CREATE TABLE channel_direct_message_publicaion ( 
    id BIGSERIAL NOT NULL,
    channel_id BIGSERIAL NOT NULL,
    -- // TODO
    created_at TIMESTAMPTZ NOT NULL,    
    PRIMARY KEY (id)
);