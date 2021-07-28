CREATE TABLE channel_subscription ( 
    id BIGSERIAL NOT NULL,
    channel_id BIGSERIAL NOT NULL,
    application_user_id BIGSERIAL NOT NULL, 
    PRIMARY KEY (id)
);