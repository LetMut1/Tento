CREATE TABLE channel ( 
    id UUID NOT NULL,
    name VARCHAR NOT NULL,
    is_private BOOLEAN NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,    
    PRIMARY KEY (id)
);
