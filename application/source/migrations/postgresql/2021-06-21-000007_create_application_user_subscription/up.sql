CREATE TABLE application_user_subscription ( 
    id BIGSERIAL NOT NULL,
    application_user_id BIGSERIAL NOT NULL,
    subscriber_application_user_id BIGSERIAL NOT NULL,
    PRIMARY KEY (id)
);