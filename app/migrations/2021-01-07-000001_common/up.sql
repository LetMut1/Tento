CREATE TABLE application_user (
    id UUID NOT NULL,
    email VARCHAR NOT NULL,
    nickname VARCHAR NOT NULL,
    password_hash VARCHAR NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    confirmed BOOLEAN NOT NULL,
    PRIMARY KEY (id)
);
-- // TODO Create Constraints !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

-- // TODO delete 
CREATE TABLE json_refresh_web_token (
    id UUID NOT NULL,
    device_id VARCHAR NOT NULL,
    value VARCHAR NOT NULL,
    user_id UUID NOT NULL,
    expired_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (user_id) REFERENCES application_user (id)
);
-- // TODO delete 