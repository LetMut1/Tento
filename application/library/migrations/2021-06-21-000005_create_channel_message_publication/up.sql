CREATE TABLE channel_message_publicaion ( 
    id UUID NOT NULL,
    channel_id UUID NOT NULL,
    -- // TODO
    created_at TIMESTAMPTZ NOT NULL,    
    PRIMARY KEY (id)
);