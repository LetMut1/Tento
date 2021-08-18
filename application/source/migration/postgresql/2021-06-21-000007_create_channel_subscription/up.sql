CREATE TABLE channel_subscription ( 
    id BIGSERIAL NOT NULL,
    channel_id BIGINT NOT NULL,
    application_user_id BIGINT NOT NULL, 
    PRIMARY KEY (id)
);