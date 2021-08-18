CREATE TABLE channel_feed_reaction ( 
    id BIGSERIAL NOT NULL,
    channel_id BIGINT NOT NULL,
    application_user_id BIGINT NOT NULL,
    value CHARACTER VARYING(300),
    public_marks_quantity BIGINT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    PRIMARY KEY (id)
);