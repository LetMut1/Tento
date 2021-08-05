pub mod public {
    table! {
        use diesel::sql_types::*;

        application_user (id) {
            id -> Int8,
            email -> Varchar,
            nickname -> Varchar,
            password_hash -> Varchar,
            created_at -> Timestamptz,
        }
    }

    table! {
        use diesel::sql_types::*;

        application_user_mark (id) {
            id -> Int8,
            channel_feed_publication_id -> Int8,
            application_user_id -> Int8,
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
            owner_application_user_administrator_id -> Int8,
            name -> Varchar,
            description -> Text,
            is_private -> Bool,
            subscribers_quantity -> Int8,
            public_marks_quantity -> Int8,
            hidden_marks_quantity -> Int8,
            reactions_quantity -> Int8,
            created_at -> Timestamptz,
        }
    }

    table! {
        use diesel::sql_types::*;

        channel_direct_message_publicaion (id) {
            id -> Int8,
            channel_id -> Int8,
            author_application_user_administrator_id -> Int8,
            #[sql_name = "type"]
            type_ -> Int2,
            type_component -> Text,
            visible_from -> Timestamptz,
            delete_on -> Timestamptz,
            created_at -> Timestamptz,
        }
    }

    table! {
        use diesel::sql_types::*;

        channel_feed_publicaion (id) {
            id -> Int8,
            channel_id -> Int8,
            author_application_user_administrator_id -> Int8,
            is_entertaining -> Bool,
            small_description -> Nullable<Text>,
            large_description -> Nullable<Text>,
            #[sql_name = "type"]
            type_ -> Int2,
            type_component -> Text,
            public_marks_quantity -> Int8,
            hidden_marks_quantity -> Int8,
            reactions_quantity -> Int8,
            visible_from -> Timestamptz,
            delete_on -> Nullable<Timestamptz>,
            created_at -> Timestamptz,
        }
    }

    table! {
        use diesel::sql_types::*;

        channel_subscription (id) {
            id -> Int8,
            channel_id -> Int8,
            application_user_id -> Int8,
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

    allow_tables_to_appear_in_same_query!(
        application_user,
        application_user_mark,
        application_user_subscription,
        channel,
        channel_direct_message_publicaion,
        channel_feed_publicaion,
        channel_subscription,
        pre_confirmed_application_user,
    );
}
