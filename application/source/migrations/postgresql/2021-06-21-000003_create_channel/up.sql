CREATE TABLE channel ( 
    id BIGSERIAL NOT NULL,
    application_user_administrator_id BIGSERIAL NOT NULL,
    name VARCHAR NOT NULL,
    is_private BOOLEAN NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,    
    PRIMARY KEY (id)
);
