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

-- // TODO нужно ли ДатаПоследнегоВхода-Выхода. По идее, да. Но это нагруза на Бд. С другой стороны, видимость пользтвателелй и активноти. Подумать, что нужно еще


-- // TODO On delete cascade (при удалении юзра должны удалятьсявсе зависимые таблицы)
-- // TODO Create Constraints (Внешние ключи обязательно ставить все. Если уникальное поле из 3 значений, то ставить внешний ключ на каждое, если эо поле по факту внешний ключ)
-- // TODO Сделать индексы на внешние ключи ( посмотреть, где это нужно)
-- // TODO все, где есть дата експирации, удалять по крону бинарнику (или из-под бд сразу)
-- // TODO проврить down