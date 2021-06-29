CREATE TABLE channel_feed_publicaion ( 
    id UUID NOT NULL,
    channel_id UUID NOT NULL,
    is_entertaining BOOLEAN NOT NULL,
    small_description TEXT DEFAULT NULL,
    large_description TEXT DEFAULT NULL,
    seeable_file_path TEXT NOT NULL,
    hearable_file_path TEXT DEFAULT NULL,
    quantity_of_normal_likes BIGINT NOT NULL,
    quantity_of_hidden_likes BIGINT NOT NULL,
    quantity_of_reactions BIGINT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,    
    PRIMARY KEY (id)
);

-- // TODO // TODO // TODO !!!!! Описать в докблоках. Здесь пока что тип поста один и тот же, поэтому здесь обыкновенное описание.
-- // TODO // TODO // TODO !!!!! Принцип описания Поста: Декомпозиция ( с точки зрения НормальныхФорм БАйса-Кода) в одну широкую таблицу, объединяющую все разновидности контента внутри Поста:
-- // TODO // TODO // TODO !!!!! Есть "Тип поста" и необходимые поля для "Тип поста". ВСе поля для "Тип поста" необязательны (!НЕ! NOT NULL). В коде проверяем заполненность полей для соответстующего типа
-- // TODO // TODO // TODO !!!!! и ставим Триггер или Ограничение в БД с той же логикой. Пишем исчерпывающие интеграционные тесты.