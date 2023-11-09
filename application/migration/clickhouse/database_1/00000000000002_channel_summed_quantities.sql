-- migrate:up



Исходный метод не подходит по причине - Проблема в том, что если класть сюде через представление, то в РепласингМТ они заменятся на 1, а тут посчитается как 10
Возможные решения:
1. использовать count()/sum() в предыдушей таблице, удалив эту.
2. делать Селект единсвенной строки перед Вставкой и обрабатывать в коде, заменив движки таблицы
3. 2 пункт, только делать все в запросе.

Бот может спамить на Лайк и отменулайка. Если не проверять, есть ли лайк, перед отменой лайка, будет неверно? Скорее всего использовать 2 способ.



CREATE TABLE IF NOT EXISTS tento.channel_summed_quantities
(
    channel_id Int64 CODEC(LZ4),
    channel_publication_marks_quantity Int64 CODEC(LZ4)
) ENGINE = SummingMergeTree()
ORDER BY (channel_id)
PRIMARY KEY (channel_id)
SETTINGS
    index_granularity = 1,
    enable_mixed_granularity_parts = false;

-- migrate:down
DROP TABLE IF EXISTS tento.channel_summed_quantities SYNC;