CREATE TABLE application_user_mark ( 
    id BIGSERIAL NOT NULL,
    channel_feed_publication_id BIGINT NOT NULL,
    application_user_id BIGINT NOT NULL,
    type SMALLINT NOT NULL,
    PRIMARY KEY (id)
);