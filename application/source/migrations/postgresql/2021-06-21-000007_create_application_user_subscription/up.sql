CREATE TABLE application_user_subscription ( 
    id BIGSERIAL NOT NULL,
    publisher_application_user_id BIGINT NOT NULL,
    subscriber_application_user_id BIGINT NOT NULL,
    PRIMARY KEY (id)
);