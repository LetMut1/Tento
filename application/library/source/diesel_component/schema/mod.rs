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

        channel (id) {
            id -> Int8,
            name -> Varchar,
            is_private -> Bool,
            created_at -> Timestamptz,
        }
    }

    table! {
        use diesel::sql_types::*;

        channel_feed_publicaion (id) {
            id -> Int8,
            channel_id -> Int8,
            is_entertaining -> Bool,
            small_description -> Nullable<Text>,
            large_description -> Nullable<Text>,
            seeable_file_path -> Text,
            hearable_file_path -> Nullable<Text>,
            quantity_of_normal_likes -> Int8,
            quantity_of_hidden_likes -> Int8,
            quantity_of_reactions -> Int8,
            created_at -> Timestamptz,
        }
    }

    table! {
        use diesel::sql_types::*;

        channel_message_publicaion (id) {
            id -> Int8,
            channel_id -> Int8,
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

    allow_tables_to_appear_in_same_query!(
        application_user,
        channel,
        channel_feed_publicaion,
        channel_message_publicaion,
        pre_confirmed_application_user,
    );
}
