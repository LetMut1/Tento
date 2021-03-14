CREATE TABLE pre_confirmed_application_user ( 
    id UUID NOT NULL,
    email VARCHAR NOT NULL
    PRIMARY KEY (id)
);
CREATE TABLE application_user ( 
    id UUID NOT NULL,
    email VARCHAR NOT NULL,
    nickname VARCHAR NOT NULL,
    password_hash VARCHAR NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    PRIMARY KEY (id)
);
-- // TODO On delete cascade (при удалении юзра должны удалятьсявсе зависимые таблицы)
CREATE TABLE pre_registered_application_user_registration_confirmation_token (
    id UUID NOT NULL,
    pre_confirmed_application_user_id UUID NOT NULL,  
    -- // TODO application_user_id  - уникальность 
    value VARCHAR NOT NULL,
    expired_at TIMESTAMPTZ NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (pre_confirmed_application_user_id) REFERENCES pre_confirmed_application_user (id)
);
-- // TODO Create Constraints !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
-- // TODO Сделать индексы на внешние ключи ( посмотреть, где это нужно)




-- // TODO delete 
CREATE TABLE json_refresh_web_token (   -- // TODO Redis
    id UUID NOT NULL,
    device_id VARCHAR NOT NULL,
    value VARCHAR NOT NULL,
    application_user_id UUID NOT NULL,
    expired_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (application_user_id) REFERENCES application_user (id)
);

-- CREATE TABLE json_access_web_token_black_list (        -- // TODO Redis
--     id UUID NOT NULL,
--     device_id VARCHAR NOT NULL,  колонки ДРУГИЕ !!!!!!!!!
--     value VARCHAR NOT NULL,
--     application_user_id UUID NOT NULL,
--     expired_at TIMESTAMPTZ NOT NULL,
--     created_at TIMESTAMPTZ NOT NULL,
--     PRIMARY KEY (id),
--     FOREIGN KEY (application_user_id) REFERENCES application_user (id)
-- );
-- // TODO delete 