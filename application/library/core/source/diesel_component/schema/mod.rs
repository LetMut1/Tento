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

        application_user_log_in_token (id) {
            id -> Uuid,
            application_user_id -> Uuid,
            device_id -> Uuid,
            value -> Varchar,
            expired_at -> Timestamptz,
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
            application_user_log_in_token_device_id -> Uuid,
            application_user_id -> Uuid,
            expired_at -> Timestamptz,
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

        json_access_web_token_black_list (json_refresh_web_token_id) {
            json_refresh_web_token_id -> Uuid,
        }
    }

    joinable!(application_user_log_in_token -> application_user (application_user_id));
    joinable!(application_user_registration_confirmation_token -> pre_confirmed_application_user (pre_confirmed_application_user_id));
    joinable!(json_refresh_web_token -> application_user (application_user_id));
    joinable!(json_access_web_token_black_list -> json_refresh_web_token (json_refresh_web_token_id));

    allow_tables_to_appear_in_same_query!(
        application_user,
        application_user_log_in_token,
        application_user_registration_confirmation_token,
        json_refresh_web_token,
        pre_confirmed_application_user,
        json_access_web_token_black_list,
    );
}
