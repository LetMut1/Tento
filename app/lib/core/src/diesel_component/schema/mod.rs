pub mod public {
    table! {
        use diesel::sql_types::*;

        application_user (id) {
            id -> Uuid,
            email -> Varchar,
            nickname -> Varchar,
            password_hash -> Varchar,
            created_at -> Timestamptz,
        }
    }

    table! {
        use diesel::sql_types::*;

        pre_confirmed_application_user (id) {
            id -> Uuid,
            email -> Varchar,
        }
    }

    table! {
        use diesel::sql_types::*;

        application_user_registration_confirmation_token (id) {
            id -> Uuid,
            pre_confirmed_application_user_id -> Uuid,
            value -> Varchar,
            expired_at -> Timestamptz,
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

    joinable!(application_user_registration_confirmation_token -> pre_confirmed_application_user (pre_confirmed_application_user_id));
    joinable!(json_refresh_web_token -> application_user (application_user_id));

    allow_tables_to_appear_in_same_query!(
        application_user,
        application_user_registration_confirmation_token,
        json_refresh_web_token,
        pre_confirmed_application_user,
    );
}
