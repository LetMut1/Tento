pub mod public {
    table! {
        use diesel::sql_types::*;

        application_user (id) {
            id -> Uuid,
            email -> Varchar,
            nickname -> Varchar,
            password_hash -> Varchar,
            created_at -> Timestamptz,
            confirmed -> Bool,
        }
    }

    table! {
        use diesel::sql_types::*;

        json_refresh_web_token (id) {
            id -> Uuid,
            device_id -> Varchar,
            value -> Varchar,
            application_user_id -> Uuid,
            expired_at -> Timestamptz,
            created_at -> Timestamptz,
        }
    }

    table! {
        use diesel::sql_types::*;

        application_user_registration_confirmation_token (id) {
            id -> Uuid,
            value -> Varchar,
            application_user_id -> Uuid,
            expired_at -> Timestamptz,
        }
    }

    joinable!(json_refresh_web_token -> application_user (application_user_id));
    joinable!(application_user_registration_confirmation_token -> application_user (application_user_id));

    allow_tables_to_appear_in_same_query!(
        application_user,
        json_refresh_web_token,
        application_user_registration_confirmation_token,
    );
}
