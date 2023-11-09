-- migrate:up

// как удалять лайк ? +1 -1 Коллапсинг


CREATE TABLE IF NOT EXISTS tento.channel_publication_mark
(
    channel_id Int64 CODEC(LZ4),
    application_user_id Int64 CODEC(LZ4),
    channel_publication_id Int64 CODEC(LZ4),
    created_at DateTime('UTC') CODEC(LZ4)
) ENGINE = ReplacingMergeTree()
ORDER BY (channel_id, application_user_id, channel_publication_id)
PRIMARY KEY (channel_id, application_user_id, channel_publication_id)
SETTINGS
    index_granularity = 1024,
    enable_mixed_granularity_parts = false;

-- migrate:down
DROP TABLE IF EXISTS tento.channel_publication_mark SYNC;

-- TODO стоит ли делать партицию по датам именно публикации  - это поле можно будет добавить  и заменить на него channel_id