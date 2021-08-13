CREATE TABLE channel_feed_publicaion ( 
    id BIGSERIAL NOT NULL,
    channel_id BIGINT NOT NULL,
    author_application_user_administrator_id BIGINT NOT NULL,
    is_entertaining BOOLEAN NOT NULL,
    content_type SMALLINT NOT NULL,
    content_type_component TEXT NOT NULL,  -- // TODO small_description CHARACTER VARYING(100), large_description CHARACTER VARYING(500), путь до картинки, путь до музыки
    public_marks_quantity BIGINT NOT NULL,
    hidden_marks_quantity BIGINT NOT NULL,
    reactions_quantity BIGINT NOT NULL,
    viewing_quantity BIGINT NOT NULL,
    visible_from TIMESTAMPTZ NOT NULL,
    delete_on TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL,    
    PRIMARY KEY (id)
);

-- // TODO // TODO // TODO !!!!! Описать в докблоках
-- // TODO // TODO // TODO !!!!! Принцип описания Поста: Декомпозиция ( с точки зрения НормальныхФорм БАйса-Кода) в одну широкую таблицу, объединяющую все разновидности контента внутри Поста:
-- // TODO // TODO // TODO !!!!! Есть "Тип поста" и необходимые поля для "Тип поста". ВСе поля для "Тип поста" необязательны (!НЕ! NOT NULL). В коде проверяем заполненность полей для соответстующего типа
-- // TODO // TODO // TODO !!!!! и ставим Триггер или Ограничение в БД с той же логикой. Пишем исчерпывающие интеграционные тесты.