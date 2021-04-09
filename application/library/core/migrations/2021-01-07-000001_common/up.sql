CREATE TABLE pre_confirmed_application_user ( 
    id UUID NOT NULL,
    -- // email value - уникальное
        -- // TODO удалять висящие кортежи (написать функцию либо через крон по бинарнику)
            -- // TODO Каскадное уделаение
    email VARCHAR NOT NULL,
    PRIMARY KEY (id)
);
CREATE TABLE application_user ( 
    id UUID NOT NULL,
    -- // TODO email - уникальное
    -- // TODO nickanme - уникальное
    email VARCHAR NOT NULL,
    nickname VARCHAR NOT NULL,
    password_hash VARCHAR NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,    
    PRIMARY KEY (id)
);
-- // TODO On delete cascade (при удалении юзра должны удалятьсявсе зависимые таблицы)
CREATE TABLE application_user_registration_confirmation_token (
    id UUID NOT NULL,
    pre_confirmed_application_user_id UUID NOT NULL,  
    -- // TODO application_user_id  - уникальность   (два отдельно)
    -- // TODO value - уникальное
    -- // TODO удалять висящие кортежи (написать функцию либо через крон по бинарнику)
    value VARCHAR NOT NULL,
    expired_at TIMESTAMPTZ NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (pre_confirmed_application_user_id) REFERENCES pre_confirmed_application_user (id)
);
CREATE TABLE application_user_log_in_token (    
    id UUID NOT NULL,
    application_user_id UUID NOT NULL,  
    -- // TODO applicationuserid + device_id- уникальное   ((как блокировать пользователя, если он делает перебор value?))
    -- // TODO удалять висящие кортежи (написать функцию либо через крон по бинарнику)
    device_id UUID NOT NULL,
    value VARCHAR NOT NULL,
    expired_at TIMESTAMPTZ NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (application_user_id) REFERENCES application_user (id)
);
-- // TODO Create Constraints (Внешние ключи обязательно ставить все. Если уникальное поле из 3 значений, то ставить внешний ключ на каждое, если эо поле по факту внешний ключ)
-- // TODO Сделать индексы на внешние ключи ( посмотреть, где это нужно)
-- // TODO все, где есть дата експирации, удалять по крону бинарнику (или из-под бд сразу)
-- // TODO проврить down



-- // TODO delete 
CREATE TABLE json_refresh_web_token (   -- // TODO Redis
    json_access_web_token_id UUID NOT NULL,
    application_user_log_in_token_device_id UUID NOT NULL,
    -- // TODO  applicationuserid + device_id +уникальное
    application_user_id UUID NOT NULL,
    expired_at TIMESTAMPTZ NOT NULL,
    PRIMARY KEY (json_access_web_token_id),
    FOREIGN KEY (application_user_id) REFERENCES application_user (id)
);

CREATE TABLE json_access_web_token_black_list (        -- // TODO Redis
    json_access_web_token_id UUID NOT NULL,
    PRIMARY KEY (json_access_web_token_id),
    FOREIGN KEY (json_access_web_token_id) REFERENCES json_refresh_web_token (json_access_web_token_id)
);
-- // TODO delete 