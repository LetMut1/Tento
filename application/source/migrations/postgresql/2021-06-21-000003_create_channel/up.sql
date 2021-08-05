CREATE TABLE channel ( 
    id BIGSERIAL NOT NULL,
    owner_application_user_administrator_id BIGINT NOT NULL,
    name VARCHAR NOT NULL,
    description VARCHAR NOT NULL,
    is_private BOOLEAN NOT NULL,
    subscribers_quantity BIGINT NOT NULL,
    public_marks_quantity BIGINT NOT NULL,
    hidden_marks_quantity BIGINT NOT NULL,
    reactions_quantity BIGINT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,    
    PRIMARY KEY (id)
);