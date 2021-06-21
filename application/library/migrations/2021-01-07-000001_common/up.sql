                                        -- // TODO // TODO // TODO // TODO // TODOделать все Ограничения (даже FK) (кроме PK) через Alter Table !!!!!!
CREATE TABLE pre_confirmed_application_user (       -- // TODO изучить полный синтаксис создания таблиц
    id UUID NOT NULL,
    -- // email value - уникальное
        -- // TODO удалять висящие кортежи (написать функцию либо через крон по бинарнику)
            -- // TODO Каскадное уделаение
    email VARCHAR NOT NULL,
    created_at TIMESTAMPTZ NOT NULL, 
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
ALTER TABLE application_user ADD CONSTRAINT email_1 UNIQUE (email);

-- // TODO On delete cascade (при удалении юзра должны удалятьсявсе зависимые таблицы)
-- // TODO Create Constraints (Внешние ключи обязательно ставить все. Если уникальное поле из 3 значений, то ставить внешний ключ на каждое, если эо поле по факту внешний ключ)
-- // TODO Сделать индексы на внешние ключи ( посмотреть, где это нужно)
-- // TODO все, где есть дата експирации, удалять по крону бинарнику (или из-под бд сразу)
-- // TODO проврить down