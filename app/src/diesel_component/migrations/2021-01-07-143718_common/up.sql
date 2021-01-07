CREATE TABLE app_user (
    email VARCHAR NOT NULL,
    id UUID NOT NULL,
    jwt_id UUID NOT NULL,
    nickname VARCHAR NOT NULL,
    PRIMARY KEY (id)
);