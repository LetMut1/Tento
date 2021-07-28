CREATE TABLE channel ( 
    id BIGSERIAL NOT NULL,
    owner_application_user_administrator_id BIGSERIAL NOT NULL,
    name VARCHAR NOT NULL,
    description TEXT NOT NULL,
    is_private BOOLEAN NOT NULL,
    subscribers_quantity BIGINT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,    
    PRIMARY KEY (id)
);
