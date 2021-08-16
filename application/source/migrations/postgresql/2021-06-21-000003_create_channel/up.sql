CREATE TABLE channel ( 
    id BIGSERIAL NOT NULL,  -- // TODO Оффет делаем как (where id < ... ORDER BY DESC) !!
    owner_application_user_administrator_id BIGINT NOT NULL,
    name CHARACTER VARYING(75) NOT NULL,
    description CHARACTER VARYING(500),
    is_private BOOLEAN NOT NULL,
    subscribers_quantity BIGINT NOT NULL,
    public_marks_quantity BIGINT NOT NULL,
    hidden_marks_quantity BIGINT NOT NULL,
    reactions_quantity BIGINT NOT NULL,
    viewing_quantity BIGINT NOT NULL,
    entertaining_seeable_only_content_quantity BIGINT NOT NULL,
    entertaining_seeable_and_hearable_content_quantity BIGINT NOT NULL,
    non_entertaining_seeable_only_content_quantity BIGINT NOT NULL,
    non_entertaining_seeable_and_hearable_content_quantity BIGINT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    PRIMARY KEY (id)
);