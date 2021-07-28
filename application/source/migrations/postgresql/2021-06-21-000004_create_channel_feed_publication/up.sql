CREATE TABLE channel_feed_publicaion ( 
    id BIGSERIAL NOT NULL,
    channel_id BIGSERIAL NOT NULL,
    author_application_user_administrator_id BIGSERIAL NOT NULL,
    is_entertaining BOOLEAN NOT NULL,
    small_description TEXT,
    large_description TEXT,
    type SMALLINT NOT NULL,
    type_component TEXT NOT NULL,
    visible_from TIMESTAMPTZ NOT NULL,
    delete_on TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL,    
    PRIMARY KEY (id)
);

-- // TODO // TODO // TODO !!!!! Описать в докблоках. Здесь пока что тип поста один и тот же, поэтому здесь обыкновенное описание.
-- // TODO // TODO // TODO !!!!! Принцип описания Поста: Декомпозиция ( с точки зрения НормальныхФорм БАйса-Кода) в одну широкую таблицу, объединяющую все разновидности контента внутри Поста:
-- // TODO // TODO // TODO !!!!! Есть "Тип поста" и необходимые поля для "Тип поста". ВСе поля для "Тип поста" необязательны (!НЕ! NOT NULL). В коде проверяем заполненность полей для соответстующего типа
-- // TODO // TODO // TODO !!!!! и ставим Триггер или Ограничение в БД с той же логикой. Пишем исчерпывающие интеграционные тесты.