pub mod public {
    table! {
        use diesel::sql_types::*;

        application_user (id) {
            id -> Int8,
            email -> Varchar,
            nickname -> Varchar,
            password_hash -> Text,
            created_at -> Timestamptz,
        }
    }

    table! {
        use diesel::sql_types::*;

        application_user_channel_administrator (id) {
            id -> Int8,
        }
    }

    table! {
        use diesel::sql_types::*;

        application_user_subscription (id) {
            id -> Int8,
            publisher_application_user_id -> Int8,
            subscriber_application_user_id -> Int8,
        }
    }

    table! {
        use diesel::sql_types::*;

        channel (id) {
            id -> Int8,
            owner_application_user_channel_administrator_id -> Int8,
            name -> Varchar,
            description -> Nullable<Varchar>,
            is_private -> Bool,
            subscribers_quantity -> Int8,
            public_marks_quantity -> Int8,
            hidden_marks_quantity -> Int8,
            reactions_quantity -> Int8,
            viewing_quantity -> Int8,
            entertaining_seeable_only_content_quantity -> Int8,
            entertaining_seeable_and_hearable_content_quantity -> Int8,
            non_entertaining_seeable_only_content_quantity -> Int8,
            non_entertaining_seeable_and_hearable_content_quantity -> Int8,
            created_at -> Timestamptz,
        }
    }

    table! {
        use diesel::sql_types::*;

        channel_direct_message_publication (id) {
            id -> Int8,
            channel_id -> Int8,
            author_application_user_channel_administrator_id -> Int8,
            content_type -> Int2,
            content_type_component -> Text,
            viewing_quantity -> Int8,
            status -> Int2,
            visible_from -> Timestamptz,
            delete_on -> Timestamptz,
            created_at -> Timestamptz,
        }
    }

    table! {
        use diesel::sql_types::*;

        channel_feed_publication (id) {
            id -> Int8,
            channel_id -> Int8,
            author_application_user_channel_administrator_id -> Int8,
            is_entertaining -> Bool,
            content_type -> Int2,
            content_type_component -> Text,
            content_type_component_preview -> Nullable<Text>,
            public_marks_quantity -> Int8,
            hidden_marks_quantity -> Int8,
            reactions_quantity -> Int8,
            viewing_quantity -> Int8,
            status -> Int2,
            visible_from -> Timestamptz,
            delete_on -> Nullable<Timestamptz>,
            created_at -> Timestamptz,
        }
    }

    table! {
        use diesel::sql_types::*;

        channel_feed_publication_mark (id) {
            id -> Int8,
            channel_feed_publication_id -> Int8,
            application_user_id -> Int8,
            #[sql_name = "type"]
            type_ -> Int2,
        }
    }

    table! {
        use diesel::sql_types::*;

        channel_feed_publication_reaction (id) {
            id -> Int8,
            channel_feed_publication_id -> Int8,
            application_user_id -> Int8,
            content_type -> Int2,
            content_type_component -> Text,
            public_marks_quantity -> Int8,
            created_at -> Timestamptz,
        }
    }

    table! {
        use diesel::sql_types::*;

        channel_subscription (id) {
            id -> Int8,
            channel_id -> Int8,
            application_user_id -> Int8,
            created_at -> Timestamptz,
        }
    }

    table! {
        use diesel::sql_types::*;

        pre_confirmed_application_user (id) {
            id -> Int8,
            email -> Varchar,
            created_at -> Timestamptz,
        }
    }

    joinable!(channel -> application_user_channel_administrator (owner_application_user_channel_administrator_id));
    joinable!(channel_direct_message_publication -> application_user_channel_administrator (author_application_user_channel_administrator_id));
    joinable!(channel_direct_message_publication -> channel (channel_id));
    joinable!(channel_feed_publication -> application_user_channel_administrator (author_application_user_channel_administrator_id));
    joinable!(channel_feed_publication -> channel (channel_id));
    joinable!(channel_feed_publication_mark -> application_user (application_user_id));
    joinable!(channel_feed_publication_mark -> channel_feed_publication (channel_feed_publication_id));
    joinable!(channel_feed_publication_reaction -> application_user (application_user_id));
    joinable!(channel_feed_publication_reaction -> channel_feed_publication (channel_feed_publication_id));
    joinable!(channel_subscription -> application_user (application_user_id));
    joinable!(channel_subscription -> channel (channel_id));

    allow_tables_to_appear_in_same_query!(
        application_user,
        application_user_channel_administrator,
        application_user_subscription,
        channel,
        channel_direct_message_publication,
        channel_feed_publication,
        channel_feed_publication_mark,
        channel_feed_publication_reaction,
        channel_subscription,
        pre_confirmed_application_user,
    );
}
